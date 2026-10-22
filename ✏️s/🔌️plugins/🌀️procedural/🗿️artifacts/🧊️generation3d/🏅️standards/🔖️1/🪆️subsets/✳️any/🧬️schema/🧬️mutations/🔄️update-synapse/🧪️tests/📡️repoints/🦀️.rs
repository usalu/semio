//! 🧪️ `update-synapse` fixture — `📡️repoints`.
//!
//! `update-synapse`'s diff builder no-ops on an identical spec; otherwise it hands the helper index `0` and relies on the id-keyed in-place replacement, so `wire-ab` keeps its own slot in the synapse list.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::diff::Generation3dDiffRead;
use semio_s_artifact_procedural_3d_mutations::inverse_generation3d_mutation;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshotRead;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_mutations::central_apply::apply_generation3d_mutation;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔄️update-synapse/📡️repoints/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔄️update-synapse/📡️repoints/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔄️update-synapse/📡️repoints/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔄️update-synapse/📡️repoints/🔺️diff/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔄️update-synapse/📡️repoints/🎯️outcome/🔣️.json"));

fn before() -> Generation3dSnapshotRead {
    Generation3dSnapshotRead::new(semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes"))
}
fn expected_after() -> Generation3dSnapshotRead {
    Generation3dSnapshotRead::new(semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes"))
}
fn mutation() -> Generation3dMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn raised_diff(base: &Generation3dSnapshot) -> (Generation3dDiffRead, Vec<protocol::MutationMessage>) {
    let (diff, messages) = <Generation3dMutation as protocol::ColdMutation<Generation3dSnapshot>>::diff(&mutation(), base).into_parts();
    (Generation3dDiffRead::new(diff), messages)
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_generation3d_mutation(&mut snapshot, &mutation()).expect("update-synapse applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "update-synapse/repoints-wire-ab-onto-the-cap-port: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_generation3d_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    let mut snapshot = Generation3dSnapshotRead::new((*base).clone());
    apply_generation3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_generation3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-synapse/repoints-wire-ab-onto-the-cap-port: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded = Generation3dSnapshotRead::new(semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes"));
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&*decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-synapse/repoints-wire-ab-onto-the-cap-port: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str((*MUTATION)).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-synapse/repoints-wire-ab-onto-the-cap-port: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str((*OUTCOME)).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let base = before();
    let (_delta, messages) = raised_diff(&base);
    let produced: Vec<(String, String)> = messages
        .iter()
        .map(|message| {
            let level = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "update-synapse/repoints-wire-ab-onto-the-cap-port: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = Generation3dSnapshotRead::new((*base).clone());
    let applied = apply_generation3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => {
            assert!(applied, "update-synapse/repoints-wire-ab-onto-the-cap-port: declared applied but the mutation was rejected");
            assert_ne!(snapshot, base, "update-synapse/repoints-wire-ab-onto-the-cap-port: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, base, "update-synapse/repoints-wire-ab-onto-the-cap-port: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("update-synapse/repoints-wire-ab-onto-the-cap-port: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields `update-synapse` is
/// allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let (delta, _messages) = raised_diff(&base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&*delta)).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str((*DIFF)).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-synapse/repoints-wire-ab-onto-the-cap-port: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded = Generation3dDiffRead::new(semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes"));
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&*decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str((*DIFF)).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-synapse/repoints-wire-ab-onto-the-cap-port: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `update-synapse` changed, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded = Generation3dDiffRead::new(semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes"));
    let produced = Generation3dSnapshotRead::new(
        protocol::apply_diff_cold(&*decoded, &before()).expect("committed diff applies to the before-snapshot"),
    );
    assert_eq!(produced, expected_after(), "update-synapse/repoints-wire-ab-onto-the-cap-port: committed diff did not carry before to after");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff and carry the after-state back to `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
