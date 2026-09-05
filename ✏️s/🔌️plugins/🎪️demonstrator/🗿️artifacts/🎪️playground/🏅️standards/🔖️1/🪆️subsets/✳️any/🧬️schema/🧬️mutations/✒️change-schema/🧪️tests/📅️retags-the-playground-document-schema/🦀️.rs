//! 🧪️ `change-schema` fixture — `📅️retags-the-playground-document-schema`.
//!
//! The demonstrator playground's whole persistent snapshot is ONE metadata string, so
//! `change-schema` is its entire mutation vocabulary. Its diff oracle is root-scoped — there is no
//! target that could be missing, only the equality guard (`base.schema == new_schema` ⇒ Warning
//! `mutation.no-op`) — and it emits `PlaygroundDiff { schema: Some(..) }`, deliberately leaving the
//! whole-artifact `artifact` replacement slot alone.
//!
//! 🔤️ Serde shape note: `PlaygroundMutation` carries NO `#[serde(tag = ..)]`, so unlike every other
//! artifact in this ticket it encodes EXTERNALLY tagged — `{"ChangeSchema": {..}}` — and its
//! payload struct has no `rename_all` either, so the field stays `new_schema`, not `newSchema`.
//! This fixture is the pin on that.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use crate::artifacts::playground::standards::v1::subsets::any::schema::diff::PlaygroundDiff;
use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::PlaygroundMutation;
use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> PlaygroundSnapshot {
    serde_json::from_str(BEFORE).expect("before playground document decodes")
}
fn expected_after() -> PlaygroundSnapshot {
    serde_json::from_str(AFTER).expect("after playground document decodes")
}
fn mutation() -> PlaygroundMutation {
    serde_json::from_str(MUTATION).expect("change-schema mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<PlaygroundDiff> {
    <PlaygroundMutation as protocol::Mutation<PlaygroundSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Retagging the document moves its one and only field from `playground.playground` to
/// `playground.experiment`.
#[test]
fn retags_the_only_persistent_field() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-schema applies to its committed before-document");
    assert_eq!(applied, expected_after(), "change-schema/retags-the-playground-document-schema: the retagged document differs from the committed after-snapshot");
    assert_eq!(applied.schema, "playground.experiment", "change-schema/retags-the-playground-document-schema: the schema tag must land on the payload's value");
}

/// ↩️ `change-schema`'s inverse re-tags with `base.schema` read out of BASE — never the diff — so
/// undoing restores `playground.playground`.
#[test]
fn retagging_back_restores_before() {
    let base = before();
    let mut snapshot = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("forward change-schema applies");
    let inverse = <PlaygroundMutation as protocol::Mutation<PlaygroundSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-schema/retags-the-playground-document-schema: the inverse of one retag is exactly one retag back");
    for step in &inverse {
        let undo = <PlaygroundMutation as protocol::Mutation<PlaygroundSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-schema inverse step applies");
    }
    assert_eq!(snapshot, base, "change-schema/retags-the-playground-document-schema: retagging back to playground.playground did not restore the before-document");
}

/// 🔣️ Both committed documents and the externally tagged `ChangeSchema` payload are canonical —
/// this is where the `{"ChangeSchema": {"new_schema": ..}}` wire shape is pinned.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PlaygroundSnapshot = serde_json::from_str(text).expect("playground document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("playground document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("playground document reparses");
        assert_eq!(reencoded, original, "change-schema/retags-the-playground-document-schema: committed {label} playground JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("ChangeSchema payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("ChangeSchema payload reparses");
    assert_eq!(reencoded, original, "change-schema/retags-the-playground-document-schema: committed ChangeSchema JSON is not canonical");
}

/// 🎯️ `playground.experiment` differs from the base tag, so the equality guard — this oracle's only
/// guard — does not fire and the declared `applied` outcome must be message-free.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-schema/retags-the-playground-document-schema: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-schema/retags-the-playground-document-schema: a genuinely new tag must not raise mutation.no-op");
    assert!(produced.messages().is_empty(), "change-schema/retags-the-playground-document-schema: an accepted retag emits no diagnostics");
}

/// 🔺️ `PlaygroundDiff` has two slots — the whole-artifact `artifact` replacement and the sparse
/// `schema` field. This mutation may set the second one only.
#[test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("produced change-schema diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-schema/retags-the-playground-document-schema: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `PlaygroundDiff` and re-encodes unchanged — `artifact` stays
/// an explicit `null` because `PlaygroundDiff` carries no `skip_serializing_if`.
#[test]
fn committed_diff_is_canonical() {
    let decoded: PlaygroundDiff = serde_json::from_str(DIFF).expect("committed change-schema diff decodes");
    assert_eq!(decoded.schema.as_deref(), Some("playground.experiment"), "change-schema/retags-the-playground-document-schema: the committed diff must set the new tag");
    assert!(decoded.artifact.is_none(), "change-schema/retags-the-playground-document-schema: a one-field retag must never escalate into a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-schema/retags-the-playground-document-schema: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: PlaygroundDiff = serde_json::from_str(DIFF).expect("committed change-schema diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "change-schema/retags-the-playground-document-schema: committed diff did not carry before to after");
}
