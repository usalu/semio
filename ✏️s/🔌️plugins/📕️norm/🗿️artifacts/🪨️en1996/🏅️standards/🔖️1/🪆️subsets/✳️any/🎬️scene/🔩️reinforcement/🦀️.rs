//! 🔩️ Reinforcement: the vertical bars and the bed-joint reinforcement layers of reinforced walls, with bar diameters derived from the declared steel areas.

use super::scene::projection_kit::{loc};
use super::kit::*;
use super::layout::{Layout, EPS};
use super::scene::*;
use super::SceneInput;

const BAR_SPACING: f64 = 0.6;
const LAYER_SPACING: f64 = 0.4;
const MIN_DIAMETER: f64 = 0.004;
const MAX_DIAMETER: f64 = 0.032;

/// 🔩️ The diameter of one bar of area `a` in m, limited to the range of rebar stock.
pub fn bar_diameter(a: f64) -> f64 {
    (4.0 * a.max(0.0) / std::f64::consts::PI).sqrt().clamp(MIN_DIAMETER, MAX_DIAMETER)
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for wall in input.document.walls.iter().filter(|w| w.reinforced) {
        if !out.wants(&wall.id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
        if l <= EPS || h <= EPS || t <= EPS {
            continue;
        }
        let margin = (0.1_f64).min(0.25 * l).min(0.25 * h);
        let binding = binding_of(input.report, &format!("en1996.6.6.reinforced.{}", wall.id));
        if wall.as_vertical_m2 > EPS {
            let n = (l / BAR_SPACING).ceil().max(1.0) as u32 + 1;
            let diameter = bar_diameter(wall.as_vertical_m2 / f64::from(n));
            let spacing = (l - 2.0 * margin) / f64::from(n - 1);
            let (from, to) = (pose.at(margin, 0.5 * t, margin), pose.at(margin, 0.5 * t, h - margin));
            let row = Path3::Polyline(Polyline3 { points: vec![from.clone(), pose.at(l - margin, 0.5 * t, margin)] });
            let geometry = Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Rebar { bend: None }, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter, head: HeadShape::None, pattern: Some(Pattern::Along { path: row, spacing }), hole_clearance: None });
            let mut spec = Spec::new(EntityKind::Fastener, format!("{}:rebar-v", wall.id), geometry, MaterialRole::Rebar, LAYER_REINFORCEMENT, loc("Vertical reinforcement", "Vertikale Bewehrung"));
            spec.detail = loc(&format!("{n} bars of {:.0} mm, A_s = {:.0} mm²", diameter * 1000.0, wall.as_vertical_m2 * 1e6), &format!("{n} Stäbe zu {:.0} mm, A_s = {:.0} mm²", diameter * 1000.0, wall.as_vertical_m2 * 1e6));
            spec.binding = binding.clone();
            out.push(spec, &[from, to]);
        }
        if wall.as_horizontal_m2 > EPS {
            let layers = (h / LAYER_SPACING).floor().max(1.0) as u32;
            let diameter = bar_diameter(wall.as_horizontal_m2 / f64::from(layers));
            let (from, to) = (pose.at(margin, 0.5 * t, LAYER_SPACING.min(h - margin)), pose.at(l - margin, 0.5 * t, LAYER_SPACING.min(h - margin)));
            let top = (f64::from(layers) * LAYER_SPACING).min(h - margin);
            let column = Path3::Polyline(Polyline3 { points: vec![from.clone(), pose.at(margin, 0.5 * t, top.max(LAYER_SPACING.min(h - margin) + 1e-3))] });
            let spacing = ((top - LAYER_SPACING.min(h - margin)).max(1e-3)) / f64::from(layers.max(2) - 1);
            let geometry = Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Rebar { bend: None }, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter, head: HeadShape::None, pattern: Some(Pattern::Along { path: column, spacing }), hole_clearance: None });
            let mut spec = Spec::new(EntityKind::Fastener, format!("{}:rebar-h", wall.id), geometry, MaterialRole::Rebar, LAYER_REINFORCEMENT, loc("Bed-joint reinforcement", "Lagerfugenbewehrung"));
            spec.detail = loc(&format!("{layers} layers of {:.0} mm, A_s = {:.0} mm²", diameter * 1000.0, wall.as_horizontal_m2 * 1e6), &format!("{layers} Lagen zu {:.0} mm, A_s = {:.0} mm²", diameter * 1000.0, wall.as_horizontal_m2 * 1e6));
            spec.binding = binding;
            out.push(spec, &[from, to]);
        }
    }
}
