/// ✂️ Node delete, label and move: the unit tests of the delete node, delete nodes, change node label and move node leaves, one module per leaf.

mod delete_node {
//! 🧪️ `delete-node` fixture — `🚫️rejects-deleting-a-node-that-is-not-in-the-graph`.
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/❌️delete-node/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/❌️delete-node/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/❌️delete-node/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/❌️delete-node/🧪️rejects/🎯️outcome/🔣️.json"));

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
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `delete-node` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.nodes.is_empty(), "rejects-deleting-a-node-that-is-not-in-the-graph's before-snapshot must hold a node-less graph");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "delete-node/rejects-deleting-a-node-that-is-not-in-the-graph: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected delete must not mint a fresh notation/results/computed triple");
}

/// ❌️ `delete-node` is the SINGULAR, id-keyed delete: it names exactly one node id, and — this is
/// the branch that separates it from a successful delete — it never reaches its edge cascade, so
/// no `mutation.cascade` Info message accompanies the refusal.
#[semio_framework_async_macros::async_test]
async fn a_missing_node_is_reported_alone_with_no_cascade_notice() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting delete-node must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "a refused delete emits the miss and nothing else, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing node is reported as target-missing");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "delete-node has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string()], "the diagnostic names the single node id the payload carried");
    assert!(messages.iter().all(|message| message.code.0 != "mutation.cascade"), "the edge cascade is unreachable when the node was never found, got {messages:?}");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "node", "delete-node", "DeletedNode"), "the fixture must be bound to delete-node's own SINGULAR descriptor");
}

/// ↩️ `delete-node` inverts by re-creating the node and re-`connect`ing every edge it severed, all
/// captured from BASE. With no such node the inverse is empty — never a speculative create.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_node_to_recreate() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "delete-node/rejects-deleting-a-node-that-is-not-in-the-graph: a rejected delete must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed
/// point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "delete-node/rejects-deleting-a-node-that-is-not-in-the-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "delete-node/rejects-deleting-a-node-that-is-not-in-the-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "delete-node/rejects-deleting-a-node-that-is-not-in-the-graph declares a rejected outcome");
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

mod delete_nodes {
//! 🧪️ `delete-nodes` fixture — `🚫️rejects-a-bulk-delete-where-every-id-is-absent`.
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-nodes/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-nodes/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-nodes/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-nodes/🧪️rejects/🎯️outcome/🔣️.json"));

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
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `delete-nodes` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.nodes.is_empty(), "rejects-a-bulk-delete-where-every-id-is-absent's before-snapshot must hold a node-less graph");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected bulk delete must not mint a fresh notation/results/computed triple");
}

/// 🗑️ The PLURAL verb is all-or-nothing at its entry gate: only when NOT ONE requested id exists
/// does it refuse, and then it reports EVERY requested id in a single diagnostic — not one per id,
/// and not just the first. The partial-hit `mutation.partial` warning is unreachable here, because
/// that branch lives past the all-missing short circuit.
#[semio_framework_async_macros::async_test]
async fn every_requested_id_travels_in_one_all_missing_diagnostic() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting delete-nodes must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "the bulk refusal is a single diagnostic, never one per id, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "an all-missing bulk delete is reported as target-missing");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "delete-nodes has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string(), "n-beta".to_string()], "the diagnostic carries the WHOLE requested id list, in payload order");
    assert!(messages.iter().all(|message| message.code.0 != "mutation.partial" && message.code.0 != "mutation.cascade"), "neither the partial warning nor the cascade notice is reachable when nothing existed, got {messages:?}");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("delete", "nodes", "delete-nodes", "DeletedNodes"),
        "the fixture must be bound to delete-nodes' own PLURAL descriptor — the sibling singular verb is `delete-node`/`DeletedNode`"
    );
}

/// ↩️ `delete-nodes` inverts by re-creating every deleted node then re-`connect`ing every severed
/// edge, all captured from BASE. With no matching node the inverse is empty.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_nodes_to_recreate() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent: a rejected bulk delete must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The plural payload's one
/// field is `ids` — a bare `Vec<String>`, never a nested selection object.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/DeleteNodes/ids").and_then(semio_framework_pack_json::Value::as_array).map(|ids| ids.len()), Some(2), "the committed payload must request two ids for the bulk branch to be meaningful");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(semio_framework_pack_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(semio_framework_pack_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target — for the plural verb that is the whole id list");
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

mod change_node_label {
//! 🧪️ `change-node-label` fixture — `🚫️rejects-relabelling-a-node-that-is-not-in-the-graph`.
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🏷️change-node-label/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🏷️change-node-label/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🏷️change-node-label/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🏷️change-node-label/🧪️rejects/🎯️outcome/🔣️.json"));

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
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `change-node-label` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.nodes.is_empty(), "rejects-relabelling-a-node-that-is-not-in-the-graph's before-snapshot must hold a node-less graph");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected relabel must not mint a fresh notation/results/computed triple");
}

/// 🏷️ `id` is this node's stable identity and is NEVER rewritten — which is exactly why the verb is
/// `change-node-label` and not `rename-node`. The diagnostic therefore addresses the untouched
/// `id`, and the requested `new_label` appears nowhere in it. The already-has-that-label
/// `mutation.no-op` guard sits BEHIND this lookup, so an absent node can never reach it.
#[semio_framework_async_macros::async_test]
async fn the_diagnostic_addresses_the_stable_id_never_the_new_label() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting change-node-label must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing node is reported as target-missing, never as a no-op");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "change-node-label has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string()], "the diagnostic names the stable node id");
    assert!(!messages[0].target.contains(&"Alpha".to_string()), "the requested label must never leak into the diagnostic target");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "node", "change-node-label", "ChangedNodeLabel"), "the fixture must be bound to change-node-label's own descriptor — `change`, never `rename`");
}

/// ↩️ The undo carries BASE's own label for that id. With no such node the inverse is empty, so a
/// refused relabel can never be replayed as an accidental label wipe.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_prior_label_to_restore() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph: a rejected relabel must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The payload field is
/// camelCase (`newLabel`): `EquationMutation`'s leaf structs carry `#[value(rename_all = "camelCase")]`
/// like the nested document types.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/ChangeNodeLabel/newLabel").and_then(semio_framework_pack_json::Value::as_str), Some("Alpha"), "the payload's label field commits camelCase");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph declares a rejected outcome");
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

mod move_node {
//! 🧪️ `move-node` fixture — `🚫️rejects-moving-a-node-that-is-not-in-the-graph`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Model (a) (design §20.15): the committed snapshot carries its graph and its point cloud INLINE as parent-owned
//! fields, every leaf decides from them, and the `notation`/`results`/`computed` handles are the content addresses of their
//! derivation (`crate::equation_children`), kept exact by the fixture writer law. This vector pins the outcome its
//! `🎯️outcome` declares on that committed state.

use crate::standards::v1::subsets::graph::schema::mutations::move_node::MoveNode;
use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/🎯️outcome/🔣️.json"));

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
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `move-node` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.nodes.is_empty(), "rejects-moving-a-node-that-is-not-in-the-graph's before-snapshot must hold a node-less graph");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "move-node/rejects-moving-a-node-that-is-not-in-the-graph: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected move must not mint a fresh notation/results/computed triple");
}

/// 🕹️ `move-node` is ID-keyed where its geometry twin `move-point` is index-keyed, and it stacks
/// three guards in a fixed order: id lookup, then the finiteness invariant, then the
/// already-there no-op. An absent id short-circuits all of them, so even a NaN destination is
/// refused as the Error `mutation.target-missing` rather than the Fatal `mutation.invariant`.
#[semio_framework_async_macros::async_test]
async fn the_id_lookup_precedes_the_finiteness_invariant() {
    let base = before();
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting move-node must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing node is reported as target-missing");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "a missing node is an Error, not the Fatal reserved for a non-finite position");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string()], "the diagnostic names the node id, never its requested coordinates");

    let non_finite = EquationMutation::MoveNode(MoveNode { id: "n-alpha".to_string(), x: f64::NAN, y: 0.0 });
    let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&non_finite, &base);
    assert_eq!(outcome.messages()[0].code.0, "mutation.target-missing", "with no node under that id, even a NaN destination is refused as target-missing — the lookup guard wins");

    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("move", "node", "move-node", "MovedNode"), "the fixture must be bound to move-node's own descriptor");
}

/// ↩️ `move-node` inverts to another `move-node` carrying the coordinates BASE held for that id.
/// With no such node the inverse is empty — base-derived, never payload-derived.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_prior_position_to_restore() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "move-node/rejects-moving-a-node-that-is-not-in-the-graph: a rejected move must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The destination is
/// ABSOLUTE, not a delta — two `f64`s that always re-encode with a `.0`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "move-node/rejects-moving-a-node-that-is-not-in-the-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "move-node/rejects-moving-a-node-that-is-not-in-the-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/MoveNode/x").and_then(semio_framework_pack_json::Value::as_f64), Some(120.0), "the payload commits an absolute destination, not an offset");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "move-node/rejects-moving-a-node-that-is-not-in-the-graph declares a rejected outcome");
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
