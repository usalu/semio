#!/usr/bin/env python3
"""🧪️ Re-emits every committed per-mutation fixture test file (`🧬️mutations/*/🧪️tests/*/🦀️.rs`) from
the one canonical template. The template retires every owned projection, sparse delta and mutation
outcome it decodes — an owned `Generation{N}dSnapshot`/`Generation{N}dDiff` rejects a bare drop
(`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`) — and encodes diagnostic severities through
the artifact's own encoder instead of `Debug`. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END."""
import re, sys, pathlib

BODY = '''
fn before() -> Generation{n}dSnapshotRead {{
    Generation{n}dSnapshotRead::new(dsl::json::from_json_str(BEFORE).expect("before snapshot decodes"))
}}
fn expected_after() -> Generation{n}dSnapshotRead {{
    Generation{n}dSnapshotRead::new(dsl::json::from_json_str(AFTER).expect("after snapshot decodes"))
}}
fn mutation() -> Generation{n}dMutation {{
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}}
fn raised_diff(base: &Generation{n}dSnapshot) -> (Generation{n}dDiffRead, Vec<protocol::MutationMessage>) {{
    let (diff, messages) = <Generation{n}dMutation as protocol::Mutation<Generation{n}dSnapshot>>::diff(&mutation(), base).into_parts();
    (Generation{n}dDiffRead::new(diff), messages)
}}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {{
    let mut snapshot = before();
    apply_generation{n}d_mutation(&mut snapshot, &mutation()).expect("{mutation} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{label}: applied state differs from committed after-snapshot");
}}

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_generation{n}d_mutation(&base, &mutation);
    let mut snapshot = Generation{n}dSnapshotRead::new((*base).clone());
    apply_generation{n}d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_generation{n}d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{label}: inverse did not restore the before-snapshot");
}}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded = Generation{n}dSnapshotRead::new(dsl::json::from_json_str(text).expect("snapshot decodes"));
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&*decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{label}: committed {{side}} JSON is not canonical");
    }}
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{label}: committed mutation JSON is not canonical");
}}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {{
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let base = before();
    let (_delta, messages) = raised_diff(&base);
    let produced: Vec<(String, String)> = messages
        .iter()
        .map(|message| {{
            let level = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        }})
        .collect();
    assert_eq!(produced, declared, "{label}: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = Generation{n}dSnapshotRead::new((*base).clone());
    let applied = apply_generation{n}d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {{
        "applied" => {{
            assert!(applied, "{label}: declared applied but the mutation was rejected");
            assert_ne!(snapshot, base, "{label}: declared applied but the snapshot came back unchanged");
        }}
        "rejected" => {{
            assert_eq!(snapshot, base, "{label}: a rejected mutation must leave the snapshot untouched");
        }}
        other => panic!("{label}: unknown outcome status {{other:?}}"),
    }}
}}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields `{mutation}` is
/// allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {{
    let base = before();
    let (delta, _messages) = raised_diff(&base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&*delta)).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{label}: produced diff differs from the committed 🔺️diff/🔣️.json");
}}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {{
    let decoded = Generation{n}dDiffRead::new(dsl::json::from_json_str(DIFF).expect("committed diff decodes"));
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&*decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{label}: committed diff JSON is not canonical");
}}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `{mutation}` changed, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {{
    let decoded = Generation{n}dDiffRead::new(dsl::json::from_json_str(DIFF).expect("committed diff decodes"));
    let produced = Generation{n}dSnapshotRead::new(
        <Generation{n}dDiff as protocol::MutationDiff<Generation{n}dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot"),
    );
    assert_eq!(produced, expected_after(), "{label}: committed diff did not carry before to after");
}}
'''

IMPORTS = '''use crate::standards::v1::subsets::any::schema::diff::{{Generation{n}dDiff, Generation{n}dDiffRead}};
use crate::standards::v1::subsets::any::schema::mutations::{{apply_generation{n}d_mutation, inverse_generation{n}d_mutation, Generation{n}dMutation}};
use crate::standards::v1::subsets::any::schema::snapshot::Generation{n}dSnapshotRead;
use crate::Generation{n}dSnapshot;
'''

def rewrite(path: pathlib.Path) -> None:
    text = path.read_text(encoding="utf8")
    match = re.search(r"Generation([23])dSnapshot", text)
    if match is None:
        raise SystemExit(f"{path}: no artifact dimension")
    n = match.group(1)
    label = re.search(r'"([a-z0-9-]+/[a-z0-9-]+): ', text)
    if label is None:
        raise SystemExit(f"{path}: no fixture label")
    label = label.group(1)
    mutation = label.split("/")[0]
    header = text.split("\nuse crate::standards")[0]
    consts = re.findall(r"^const .*$", text, flags=re.M)
    if len(consts) != 5:
        raise SystemExit(f"{path}: expected 5 fixture consts, saw {len(consts)}")
    out = header + "\n" + IMPORTS.format(n=n) + "\n" + "\n".join(consts) + "\n" + BODY.format(n=n, label=label, mutation=mutation)
    path.write_text(out, encoding="utf8")
    print("re-emitted", path)

for root in sys.argv[1:]:
    for path in sorted(pathlib.Path(root).glob("*/🧪️tests/*/🦀️.rs")):
        if "💾️binary" in str(path):
            continue
        rewrite(path)
