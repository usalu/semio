//! 🐚️ The `brep.topology` widgets: vertices, decomposition into vertices, edges, faces and shells, compounds, persistent labels, sewing, healing and NURBS conversion.
//!
//! Selected edges and faces are persistent labels of the input value and resolve through the imported session (`generation3d.geometry.selection-stale` when a label is gone). Every component of a decomposition is exported one per unit of fuel, so a large shape never holds the interactive turn.

use super::phased_job::{launch, Pipeline, Work};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::brep::engine::{GeometryHandle, GeometryKind};

const EMPTY: &str = "generation3d.geometry.topology-empty";
const LABEL_MISSING: &str = "generation3d.geometry.label-missing";

fn kernel(error: semio_framework_3d::brep::engine::BrepError) -> WidgetFault {
    kernel_fault(&error)
}

fn solids_of(work: &mut Work, handle: &GeometryHandle, compound: bool) -> Result<Vec<GeometryHandle>, WidgetFault> {
    if compound {
        work.session.brep().explode_sync(handle).map_err(kernel)
    } else {
        Ok(vec![handle.clone()])
    }
}

fn list(values: Vec<GeometryValue>) -> GeometryValue {
    GeometryValue::List(values)
}

fn vertex(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let point = inputs.point("point")?;
        Ok(Pipeline::new(kind).once(move |work| {
            let handle = work.session.brep().vertex_sync(point).map_err(kernel)?;
            work.groups = vec![vec![handle]];
            Ok(())
        }).exported("shape"))
    })
}

fn deconstruct(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let shape = inputs.shape("shape")?.clone();
        let edges = match inputs.get("edges") {
            Some(_) => Some(inputs.selection("edges")?.clone()),
            None => None,
        };
        let faces = match inputs.get("faces") {
            Some(_) => Some(inputs.selection("faces")?.clone()),
            None => None,
        };
        let (has_edges, has_faces) = (edges.is_some(), faces.is_some());
        Ok(Pipeline::new(kind).import(&shape).once(move |work| {
            let handle = work.handle(0)?;
            let topology = work.session.brep().deconstruct_sync(&handle).map_err(kernel)?;
            let mut groups = vec![topology.vertices, topology.edges, topology.faces, topology.shells];
            if let Some(edges) = edges {
                groups.push(work.pick(0, &edges, SelectionKind::Edge, "edges")?);
            }
            if let Some(faces) = faces {
                groups.push(work.pick(0, &faces, SelectionKind::Face, "faces")?);
            }
            work.groups = groups;
            Ok(())
        }).export_groups().finish(move |work| {
            let mut groups = std::mem::take(&mut work.values).into_iter();
            let mut next = || groups.next().map(list).ok_or_else(|| WidgetFault::new(EMPTY, "The decomposition produced no components.", "Die Zerlegung lieferte keine Bestandteile."));
            let mut result = outputs([("vertices", next()?), ("edges", next()?), ("faces", next()?), ("shells", next()?)]);
            if has_edges {
                result.insert("selectedEdges".to_string(), next()?);
            }
            if has_faces {
                result.insert("selectedFaces".to_string(), next()?);
            }
            Ok(result)
        }))
    })
}

fn shells(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let solid = inputs.shape("solid")?.clone();
        let compound = solid.kind() == GeometryKind::Compound;
        Ok(Pipeline::new(kind).import(&solid).once(move |work| {
            let handle = work.handle(0)?;
            let mut all = Vec::new();
            for member in solids_of(work, &handle, compound)? {
                all.extend(work.session.brep().solid_shells_sync(&member).map_err(kernel)?);
            }
            work.groups = vec![all];
            Ok(())
        }).export_groups().finish(|work| Ok(outputs([("shells", list(work.values.first().cloned().unwrap_or_default()))]))))
    })
}

fn compound(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let solids: Vec<_> = inputs.shapes("solids")?.into_iter().cloned().collect();
        let mut pipeline = Pipeline::new(kind);
        for solid in &solids {
            pipeline = pipeline.import(solid);
        }
        let compounds: Vec<bool> = solids.iter().map(|solid| solid.kind() == GeometryKind::Compound).collect();
        Ok(pipeline.once(move |work| {
            let mut members = Vec::new();
            for (handle, is_compound) in work.handles().into_iter().zip(compounds) {
                members.extend(solids_of(work, &handle, is_compound)?);
            }
            if members.is_empty() {
                return Err(WidgetFault::new(EMPTY, "The inputs hold no solid to group.", "Die Eingaben enthalten keinen Körper zum Gruppieren.").at("solids"));
            }
            let grouped = work.session.brep().compound_sync(&members).map_err(kernel)?;
            work.groups = vec![vec![grouped]];
            Ok(())
        }).exported("shape"))
    })
}

fn explode(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let compound = inputs.shape("compound")?.clone();
        Ok(Pipeline::new(kind).import(&compound).once(|work| {
            let handle = work.handle(0)?;
            let members = work.session.brep().explode_sync(&handle).map_err(kernel)?;
            work.groups = vec![members];
            Ok(())
        }).export_groups().finish(|work| Ok(outputs([("solids", list(work.values.first().cloned().unwrap_or_default()))]))))
    })
}

fn label(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| {
        let label = inputs.shape("shape")?.label().ok_or_else(|| WidgetFault::new(LABEL_MISSING, "The shape carries no persistent label.", "Die Form trägt keine dauerhafte Bezeichnung.").at("shape"))?;
        Ok(outputs([("label", GeometryValue::Text(label.0.to_string()))]))
    })())
}

fn sew(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let faces: Vec<_> = inputs.shapes("faces")?.into_iter().cloned().collect();
        let tolerance = inputs.number("tolerance")?;
        let mut pipeline = Pipeline::new(kind);
        for face in &faces {
            pipeline = pipeline.import(face);
        }
        Ok(pipeline.once(move |work| {
            let sewn = work.session.brep().sew_faces_sync(&work.handles(), tolerance).map_err(kernel)?;
            work.groups = vec![vec![sewn]];
            Ok(())
        }).exported("shape"))
    })
}

fn heal(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let shape = inputs.shape("shape")?.clone();
        let tolerance = inputs.number("tolerance")?;
        Ok(Pipeline::new(kind).import(&shape).once(move |work| {
            let handle = work.handle(0)?;
            let healed = work.session.brep().heal_solid_sync(&handle, tolerance).map_err(kernel)?;
            work.groups = vec![vec![healed]];
            Ok(())
        }).exported("shape"))
    })
}

fn convert_to_nurbs(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    launch(kind, || {
        let shape = inputs.shape("shape")?.clone();
        let compound = shape.kind() == GeometryKind::Compound;
        Ok(Pipeline::new(kind).import(&shape).once(move |work| {
            let handle = work.handle(0)?;
            for member in solids_of(work, &handle, compound)? {
                work.session.brep().convert_to_nurbs_sync(&member).map_err(kernel)?;
            }
            work.groups = vec![vec![handle]];
            Ok(())
        }).exported("shape"))
    })
}

/// 🗃️ Every `brep.topology` kind and the compute that starts its job.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "brep.topology.vertex", start: vertex },
    ComputeEntry { id: "brep.topology.deconstruct", start: deconstruct },
    ComputeEntry { id: "brep.topology.shells", start: shells },
    ComputeEntry { id: "brep.topology.compound", start: compound },
    ComputeEntry { id: "brep.topology.explode", start: explode },
    ComputeEntry { id: "brep.topology.label", start: label },
    ComputeEntry { id: "brep.topology.sew", start: sew },
    ComputeEntry { id: "brep.topology.heal", start: heal },
    ComputeEntry { id: "brep.topology.convertToNurbs", start: convert_to_nurbs },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
