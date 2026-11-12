//! 📐️ Yield lines: the governing fracture pattern of the laterally loaded panel (EN 1996-1-1 Annex E coefficients by Johansen yield lines, M-31): the rigid facets (coloured by the moment utilisation) with the ridge and the diagonals to the corners of the held edges as rods on the deformed mechanism, bound to the yield-line check.

use super::scene::projection_kit::{loc};
use super::kit::*;
use super::layout::{Layout, EPS};
use super::scene::*;
use super::{masonry as m, profile, SceneInput};

const HALF: f64 = 0.012;
const OFFSET: f64 = 0.01;

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for wall in &input.document.walls {
        if !out.wants(&wall.id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        let (l, h) = (wall.length_m, wall.height_m);
        if l <= EPS || h <= EPS || wall.thickness_m <= EPS {
            continue;
        }
        let Some(lat) = profile::lateral(input.document, wall) else { continue };
        let width = lat.l_ef.min(l);
        let pattern = m::numerical_models::yield_line_pattern(lat.edges, width, h, lat.m1, lat.m2);
        if !pattern.pressure.is_finite() {
            continue;
        }
        let x0 = 0.5 * (l - width);
        let amplitude = wall.thickness_m;
        let at = |(x, z): (f64, f64)| [x0 + x, -OFFSET - amplitude * pattern.deflection(width, h, x, z), z];
        let mut mesh = MeshBuilder::default();
        for facet in pattern.facets(width, h) {
            mesh.sheet(pose, &facet.into_iter().map(at).collect::<Vec<_>>());
        }
        for (a, b) in pattern.lines(width, h) {
            mesh.rod(pose, at(a), at(b), HALF);
        }
        if mesh.is_empty() {
            continue;
        }
        let points = mesh.points().to_vec();
        let id = format!("{}:{YIELD_TAG}", wall.id);
        let mut spec = Spec::new(EntityKind::Solid, id, mesh.solid(), MaterialRole::Neutral, LAYER_PATTERNS, loc("Yield-line pattern", "Bruchlinienbild"));
        spec.detail = loc(
            &format!("Governing pattern of the {width:.2} m × {h:.2} m panel, rigid facets coloured by the moment utilisation w_Ed/w_Rd of the yield-line check (deflection drawn at wall thickness, not to scale), {} ridge, collapse pressure w_Rd = {:.0} Pa against w_Ed = {:.0} Pa (edge coefficients unverified for fixed edges)", if pattern.transposed { "vertical" } else { "horizontal" }, pattern.pressure, lat.w_ed),
            &format!("Maßgebendes Bruchlinienbild der {width:.2} m × {h:.2} m Wand, starre Scheiben nach der Momentenauslastung w_Ed/w_Rd des Bruchlinien-Nachweises gefärbt (Verformung in Wanddicke, nicht maßstäblich), {} Grat, Bruchlast w_Rd = {:.0} Pa gegenüber w_Ed = {:.0} Pa (Randbeiwerte eingespannter Ränder ungeprüft)", if pattern.transposed { "vertikaler" } else { "horizontaler" }, pattern.pressure, lat.w_ed),
        );
        spec.binding = binding_of(input.report, &format!("en1996.m-31.w-rd.{}.{}", wall.id, lat.load_case));
        spec.channel = Some(ColourChannel::Utilisation);
        spec.handles = super::handles::ids(input.document, layout, &spec.id);
        out.push(spec, &points);
    }
}

#[cfg(test)]
mod tests {
    use super::m::numerical_models::{yield_line_pattern, Edges};

    #[test]
    fn a_square_panel_on_four_simple_edges_breaks_along_its_diagonals() {
        let pattern = yield_line_pattern(Edges::parse("SSSS").unwrap(), 4.0, 4.0, 1000.0, 1000.0);
        assert!((pattern.pressure - 24.0 * 1000.0 / 16.0).abs() < 1e-6 * pattern.pressure, "{}", pattern.pressure);
        let lines = pattern.lines(4.0, 4.0);
        assert_eq!(lines.len(), 4, "{lines:?}");
        let area = |f: &[(f64, f64)]| 0.5 * (0..f.len()).map(|i| f[i].0 * f[(i + 1) % f.len()].1 - f[(i + 1) % f.len()].0 * f[i].1).sum::<f64>().abs();
        let facets = pattern.facets(4.0, 4.0);
        assert_eq!(facets.len(), 4);
        assert!((facets.iter().map(|f| area(f)).sum::<f64>() - 16.0).abs() < 1e-6);
        assert!((pattern.deflection(4.0, 4.0, 2.0, 2.0) - 1.0).abs() < 0.05 && pattern.deflection(4.0, 4.0, 0.0, 2.0) == 0.0);
        assert!(lines.iter().all(|(a, b)| (a.0 - 2.0).abs() < 0.05 && (a.1 - 2.0).abs() < 0.05 || (b.0 - 2.0).abs() < 0.05 && (b.1 - 2.0).abs() < 0.05));
    }
}
