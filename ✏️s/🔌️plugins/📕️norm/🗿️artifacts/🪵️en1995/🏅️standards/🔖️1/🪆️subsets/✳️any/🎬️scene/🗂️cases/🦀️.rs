//! 🗂️ Catalogue cases: every document case stands in the scene as its own entity drawn from its declared geometry by the shared forms of the projection kit (the solids of its form sized by the case inputs in their stored units, a load bar or gauge pillar scaled by the load input, the grain hatch of timber, the notch void of a support notch, the fastener pattern of a joint, the profile prism of a tapered or curved member, the ply stack of a CLT panel), coloured by its live utilisation, placed by `placements` and dragged by the grips the form declares. The cases whose picture needs the timber kernel (fire sections, floor modes, assemblies) hand their data to the forms through [`entities::case_extras`]. See [`super`] and the projection kit `design_case::forms`.

use super::kit::*;
use super::layout::Layout;
use super::model::CaseRow;
use super::scene::projection_kit::design_case::forms::{self, CaseDraw, Size};
use super::scene::*;
use super::schema::checks::timber::En1995Cases;
use super::SceneInput;

pub(crate) const COLLECTION: &str = "cases";

/// 🧊️ Whether the entry of `row` declares geometry, so the case stands in the scene.
pub fn declared(row: &CaseRow) -> bool {
    forms::declared::<En1995Cases>(row).is_some()
}

/// 🎬️ Every case of the document as scene entities, notes, legends and label.
pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let ext = |case: &CaseRow, size: Size| entities::case_extras(input.document.annex, case, size);
    for row in &input.document.cases {
        if !out.wants(&row.id) {
            continue;
        }
        let Some(pose) = layout.case(&row.id) else { continue };
        let Some(CaseDraw { entities: solids, points, label, anchor, notes, legends }) = forms::draw_with::<En1995Cases>(row, pose, input.report, COLLECTION, LAYER_MEMBERS, &MaterialRole::Timber, &ext) else { continue };
        for (index, entity) in solids.into_iter().enumerate() {
            let bound: &[Vec3] = if index == 0 { &points } else { &[] };
            out.push_entity(entity, bound);
        }
        for note in notes {
            let at = match &note {
                Annotation::Label { anchor, .. } => anchor.clone(),
                _ => anchor.clone(),
            };
            out.annotate(note, &at);
        }
        for legend in legends {
            entities::Sink::legend(out, legend);
        }
        out.annotate(label, &anchor);
    }
}
