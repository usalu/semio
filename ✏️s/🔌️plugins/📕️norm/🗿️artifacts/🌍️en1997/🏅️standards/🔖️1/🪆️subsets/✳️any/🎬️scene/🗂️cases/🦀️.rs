//! 🗂️ Design cases in 3D: every catalogue case stands in the ground model as its own entity drawn from its declared geometry by the generic forms of the projection kit (footing on soil layers, pile and pile group, retaining and sheet walls, slope, soil block with rod, ground anchor, seepage path, pumped well, and the joint and symbol forms), sized by the case inputs in their stored units, coloured by its live utilisation, placed by `placements` like every other entity and dragged by the grips the form declares. See [`super`] and the projection kit `design_case::forms`.

use super::kit::*;
use super::layout::Pose;
use super::scene::projection_kit::design_case::forms::{self, CaseDraw, CasePose};
use super::scene::*;
use super::Ctx;
use checks::catalogue::En1997Cases;
use norm::design_case::CaseRow;

pub const COLLECTION: &str = "cases";

/// 🧊️ Whether the entry of `row` declares geometry, so the case stands in the scene.
pub fn declared(row: &CaseRow) -> bool {
    forms::declared::<En1997Cases>(row).is_some()
}

/// 📍️ The plan rectangle `[x0, x1, y0, y1]` a case occupies around its origin, with margin; `None` when the entry declares no geometry.
pub fn rect(row: &CaseRow) -> Option<[f64; 4]> {
    forms::extent::<En1997Cases>(row)
}

/// 📍️ The pose of a case as the case forms take it.
pub fn case_pose(pose: &Pose) -> CasePose {
    CasePose { origin: pose.origin.clone(), yaw: pose.yaw }
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for row in &d.cases {
        if !out.own(COLLECTION, &row.id) {
            continue;
        }
        let pose = case_pose(&layout.pose(&row.id));
        let Some(CaseDraw { entities, points, label, anchor, notes, .. }) = forms::draw::<En1997Cases>(row, &pose, report, COLLECTION, LAYER_CASES, &MaterialRole::Concrete) else { continue };
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
