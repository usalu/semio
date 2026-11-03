//! 🗂️ Design cases in 3D: every catalogue case stands in the scene as its own entity drawn from its declared geometry (the solids of its form sized by the dimension inputs, a load bar or gauge pillar scaled by the load input), coloured by its live utilisation, placed by `placements` and dragged by grips that write the same inputs back. See [`super`] and the projection kit `design_case::forms`.

use super::checks::catalogue::En1991Cases;
use super::kit::*;
use super::scene::projection_kit::design_case::forms::{self, CaseDraw};
use super::scene::*;
use super::Ctx;

pub const COLLECTION: &str = "cases";

/// 🎬️ Every case of the document as scene entities with their label.
pub fn project(ctx: &Ctx, out: &mut Out) {
    for row in &ctx.input.document.cases {
        let Some(pose) = ctx.layout.case(&row.id) else { continue };
        if !out.own(&row.id) {
            continue;
        }
        let Some(CaseDraw { entities, points, label, anchor, notes, .. }) = forms::draw::<En1991Cases>(row, pose, ctx.input.report, COLLECTION, LAYER_CASES, &MaterialRole::Neutral) else { continue };
        for (index, entity) in entities.into_iter().enumerate() {
            let bound: &[Vec3] = if index == 0 { &points } else { &[] };
            out.push_entity(entity, bound);
        }
        for note in notes {
            out.annotate(note, &anchor);
        }
        out.annotate(label, &anchor);
    }
}
