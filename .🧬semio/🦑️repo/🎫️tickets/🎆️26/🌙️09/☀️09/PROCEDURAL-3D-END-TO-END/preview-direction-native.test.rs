#![allow(dead_code)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use semio_framework_pack_json::{JsonMemberPolicy, Object, Value, parse};

pub const INTERACTION_HOVER_GROUP_GRANULARITY: &str = "framework.hover-group";
mod semio_plugin { pub use crate::INTERACTION_HOVER_GROUP_GRANULARITY; }
#[derive(Debug)]
struct Fault;
mod protocol {
    #[derive(Debug, PartialEq, Eq)]
    pub struct InteractionTarget { pub id: String, pub granularity: String }
}

const GENERATION_3D_INTERACTION_GRANULARITY: &str = "handle";
const GENERATION3D_VIEW_INTERACTION_GRANULARITY: &str = "handle";

pub mod semio_framework_artifact_flow_flow {
    #[derive(Default)]
    pub struct FlowHostSnapshot { pub widgets: Vec<Widget>, pub export: Vec<(String, Option<String>)> }
    pub enum Widget { Neuron { id: String, preview: bool }, Variable { id: String, schema: String } }
}
#[path = GRAPH_PORT_SOURCE]
mod graph_ports;
pub mod semio_s_artifact_procedural_3d_model {
    pub use super::graph_ports::{graph_instance_id, graph_port_id, GraphTargetKind};
    use super::semio_framework_artifact_flow_flow::Widget;
    pub fn widget_id(widget: &Widget) -> &str { match widget { Widget::Neuron { id, .. } | Widget::Variable { id, .. } => id } }
    pub fn widget_previews(widget: &Widget) -> bool { matches!(widget, Widget::Neuron { preview: true, .. }) }
}
pub mod semio_s_artifact_procedural_3d_mesh_bridge {
    use super::semio_framework_artifact_flow_flow::FlowHostSnapshot;
    pub fn retained_geometry_channels(host: &FlowHostSnapshot) -> Vec<(String, Option<String>)> { host.export.clone() }
}
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
extern crate self as semio_s_artifact_procedural_3d_preview_eval;
pub mod semio_os_flow {
    pub use super::FlowEvalSession;
    pub fn preview_tessellate_node_hash(handle: &str, bits: u64) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        (handle, bits).hash(&mut hasher);
        hasher.finish()
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeshData { positions: Vec<f32>, indices: Vec<u32>, edge_positions: Vec<f32> }
std::thread_local! { static MESH_CONVERSIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
impl From<MeshData> for Value {
    fn from(mesh: MeshData) -> Value {
        MESH_CONVERSIONS.with(|count| count.set(count.get() + 1));
        semio_framework_pack_json::object([
            ("vertices".into(), Value::from(mesh.positions.len() / 3)),
            ("position".into(), mesh.positions.first().map_or(Value::Null, |x| Value::from(f64::from(*x)))),
        ])
    }
}
#[derive(Default)]
pub struct FlowEvalSession { owner: u64, epoch: u64, meshes: HashMap<String, MeshData>, retained: HashMap<String, String> }
impl FlowEvalSession {
    fn preview_mesh_epoch(&self) -> u64 { self.epoch }
    fn preview_mesh_cache_identity(&self) -> u64 { self.owner }
    fn in_flight_preview_handles(&self) -> std::iter::Empty<&str> { std::iter::empty() }
    fn preview_diagnostics(&self, _: &str) -> Option<()> { None }
    fn preview_mesh_for_tolerance(&self, handle: &str, _: f64) -> Option<&MeshData> { self.meshes.get(handle) }
    fn preview_tessellate_failed(&self, _: u64) -> bool { false }
}
pub struct PreviewMeshLookup<'a>(&'a FlowEvalSession, PreviewEvaluationSource);
impl<'a> PreviewMeshLookup<'a> {
    fn new(session: &'a FlowEvalSession) -> Self { Self(session, PreviewEvaluationSource::WithFallback) }
    fn resolve<'item>(&self, _: &str, item: &'item PreviewChannelItem<'_>) -> Option<(std::borrow::Cow<'item, str>, &'a MeshData)> {
        self.0.meshes.get(&item.handle).map(|mesh| (std::borrow::Cow::Borrowed(item.handle.as_str()), mesh)).or_else(|| {
            if self.1 == PreviewEvaluationSource::Current { return None; }
            let source = self.0.retained.get(&item.handle)?;
            Some((std::borrow::Cow::Owned(source.clone()), self.0.meshes.get(source)?))
        })
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum PreviewEvaluationSource { WithFallback, Current }
struct PreviewEvaluation<'a> { source: &'a str, value: Value, evaluation_source: PreviewEvaluationSource }
impl<'a> PreviewEvaluation<'a> {
    fn source(&self) -> &'a str { self.source }
    fn geometry(&self) -> Option<&Value> { Some(&self.value) }
    fn mesh_lookup<'session>(&self, session: &'session FlowEvalSession) -> PreviewMeshLookup<'session> { PreviewMeshLookup(session, self.evaluation_source) }
}
struct PreviewWireHover;
impl PreviewWireHover { fn matches(&self, _: &str, _: &str, _: bool) -> bool { false } }
fn preview_wire_hover(_: &FlowHostSnapshot, _: &PreviewGraphTargets<'_>) -> PreviewWireHover { PreviewWireHover }
fn inline_preview_mesh(_: &PreviewInlineGeometry<'_>) -> Option<MeshData> {
    Some(MeshData { positions: vec![0.0, 1.0, 2.0], ..Default::default() })
}
fn apply_show_mode_mesh(data: MeshData, _: &str) -> MeshData { data }

pub mod selection {
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ComponentSelection { pub selected: Vec<String>, pub granularity: String }
    impl ComponentSelection { pub fn active(&self) -> bool { !self.granularity.is_empty() } }
    pub struct ComponentTarget<'a> { pub instance: &'a str, pub granularity: &'a str, pub index: usize }
    impl<'a> ComponentTarget<'a> { pub fn parse(_: &'a str) -> Option<Self> { None } }
    pub fn component_group(_: &[String]) -> Result<(ComponentTarget<'_>, Vec<u32>), ()> { Err(()) }
    pub fn component_pivot(_: &str, _: &str, _: &[u32]) -> Option<[f64; 3]> { None }
}
#[derive(Default)]
struct Generation3dConfig { show_mode: String, lod_mode: String }
struct Generation3dViewConfig(f64, &'static str);
impl Default for Generation3dViewConfig {
    fn default() -> Self { Self(0.05, "solid") }
}
impl Generation3dViewConfig {
    fn tolerance(&self) -> f64 { self.0 }
    fn effective_show_mode(&self) -> &'static str { self.1 }
}
#[derive(Default)]
struct PreviewInteractionMarks {
    selected: BTreeSet<String>, hovered: Vec<String>,
    components: selection::ComponentSelection, source_revision: Option<String>,
    graph_hover_source: Option<Option<String>>, component_hover_source: Option<Option<String>>,
}
impl PreviewInteractionMarks {
    fn component_hover(&self) -> Option<selection::ComponentTarget<'_>> { None }
    fn preview_components(&self) -> selection::ComponentSelection { self.components.clone() }
}
#[derive(Default)]
struct Generation3dViewMarks { selected: BTreeSet<String>, hovered: BTreeSet<String>, hover_source_window: Option<Option<String>> }

fn law() -> Value { parse(include_str!(DIRECTION_FIXTURE), JsonMemberPolicy::Reject).unwrap() }
fn strings(value: &Value) -> Vec<String> { value.as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_owned()).collect() }
fn host(law: &Value) -> FlowHostSnapshot {
    let hidden = &law["exportOnly"];
    let id = hidden["widget"].as_str().unwrap().to_owned();
    let visible = law["rows"].as_array().unwrap().iter().map(|row| {
        let id = row["widget"].as_str().unwrap().into();
        match row["variable"]["schema"].as_str() {
            Some(schema) => Widget::Variable { id, schema: schema.into() },
            None => Widget::Neuron { id, preview: true },
        }
    });
    FlowHostSnapshot {
        widgets: visible.chain([Widget::Neuron { id: id.clone(), preview: false }]).collect(),
        export: vec![("source".into(), Some("result".into())), (id, Some(hidden["channel"].as_str().unwrap().into()))],
    }
}

#[test]
fn viewport_admission_includes_inputs_and_export_keeps_its_preferred_channels() {
    let fixture = law();
    let evaluation = &fixture["evaluation"];
    let host = host(&fixture);
    let mut handles = Vec::new();
    for row in fixture["rows"].as_array().unwrap() {
        let widget = row["widget"].as_str().unwrap();
        let items = preview_viewport_items_for_widget(evaluation, widget);
        let expected = row["viewport"].as_array().unwrap();
        assert_eq!(items.len(), expected.len());
        for (item, expected) in items.iter().zip(expected) {
            let address = item.address(widget);
            assert_eq!(address.port, expected["port"].as_str().unwrap());
            assert_eq!(address.instance, expected["instance"].as_str().unwrap());
            assert_eq!(item.handle, expected["handle"].as_str().unwrap());
            assert_eq!(item.is_input(), expected["input"].as_bool().unwrap());
        }
        let export: Vec<_> = preview_channel_items_for_widget(evaluation, widget).into_iter().map(|item| item.handle).collect();
        assert_eq!(export, strings(&row["export"]));
        handles.extend(items.into_iter().map(|item| item.handle));
    }
    let session = FlowEvalSession::default();
    handles.push(fixture["exportOnly"]["pending"].as_str().unwrap().into());
    assert_eq!(pending_preview_tessellate_handles(evaluation, &host, &session, 0.05), handles);
    let expected_live: HashSet<_> = handles.into_iter().chain(["face-unselected".into(), "wire-hidden-input".into()]).collect();
    assert_eq!(preview_mesh_retention_handles(evaluation, &host, &session), expected_live);
}

#[test]
fn paired_payloads_publish_and_highlight_directional_instances_from_the_same_mesh_table() {
    let fixture = law();
    let source = fixture["evaluation"].to_string();
    let evaluation = PreviewEvaluation { source: &source, value: fixture["evaluation"].clone(), evaluation_source: PreviewEvaluationSource::WithFallback };
    let host = host(&fixture);
    let mut session = FlowEvalSession::default();
    let expected: Vec<_> = fixture["rows"].as_array().unwrap().iter().flat_map(|row| row["viewport"].as_array().unwrap()).collect();
    for (index, row) in expected.iter().enumerate() {
        let handle = row["handle"].as_str().unwrap();
        session.meshes.entry(handle.into()).or_insert_with(|| MeshData { positions: vec![index as f32, 0., 0.], ..Default::default() });
    }
    let edit_config = Generation3dConfig::default();
    let view_config = Generation3dViewConfig::default();
    let draw = |sets: &[String]| {
        let set = || sets.iter().cloned().collect();
        let edit = editor::preview_payload(&evaluation, &host, &edit_config, Some(&session),
            &PreviewInteractionMarks { hovered: sets.to_vec(), selected: set(), ..Default::default() });
        let view = viewer::preview_payload(&evaluation, &host, &view_config, Some(&session),
            &Generation3dViewMarks { hovered: set(), selected: set(), ..Default::default() });
        assert_eq!(edit.instances_json, view.instances_json);
        assert_eq!(edit.meshes_json, view.meshes_json);
        assert_eq!(edit.hovered_ids, view.hovered_ids);
        assert_eq!(edit.selected_ids, view.selected_ids);
        assert_eq!(edit.bounds, view.bounds);
        edit
    };
    let quiet = draw(&[]);
    let instances = parse(&quiet.instances_json, JsonMemberPolicy::Reject).unwrap();
    let instances = instances.as_array().unwrap();
    let meshes = parse(&quiet.meshes_json, JsonMemberPolicy::Reject).unwrap();
    assert_eq!(instances.len(), expected.len());
    assert_eq!(meshes.as_array().unwrap().len(), session.meshes.len());
    for (instance, row) in instances.iter().zip(expected) {
        assert_eq!(instance["id"], row["instance"]);
        assert_eq!(instance["interactionId"].as_str().unwrap(), graph_ports::GraphTargetKind::Handle.target(row["port"].as_str().unwrap()));
        assert_eq!(instance["hoverTargets"], fixture["hoverTargetsByHandle"][row["handle"].as_str().unwrap()]);
        assert_eq!(instance["label"].as_str().unwrap().starts_with("← "), row["input"].as_bool().unwrap());
    }
    let end = instances.len();
    assert_eq!(instances[end - 1]["meshId"], instances[end - 2]["meshId"]);
    for mark in fixture["rows"].as_array().unwrap().iter().flat_map(|row| row["marks"].as_array().unwrap()) {
        let payload = draw(&strings(&mark["ids"]));
        assert_eq!(payload.hovered_ids, strings(&mark["instances"]));
        assert_eq!(payload.selected_ids, payload.hovered_ids);
        assert_eq!(payload.meshes_json, quiet.meshes_json);
    }
    let components = editor::preview_payload(&evaluation, &host, &edit_config, Some(&session), &PreviewInteractionMarks {
        hovered: vec!["h:operator@!isame".into()],
        graph_hover_source: Some(Some("flow".into())),
        source_revision: Some("a".repeat(64)),
        components: selection::ComponentSelection { granularity: "face".into(), ..Default::default() },
        ..Default::default()
    });
    let component_instances = parse(&components.instances_json, JsonMemberPolicy::Reject).unwrap();
    let component_instances = component_instances.as_array().unwrap();
    assert_eq!(component_instances.len(), instances.len());
    assert_eq!(components.hovered_ids, ["operator@!isame#in-0", "operator@!isame#in-1"]);
    assert_eq!(components.hover_source_window, Some(Some("flow".into())));
    for instance in component_instances {
        let id = instance["id"].as_str().unwrap();
        if id.contains("#in-") {
            assert_eq!(instance["componentPickable"].as_bool(), Some(false));
            assert_eq!(instance["componentSource"], Value::Null);
        } else {
            assert_ne!(instance["componentPickable"].as_bool(), Some(false));
            assert_eq!(instance["componentSource"]["revision"].as_str(), Some("a".repeat(64).as_str()));
            assert_eq!(instance["interactionId"], instance["id"]);
        }
    }
    assert!(!components.instances_json.contains("hoverTargets"));
}

#[test]
fn viewer_populates_the_same_evaluation_when_a_hidden_input_mesh_arrives() {
    let fixture = law();
    let source = fixture["evaluation"].to_string();
    let evaluation = PreviewEvaluation { source: &source, value: fixture["evaluation"].clone(), evaluation_source: PreviewEvaluationSource::WithFallback };
    let host = host(&fixture);
    let mut session = FlowEvalSession::default();
    let marks = Generation3dViewMarks { hovered: ["h:meter@!isource".to_owned()].into_iter().collect(), ..Default::default() };
    let cold = viewer::preview_payload(&evaluation, &host, &Generation3dViewConfig::default(), Some(&session), &marks);
    assert_eq!(cold.instances_json, "[]");
    session.meshes.insert("solid-measured".into(), MeshData { positions: vec![3.0, 0.0, 0.0], ..Default::default() });
    session.epoch += 1;
    let ready = viewer::preview_payload(&evaluation, &host, &Generation3dViewConfig::default(), Some(&session), &marks);
    let instances = parse(&ready.instances_json, JsonMemberPolicy::Reject).unwrap();
    assert_eq!(instances.as_array().unwrap().len(), 1);
    assert_eq!(instances[0]["id"].as_str(), Some("meter@!isource#in-0"));
    assert_eq!(ready.hovered_ids, ["meter@!isource#in-0"]);
}

#[test]
fn inline_meshes_vectors_points_and_planes_group_only_exact_geometry() {
    let fixture = parse(include_str!(INLINE_FIXTURE), JsonMemberPolicy::Reject).unwrap();
    let widget = fixture["widget"].as_str().unwrap();
    let source = fixture["evaluation"].to_string();
    let evaluation = PreviewEvaluation { source: &source, value: fixture["evaluation"].clone(), evaluation_source: PreviewEvaluationSource::WithFallback };
    let host = FlowHostSnapshot { widgets: vec![Widget::Neuron { id: widget.into(), preview: true }], ..Default::default() };
    let edit = editor::preview_payload(&evaluation, &host, &Generation3dConfig::default(), None, &PreviewInteractionMarks::default());
    let view = viewer::preview_payload(&evaluation, &host, &Generation3dViewConfig::default(), None, &Generation3dViewMarks::default());
    assert_eq!(edit.instances_json, view.instances_json);
    assert_eq!(edit.meshes_json, view.meshes_json);
    let meshes = parse(&edit.meshes_json, JsonMemberPolicy::Reject).unwrap();
    assert_eq!(meshes.as_array().unwrap().len(), fixture["distinctMeshes"].as_u64().unwrap() as usize);
    let instances = parse(&edit.instances_json, JsonMemberPolicy::Reject).unwrap();
    assert_eq!(instances.as_array().unwrap().len(), fixture["instances"].as_u64().unwrap() as usize);
    let groups: Vec<_> = fixture["groups"].as_array().unwrap().iter().map(strings).collect();
    for instance in instances.as_array().unwrap() {
        let id = instance["interactionId"].as_str().unwrap();
        let expected = groups.iter().find(|group| group.iter().any(|member| member == id));
        let targets = instance["hoverTargets"].as_array().map(|rows| rows.iter().map(|row| row["id"].as_str().unwrap().to_owned()).collect::<Vec<_>>());
        assert_eq!(targets.as_ref(), expected, "{id}");
    }
}

#[test]
fn component_mode_retains_graph_hover_for_pending_outputs_and_inline_markers() {
    let value = parse(r#"{"shape":{"out":{
        "now":{"$schema":"geometry","handle":"solid-current"},
        "pending":{"$schema":"geometry","handle":"solid-pending"},
        "point":{"$schema":"point","x":1,"y":2,"z":3},
        "mesh":{"$schema":"mesh","data":"data","preview":"preview"},
        "list":{"$schema":"list","0":{"$schema":"mesh","data":"data","preview":"preview"}}
    }}}"#, JsonMemberPolicy::Reject).unwrap();
    let source = value.to_string();
    let evaluation = PreviewEvaluation { source: &source, value, evaluation_source: PreviewEvaluationSource::WithFallback };
    let host = FlowHostSnapshot { widgets: vec![Widget::Neuron { id: "shape".into(), preview: true }], ..Default::default() };
    let mut session = FlowEvalSession::default();
    for key in ["solid-current", "solid-previous"] {
        session.meshes.insert(key.into(), MeshData { positions: vec![0.0, 0.0, 0.0], ..Default::default() });
    }
    session.retained.insert("solid-pending".into(), "solid-previous".into());
    let revision = "b".repeat(64);
    let payload = editor::preview_payload(&evaluation, &host, &Generation3dConfig::default(), Some(&session), &PreviewInteractionMarks {
        hovered: vec!["n:shape".into()], source_revision: Some(revision.clone()),
        components: selection::ComponentSelection { granularity: "vertex".into(), ..Default::default() },
        ..Default::default()
    });
    let ids = ["shape@list#0", "shape@mesh#0", "shape@now#0", "shape@pending#0", "shape@point#0"];
    assert_eq!(payload.hovered_ids, ids);
    let instances = parse(&payload.instances_json, JsonMemberPolicy::Reject).unwrap();
    let instances = instances.as_array().unwrap();
    assert_eq!(instances.len(), ids.len());
    for (instance, id) in instances.iter().zip(ids) {
        assert_eq!(instance["id"].as_str(), Some(id));
        let editable = matches!(id, "shape@mesh#0" | "shape@now#0");
        assert_eq!(instance["componentPickable"].as_bool(), (!editable).then_some(false));
        assert_eq!(instance["componentSource"]["revision"].as_str(), (id == "shape@now#0").then_some(revision.as_str()));
    }
}

#[test]
fn independent_exact_retained_surfaces_share_hover_and_keep_mesh_identity() {
    let fixture = parse(include_str!(NATIVE_OVERLAP_FIXTURE), JsonMemberPolicy::Reject).unwrap();
    let widget = fixture["widget"].as_str().unwrap();
    let source = fixture["evaluation"].to_string();
    let evaluation = PreviewEvaluation { source: &source, value: fixture["evaluation"].clone(), evaluation_source: PreviewEvaluationSource::WithFallback };
    let host = FlowHostSnapshot { widgets: vec![Widget::Neuron { id: widget.into(), preview: true }], ..Default::default() };
    let mut session = FlowEvalSession::default();
    for (key, value) in fixture["meshes"].as_object().unwrap().iter() {
        let floats = |field: &str| value[field].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect();
        session.meshes.insert(key.to_owned(), MeshData {
            positions: floats("positions"),
            indices: value["indices"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect(),
            edge_positions: floats("edge_positions"),
        });
    }
    let edit = editor::preview_payload(&evaluation, &host, &Generation3dConfig::default(), Some(&session), &PreviewInteractionMarks::default());
    let view = viewer::preview_payload(&evaluation, &host, &Generation3dViewConfig::default(), Some(&session), &Generation3dViewMarks::default());
    assert_eq!(edit.instances_json, view.instances_json);
    let rows = parse(&edit.instances_json, JsonMemberPolicy::Reject).unwrap();
    let instances = rows.as_array().unwrap();
    let expected = strings(&fixture["expected"]["group"]);
    assert_eq!(instances.len(), fixture["expected"]["instances"].as_u64().unwrap() as usize);
    let distinct: HashSet<_> = instances.iter().map(|instance| instance["meshId"].as_str().unwrap()).collect();
    assert_eq!(distinct.len(), fixture["expected"]["meshes"].as_u64().unwrap() as usize);
    for instance in instances {
        let target = instance["interactionId"].as_str().unwrap();
        if expected.iter().any(|value| value == target) {
            let group: Vec<_> = instance["hoverTargets"].as_array().unwrap().iter().map(|entry| entry["id"].as_str().unwrap().to_owned()).collect();
            assert_eq!(group, expected, "{target}");
        } else {
            assert_eq!(instance["hoverTargets"], Value::Null, "{target}");
        }
    }
}

#[test]
fn hover_reuses_exact_mesh_publication_across_edit_and_view_without_cross_session_leaks() {
    let fixture = parse(include_str!(MESH_CACHE_FIXTURE), JsonMemberPolicy::Reject).unwrap();
    let geometry = &fixture["evaluation"];
    let source = geometry.to_string();
    let target = fixture["target"].as_str().unwrap();
    let instance = fixture["instance"].as_str().unwrap();
    let mesh = |positions| MeshData { positions, ..Default::default() };
    for edit in [false, true] {
        viewer::reset_preview_mesh_table();
        MESH_CONVERSIONS.with(|count| count.set(0));
        let mut sessions: HashMap<_, _> = fixture["sessions"].as_object().unwrap().iter().map(|(id, recipe)| {
            let meshes = recipe["meshes"].as_object().unwrap().iter().map(|(handle, xyz)| {
                (handle.to_owned(), mesh(xyz.as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect()))
            }).collect();
            (id.to_owned(), FlowEvalSession { owner: recipe["owner"].as_u64().unwrap(), epoch: recipe["epoch"].as_u64().unwrap(), meshes, ..Default::default() })
        }).collect();
        for step in fixture["steps"].as_array().unwrap() {
            let session = sessions.get_mut(step["session"].as_str().unwrap()).unwrap();
            if let Some(xyz) = step["replace"].as_array() {
                session.meshes.insert("solid-work".into(), mesh(xyz.iter().map(|value| value.as_f64().unwrap() as f32).collect()));
                session.epoch += 1;
            }
            if step["fallback"].as_bool() == Some(true) {
                session.meshes.remove("solid-work");
                session.retained.insert("solid-work".into(), "solid-fallback".into());
            }
            let visible = !step["hidden"].as_bool().unwrap_or(false);
            let host = FlowHostSnapshot { widgets: vec![Widget::Neuron { id: "shape".into(), preview: visible }], ..Default::default() };
            let evaluation_source = if step["current"].as_bool() == Some(true) { PreviewEvaluationSource::Current } else { PreviewEvaluationSource::WithFallback };
            let evaluation = PreviewEvaluation { source: &source, value: geometry.clone(), evaluation_source };
            let detail = step["detail"].as_f64().unwrap_or(0.05);
            let show_mode = if step["mode"].as_str() == Some("wireframe") { "wireframe" } else { "solid" };
            let marks: Vec<String> = if step["hover"].as_bool() == Some(true) { vec![target.into()] } else { vec![] };
            let (mesh_json, hovered) = if edit {
                let config = Generation3dConfig { show_mode: show_mode.into(), lod_mode: if detail > 0.1 { "coarse".into() } else { "medium".into() } };
                let payload = editor::preview_payload(&evaluation, &host, &config, Some(session), &PreviewInteractionMarks { hovered: marks, ..Default::default() });
                (payload.meshes_json, payload.hovered_ids)
            } else {
                let config = Generation3dViewConfig(detail, show_mode);
                let payload = viewer::preview_payload(&evaluation, &host, &config, Some(session), &Generation3dViewMarks { hovered: marks.into_iter().collect(), ..Default::default() });
                (payload.meshes_json, payload.hovered_ids)
            };
            let mesh_json = parse(&mesh_json, JsonMemberPolicy::Reject).unwrap();
            assert_eq!(mesh_json[0]["data"]["position"].as_f64(), step["position"].as_f64(), "edit {edit} step {step}");
            assert_eq!(MESH_CONVERSIONS.with(std::cell::Cell::get) as u64, step["conversions"].as_u64().unwrap(), "edit {edit} step {step}");
            assert_eq!(hovered, if step["hover"].as_bool() == Some(true) { vec![instance.into()] } else { Vec::<String>::new() });
        }
    }
}

#[test]
fn large_coincident_hover_group_keeps_every_channel() {
    let fixture = parse(include_str!(LARGE_HOVER_FIXTURE), JsonMemberPolicy::Reject).unwrap();
    let count = fixture["count"].as_u64().unwrap() as usize;
    let width = fixture["widgetBytes"].as_u64().unwrap() as usize;
    let channel = fixture["channel"].as_str().unwrap();
    let capacity = fixture["maximumActionBytes"].as_u64().unwrap() as usize;
    let mut snapshot = FlowHostSnapshot::default();
    let mut entries = Object::new();
    let mut expected = Vec::new();
    for index in 0..count {
        let widget = format!("copy-{index:03}-{}", "q".repeat(width - 9));
        let input = index % 2 == 0;
        let address = PreviewChannelAddress::new(&widget, channel, 0, input);
        expected.push(address.interaction_id());
        let mut values = Object::new();
        values.insert(channel, parse(r#"{"$schema":"point","x":2,"y":0,"z":0}"#, JsonMemberPolicy::Reject).unwrap());
        let mut entry = Object::new();
        entry.insert(if input { "in" } else { "out" }, Value::Object(values));
        entries.insert(widget.clone(), Value::Object(entry));
        snapshot.widgets.push(Widget::Neuron { id: widget, preview: true });
    }
    expected.sort();
    assert!(expected.iter().all(|id| id.len() <= 256));
    assert!(expected.iter().map(String::len).sum::<usize>() > capacity);
    let source = semio_framework_pack_json::to_string(&Value::Object(entries.clone()));
    let evaluation = PreviewEvaluation { source: &source, value: Value::Object(entries), evaluation_source: PreviewEvaluationSource::WithFallback };
    let session = FlowEvalSession { owner: 139, ..Default::default() };
    let edit_config = Generation3dConfig::default();
    let view_config = Generation3dViewConfig::default();
    let published = editor::preview_payload(&evaluation, &snapshot, &edit_config, Some(&session), &PreviewInteractionMarks::default());
    let instances = parse(&published.instances_json, JsonMemberPolicy::Reject).unwrap();
    let instances = instances.as_array().unwrap();
    let targets = &instances[0]["hoverTargets"];
    assert_eq!(targets.as_array().unwrap().len(), 1);
    assert!(semio_framework_pack_json::to_string(targets).len() < capacity);
    assert!(instances.iter().all(|instance| &instance["hoverTargets"] == targets));
    let reference = targets[0]["id"].as_str().unwrap().to_owned();
    assert_eq!(published.hover_groups.targets(&reference), Some(expected.as_slice()));
    let mut publications = PreviewHoverPublications::default();
    publications.publish("edit", published.hover_groups);
    let event = || vec![protocol::InteractionTarget { id: reference.clone(), granularity: INTERACTION_HOVER_GROUP_GRANULARITY.into() }];
    let hovered = resolve_interaction_hover_group(event(), |id| Ok(publications.targets("edit", id).map(|ids| ("handle", ids.to_vec())))).unwrap().unwrap();
    assert!(hovered.iter().all(|target| target.granularity == "handle"));
    let hovered: Vec<_> = hovered.into_iter().map(|target| target.id).collect();
    let edit_marks = PreviewInteractionMarks { hovered: hovered.clone(), ..Default::default() };
    let view_marks = Generation3dViewMarks { hovered: hovered.into_iter().collect(), ..Default::default() };
    let edit = editor::preview_payload(&evaluation, &snapshot, &edit_config, Some(&session), &edit_marks);
    let view = viewer::preview_payload(&evaluation, &snapshot, &view_config, Some(&session), &view_marks);
    assert_eq!(edit.hovered_ids.len(), count);
    assert_eq!(view.hovered_ids, edit.hovered_ids);
    assert!(edit.selected_ids.is_empty() && view.selected_ids.is_empty());
    assert_eq!(edit.hover_groups.targets(&reference), Some(expected.as_slice()));
    assert!(PreviewHoverPublications::default().targets("edit", &reference).is_none());
    assert!(resolve_interaction_hover_group(event(), |id| Ok(publications.targets("view", id).map(|ids| ("handle", ids.to_vec())))).unwrap().is_none());
    publications.publish("view", view.hover_groups);
    publications.retain_windows(|id| id == "view");
    assert_eq!(publications.targets("view", &reference), Some(expected.as_slice()));
    assert!(publications.targets("edit", &reference).is_none());
    publications.publish("view", PreviewHoverGroups::default());
    assert!(publications.targets("view", &reference).is_none());
    let publish = |text: &str| parse(text, JsonMemberPolicy::Reject).unwrap();
    let evidence = semio_framework_pack_json::object([
        ("instances".into(), publish(&published.instances_json)),
        ("hoveredIds".into(), Value::Array(edit.hovered_ids.into_iter().map(Value::String).collect())),
    ]);
    std::fs::write(LARGE_HOVER_EVIDENCE, semio_framework_pack_json::to_string(&evidence)).unwrap();
}
