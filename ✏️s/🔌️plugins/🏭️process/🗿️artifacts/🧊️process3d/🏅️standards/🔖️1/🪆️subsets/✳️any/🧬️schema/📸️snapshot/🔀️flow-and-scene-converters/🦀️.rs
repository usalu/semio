//! 🔀️ Flow and scene converters of the process3d snapshot: the genesis child pack and restore projection, the conversions between process steps and flow nodes, the working scene to and from the persisted snapshot and the empty snapshot. See [`super`].

use super::brep_converters::{brep_snapshot_for_working_solid, working_solid_child_handle};
use super::document_records::{Pose, ProcessMeasure, ProcessStep, ProcessWorkingScene, StepOrigin, WorkingSolid};
use super::machines_and_catalogs::Workshop;
use super::Process3dSnapshot;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, PortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};

//#region 🔖️Document
//#region 🔖️WorkingScene
//#region 🌱️GenesisChildren
/// 🌱️ `ArtifactApp::genesis_child_pack` for the three composed members every process3d document
/// declares — `stockSolid` (the stock's B-Rep), `steps` (the timeline flow) and each `toolSolids`
/// entry (a cut tool / attached component's B-Rep) — all pure functions of the inline
/// `stock_payload`/`step_payloads` records, so a fresh boot and a whole-document load
/// (`Effect::LoadDocument`, whose archive the react shell sends member-less) materialise exactly the
/// children `process_working_scene_to_snapshot` minted handles for. Mirrors `🌍️gis`'s
/// `genesis_gis_map_child_pack`; without it the archive closure reports `Incomplete` and every example
/// switch fails.
pub fn genesis_process3d_child_pack(snapshot: &Process3dSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use pack::snapshot::ArtifactPack;
    match slot {
        "stockSolid" if child_id == snapshot.stock_solid.child_id => Some(<SemioBrepSnapshot as ArtifactPack>::encode_pack(&brep_snapshot_for_working_solid(&snapshot.stock_payload.solid))),
        "steps" if child_id == snapshot.steps.child_id => {
            let tool_child_ids = snapshot.step_payloads.iter().zip(step_tool_solids(&snapshot.step_payloads)).filter_map(|(step, solid)| solid.map(|solid| (step.id.clone(), working_solid_child_handle(&format!("tool-{}", step.id), solid).child_id))).collect();
            Some(<SemioFlowSnapshot as ArtifactPack>::encode_pack(&flow_snapshot_for_steps(&snapshot.step_payloads, &tool_child_ids)))
        }
        "toolSolids" => snapshot
            .step_payloads
            .iter()
            .zip(step_tool_solids(&snapshot.step_payloads))
            .filter_map(|(step, solid)| solid.map(|solid| (step, solid)))
            .find(|(step, solid)| working_solid_child_handle(&format!("tool-{}", step.id), solid).child_id == child_id)
            .map(|(_, solid)| <SemioBrepSnapshot as ArtifactPack>::encode_pack(&brep_snapshot_for_working_solid(solid))),
        _ => None,
    }
}

/// 🪚️ The tool/component solid each step carries — `None` for a `Drill`, which mints no child.
fn step_tool_solids(steps: &[ProcessStep]) -> impl Iterator<Item = Option<&WorkingSolid>> {
    steps.iter().map(|step| match &step.measure {
        ProcessMeasure::Cut { tool, .. } => Some(tool),
        ProcessMeasure::Attach { component, .. } => Some(component),
        ProcessMeasure::Drill { .. } => None,
    })
}

/// 🧬️ The bounded projection of this snapshot's composed child handles (`stockSolid`, `steps`, every
/// `toolSolids` row), as both surfaces hand it to the framework's replacement pump
/// (`ArtifactEditor::child_restore_projection` / `ArtifactViewer::child_restore_projection`). The
/// trait default grants no restore authority, so without it every live envelope load faults
/// `editor did not declare a loaded-parent child projection`. Mirrors `energy_child_restore_projection`.
pub fn process3d_child_restore_projection(snapshot: &Process3dSnapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::from(format!("process3d child projection failed: {error}")))
}
//#endregion 🌱️GenesisChildren
//#endregion 🔖️WorkingScene
//#endregion 🔖️Document

//#region 🔖️Document
//#region 🔖️WorkingScene
//#region 🔖️FlowConverters
/// 🌉️ WRITE direction, real: one `FlowNode` per `ProcessStep`, laid out left-to-right in timeline
/// order. `enabled`/`origin`/measure-specific scalars round-trip exactly via string params;
/// `Cut`/`Attach`'s `WorkingSolid` tool/component is addressed indirectly by a `toolChildId` param
/// naming its entry in `Process3dSnapshot::tool_solids` (minted alongside by the caller — see
/// `process_working_scene_to_snapshot`), since a `FlowParam` value is a plain string, never a child
/// handle itself.
pub fn flow_node_from_process_step(step: &ProcessStep, index: usize, tool_child_id: Option<&str>) -> FlowNode {
    let mut params = vec![FlowParam { key: "enabled".into(), value: step.enabled.to_string() }];
    if let Some(origin) = &step.origin {
        params.push(FlowParam { key: "originMachineId".into(), value: origin.machine_id.clone() });
        params.push(FlowParam { key: "originCapabilityId".into(), value: origin.capability_id.clone() });
    }
    let pose = step.measure.pose();
    params.push(FlowParam { key: "posePositionX".into(), value: pose.position[0].to_string() });
    params.push(FlowParam { key: "posePositionY".into(), value: pose.position[1].to_string() });
    params.push(FlowParam { key: "posePositionZ".into(), value: pose.position[2].to_string() });
    params.push(FlowParam { key: "poseAxisX".into(), value: pose.axis[0].to_string() });
    params.push(FlowParam { key: "poseAxisY".into(), value: pose.axis[1].to_string() });
    params.push(FlowParam { key: "poseAxisZ".into(), value: pose.axis[2].to_string() });
    params.push(FlowParam { key: "poseAngle".into(), value: pose.angle.to_string() });
    match &step.measure {
        ProcessMeasure::Drill { radius, depth, .. } => {
            params.push(FlowParam { key: "radius".into(), value: radius.to_string() });
            params.push(FlowParam { key: "depth".into(), value: depth.to_string() });
        }
        ProcessMeasure::Cut { .. } | ProcessMeasure::Attach { .. } => {
            if let Some(child_id) = tool_child_id {
                params.push(FlowParam { key: "toolChildId".into(), value: child_id.to_string() });
            }
        }
    }
    FlowNode { id: step.id.clone(), kind: step.measure.kind_slug().into(), label: step.label.clone(), params, position: SemioPoint2 { x: index as f64 * 200.0, y: 0.0 } }
}

/// 🌉️ READ direction, real for everything reachable from the node's own params (`enabled`/`origin`/
/// pose/radius/depth); `Cut`/`Attach`'s tool/component solid is a documented gap
/// (`working_solid_from_brep_snapshot`'s own doc comment — a `toolChildId` param names WHICH child
/// holds the resolved geometry, but resolving that handle to real content needs a `LinkResolver`
/// this ticket doesn't add).
pub fn process_step_from_flow_node(node: &FlowNode) -> ProcessStep {
    let param = |key: &str| node.params.iter().find(|p| p.key == key).map(|p| p.value.as_str());
    let f = |key: &str| -> f64 { param(key).and_then(|v| v.parse().ok()).unwrap_or(0.0) };
    let enabled = param("enabled").is_none_or(|v| v == "true");
    let origin = match (param("originMachineId"), param("originCapabilityId")) {
        (Some(machine_id), Some(capability_id)) => Some(StepOrigin { machine_id: machine_id.into(), capability_id: capability_id.into() }),
        _ => None,
    };
    let pose = Pose { position: [f("posePositionX"), f("posePositionY"), f("posePositionZ")], axis: [f("poseAxisX"), f("poseAxisY"), f("poseAxisZ")], angle: f("poseAngle") };
    let measure = match node.kind.as_str() {
        "drill" => ProcessMeasure::Drill { radius: f("radius"), depth: f("depth"), pose },
        "attach" => ProcessMeasure::Attach { component: WorkingSolid::default(), pose },
        _ => ProcessMeasure::Cut { tool: WorkingSolid::default(), pose },
    };
    ProcessStep { id: node.id.clone(), label: node.label.clone(), enabled, origin, measure }
}

/// 🪪️ Mint a deterministic, content-addressed `s.stdio.semio.flow` CHILD HANDLE from `content`.
pub fn flow_child_handle(content: &SemioFlowSnapshot) -> store::os_store::artifact_child::ArtifactChild<SemioFlowSnapshot> {
    let child_id = store::content_id("steps-flow", semio_framework_pack_json::to_json_string(content).as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

/// 🌉️ WRITE direction, real: the full flow graph for a step timeline — one node per step (see
/// `flow_node_from_process_step`) plus a linear sequence edge between consecutive steps (this
/// plugin's timeline has no branching, so a simple chain is an honest, lossless topology — not an
/// approximation of a richer graph the old `Vec<ProcessStep>` never had either).
pub fn flow_snapshot_for_steps(steps: &[ProcessStep], tool_child_ids: &std::collections::BTreeMap<String, String>) -> SemioFlowSnapshot {
    let nodes: Vec<FlowNode> = steps.iter().enumerate().map(|(i, step)| flow_node_from_process_step(step, i, tool_child_ids.get(&step.id).map(|s| s.as_str()))).collect();
    let edges: Vec<FlowEdge> = steps
        .windows(2)
        .map(|pair| FlowEdge { id: format!("e-{}-{}", pair[0].id, pair[1].id), from: PortRef { node: pair[0].id.clone(), port: "out".into() }, to: PortRef { node: pair[1].id.clone(), port: "in".into() }, kind: "sequence".into() })
        .collect();
    SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes, edges }
}

/// 🌉️ READ direction: node order in `SemioFlowSnapshot.nodes` IS the step order (matches
/// `flow_snapshot_for_steps`'s construction — the `position.x` synthetic layout is display-only,
/// never re-derived from).
pub fn process_steps_from_flow_snapshot(flow: &SemioFlowSnapshot) -> Vec<ProcessStep> {
    flow.nodes.iter().map(process_step_from_flow_node).collect()
}
//#endregion 🔖️FlowConverters
//#endregion 🔖️WorkingScene
//#endregion 🔖️Document

//#region 🔖️Document
//#region 🔖️WorkingScene
//#region 🔖️SceneConverters
// 🧠️ Same-process working-scene cache, keyed by the composed child's own content-addressed
// `child_id` — populated at mint time (`process_working_scene_to_snapshot`, the only place literal
// content is in hand), read at scene-reconstruction time
// (`process_working_scene_from_snapshot`). Mirrors `🌊️flow`'s own `FLOW_SCRATCH` pattern (this
// ticket's wave 4, `📓️wave4-reports/flow-report.md`) rather than lowpoly/cad/writer's plain
// "always empty" fallback: within the SAME process, a document round-tripped only through this
// module (never serialized out and a fresh process spun up to read it back) gets its real content
// back, not a fabricated empty scene. Still an `EngineRep`-shaped bridge, not a real resolver —
// crossing a process boundary (a fresh process loading a saved document, or an undo/redo that
// bypasses `ArtifactApp::handle`) still degrades to the honest empty fallback, matching every
// other exemplar's documented staleness gap.
/// 🌉️ WRITE direction, real: builds the full persisted `Process3dSnapshot` from a literal
/// `ProcessWorkingScene` — the only place this migration can mint real composed-child CONTENT
/// (fixture construction, `empty_process3d_snapshot`, and any host-level "mint then dispatch"
/// gesture), since there is no `LinkResolver` to pull literal content back out of an
/// already-persisted child handle later. Mints one `stock_solid` brep child, one `steps` flow
/// child, and one `tool_solids` brep child per `Cut`/`Attach` step (skipped for `Drill`, which
/// carries no `WorkingSolid`) — and caches the literal `Stock`/`Vec<ProcessStep>` behind each
/// snapshot-owned payload records so reopen and worker migration reconstruct the same scene.
pub fn process_working_scene_to_snapshot(scene: &ProcessWorkingScene, workshop: Workshop) -> Process3dSnapshot {
    let mut tool_solids = Vec::new();
    let mut tool_child_ids = std::collections::BTreeMap::new();
    for step in &scene.steps {
        let solid = match &step.measure {
            ProcessMeasure::Cut { tool, .. } => Some(tool),
            ProcessMeasure::Attach { component, .. } => Some(component),
            ProcessMeasure::Drill { .. } => None,
        };
        if let Some(solid) = solid {
            let handle = working_solid_child_handle(&format!("tool-{}", step.id), solid);
            tool_child_ids.insert(step.id.clone(), handle.child_id.clone());
            tool_solids.push(handle);
        }
    }
    let stock_solid = working_solid_child_handle("stock", &scene.stock.solid);
    let flow_content = flow_snapshot_for_steps(&scene.steps, &tool_child_ids);
    let steps = flow_child_handle(&flow_content);
    Process3dSnapshot {
        workshop,
        stock_id: scene.stock.id.clone(),
        stock_label: scene.stock.label.clone(),
        stock_pose: scene.stock.pose.clone(),
        stock_payload: scene.stock.clone(),
        stock_solid,
        steps,
        step_payloads: scene.steps.clone(),
        tool_solids,
    }
}

/// 🌉️ READ direction: `Process3dSnapshot` composes its `stock_solid`/`steps`/`tool_solids` fields
/// as HANDLES only (`child_id`+`target`, never resolved content — see `🏪️store/🦀️.rs`'s
/// `🔖️Composition` region), and no `LinkResolver`/`ChildStoreFactory` seam reaches
/// `ArtifactApp::handle` yet (confirmed directly against `🔌️plugin/🦀️.rs`, W1-owned,
/// read-only for this wave). The durable stock/step records are authoritative while the child
/// handles retain composition identity.
pub fn process_working_scene_from_snapshot(snapshot: &Process3dSnapshot) -> ProcessWorkingScene {
    let mut stock = snapshot.stock_payload.clone();
    stock.id = snapshot.stock_id.clone();
    stock.label = snapshot.stock_label.clone();
    stock.pose = snapshot.stock_pose.clone();
    ProcessWorkingScene { stock, steps: snapshot.step_payloads.clone() }
}
//#endregion 🔖️SceneConverters
//#endregion 🔖️WorkingScene
//#endregion 🔖️Document

//#region 🔖️Document
/// 🗄️ Empty process3d snapshot (default workshop + stock, no steps) — real composed children minted
/// from the default `ProcessWorkingScene` via `process_working_scene_to_snapshot`, never bare/unset
/// handles.
pub fn empty_process3d_snapshot() -> Process3dSnapshot {
    process_working_scene_to_snapshot(&ProcessWorkingScene::default(), Workshop::default())
}
//#endregion 🔖️Document
