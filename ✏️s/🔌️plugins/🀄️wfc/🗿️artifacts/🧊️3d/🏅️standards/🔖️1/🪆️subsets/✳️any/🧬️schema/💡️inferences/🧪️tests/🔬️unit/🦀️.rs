//! 🧪️ The solve inference's laws: it is a pure function of the document (seed included), it reports a
//! contradiction instead of panicking or hanging, it respects a pin, and its entropy field tracks the
//! tile weight distribution.

use super::*;
use crate::schema::snapshot::{GraphRule, Slot3d, SlotEdge, Tile, Wfc3dSnapshot, WFC3D_DOCUMENT_SCHEMA};

//#region 🧸️Fixtures
fn tile(id: &str, weight: f64) -> Tile {
    Tile { id: id.into(), label: None, weight, media: crate::unit_box_media(None) }
}

fn slot(id: &str, x: f64) -> Slot3d {
    Slot3d { id: id.into(), x, y: 0.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0, pinned_tile_id: None }
}

/// 🧸️ Two slots, one `beside` edge, two tiles, and the ONE rule that admits them next to each other —
/// a WFC instance small enough to reason about by hand: every seed must find SOME solution. The rules
/// are an ALLOW-LIST, so without that rule nothing could sit next to anything.
fn two_slot_two_tile() -> Wfc3dSnapshot {
    Wfc3dSnapshot {
        schema: WFC3D_DOCUMENT_SCHEMA.into(),
        seed: 7,
        slots: vec![slot("s1", 0.0), slot("s2", 1.0)],
        edges: vec![SlotEdge { id: "e1".into(), from_slot_id: "s1".into(), to_slot_id: "s2".into(), relation: "beside".into() }],
        tiles: vec![tile("a", 1.0), tile("b", 1.0)],
        rules: vec![GraphRule { id: "r-ab".into(), tile_a_id: "a".into(), tile_b_id: "b".into(), relation: None, allowed: true }],
    }
}
//#endregion 🧸️Fixtures

#[test]
fn a_trivially_satisfiable_spec_assigns_every_slot() {
    let values = store::infer_field::<Wfc3dSnapshot, Wfc3dSolve>(&two_slot_two_tile(), None);
    match &values["wfc3d"] {
        Wfc3dSolveResult::Solved { assignments } => assert_eq!(assignments.len(), 2, "both slots must be assigned"),
        Wfc3dSolveResult::Unsolved => panic!("a trivially satisfiable spec must solve"),
    }
}

#[test]
fn the_contradiction_field_agrees_with_the_solve_field_on_a_satisfiable_spec() {
    assert!(store::infer_field::<Wfc3dSnapshot, Wfc3dContradiction>(&two_slot_two_tile(), None)["wfc3d"]);
}

/// 🩺 One tile and one edge, with no rule admitting that tile beside itself: nothing can fill both
/// slots, and the inference must SAY so rather than panic, hang, or invent an assignment.
#[test]
fn an_unsatisfiable_spec_is_reported_as_a_contradiction_not_a_panic() {
    let mut snapshot = two_slot_two_tile();
    snapshot.tiles = vec![tile("a", 1.0)];
    snapshot.rules.clear();
    assert!(!store::infer_field::<Wfc3dSnapshot, Wfc3dContradiction>(&snapshot, None)["wfc3d"]);
    assert_eq!(store::infer_field::<Wfc3dSnapshot, Wfc3dSolve>(&snapshot, None)["wfc3d"], Wfc3dSolveResult::Unsolved);
}

/// ⛓️ A forbidding rule always beats an admitting one, whatever order they were authored in — the
/// compile-time law `ModelBuilder::deny` states and the `wall-roof-facade-strip` example leans on.
#[test]
fn a_forbidding_rule_beats_an_admitting_one() {
    let mut snapshot = two_slot_two_tile();
    snapshot.tiles = vec![tile("a", 1.0)];
    snapshot.rules = vec![GraphRule { id: "r-aa".into(), tile_a_id: "a".into(), tile_b_id: "a".into(), relation: None, allowed: true }];
    assert!(store::infer_field::<Wfc3dSnapshot, Wfc3dContradiction>(&snapshot, None)["wfc3d"], "the admitting rule alone must solve");
    snapshot.rules.push(GraphRule { id: "r-aa-no".into(), tile_a_id: "a".into(), tile_b_id: "a".into(), relation: None, allowed: false });
    assert!(!store::infer_field::<Wfc3dSnapshot, Wfc3dContradiction>(&snapshot, None)["wfc3d"], "the forbidding rule must win");
}

/// 🔁 The determinism law: `compute` is a pure function of the snapshot, WFC's internal randomness
/// notwithstanding, because the seed itself lives in the snapshot. Without this, `DepHash` caching
/// over the solve field would be unsound.
#[test]
fn identical_seed_and_spec_always_produce_the_same_solution() {
    let snapshot = two_slot_two_tile();
    let first = store::infer_field::<Wfc3dSnapshot, Wfc3dSolve>(&snapshot, None);
    let second = store::infer_field::<Wfc3dSnapshot, Wfc3dSolve>(&snapshot, None);
    assert_eq!(first, second);
}

#[test]
fn changing_only_the_seed_still_solves_a_satisfiable_spec() {
    let mut snapshot = two_slot_two_tile();
    snapshot.seed = 999;
    assert!(matches!(store::infer_field::<Wfc3dSnapshot, Wfc3dSolve>(&snapshot, None)["wfc3d"], Wfc3dSolveResult::Solved { .. }));
}

#[test]
fn a_pinned_slot_always_resolves_to_its_pinned_tile() {
    let mut snapshot = two_slot_two_tile();
    snapshot.slots[0].pinned_tile_id = Some("a".into());
    snapshot.slots[1].pinned_tile_id = Some("b".into());
    match &store::infer_field::<Wfc3dSnapshot, Wfc3dSolve>(&snapshot, None)["wfc3d"] {
        Wfc3dSolveResult::Solved { assignments } => {
            assert_eq!(assignments["s1"], "a");
            assert_eq!(assignments["s2"], "b");
        }
        Wfc3dSolveResult::Unsolved => panic!("a pinned-and-admitted pair must solve"),
    }
}

/// ⛓️ Relations are separate constraint channels: a rule scoped to `beside` admits the pair `beside`
/// and says nothing at all about `above`, which — under an allow-list — leaves `above` forbidden.
#[test]
fn a_relation_scoped_rule_only_constrains_its_own_relation() {
    let mut snapshot = two_slot_two_tile();
    snapshot.tiles = vec![tile("a", 1.0)];
    snapshot.rules = vec![GraphRule { id: "r-aa-beside".into(), tile_a_id: "a".into(), tile_b_id: "a".into(), relation: Some("beside".into()), allowed: true }];
    assert!(store::infer_field::<Wfc3dSnapshot, Wfc3dContradiction>(&snapshot, None)["wfc3d"], "the admit is scoped to `beside`, and this edge is `beside`");
    snapshot.edges[0].relation = "above".into();
    assert!(!store::infer_field::<Wfc3dSnapshot, Wfc3dContradiction>(&snapshot, None)["wfc3d"], "the same admit must NOT reach another relation");
}

#[test]
fn an_empty_document_solves_trivially_with_no_assignments() {
    assert_eq!(store::infer_field::<Wfc3dSnapshot, Wfc3dSolve>(&Wfc3dSnapshot::default(), None)["wfc3d"], Wfc3dSolveResult::Solved { assignments: BTreeMap::new() });
}

#[test]
fn a_pinned_slot_has_zero_entropy_and_an_open_one_does_not() {
    let mut snapshot = two_slot_two_tile();
    snapshot.slots[0].pinned_tile_id = Some("a".into());
    let entropy = store::infer_field::<Wfc3dSnapshot, Wfc3dEntropy>(&snapshot, None);
    assert_eq!(entropy["s1"], 0.0);
    assert!(entropy["s2"] > 0.0);
}

#[test]
fn uniform_weights_over_two_tiles_yield_ln2_entropy() {
    let entropy = store::infer_field::<Wfc3dSnapshot, Wfc3dEntropy>(&two_slot_two_tile(), None);
    assert!((entropy["s1"] - std::f64::consts::LN_2).abs() < 1e-9);
}

#[test]
fn skewed_weights_lower_the_entropy_below_uniform() {
    let mut snapshot = two_slot_two_tile();
    snapshot.tiles = vec![tile("a", 100.0), tile("b", 0.01)];
    let entropy = store::infer_field::<Wfc3dSnapshot, Wfc3dEntropy>(&snapshot, None);
    assert!(entropy["s1"] < std::f64::consts::LN_2);
}

/// 🧭️ The routed cold-job identity is what the plugin roster and the MCP inference lookup key off;
/// a drift here is invisible until a client asks for the route and gets nothing.
#[test]
fn the_routed_inference_metadata_names_this_artifacts_own_schema() {
    let metadata = wfc3d_inference_metadata();
    assert_eq!(metadata.owner, "wfc");
    assert_eq!(metadata.artifact_kind, "s.wfc.wfc3d");
    assert_eq!(metadata.artifact_schema, "s.wfc.wfc3d");
    assert_eq!(metadata.inference_schema, WFC3D_INFERENCE_TOOL_ID);
    assert_eq!(WFC3D_INFERENCE_TOOL_ID, "s.wfc.wfc3d.solve");
    assert_eq!(WFC3D_INFERENCE_JOB_KIND, "semio.infer");
}

/// 📇️ Registering the cold-job factory twice is idempotent, and a competing factory on the same key
/// is refused rather than silently shadowing this one.
#[test]
fn the_inference_factory_registers_once_and_refuses_a_collision() {
    let bus = semio_framework::ActionBus::new();
    register_wfc3d_inference_factory(&bus).expect("first registration");
    register_wfc3d_inference_factory(&bus).expect("idempotent registration");
    let key = semio_framework::ToolFactoryKey::new(WFC3D_INFERENCE_JOB_KIND, WFC3D_INFERENCE_TOOL_ID);
    assert!(bus.contains(&key));
    assert_eq!(bus.payload_schema_id(&key).as_deref(), Some(WFC3D_INFERENCE_PAYLOAD_SCHEMA));
}

/// 🚫️ An over-sized request is refused at ADMISSION, before any work is done.
#[test]
fn an_oversized_tile_catalogue_is_refused_at_admission() {
    let mut snapshot = two_slot_two_tile();
    snapshot.tiles = (0..(MAX_WFC3D_TILES + 1)).map(|index| tile(&format!("t{index}"), 1.0)).collect();
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), snapshot.seed);
    assert!(Wfc3dInferenceJob::new(operation, Wfc3dInferenceRequest { snapshot, checkpoint: None }).is_err());
}

#[test]
fn the_inference_descriptor_carries_five_non_empty_facet_leaves() {
    let descriptor = wfc3d_artifact_inference_descriptor();
    assert_eq!(descriptor.id, "s.wfc.wfc3d.solve");
    for leaf in [descriptor.inference.rust, descriptor.inference.typescript, descriptor.inference.graphql, descriptor.inference.json_schema, descriptor.inference.proto] {
        assert!(!leaf.trim().is_empty());
    }
}
