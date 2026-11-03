//! 🗂️ Design cases in 3D: every catalogue case stands in the scene as its own entity drawn from its declared geometry (the solids of its form sized by the dimension inputs, a load bar or gauge pillar scaled by the load input), coloured by its live utilisation, placed by `placements` and dragged by grips that write the same inputs back. See [`super`] and the projection kit `design_case::forms`.

use super::checks::En1992Cases;
use super::kit::*;
use super::layout::Layout;
use super::model::CaseRow;
use super::scene::projection_kit::design_case::forms::{self, CaseDraw, Size};
use super::scene::*;
use super::SceneInput;

pub(crate) const COLLECTION: &str = "cases";

/// 🧊️ Whether the entry of `row` declares geometry, so the case stands in the scene.
pub(crate) fn declared(row: &CaseRow) -> bool {
    forms::declared::<En1992Cases>(row).is_some()
}

/// 🎬️ Every case of the document as scene entities, label and grips.
pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let ext = |row: &CaseRow, size: Size| super::ties::extras(input.document.annex, row, size);
    for row in &input.document.cases {
        let (Some(pose), true) = (layout.case(&row.id), out.wants(&row.id)) else { continue };
        let Some(CaseDraw { entities, points, label, anchor, notes, .. }) = forms::draw_with::<En1992Cases>(row, pose, input.report, COLLECTION, LAYER_CASES, &MaterialRole::Concrete, &ext) else { continue };
        for (index, entity) in entities.into_iter().enumerate() {
            let bound: &[Vec3] = if index == 0 { &points } else { &[] };
            out.push_entity(entity, bound);
        }
        for note in notes {
            out.annotate(note, &anchor);
        }
        out.annotate(label, &anchor);
        out.handles.extend(forms::handles_with::<En1992Cases>(row, pose, COLLECTION, &ext));
    }
}
