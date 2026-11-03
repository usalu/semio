//! 🛡️ Live factor of safety: one label on every geotechnical entity with a check, FoS = 1 / governing utilisation of the entity, toned by its status. The label is part of the entity's fragment, so every edit that moves the governing check re-projects it.

use super::kit::*;
use super::scene::*;
use super::Ctx;

const CEILING: f64 = 10.0;

/// 🛡️ Factor of safety of a utilisation ratio: its reciprocal, unbounded above where nothing is utilised.
pub fn factor_of_safety(ratio: f64) -> f64 {
    if ratio > 1e-9 { 1.0 / ratio } else { f64::INFINITY }
}

fn rows(d: &super::model::En1997Snapshot) -> Vec<(&'static str, &str, f64)> {
    let mut rows: Vec<(&'static str, &str, f64)> = Vec::new();
    rows.extend(d.footings.iter().map(|e| ("footings", e.id.as_str(), 1.2)));
    rows.extend(d.piles.iter().map(|e| ("piles", e.id.as_str(), 1.4)));
    rows.extend(d.retaining_walls.iter().map(|e| ("retainingWalls", e.id.as_str(), e.height + 0.9)));
    rows.extend(d.slopes.iter().map(|e| ("slopes", e.id.as_str(), e.height + 0.9)));
    rows.extend(d.uplift_cases.iter().map(|e| ("upliftCases", e.id.as_str(), 1.0)));
    rows.extend(d.embedded_walls.iter().map(|e| ("embeddedWalls", e.id.as_str(), e.retained_height + 0.9)));
    rows.extend(d.anchors.iter().map(|e| ("anchors", e.id.as_str(), 1.0)));
    rows.extend(d.pile_groups.iter().map(|e| ("pileGroups", e.id.as_str(), 1.4)));
    rows.extend(d.lab_tests.iter().map(|e| ("labTests", e.id.as_str(), 1.2)));
    rows.extend(d.layer_waters.iter().map(|e| ("layerWaters", e.id.as_str(), 0.8)));
    rows
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for (collection, id, height) in rows(d) {
        if !out.own(collection, id) {
            continue;
        }
        let bound = ctx.binding(id, &[]);
        let Some(b) = bound.as_ref() else { continue };
        let fos = factor_of_safety(b.ratio);
        let (en, de) = if fos > CEILING { (format!("FoS > {CEILING:.0}"), format!("Sicherheit > {CEILING:.0}")) } else { (format!("FoS {fos:.2} (utilisation {:.2})", b.ratio), format!("Sicherheit {fos:.2} (Ausnutzung {:.2})", b.ratio)) };
        let anchor = layout.pose(id).at(0.0, 0.0, height);
        out.annotate(Annotation::Label { id: format!("{id}:fos"), anchor: anchor.clone(), text: LocalizedCopy::new(en, de), tone: tone_of(&bound) }, &anchor);
    }
}
