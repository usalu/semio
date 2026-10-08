//! 🛂️ Native polygon-source admission for the catalogue construction operation.
use crate::standards::v1::subsets::any::schema::inferences::geometry::{prelude::*,registry::mesh_support::{from_source,import_fault,Parse,BATCH}};
use semio_framework_mesh_engine::io::text::PolygonSourcePreparation;

fn construct(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    from_source(kind, move || {
        let text = inputs.text("data")?.to_string();
        let mut preparation = PolygonSourcePreparation::new();
        Ok(Box::new(move |fuel: usize| preparation.step(&text, semio_framework_mesh_engine::io::text::PolygonSourceGrant{maximum_units:fuel.saturating_mul(BATCH),maximum_projection_bytes:fuel.saturating_mul(4096),retirement:semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:4096,maximum_release_bytes:1024*1024,maximum_depth:256}}).map(|step|step.source).map_err(import_fault)) as Parse)
    })
}

/// 🗃️ Native source admission registrations.
pub const COMPUTES:&[ComputeEntry]=&[ComputeEntry {id:"mesh.primitive.construct",start:construct}];
