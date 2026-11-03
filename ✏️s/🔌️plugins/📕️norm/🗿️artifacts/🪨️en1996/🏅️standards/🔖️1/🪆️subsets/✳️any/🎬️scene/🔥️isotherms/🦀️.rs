//! 🔥️ Isotherms: the zone of the wall beyond the limit isotherm of the reduced-section method (EN 1996-1-2, M-91) as a 3D solid on both faces, hatched on the fire layer, bound to the reduced-section check; the depth a_θ drags from its grip.

use super::kit::*;
use super::layout::{solid_rects, Layout, WallPose, EPS};
use super::model::MasonryWall;
use super::scene::*;
use super::SceneInput;

const PROUD: f64 = 0.004;
const ZONE_OPACITY: f64 = 0.6;

/// 🧮️ The thickness left after both faces lost the isotherm depth: `t − 2 a_θ`, never negative.
pub fn residual_thickness(wall: &MasonryWall) -> f64 {
    (wall.thickness_m - 2.0 * wall.isotherm_depth_m).max(0.0)
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for wall in &input.document.walls {
        if !out.wants(&wall.id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        let (h, t, a) = (wall.height_m, wall.thickness_m, wall.isotherm_depth_m.min(0.5 * wall.thickness_m));
        if a <= EPS || h <= EPS || t <= EPS || wall.length_m <= EPS {
            continue;
        }
        let mut mesh = MeshBuilder::default();
        for (x0, x1, z0, z1) in solid_rects(wall, 0.0, h) {
            mesh.cuboid(pose, (x0, x1), (-PROUD, a), (z0, z1));
            mesh.cuboid(pose, (x0, x1), (t - a, t + PROUD), (z0, z1));
        }
        if mesh.is_empty() {
            continue;
        }
        let points = mesh.points().to_vec();
        let id = format!("{}:{}", wall.id, ISOTHERM_TAG);
        let mut spec = Spec::new(EntityKind::Solid, id.clone(), mesh.solid(), MaterialRole::Neutral, LAYER_FIRE, loc("Fire isotherm zone", "Brand-Isothermenzone"));
        spec.nested = ".isothermDepthM".into();
        let rest = residual_thickness(wall);
        spec.detail = loc(&format!("Zone beyond the limit isotherm, a_θ = {:.0} mm per face; residual thickness {:.0} mm of {:.0} mm (reduced-section method, k_θ unverified)", a * 1000.0, rest * 1000.0, t * 1000.0), &format!("Zone jenseits der Grenzisotherme, a_θ = {:.0} mm je Seite; Restdicke {:.0} mm von {:.0} mm (Querschnittsreduktion, k_θ ungeprüft)", a * 1000.0, rest * 1000.0, t * 1000.0));
        spec.binding = binding_by(input.report, |c| c.id.starts_with("en1996.m-91.") && c.id.ends_with(&format!("{}.isotherm", wall.id)));
        spec.opacity = ZONE_OPACITY;
        spec.hatch = Some(Hatch::Diagonal);
        spec.channel = Some(ColourChannel::Utilisation);
        spec.handles = super::handles::ids(input.document, layout, &id);
        out.push(spec, &points);
        let from = pose.at(0.5 * wall.length_m, 0.0, h);
        out.annotate(Annotation::Dimension { id: format!("{}:isotherm-depth", wall.id), from: from.clone(), to: pose.at(0.5 * wall.length_m, a, h), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, a), binding: None }, &from);
    }
}

/// 🕹️ The grip of the isotherm depth, dragged through the wall thickness.
pub(crate) fn handles(wall: &MasonryWall, pose: &WallPose) -> Vec<Handle> {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let entity = EntityRef::new(EntityKind::Solid, format!("{}:{}", wall.id, ISOTHERM_TAG), format!("walls[id={}]", wall.id));
    vec![super::members::grip(&entity, "depth", HandleKind::ScalarAlong { axis: pose.axis_y() }, pose.at(0.5 * l, wall.isotherm_depth_m.min(0.5 * t), 0.5 * h), GripStyle::Arrow, super::members::field(format!("walls[id={}].isothermDepthM", wall.id), Affine::IDENTITY, "m", 0.005), 0.005, (0.0, 0.5 * t), loc("Isotherm depth", "Isothermentiefe"))]
}
