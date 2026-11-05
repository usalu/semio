//! 🧪️ Interaction topology tests: a hand-built document, scene and report, and a scene far beyond the 512-byte action argument the retired touch-menu payload could carry.

use super::*;
use semio_s_plugin_norm_compliance::{CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, SubjectRef};
use semio_s_plugin_norm_structural_scene::{Aabb, Appearance, EntityKind, EntityRef, Geometry, LocalizedCopy as Text, MaterialRole, PickPolicy, SceneEntity, SceneRevision, Vec3, SCENE_SCHEMA_ID};

fn entity(kind: EntityKind, id: &str) -> SceneEntity {
    SceneEntity {
        entity: EntityRef::new(kind, id, format!("members[id={id}]")),
        geometry: Geometry::Group { members: Vec::new() },
        appearance: Appearance::role(MaterialRole::Steel),
        utilisation: None,
        label: None,
        accessible_name: Text::new(id, id),
        accessible_detail: Text::new("", ""),
        layer: "members".into(),
        pick: PickPolicy::Whole,
        handles: Vec::new(),
        content_hash: String::new(),
    }
    .sealed()
}

fn scene(entities: Vec<SceneEntity>) -> StructuralScene {
    StructuralScene {
        schema: SCENE_SCHEMA_ID.into(),
        revision: SceneRevision { document_seq: 1, report_seq: 1 },
        bounds: Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
        entities,
        annotations: Vec::new(),
        legends: Vec::new(),
        layers: Vec::new(),
        tracks: Default::default(),
        owners: Vec::new(),
        description: Text::new("scene", "Szene"),
    }
}

fn row(id: &str) -> DslValue {
    DslValue::Object(vec![("id".to_string(), DslValue::String(id.into()))])
}

fn check(id: &str, entity: &str) -> CheckResult {
    CheckResult::assess(id, "ULS", ClauseId::new("EN 1993", "1-1", "6.2.9"), SubjectRef::new(entity, "path", LocalizedCopy::new(entity, entity)), LocalizedCopy::new(id, id))
        .utilization(Quantity::new(QuantityKind::Length, 0.5), Quantity::new(QuantityKind::Length, 1.0))
        .build()
}

fn address(kind: &str, id: &str) -> String {
    crate::protocol::wire::encode_scoped_identity(&[kind, id]).expect("address")
}

fn select_all(topology: &DomainTopology, granularity: &str) -> Vec<String> {
    topology.ordered.iter().filter(|node| node.granularity == granularity).map(|node| node.id.clone()).collect()
}

#[test]
fn the_topology_names_scene_entities_outliner_rows_and_check_rows_once_each() {
    let tree = DslValue::Object(vec![("members".to_string(), DslValue::Array(vec![row("b1"), row("b2")])), ("title".to_string(), DslValue::String("t".into()))]);
    let scene = scene(vec![entity(EntityKind::Member, "b1"), entity(EntityKind::Member, "b2"), entity(EntityKind::Support, "s1")]);
    let mut report = CheckReport::default();
    report.push(check("c1", "b1"));
    report.push(check("c1", "b1"));
    let topology = domain_topology(&tree, &scene, &report, None);
    assert_eq!(select_all(&topology, "member"), [address("member", "b1"), address("member", "b2")], "Select all picks the active granularity in scene order");
    assert_eq!(select_all(&topology, "support"), [address("support", "s1")]);
    assert_eq!(select_all(&topology, "members"), [address("members", "b1"), address("members", "b2")], "outliner rows stay selectable");
    assert_eq!(select_all(&topology, "check"), [address("check", "c1")], "a duplicated check id is one node");
    let ids: HashSet<&String> = topology.ordered.iter().map(|node| &node.id).collect();
    assert_eq!(ids.len(), topology.ordered.len());
    assert!(topology.ordered.iter().all(|node| node.parent.is_none()));
}

#[test]
fn an_outliner_filter_narrows_entities_and_rows_but_keeps_every_check() {
    let tree = DslValue::Object(vec![("members".to_string(), DslValue::Array(vec![row("b1"), row("b2")]))]);
    let scene = scene(vec![entity(EntityKind::Member, "b1"), entity(EntityKind::Member, "b2"), entity(EntityKind::Support, "s1")]);
    let mut report = CheckReport::default();
    report.push(check("c1", "b1"));
    let visible = ["b2".to_owned()];
    let topology = domain_topology(&tree, &scene, &report, Some(&visible));
    assert_eq!(select_all(&topology, "member"), [address("member", "b2")]);
    assert!(select_all(&topology, "support").is_empty());
    assert_eq!(select_all(&topology, "members"), [address("members", "b2")]);
    assert_eq!(select_all(&topology, "check"), [address("check", "c1")]);
}

#[test]
fn a_large_scene_is_selected_whole_where_the_action_argument_would_have_overflowed() {
    let entities: Vec<SceneEntity> = (0..2_000).map(|at| entity(EntityKind::Member, &format!("member-{at}"))).collect();
    let topology = norm_interaction_topology("norm.en9001", &DslValue::Object(Vec::new()), &scene(entities), &CheckReport::default());
    let all = select_all(&topology.domains["norm.en9001"], "member");
    assert_eq!(all.len(), 2_000);
    assert!(semio_framework_pack_json::to_json_string(&DslValue::Array(all.iter().map(|id| DslValue::String(id.clone())).collect())).len() > 512, "far beyond the 512-byte action argument");
    assert!(norm_interaction_topology("norm.en9001", &DslValue::Null, &scene(Vec::new()), &CheckReport::default()).domains["norm.en9001"].ordered.is_empty(), "an empty document has an empty domain");
}
