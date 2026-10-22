/// 🔗️ Node connect and disconnect: the unit tests of the connect nodes and disconnect nodes leaves, one module per leaf.

mod connect_nodes {
//! 🧪️ `connect-nodes` fixture — `🚫️rejects-an-edge-between-two-absent-endpoints`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Model (a) (design §20.15): the committed snapshot carries its graph and its point cloud INLINE as parent-owned
//! fields, every leaf decides from them, and the `notation`/`results`/`computed` handles are the content addresses of their
//! derivation (`crate::equation_children`), kept exact by the fixture writer law. This vector pins the outcome its
//! `🎯️outcome` declares on that committed state.

use crate::standards::v1::subsets::graph::schema::mutations::disconnect_nodes::DisconnectNodes;
use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🔗️connect-nodes/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🔗️connect-nodes/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🔗️connect-nodes/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🔗️connect-nodes/🧪️rejects/🎯️outcome/🔣️.json"));

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::ColdMutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `connect-nodes` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.nodes.is_empty() && base.graph.edges.is_empty(), "rejects-an-edge-between-two-absent-endpoints' before-snapshot must hold a empty graph");
    let applied = protocol::apply_diff_cold(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "connect-nodes/rejects-an-edge-between-two-absent-endpoints: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected connect must not mint a fresh notation/results/computed triple");
}

/// 🔗️ `connect-nodes` is the only verb in this vocabulary whose diagnostic names something OTHER
/// than the entity it was asked to create: the edge id `e-alpha-beta` never appears — the target is
/// the list of missing ENDPOINT node ids, both of them, in source-then-target order. (Its own id is
/// only ever reported by the earlier Fatal `mutation.duplicate-id` branch, which an edge-less graph
/// cannot reach.)
#[semio_framework_async_macros::async_test]
async fn the_missing_endpoints_are_named_not_the_new_edge_id() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting connect-nodes must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "absent endpoints are reported as target-missing, not duplicate-id");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "a missing endpoint is an Error — the Fatal is reserved for a colliding edge id");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string(), "n-beta".to_string()], "the diagnostic names the endpoint nodes, in source-then-target order");
    assert!(!messages[0].target.contains(&"e-alpha-beta".to_string()), "the edge id must not appear in a target-missing diagnostic");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("connect", "node", "connect-nodes", "ConnectedNodes"), "the fixture must be bound to connect-nodes' own descriptor — a PLURAL kind over a SINGULAR entity");
}

/// ↩️ `connect-nodes`' inverse is PAYLOAD-derived, unlike every other refusing verb in this
/// vocabulary: because BASE holds no edge under that id, the undo is a `disconnect-nodes` of the id
/// it was asked to create, even though the connect itself was refused.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_disconnect_of_the_requested_edge_id() {
    let inverse = <EquationMutation as protocol::ColdMutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![EquationMutation::DisconnectNodes(DisconnectNodes { id: "e-alpha-beta".to_string() })], "connect-nodes undoes with exactly one disconnect of the requested edge id, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The payload carries three
/// flat id strings — `id`, `source`, `target` — never a nested endpoint object.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "connect-nodes/rejects-an-edge-between-two-absent-endpoints: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "connect-nodes/rejects-an-edge-between-two-absent-endpoints: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/ConnectNodes/source").and_then(semio_framework_pack_json::Value::as_str), Some("n-alpha"), "the payload addresses its endpoints by bare node id");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "connect-nodes/rejects-an-edge-between-two-absent-endpoints declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(semio_framework_pack_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(semio_framework_pack_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod disconnect_nodes {
//! 🧪️ `disconnect-nodes` fixture — `🚫️rejects-severing-an-edge-that-is-not-in-the-graph`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Model (a) (design §20.15): the committed snapshot carries its graph and its point cloud INLINE as parent-owned
//! fields, every leaf decides from them, and the `notation`/`results`/`computed` handles are the content addresses of their
//! derivation (`crate::equation_children`), kept exact by the fixture writer law. This vector pins the outcome its
//! `🎯️outcome` declares on that committed state.

use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/🎯️outcome/🔣️.json"));

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::ColdMutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `disconnect-nodes` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.edges.is_empty(), "rejects-severing-an-edge-that-is-not-in-the-graph's before-snapshot must hold a edge-less graph");
    let applied = protocol::apply_diff_cold(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected disconnect must not mint a fresh notation/results/computed triple");
}

/// ✂️ `disconnect-nodes` is this vocabulary's only verb addressed by EDGE id: it searches
/// `graph.edges`, never `graph.nodes`, so the reported target is the edge id verbatim and neither
/// endpoint is ever named. It also has no duplicate/invariant branch — one Error and nothing else.
#[semio_framework_async_macros::async_test]
async fn a_missing_edge_is_reported_by_its_edge_id() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting disconnect-nodes must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing edge is reported as target-missing");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "disconnect-nodes has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["e-alpha-beta".to_string()], "the diagnostic names the EDGE id, not either endpoint node");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("disconnect", "node", "disconnect-nodes", "DisconnectedNodes"), "the fixture must be bound to disconnect-nodes' own descriptor");
}

/// ↩️ `disconnect-nodes` inverts by reconstructing the exact edge BASE showed — id plus both
/// endpoints. With no such edge captured the inverse is empty, the mirror image of its
/// `connect-nodes` twin, whose inverse is payload-derived and therefore never empty.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_edge_to_reconnect() {
    let inverse = <EquationMutation as protocol::ColdMutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: a rejected disconnect must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The payload is a single
/// `id` — the edge's — with no source/target pair, which is what makes this verb edge-addressed.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/DisconnectNodes").and_then(semio_framework_pack_json::Value::as_object).map(|fields| fields.len()), Some(1), "the payload carries the edge id and nothing else");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(semio_framework_pack_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(semio_framework_pack_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}
