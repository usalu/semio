"""🧰️ Authoring helpers shared by the four per-artifact fixture authoring scripts.

These write the hand-authored per-case content out to disk; every case's snapshots, mutation,
diff, outcome and Rust assertions are spelled out explicitly in the per-artifact script.
"""
import json
import os

REPO = "/Users/ueli/Documents/semio"


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def dump(obj):
    return json.dumps(obj, indent=2, ensure_ascii=False) + "\n"


def emit_case(root, leaf, case, before, after, mutation, diff, outcome, rust):
    base = os.path.join(root, leaf, "🧪️tests", case)
    write(os.path.join(base, "📸️snapshot/⬅️before/🔣️component.json"), dump(before))
    write(os.path.join(base, "📸️snapshot/➡️after/🔣️component.json"), dump(after))
    write(os.path.join(base, "🦠️mutation/🔣️component.json"), dump(mutation))
    write(os.path.join(base, "🔺️diff/🔣️component.json"), dump(diff))
    write(os.path.join(base, "🎯️outcome/🔣️component.json"), dump(outcome))
    write(os.path.join(base, "🦀️component.rs"), rust)


def test_source(*, artifact, snapshot_ty, diff_ty, mutation_ty, kind, case, summary, extra_applied, extra_diff):
    """🦀️ Renders one case's seven assertions with this mutation's own wording."""
    label = "{}/{}".format(kind, case)
    return f'''//! 🧪️ `{kind}` fixture — `{case}`.
//!
//! {summary}
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::{artifact}::{{{diff_ty}, {mutation_ty}, {snapshot_ty}}};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> {snapshot_ty} {{
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> {snapshot_ty} {{
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> {mutation_ty} {{
    serde_json::from_str(MUTATION).expect("mutation decodes")
}}
fn applied() -> {snapshot_ty} {{
    let base = before();
    let raised = <{mutation_ty} as protocol::Mutation<{snapshot_ty}>>::diff(&mutation(), &base);
    <{diff_ty} as protocol::MutationDiff<{snapshot_ty}>>::apply(raised.diff(), &base).expect("{kind} applies to its committed before-snapshot")
}}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {{
    let snapshot = applied();
{extra_applied}
    assert_eq!(snapshot, expected_after(), "{label}: applied state differs from committed after-snapshot");
}}

/// ↩️ Applying the mutation then every step of its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = <{mutation_ty} as protocol::Mutation<{snapshot_ty}>>::inverse(&mutation, &base);
    assert!(!inverse.is_empty(), "{label}: this mutation changes state, so its inverse must not be empty");
    let mut snapshot = applied();
    for step in &inverse {{
        let raised = <{mutation_ty} as protocol::Mutation<{snapshot_ty}>>::diff(step, &snapshot);
        snapshot = <{diff_ty} as protocol::MutationDiff<{snapshot_ty}>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{label}: inverse did not restore the before-snapshot");
}}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {{
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: {snapshot_ty} = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{label}: committed {{side}} JSON is not canonical");
    }}
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{label}: committed mutation JSON is not canonical");
}}

/// 🎯️ The declared outcome — status AND every diagnostic `{kind}`'s own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {{
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <{mutation_ty} as protocol::Mutation<{snapshot_ty}>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {{
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        }})
        .collect();
    assert_eq!(produced, declared, "{label}: raised diagnostics differ from the committed 🎯️outcome messages");
    let snapshot = applied();
    match status {{
        "applied" => assert_ne!(snapshot, before(), "{label}: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "{label}: a rejected mutation must leave the snapshot untouched"),
        other => panic!("{label}: unknown outcome status {{other:?}}"),
    }}
}}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields `{kind}` is allowed to
/// touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {{
    let raised = <{mutation_ty} as protocol::Mutation<{snapshot_ty}>>::diff(&mutation(), &before());
    let raised_diff = raised.diff();
{extra_diff}
    let produced = serde_json::to_value(raised_diff).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{label}: produced diff differs from the committed 🔺️diff/🔣️component.json");
}}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type. Note
/// `{diff_ty}` carries no `skip_serializing_if`, so every untouched field must be present as an
/// explicit `null` — and its `Option<Option<u32>>` presence field cannot distinguish "cleared" from
/// "untouched" across a JSON round trip, which is why no case here writes it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {{
    let decoded: {diff_ty} = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{label}: committed diff JSON is not canonical");
}}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `{kind}` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {{
    let decoded: {diff_ty} = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <{diff_ty} as protocol::MutationDiff<{snapshot_ty}>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{label}: committed diff did not carry before to after");
}}
'''


def emit_rejected_case(root, leaf, case, before, mutation, outcome, rust):
    """🚫️ A rejected case carries no diff JSON — contract D6's empty `🚫️component.absent` marker
    stands in its place, and `after` is `before` verbatim."""
    base = os.path.join(root, leaf, "🧪️tests", case)
    write(os.path.join(base, "📸️snapshot/⬅️before/🔣️component.json"), dump(before))
    write(os.path.join(base, "📸️snapshot/➡️after/🔣️component.json"), dump(before))
    write(os.path.join(base, "🦠️mutation/🔣️component.json"), dump(mutation))
    write(os.path.join(base, "🔺️diff/🚫️component.absent"), "")
    write(os.path.join(base, "🎯️outcome/🔣️component.json"), dump(outcome))
    write(os.path.join(base, "🦀️component.rs"), rust)


def rejected_test_source(*, artifact, snapshot_ty, diff_ty, mutation_ty, kind, case, summary, extra_rejected):
    """🦀️ Renders one REJECTED case's seven assertions with this mutation's own wording."""
    label = "{}/{}".format(kind, case)
    return f'''//! 🧪️ `{kind}` fixture — `{case}`.
//!
//! {summary}
//!
//! Source of truth is the committed JSON quartet beside this file plus contract D6's empty
//! `🔺️diff/🚫️component.absent` marker (ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The
//! `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio` encodings are derived from it by
//! `fixtures generate` and are asserted by the shared codec-matrix harness, not here.

use crate::artifacts::{artifact}::{{{diff_ty}, {mutation_ty}, {snapshot_ty}}};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️component.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> {snapshot_ty} {{
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> {snapshot_ty} {{
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> {mutation_ty} {{
    serde_json::from_str(MUTATION).expect("mutation decodes")
}}
fn applied() -> {snapshot_ty} {{
    let base = before();
    let raised = <{mutation_ty} as protocol::Mutation<{snapshot_ty}>>::diff(&mutation(), &base);
    <{diff_ty} as protocol::MutationDiff<{snapshot_ty}>>::apply(raised.diff(), &base).expect("an empty rejection diff still applies cleanly")
}}

/// ▶️ The rejected mutation carries `before` to exactly the committed `after` — which, for a
/// rejection, is `before` verbatim.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {{
    let snapshot = applied();
{extra_rejected}
    assert_eq!(snapshot, expected_after(), "{label}: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "{label}: a rejected case's after-snapshot must be its before-snapshot verbatim");
}}

/// ↩️ A rejection changes nothing, and replaying its inverse on top still lands on `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let mut snapshot = applied();
    for step in &<{mutation_ty} as protocol::Mutation<{snapshot_ty}>>::inverse(&mutation, &base) {{
        let raised = <{mutation_ty} as protocol::Mutation<{snapshot_ty}>>::diff(step, &snapshot);
        snapshot = <{diff_ty} as protocol::MutationDiff<{snapshot_ty}>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{label}: replaying the inverse of a rejection must still leave the before-snapshot");
}}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {{
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: {snapshot_ty} = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{label}: committed {{side}} JSON is not canonical");
    }}
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{label}: committed mutation JSON is not canonical");
}}

/// 🎯️ The declared outcome — the `rejected` status AND the exact diagnostic `{kind}`'s own
/// diff builder raises — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {{
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "rejected", "{label}: this case exists to pin a rejection");
    let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("a rejected outcome carries a machine-readable code");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <{mutation_ty} as protocol::Mutation<{snapshot_ty}>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {{
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        }})
        .collect();
    assert_eq!(produced, declared, "{label}: raised diagnostics differ from the committed 🎯️outcome messages");
    assert!(produced.iter().any(|(_, raised_code)| raised_code == code), "{label}: the outcome's declared code must be one the builder actually raised");
    assert_eq!(applied(), before(), "{label}: a rejected mutation must leave the snapshot untouched");
}}

/// 🔺️ A rejection publishes NO delta: the raised diff must be the diff type's own `default()`, and
/// the case must carry contract D6's empty `🚫️component.absent` marker instead of an invented
/// empty patch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {{
    let raised = <{mutation_ty} as protocol::Mutation<{snapshot_ty}>>::diff(&mutation(), &before());
    assert_eq!(raised.diff(), &{diff_ty}::default(), "{label}: a fatal rejection must publish the default (all-null) diff");
    assert!(DIFF_ABSENT.is_empty(), "{label}: 🔺️diff/🚫️component.absent must be a zero-byte marker");
}}

/// 🔣️ There is no committed diff JSON to be canonical — the absent marker is the committed form,
/// and it must stay empty.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {{
    assert_eq!(DIFF_ABSENT.len(), 0, "{label}: the absence marker must carry no bytes at all");
    let produced = serde_json::to_value({diff_ty}::default()).expect("default diff encodes");
    assert!(produced.as_object().is_some_and(|fields| fields.values().all(serde_json::Value::is_null)), "{label}: the default diff must serialize as an all-null object, never as an omitted-field one");
}}

/// 🩹 Applying the rejection's own (default) diff to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {{
    let produced = <{diff_ty} as protocol::MutationDiff<{snapshot_ty}>>::apply(&{diff_ty}::default(), &before()).expect("the default diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{label}: the rejection's empty diff must leave before exactly as committed in after");
}}
'''
