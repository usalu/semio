//! 🗂️ Design cases in 3D: every catalogue case stands in the scene as its own entity drawn from its declared geometry (the solids of its form sized by the dimension inputs, a load bar or gauge pillar scaled by the load input), coloured by its live utilisation, placed by `placements` and dragged by grips that write the same inputs back. See [`super`] and the projection kit `design_case::forms`.

use super::checks::catalogue::En1990Cases;
use super::kit::*;
use super::layout::Layout;
use super::scene::projection_kit::design_case::forms::{self, CaseDraw, CasePose};
use super::scene::*;
use super::SceneInput;
use super::model::CaseRow;

pub(crate) const COLLECTION: &str = "cases";

/// 🧱️ The role of the solids of a case.
pub(crate) const ROLE: MaterialRole = MaterialRole::Neutral;

/// 🎬️ The drawing of one case in the pose; `None` when its entry declares no geometry.
pub(crate) fn draw(input: &SceneInput, row: &CaseRow, pose: &CasePose) -> Option<CaseDraw> {
    forms::draw::<En1990Cases>(row, pose, input.report, COLLECTION, LAYER_CASES, &ROLE)
}

/// 🎬️ Every case of the document as scene entities with their label.
pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for row in &input.document.cases {
        let (Some(pose), true) = (layout.case(&row.id), out.wants(&row.id)) else { continue };
        let Some(drawn) = draw(input, row, pose) else { continue };
        let CaseDraw { entities, points, label, anchor, notes, .. } = drawn;
        for (index, entity) in entities.into_iter().enumerate() {
            let bound: &[Vec3] = if index == 0 { &points } else { &[] };
            out.push_entity(entity, COLLECTION, bound);
        }
        for note in notes {
            out.annotate(note, &anchor);
        }
        out.annotate(label, &anchor);
    }
}
