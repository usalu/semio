//! 🔪️ Cuts one selected mesh face with two point controls and a retained graph edit.
use crate::editor::generation3d::{config::{Generation3dConfig, Generation3dConfigMutation}, edit_mesh_selection::{mesh_edit_emit, mesh_operation_rows}, selection::{component_group, DOMAIN}};
use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::WidgetInputValue;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, InteractionWrite};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "knife-mesh-selection")]
pub struct KnifeMeshSelection { pub start: [f64; 3], pub end: [f64; 3] }

/// 📍️ Language-neutral knife parameters resolved from a single selected face.
#[derive(Clone, Debug, PartialEq)]
pub struct KnifeParameters { pub face: u32, pub start: [f64; 3], pub end: [f64; 3] }

pub fn parameters(payload: &KnifeMeshSelection, ids: &[String]) -> Result<KnifeParameters, String> {
    let (target, components) = component_group(ids)?;
    if target.index != 0 || target.granularity != "face" || components.len() != 1 { return Err("Select exactly one face of a single mesh".into()); }
    if payload.start.iter().chain(&payload.end).any(|value| !value.is_finite() || value.abs() > f32::MAX as f64) { return Err("Knife points must be finite mesh coordinates".into()); }
    if payload.start.map(|value| value as f32) == payload.end.map(|value| value as f32) { return Err("Knife start and end must be different points".into()); }
    Ok(KnifeParameters { face: components[0], start: payload.start, end: payload.end })
}

/// 🎛️ The inputs a knife cut sets on its operator: the face and the two cut points.
pub fn inputs(payload: &KnifeMeshSelection, ids: &[String]) -> Result<Vec<(&'static str, WidgetInputValue)>, String> {
    let cut = parameters(payload, ids)?;
    Ok(vec![("face", WidgetInputValue::Number(f64::from(cut.face))), ("start", WidgetInputValue::Point(cut.start)), ("end", WidgetInputValue::Point(cut.end))])
}

/// 🧾️ The rows ONE knife cut of the face `ids` is on `host_snapshot`, and the inserted operator's id.
pub fn cut_rows(payload: &KnifeMeshSelection, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, ids: &[String]) -> Result<(String, Vec<Generation3dMutation>), String> {
    mesh_operation_rows(host_snapshot, "knifeCut", "face", ids, inputs(payload, ids)?)
}

/// 🔪️ ONE knife cut as ONE tool transaction of `<app>#knifeMeshSelection` ([`mesh_edit_emit`]).
pub fn apply_selected(payload: &KnifeMeshSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, ids: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let (id, rows) = cut_rows(payload, &doc.snapshot.host_snapshot, ids).map_err(Fault::from)?;
    Ok(Emit { interaction_writes: vec![InteractionWrite::replace(DOMAIN, "face", std::iter::empty::<String>()), InteractionWrite::replace("graph", "node", [id])], ..mesh_edit_emit("knifeMeshSelection", doc, rows) })
}

pub fn handle(payload: &KnifeMeshSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    apply_selected(payload, doc, &[])
}
