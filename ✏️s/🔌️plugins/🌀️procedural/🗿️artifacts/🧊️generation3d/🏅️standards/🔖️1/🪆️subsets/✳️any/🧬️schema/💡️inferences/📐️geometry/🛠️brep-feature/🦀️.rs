//! 🛠️ The `brep.feature` widgets: fillet, chamfer, shell, draft, offset, thicken and defeature as stepped lane J operation jobs over a fresh kernel session.
//!
//! Face and edge selections carry the persistent labels of the input shape value; they resolve through the imported session and fault with `generation3d.geometry.selection-stale` when a label no longer exists.

use super::phased_job::{launch, Pipeline, Work};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::brep::engine::BrepOperation;

fn feature<P: Send + 'static>(kind: &Kind, inputs: &WidgetInputs, port: &'static str, read: impl FnOnce(&WidgetInputs) -> Result<P, WidgetFault>, plan: impl FnOnce(&mut Work, P) -> Result<BrepOperation, WidgetFault> + Send + 'static) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let shape = inputs.shape(port)?.clone();
        let parameters = read(inputs)?;
        Ok(Pipeline::new(kind).import(&shape).operation(move |work| plan(work, parameters)).exported("shape"))
    })
}

fn selected(inputs: &WidgetInputs, port: &str) -> Result<SelectionValue, WidgetFault> {
    inputs.selection(port).cloned()
}

fn optionally_selected(inputs: &WidgetInputs, port: &str) -> Result<Option<SelectionValue>, WidgetFault> {
    match inputs.get(port) {
        None => Ok(None),
        Some(_) => selected(inputs, port).map(Some),
    }
}

fn fillet(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    feature(kind, &inputs, "shape", |inputs| inputs.number("radius"), |work, radius| Ok(BrepOperation::Fillet { shape: work.handle(0)?, radius }))
}

fn fillet_edges(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    feature(kind, &inputs, "shape", |inputs| Ok((inputs.number("radius")?, selected(inputs, "edges")?)), |work, (radius, edges)| {
        let edges = work.pick(0, &edges, SelectionKind::Edge, "edges")?;
        Ok(BrepOperation::FilletEdges { shape: work.handle(0)?, edges, radius })
    })
}

fn fillet_variable(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    feature(kind, &inputs, "shape", |inputs| Ok((inputs.number("radiusStart")?, inputs.number("radiusEnd")?)), |work, (radius_start, radius_end)| Ok(BrepOperation::FilletVariable { shape: work.handle(0)?, radius_start, radius_end }))
}

fn chamfer(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    feature(kind, &inputs, "shape", |inputs| inputs.number("distance"), |work, distance| Ok(BrepOperation::Chamfer { shape: work.handle(0)?, distance }))
}

fn chamfer_edges(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    feature(kind, &inputs, "shape", |inputs| Ok((inputs.number("distance")?, selected(inputs, "edges")?)), |work, (distance, edges)| {
        let edges = work.pick(0, &edges, SelectionKind::Edge, "edges")?;
        Ok(BrepOperation::ChamferEdges { shape: work.handle(0)?, edges, distance })
    })
}

fn chamfer_asymmetric(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    feature(kind, &inputs, "shape", |inputs| Ok((inputs.number("d1")?, inputs.number("d2")?)), |work, (first, second)| Ok(BrepOperation::ChamferAsymmetric { shape: work.handle(0)?, first, second }))
}

fn shell(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    feature(kind, &inputs, "shape", |inputs| Ok((inputs.number("thickness")?, optionally_selected(inputs, "openFaces")?)), |work, (thickness, open)| {
        let open_faces = match open {
            Some(selection) => work.pick(0, &selection, SelectionKind::Face, "openFaces")?,
            None => Vec::new(),
        };
        Ok(BrepOperation::Shell { shape: work.handle(0)?, thickness, open_faces })
    })
}

fn draft(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    feature(kind, &inputs, "shape", |inputs| Ok((selected(inputs, "faces")?, inputs.vector("pullDirection")?, inputs.point("neutralPoint")?, inputs.number("angle")?)), |work, (faces, pull_direction, neutral_point, angle)| {
        let faces = work.pick(0, &faces, SelectionKind::Face, "faces")?;
        Ok(BrepOperation::Draft { shape: work.handle(0)?, faces, pull_direction, neutral_point, angle })
    })
}

fn offset_solid(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    feature(kind, &inputs, "shape", |inputs| inputs.number("distance"), |work, distance| Ok(BrepOperation::OffsetSolid { shape: work.handle(0)?, distance }))
}

fn thicken(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    feature(kind, &inputs, "face", |inputs| inputs.number("thickness"), |work, thickness| Ok(BrepOperation::ThickenFace { face: work.handle(0)?, thickness }))
}

fn defeature(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    feature(kind, &inputs, "shape", |inputs| selected(inputs, "faces"), |work, faces| {
        let faces = work.pick(0, &faces, SelectionKind::Face, "faces")?;
        Ok(BrepOperation::Defeature { shape: work.handle(0)?, faces })
    })
}

/// 🗃️ Every `brep.feature` kind and the compute that starts its job.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "brep.feature.fillet", start: fillet },
    ComputeEntry { id: "brep.feature.filletEdges", start: fillet_edges },
    ComputeEntry { id: "brep.feature.filletVariable", start: fillet_variable },
    ComputeEntry { id: "brep.feature.chamfer", start: chamfer },
    ComputeEntry { id: "brep.feature.chamferEdges", start: chamfer_edges },
    ComputeEntry { id: "brep.feature.chamferAsymmetric", start: chamfer_asymmetric },
    ComputeEntry { id: "brep.feature.shell", start: shell },
    ComputeEntry { id: "brep.feature.draft", start: draft },
    ComputeEntry { id: "brep.feature.offsetSolid", start: offset_solid },
    ComputeEntry { id: "brep.feature.thicken", start: thicken },
    ComputeEntry { id: "brep.feature.defeature", start: defeature },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
