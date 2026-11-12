//! 🏹️ Arching: the thrust line of the three-hinge arch of a wall held at floor and base (EN 1996-1-1 6.3.2, M-32): the parabola from the back face at the supports to the front face at mid-height with the three compression blocks, bound to the arching check. The block depth is derived so that the arch statics reproduce the catalogue capacity.

use super::scene::projection_kit::{loc};
use super::kit::*;
use super::layout::{Layout, EPS};
use super::scene::*;
use super::{masonry as m, profile, SceneInput};

const SAMPLES: usize = 16;
const HALF: f64 = 0.01;
const BLOCK_WIDTH: f64 = 1.0;
const BLOCK_HEIGHT: f64 = 0.05;

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for wall in &input.document.walls {
        if !out.wants(&wall.id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
        if l <= EPS || h <= EPS || t <= EPS {
            continue;
        }
        let Some(lat) = profile::lateral(input.document, wall) else { continue };
        let Some(prof) = profile::governing(input.document, wall) else { continue };
        let xc = 0.5 * l;
        let line = m::numerical_models::arch_thrust_line(t, h, SAMPLES);
        let block = m::numerical_models::arch_block_depth(t);
        let mut mesh = MeshBuilder::default();
        for pair in line.windows(2) {
            mesh.rod(pose, [xc, t - pair[0].1, pair[0].0], [xc, t - pair[1].1, pair[1].0], HALF);
        }
        let width = BLOCK_WIDTH.min(l);
        let x = (xc - 0.5 * width, xc + 0.5 * width);
        let step = BLOCK_HEIGHT.min(0.1 * h);
        mesh.cuboid(pose, x, (t - block, t), (0.0, step));
        mesh.cuboid(pose, x, (t - block, t), (h - step, h));
        mesh.cuboid(pose, x, (0.0, block), (0.5 * h - 0.5 * step, 0.5 * h + 0.5 * step));
        let points = mesh.points().to_vec();
        let id = format!("{}:{ARCH_TAG}", wall.id);
        let capacity = m::numerical_models::arching_pressure(prof.f_d, t, h);
        let mut spec = Spec::new(EntityKind::Solid, id, mesh.solid(), MaterialRole::Load { kind: LoadKind::Permanent }, LAYER_PATTERNS, loc("Arch thrust line", "Stützlinie des Gewölbes"));
        spec.detail = loc(
            &format!("Three-hinge arch of span {h:.2} m in {:.0} mm masonry: compression blocks of {:.1} mm, rise {:.1} mm, capacity q_lat,d = f_d (t/ℓ_a)² = {capacity:.0} Pa against w_Ed = {:.0} Pa (block depth derived from the catalogue capacity, unverified against the print)", t * 1000.0, block * 1000.0, (t - block) * 1000.0, lat.w_ed),
            &format!("Dreigelenkbogen mit Spannweite {h:.2} m in {:.0} mm Mauerwerk: Druckblöcke {:.1} mm, Stich {:.1} mm, Tragfähigkeit q_lat,d = f_d (t/ℓ_a)² = {capacity:.0} Pa gegenüber w_Ed = {:.0} Pa (Blocktiefe aus der Katalogtragfähigkeit abgeleitet, am Normtext ungeprüft)", t * 1000.0, block * 1000.0, (t - block) * 1000.0, lat.w_ed),
        );
        spec.binding = binding_of(input.report, &format!("en1996.m-32.q-lat.{}.{}.arch", wall.id, lat.load_case));
        spec.channel = Some(ColourChannel::Utilisation);
        spec.handles = super::handles::ids(input.document, layout, &spec.id);
        out.push(spec, &points);
        let from = pose.at(xc, t - 0.5 * block, 0.0);
        out.annotate(Annotation::Dimension { id: format!("{}:arch-rise", wall.id), from: from.clone(), to: pose.at(xc, 0.5 * block, 0.0), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, t - block), binding: None }, &from);
    }
}

#[cfg(test)]
mod tests {
    use super::m::numerical_models::{arch_block_depth, arch_thrust_line, arching_pressure};

    #[test]
    fn the_arch_statics_reproduce_the_catalogue_capacity() {
        let (t, span, fd) = (0.24, 2.6, 3.0e6);
        let d = arch_block_depth(t);
        let q = 8.0 * fd * d * (t - d) / (span * span);
        assert!((q - arching_pressure(fd, t, span)).abs() < 1e-9 * q);
        let line = arch_thrust_line(t, span, 4);
        assert!((line[0].1 - 0.5 * d).abs() < 1e-12 && (line[2].1 - (t - 0.5 * d)).abs() < 1e-12 && (line[4].1 - 0.5 * d).abs() < 1e-12);
    }
}
