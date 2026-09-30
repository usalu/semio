//! 🔪️ Cuts one selected mesh face with two point controls and a retained graph edit.
use crate::editor::generation3d::{config::{Generation3dConfig, Generation3dConfigMutation}, edit_mesh_selection::insert_mesh_operation, selection::{component_group, DOMAIN}};
use crate::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host, mutations::text::Generation3dMutation};
use crate::Generation3dSnapshot;
use semio_framework_os_flow::{FlowEvalSession, FlowHost};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, InteractionWrite};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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

pub fn insert_operation(host: &mut FlowHost, payload: &KnifeMeshSelection, ids: &[String]) -> Result<String, String> {
    let cut = parameters(payload, ids)?;
    let point = |p: [f64; 3]| serde_json::json!({"$schema":"point","x":p[0],"y":p[1],"z":p[2]});
    let params = serde_json::json!({"face":{"$schema":"number","value":cut.face},"start":point(cut.start),"end":point(cut.end)});
    insert_mesh_operation(host, "knifeCut", "face", ids, &params.to_string())
}

pub fn apply_selected(payload: &KnifeMeshSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, ids: &[String]) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    with_host(&doc.snapshot.host_snapshot, |host| {
        let id = insert_operation(host, payload, ids).map_err(Fault::from)?;
        Ok(Emit {
            artifact_mutations: commit_host_snapshot(&doc.snapshot.host_snapshot, &host.host_snapshot),
            interaction_writes: vec![InteractionWrite::replace(DOMAIN, "face", std::iter::empty::<String>()), InteractionWrite::replace("graph", "node", [id])],
            ..Default::default()
        })
    })
}

pub fn handle(payload: &KnifeMeshSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    apply_selected(payload, doc, &[])
}
