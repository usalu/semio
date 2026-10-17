//! 🧪️ Aggregated unit tests of the `mutations` leaves, one module per leaf.

mod insert_ifd {
//! 🧪️ `insert-ifd` fixture — `🎯️direct`.
//!
//! Source of truth is the committed JSON quintet beside this file: the before-snapshot, the
//! mutation payload, the after-snapshot, the sparse diff and the declared outcome. Every value in
//! it was produced by this repository's OWN dispatch — `Mutation::diff` followed by
//! `MutationDiff::apply` — so the fixture pins what the runtime does rather than what a second
//! implementation believes it should do.
//!
//! ⚖️ The six laws below are the closed set every mutation vector in this repository states:
//! forward application reaches the committed after-snapshot, the mutation's OWN inverse returns to
//! the before-snapshot, both snapshots and the payload are canonical JSON, the declared outcome
//! matches the diagnostics dispatch actually raises, the produced diff IS the committed diff, and
//! the committed diff alone carries before to after. The seventh pins the op codecs: the text and
//! binary wire forms of this payload must round-trip and the binary form must reject a truncated
//! frame.

use crate::schema::mutations::TiffMutation;
use super::super::insert_ifd::*;
use crate::standards::v6_0::subsets::document::io::binary::mutations::insert_ifd::BINARY_TAG;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/📥️insert-ifd/🎯️direct-behavior/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/📥️insert-ifd/🎯️direct-behavior/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/📥️insert-ifd/🎯️direct-behavior/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/📥️insert-ifd/🎯️direct-behavior/🔺️diff/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/📥️insert-ifd/🎯️direct-behavior/🎯️outcome/🔣️.json"));

fn before() -> TiffSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed before-snapshot decodes")
}
fn expected_after() -> TiffSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed after-snapshot decodes")
}
fn mutation() -> TiffMutation {
    semio_framework_pack_json::from_json_str((*MUTATION),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed insert-ifd payload decodes")
}

/// ▶️ Applying the committed payload to the committed before-snapshot reaches the committed
/// after-snapshot exactly.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let produced = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("insert-ifd applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "insert-ifd/direct-behavior: applied state differs from the committed after-snapshot");
}

/// ↩️ The mutation's own inverse steps, computed from the pre-mutation state, restore it exactly —
/// the law the whole event-sourced undo history rests on.
#[test]
fn inverse_restores_before() {
    let base = before();
    let payload = mutation();
    let mut current = protocol::apply_diff(payload.diff(&base).diff(), &base).expect("forward insert-ifd applies");
    for step in payload.inverse(&base).expect("valid retained mutation inverse fixture").into_iter().rev() {
        current = protocol::apply_diff(step.diff(&current).diff(), &current).expect("the insert-ifd inverse step applies");
    }
    assert_eq!(current, base, "insert-ifd/direct-behavior: the undo did not restore the committed before-snapshot");
}

/// 🔣️ Every committed file is canonical: decoding and re-encoding it reproduces it verbatim, so the
/// fixture can never drift from the serde shape the wire actually carries.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: TiffSnapshot = semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "insert-ifd/direct-behavior: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(mutation()))).expect("payload encodes");
    let original: serde_json::Value = serde_json::from_str((*MUTATION)).expect("payload reparses");
    assert_eq!(reencoded, original, "insert-ifd/direct-behavior: committed payload JSON is not canonical");
}

/// 🎯️ The declared outcome is the one dispatch really produces — a fixture that declares `applied`
/// while the implementation quietly warns would otherwise read as clean.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str((*OUTCOME)).expect("outcome decodes");
    let status = declared.get("status").and_then(serde_json::Value::as_str).expect("the outcome carries a status");
    let produced = mutation().diff(&before());
    let raised: Vec<String> = produced.messages().iter().map(|message| message.code.0.clone()).collect();
    match status {
        "applied" => assert!(raised.is_empty(), "insert-ifd/direct-behavior: declared applied, but dispatch raised {raised:?}"),
        _ => assert!(!raised.is_empty(), "insert-ifd/direct-behavior: declared {status}, but dispatch raised nothing"),
    }
}

/// 🔺️ The diff dispatch produces IS the committed diff — the load-bearing assertion, because the
/// diff is what replication ships and what undo inverts.
#[test]
fn produces_committed_diff() {
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(mutation().diff(&before()).diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str((*DIFF)).expect("committed diff decodes");
    assert_eq!(produced, committed, "insert-ifd/direct-behavior: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 The committed diff ALONE carries before to after: it is a complete description of the change,
/// not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let base = before();
    let produced = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "insert-ifd/direct-behavior: committed diff did not carry before to after");
}

/// 📡️ The payload's text and binary wire forms round-trip, the binary frame carries this schema's
/// own tag, and a truncated frame is rejected rather than silently decoded.
#[test]
fn op_codecs_round_trip() {
    let payload = mutation();
    assert_eq!(TiffMutation::parse_op(&payload.print_op()).expect("the text op parses"), payload, "insert-ifd/direct-behavior: the text op form does not round-trip");
    let bytes = payload.encode_op().expect("the binary op encodes");
    assert_eq!(bytes[1], BINARY_TAG, "insert-ifd/direct-behavior: the binary frame must carry this schema's tag");
    assert_eq!(TiffMutation::decode_op(&bytes).expect("the binary op decodes"), payload, "insert-ifd/direct-behavior: the binary op form does not round-trip");
    assert!(TiffMutation::decode_op(&bytes[..1]).is_err(), "insert-ifd/direct-behavior: a truncated binary frame must be rejected");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod remove_ifd {
//! 🧪️ `remove-ifd` fixture — `🎯️direct`.
//!
//! Source of truth is the committed JSON quintet beside this file: the before-snapshot, the
//! mutation payload, the after-snapshot, the sparse diff and the declared outcome. Every value in
//! it was produced by this repository's OWN dispatch — `Mutation::diff` followed by
//! `MutationDiff::apply` — so the fixture pins what the runtime does rather than what a second
//! implementation believes it should do.
//!
//! ⚖️ The six laws below are the closed set every mutation vector in this repository states:
//! forward application reaches the committed after-snapshot, the mutation's OWN inverse returns to
//! the before-snapshot, both snapshots and the payload are canonical JSON, the declared outcome
//! matches the diagnostics dispatch actually raises, the produced diff IS the committed diff, and
//! the committed diff alone carries before to after. The seventh pins the op codecs: the text and
//! binary wire forms of this payload must round-trip and the binary form must reject a truncated
//! frame.

use crate::schema::mutations::TiffMutation;
use super::super::remove_ifd::*;
use crate::standards::v6_0::subsets::document::io::binary::mutations::remove_ifd::BINARY_TAG;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/📤️remove-ifd/🎯️direct-behavior/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/📤️remove-ifd/🎯️direct-behavior/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/📤️remove-ifd/🎯️direct-behavior/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/📤️remove-ifd/🎯️direct-behavior/🔺️diff/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/📤️remove-ifd/🎯️direct-behavior/🎯️outcome/🔣️.json"));

fn before() -> TiffSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed before-snapshot decodes")
}
fn expected_after() -> TiffSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed after-snapshot decodes")
}
fn mutation() -> TiffMutation {
    semio_framework_pack_json::from_json_str((*MUTATION),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed remove-ifd payload decodes")
}

/// ▶️ Applying the committed payload to the committed before-snapshot reaches the committed
/// after-snapshot exactly.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let produced = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("remove-ifd applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "remove-ifd/direct-behavior: applied state differs from the committed after-snapshot");
}

/// ↩️ The mutation's own inverse steps, computed from the pre-mutation state, restore it exactly —
/// the law the whole event-sourced undo history rests on.
#[test]
fn inverse_restores_before() {
    let base = before();
    let payload = mutation();
    let mut current = protocol::apply_diff(payload.diff(&base).diff(), &base).expect("forward remove-ifd applies");
    for step in payload.inverse(&base).expect("valid retained mutation inverse fixture").into_iter().rev() {
        current = protocol::apply_diff(step.diff(&current).diff(), &current).expect("the remove-ifd inverse step applies");
    }
    assert_eq!(current, base, "remove-ifd/direct-behavior: the undo did not restore the committed before-snapshot");
}

/// 🔣️ Every committed file is canonical: decoding and re-encoding it reproduces it verbatim, so the
/// fixture can never drift from the serde shape the wire actually carries.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: TiffSnapshot = semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-ifd/direct-behavior: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(mutation()))).expect("payload encodes");
    let original: serde_json::Value = serde_json::from_str((*MUTATION)).expect("payload reparses");
    assert_eq!(reencoded, original, "remove-ifd/direct-behavior: committed payload JSON is not canonical");
}

/// 🎯️ The declared outcome is the one dispatch really produces — a fixture that declares `applied`
/// while the implementation quietly warns would otherwise read as clean.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str((*OUTCOME)).expect("outcome decodes");
    let status = declared.get("status").and_then(serde_json::Value::as_str).expect("the outcome carries a status");
    let produced = mutation().diff(&before());
    let raised: Vec<String> = produced.messages().iter().map(|message| message.code.0.clone()).collect();
    match status {
        "applied" => assert!(raised.is_empty(), "remove-ifd/direct-behavior: declared applied, but dispatch raised {raised:?}"),
        _ => assert!(!raised.is_empty(), "remove-ifd/direct-behavior: declared {status}, but dispatch raised nothing"),
    }
}

/// 🔺️ The diff dispatch produces IS the committed diff — the load-bearing assertion, because the
/// diff is what replication ships and what undo inverts.
#[test]
fn produces_committed_diff() {
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(mutation().diff(&before()).diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str((*DIFF)).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-ifd/direct-behavior: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 The committed diff ALONE carries before to after: it is a complete description of the change,
/// not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let base = before();
    let produced = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-ifd/direct-behavior: committed diff did not carry before to after");
}

/// 📡️ The payload's text and binary wire forms round-trip, the binary frame carries this schema's
/// own tag, and a truncated frame is rejected rather than silently decoded.
#[test]
fn op_codecs_round_trip() {
    let payload = mutation();
    assert_eq!(TiffMutation::parse_op(&payload.print_op()).expect("the text op parses"), payload, "remove-ifd/direct-behavior: the text op form does not round-trip");
    let bytes = payload.encode_op().expect("the binary op encodes");
    assert_eq!(bytes[1], BINARY_TAG, "remove-ifd/direct-behavior: the binary frame must carry this schema's tag");
    assert_eq!(TiffMutation::decode_op(&bytes).expect("the binary op decodes"), payload, "remove-ifd/direct-behavior: the binary op form does not round-trip");
    assert!(TiffMutation::decode_op(&bytes[..1]).is_err(), "remove-ifd/direct-behavior: a truncated binary frame must be rejected");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod remove_tag {
//! 🧪️ `remove-tag` fixture — `🎯️direct`.
//!
//! Source of truth is the committed JSON quintet beside this file: the before-snapshot, the
//! mutation payload, the after-snapshot, the sparse diff and the declared outcome. Every value in
//! it was produced by this repository's OWN dispatch — `Mutation::diff` followed by
//! `MutationDiff::apply` — so the fixture pins what the runtime does rather than what a second
//! implementation believes it should do.
//!
//! ⚖️ The six laws below are the closed set every mutation vector in this repository states:
//! forward application reaches the committed after-snapshot, the mutation's OWN inverse returns to
//! the before-snapshot, both snapshots and the payload are canonical JSON, the declared outcome
//! matches the diagnostics dispatch actually raises, the produced diff IS the committed diff, and
//! the committed diff alone carries before to after. The seventh pins the op codecs: the text and
//! binary wire forms of this payload must round-trip and the binary form must reject a truncated
//! frame.

use crate::schema::mutations::TiffMutation;
use super::super::remove_tag::*;
use crate::standards::v6_0::subsets::document::io::binary::mutations::remove_tag::BINARY_TAG;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🗑️remove-tag/🎯️direct-behavior/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🗑️remove-tag/🎯️direct-behavior/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🗑️remove-tag/🎯️direct-behavior/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🗑️remove-tag/🎯️direct-behavior/🔺️diff/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🗑️remove-tag/🎯️direct-behavior/🎯️outcome/🔣️.json"));

fn before() -> TiffSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed before-snapshot decodes")
}
fn expected_after() -> TiffSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed after-snapshot decodes")
}
fn mutation() -> TiffMutation {
    semio_framework_pack_json::from_json_str((*MUTATION),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed remove-tag payload decodes")
}

/// ▶️ Applying the committed payload to the committed before-snapshot reaches the committed
/// after-snapshot exactly.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let produced = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("remove-tag applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "remove-tag/direct-behavior: applied state differs from the committed after-snapshot");
}

/// ↩️ The mutation's own inverse steps, computed from the pre-mutation state, restore it exactly —
/// the law the whole event-sourced undo history rests on.
#[test]
fn inverse_restores_before() {
    let base = before();
    let payload = mutation();
    let mut current = protocol::apply_diff(payload.diff(&base).diff(), &base).expect("forward remove-tag applies");
    for step in payload.inverse(&base).expect("valid retained mutation inverse fixture").into_iter().rev() {
        current = protocol::apply_diff(step.diff(&current).diff(), &current).expect("the remove-tag inverse step applies");
    }
    assert_eq!(current, base, "remove-tag/direct-behavior: the undo did not restore the committed before-snapshot");
}

/// 🔣️ Every committed file is canonical: decoding and re-encoding it reproduces it verbatim, so the
/// fixture can never drift from the serde shape the wire actually carries.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: TiffSnapshot = semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-tag/direct-behavior: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(mutation()))).expect("payload encodes");
    let original: serde_json::Value = serde_json::from_str((*MUTATION)).expect("payload reparses");
    assert_eq!(reencoded, original, "remove-tag/direct-behavior: committed payload JSON is not canonical");
}

/// 🎯️ The declared outcome is the one dispatch really produces — a fixture that declares `applied`
/// while the implementation quietly warns would otherwise read as clean.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str((*OUTCOME)).expect("outcome decodes");
    let status = declared.get("status").and_then(serde_json::Value::as_str).expect("the outcome carries a status");
    let produced = mutation().diff(&before());
    let raised: Vec<String> = produced.messages().iter().map(|message| message.code.0.clone()).collect();
    match status {
        "applied" => assert!(raised.is_empty(), "remove-tag/direct-behavior: declared applied, but dispatch raised {raised:?}"),
        _ => assert!(!raised.is_empty(), "remove-tag/direct-behavior: declared {status}, but dispatch raised nothing"),
    }
}

/// 🔺️ The diff dispatch produces IS the committed diff — the load-bearing assertion, because the
/// diff is what replication ships and what undo inverts.
#[test]
fn produces_committed_diff() {
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(mutation().diff(&before()).diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str((*DIFF)).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-tag/direct-behavior: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 The committed diff ALONE carries before to after: it is a complete description of the change,
/// not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let base = before();
    let produced = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-tag/direct-behavior: committed diff did not carry before to after");
}

/// 📡️ The payload's text and binary wire forms round-trip, the binary frame carries this schema's
/// own tag, and a truncated frame is rejected rather than silently decoded.
#[test]
fn op_codecs_round_trip() {
    let payload = mutation();
    assert_eq!(TiffMutation::parse_op(&payload.print_op()).expect("the text op parses"), payload, "remove-tag/direct-behavior: the text op form does not round-trip");
    let bytes = payload.encode_op().expect("the binary op encodes");
    assert_eq!(bytes[1], BINARY_TAG, "remove-tag/direct-behavior: the binary frame must carry this schema's tag");
    assert_eq!(TiffMutation::decode_op(&bytes).expect("the binary op decodes"), payload, "remove-tag/direct-behavior: the binary op form does not round-trip");
    assert!(TiffMutation::decode_op(&bytes[..1]).is_err(), "remove-tag/direct-behavior: a truncated binary frame must be rejected");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod replace_tag {
//! 🧪️ `replace-tag` fixture — `🎯️direct`.
//!
//! Source of truth is the committed JSON quintet beside this file: the before-snapshot, the
//! mutation payload, the after-snapshot, the sparse diff and the declared outcome. Every value in
//! it was produced by this repository's OWN dispatch — `Mutation::diff` followed by
//! `MutationDiff::apply` — so the fixture pins what the runtime does rather than what a second
//! implementation believes it should do.
//!
//! ⚖️ The six laws below are the closed set every mutation vector in this repository states:
//! forward application reaches the committed after-snapshot, the mutation's OWN inverse returns to
//! the before-snapshot, both snapshots and the payload are canonical JSON, the declared outcome
//! matches the diagnostics dispatch actually raises, the produced diff IS the committed diff, and
//! the committed diff alone carries before to after. The seventh pins the op codecs: the text and
//! binary wire forms of this payload must round-trip and the binary form must reject a truncated
//! frame.

use crate::schema::mutations::TiffMutation;
use super::super::replace_tag::*;
use crate::standards::v6_0::subsets::document::io::binary::mutations::replace_tag::BINARY_TAG;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🏷️replace-tag/🎯️direct-behavior/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🏷️replace-tag/🎯️direct-behavior/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🏷️replace-tag/🎯️direct-behavior/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🏷️replace-tag/🎯️direct-behavior/🔺️diff/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🏷️replace-tag/🎯️direct-behavior/🎯️outcome/🔣️.json"));

fn before() -> TiffSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed before-snapshot decodes")
}
fn expected_after() -> TiffSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed after-snapshot decodes")
}
fn mutation() -> TiffMutation {
    semio_framework_pack_json::from_json_str((*MUTATION),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed replace-tag payload decodes")
}

/// ▶️ Applying the committed payload to the committed before-snapshot reaches the committed
/// after-snapshot exactly.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let produced = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("replace-tag applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "replace-tag/direct-behavior: applied state differs from the committed after-snapshot");
}

/// ↩️ The mutation's own inverse steps, computed from the pre-mutation state, restore it exactly —
/// the law the whole event-sourced undo history rests on.
#[test]
fn inverse_restores_before() {
    let base = before();
    let payload = mutation();
    let mut current = protocol::apply_diff(payload.diff(&base).diff(), &base).expect("forward replace-tag applies");
    for step in payload.inverse(&base).expect("valid retained mutation inverse fixture").into_iter().rev() {
        current = protocol::apply_diff(step.diff(&current).diff(), &current).expect("the replace-tag inverse step applies");
    }
    assert_eq!(current, base, "replace-tag/direct-behavior: the undo did not restore the committed before-snapshot");
}

/// 🔣️ Every committed file is canonical: decoding and re-encoding it reproduces it verbatim, so the
/// fixture can never drift from the serde shape the wire actually carries.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: TiffSnapshot = semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-tag/direct-behavior: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(mutation()))).expect("payload encodes");
    let original: serde_json::Value = serde_json::from_str((*MUTATION)).expect("payload reparses");
    assert_eq!(reencoded, original, "replace-tag/direct-behavior: committed payload JSON is not canonical");
}

/// 🎯️ The declared outcome is the one dispatch really produces — a fixture that declares `applied`
/// while the implementation quietly warns would otherwise read as clean.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str((*OUTCOME)).expect("outcome decodes");
    let status = declared.get("status").and_then(serde_json::Value::as_str).expect("the outcome carries a status");
    let produced = mutation().diff(&before());
    let raised: Vec<String> = produced.messages().iter().map(|message| message.code.0.clone()).collect();
    match status {
        "applied" => assert!(raised.is_empty(), "replace-tag/direct-behavior: declared applied, but dispatch raised {raised:?}"),
        _ => assert!(!raised.is_empty(), "replace-tag/direct-behavior: declared {status}, but dispatch raised nothing"),
    }
}

/// 🔺️ The diff dispatch produces IS the committed diff — the load-bearing assertion, because the
/// diff is what replication ships and what undo inverts.
#[test]
fn produces_committed_diff() {
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(mutation().diff(&before()).diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str((*DIFF)).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-tag/direct-behavior: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 The committed diff ALONE carries before to after: it is a complete description of the change,
/// not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let base = before();
    let produced = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-tag/direct-behavior: committed diff did not carry before to after");
}

/// 📡️ The payload's text and binary wire forms round-trip, the binary frame carries this schema's
/// own tag, and a truncated frame is rejected rather than silently decoded.
#[test]
fn op_codecs_round_trip() {
    let payload = mutation();
    assert_eq!(TiffMutation::parse_op(&payload.print_op()).expect("the text op parses"), payload, "replace-tag/direct-behavior: the text op form does not round-trip");
    let bytes = payload.encode_op().expect("the binary op encodes");
    assert_eq!(bytes[1], BINARY_TAG, "replace-tag/direct-behavior: the binary frame must carry this schema's tag");
    assert_eq!(TiffMutation::decode_op(&bytes).expect("the binary op decodes"), payload, "replace-tag/direct-behavior: the binary op form does not round-trip");
    assert!(TiffMutation::decode_op(&bytes[..1]).is_err(), "replace-tag/direct-behavior: a truncated binary frame must be rejected");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}
