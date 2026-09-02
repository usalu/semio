//! 🧪️ `change-data-fields` fixture — `attaches-a-data-fields-payload`.
//!
//! Proves the opaque `data_fields_json` blob is replaced wholesale.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> LayoutSnapshot {
    serde_json::from_str(BEFORE).expect("change-data-fields/attaches-a-data-fields-payload: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("change-data-fields/attaches-a-data-fields-payload: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("change-data-fields/attaches-a-data-fields-payload: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("change-data-fields applies to its committed before-snapshot")
}

/// ▶️ `change-data-fields` stores the payload string byte-for-byte, unparsed.
#[semio_framework_async_macros::async_test]
async fn stores_the_opaque_json_blob_verbatim() {
    let after = applied();
    assert_eq!(after.data_fields_json.as_deref(), Some("{\"client\":\"acme\"}"), "change-data-fields must store the payload JSON string verbatim");
    assert!(after.print_target.is_none(), "change-data-fields must not touch the print target");
    assert_eq!(after.name, "Fixture Layout", "change-data-fields must not rename the document");
    assert_eq!(after, expected_after(), "change-data-fields/attaches-a-data-fields-payload: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse restores BASE's absent payload, i.e. clears the field.
#[semio_framework_async_macros::async_test]
async fn inverse_clears_the_data_fields_payload() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "change-data-fields inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ChangeDataFields(step) => assert!(step.new_json.is_none(), "the inverse must carry BASE's absent data-fields payload"),
        other => panic!("change-data-fields must invert to change-data-fields, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("change-data-fields/attaches-a-data-fields-payload: inverse step applies");
    }
    assert_eq!(snapshot, base, "change-data-fields/attaches-a-data-fields-payload: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-data-fields/attaches-a-data-fields-payload: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-data-fields/attaches-a-data-fields-payload: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-data-fields`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-data-fields/attaches-a-data-fields-payload: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "change-data-fields/attaches-a-data-fields-payload: declared clean-applied but the diff builder reported {:?}", produced.messages());
    assert_eq!(produced.diff().data_fields_json, Some(Some("{\"client\":\"acme\"}".to_string())), "change-data-fields fills the doubly-optional `data_fields_json` diff field");
    assert!(produced.diff().print_target.is_none(), "change-data-fields leaves `print_target` untouched in the diff");
}

/// 🔺️ The sparse delta `change-data-fields` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the ONLY populated field is `dataFieldsJson`, carrying the opaque blob verbatim.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-data-fields/attaches-a-data-fields-payload: change-data-fields must emit a diff whose sole populated field is `dataFieldsJson`");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-data-fields/attaches-a-data-fields-payload: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `change-data-fields` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-data-fields/attaches-a-data-fields-payload: committed diff did not carry before to after");
}
