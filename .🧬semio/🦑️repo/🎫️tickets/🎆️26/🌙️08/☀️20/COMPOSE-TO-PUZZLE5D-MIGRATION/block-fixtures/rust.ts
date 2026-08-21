// 🦀️ Renders one fixture's `🦀️component.rs` — the shape puzzle5d's committed reference uses, with
// every label and the closing state assertion worded for the mutation under test.
export type RustSpec = {
  readonly artifact: string;          // block5d | block3d | block2d
  readonly snapshotType: string;      // Block5dSnapshot
  readonly mutationType: string;      // Block5dMutation
  readonly diffType: string;          // Block5dDiff
  readonly applyFn: string;           // apply_block5d_mutation
  readonly inverseFn: string;         // inverse_block5d_mutation
  readonly leaf: string;              // move-grip-2d
  readonly caseName: string;          // swings-north-grip-on-the-rim
  readonly summary: string;           // human sentence for the header docstring
  readonly beforePrelude: string;     // extra statements inside `before()`
  readonly stateAssertion: string;    // one mutation-specific assertion over `snapshot`
};

export function renderRust(spec: RustSpec): string {
  const tag = `${spec.leaf}/${spec.caseName}`;
  return `//! 🧪️ \`${spec.leaf}\` fixture — \`${spec.caseName}\`: ${spec.summary}
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! \`26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION\`). The \`.op.semio\`/\`.spr.semio\`/\`.dsl.semio\`/
//! \`.pack.semio\`/\`.patch.semio\` encodings are derived from it by \`fixtures generate\` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::${spec.artifact}::mutations::{${spec.applyFn}, ${spec.inverseFn}};
use crate::artifacts::${spec.artifact}::mutations::${spec.mutationType};
use crate::artifacts::${spec.artifact}::${spec.snapshotType};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> ${spec.snapshotType} {
${spec.beforePrelude}    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> ${spec.snapshotType} {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> ${spec.mutationType} {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries \`before\` to exactly the committed \`after\`: ${spec.summary}
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    ${spec.applyFn}(&mut snapshot, &mutation()).expect("${spec.leaf} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "${tag}: applied state differs from committed after-snapshot");
    ${spec.stateAssertion}
}

/// ↩️ Applying the mutation then its inverse restores \`before\` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = ${spec.inverseFn}(&base, &mutation);
    let mut snapshot = base.clone();
    ${spec.applyFn}(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        ${spec.applyFn}(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "${tag}: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ${spec.snapshotType} = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "${tag}: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "${tag}: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = ${spec.applyFn}(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "${tag}: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "${tag}: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "${tag}: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("${tag}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields \`${spec.leaf}\` is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <${spec.mutationType} as protocol::Mutation<${spec.snapshotType}>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "${tag}: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::${spec.artifact}::diff::${spec.diffType} = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "${tag}: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to \`before\` yields the committed \`after\` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::${spec.artifact}::diff::${spec.diffType} = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::${spec.artifact}::diff::${spec.diffType} as protocol::MutationDiff<${spec.snapshotType}>>::apply(&decoded, &before())
        .expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "${tag}: committed diff did not carry before to after");
}
`;
}
