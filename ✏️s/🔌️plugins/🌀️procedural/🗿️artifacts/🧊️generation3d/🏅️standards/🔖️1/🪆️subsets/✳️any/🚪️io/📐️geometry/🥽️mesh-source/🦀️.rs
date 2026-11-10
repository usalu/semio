//! 🛂️ Native polygon-source admission for the catalogue construction operation.
use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use semio_s_artifact_procedural_3d_geometry_compute::ComputeEntry;
use semio_s_artifact_procedural_3d_geometry_compute::StepContext;
use semio_s_artifact_procedural_3d_geometry_compute::WidgetJob;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_s_artifact_procedural_3d_mesh_support::from_source;
use semio_s_artifact_procedural_3d_mesh_support::import_fault;
use semio_s_artifact_procedural_3d_mesh_support::Parse;
use semio_s_artifact_procedural_3d_mesh_support::BATCH;
use semio_framework_mesh_engine::io::text::PolygonSourcePreparation;

fn construct(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    from_source(kind, move || {
        let text = inputs.text("data")?.to_string();
        let mut preparation = PolygonSourcePreparation::new();
        Ok(Box::new(move |fuel: usize, cx: &mut StepContext<'_>| {
            let grant = preparation.cold_grant(&text, fuel.saturating_mul(BATCH), fuel.saturating_mul(4096)).map_err(import_fault)?;
            let Some((step, _)) = cx.with_original_effect(cx.retained_grant(), |_, effect| preparation.step(&text, grant, effect)).map_err(|error| import_fault(error.to_string()))? else { return Ok(None) };
            step.map(|step| step.source).map_err(import_fault)
        }) as Parse)
    })
}

/// 🗃️ Native source admission registrations.
pub const COMPUTES:&[ComputeEntry]=&[ComputeEntry {id:"mesh.primitive.construct",start:construct}];
