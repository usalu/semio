#!/usr/bin/env bun
/** 🛠️ Ticket-local fixture writer (temporary, contract D1 authoring aid).
 *
 * It carries NO mutation knowledge of its own: every byte of every case — before, after, mutation,
 * diff, outcome, and the wording of every assertion label — comes from the handcrafted spec object
 * the caller passes in. It only does the mechanical parts: emitting `f64` literals with a forced
 * decimal point (so the canonical-JSON assertion holds exactly), creating the case directories, and
 * splicing the `#[cfg(test)] mod tests_*;` line into the plugin's `📦️glue.rs` right after that
 * mutation's own `pub mod inverse;`.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

export const REPO = "/Users/ueli/Documents/semio";

/** 🔢️ Marks a JSON number that must be emitted as an `f64` literal (`1` ⇒ `1.0`). */
export const f = (value: number) => ({ __float: value });

const formatFloat = (value: number): string => {
  const text = String(value);
  return text.includes(".") || text.includes("e") || text.includes("E") || text.includes("N") || text.includes("I") ? text : `${text}.0`;
};

/** 🔣️ Canonical-JSON encoder: 2-space indent, `f()` wrappers become decimal-pointed literals. */
export function enc(value: unknown, indent = ""): string {
  if (value === null) return "null";
  if (typeof value === "boolean") return String(value);
  if (typeof value === "number") return String(value);
  if (typeof value === "string") return JSON.stringify(value);
  if (Array.isArray(value)) {
    if (value.length === 0) return "[]";
    const inner = indent + "  ";
    return `[\n${value.map((item) => inner + enc(item, inner)).join(",\n")}\n${indent}]`;
  }
  if (typeof value === "object") {
    const record = value as Record<string, unknown>;
    if ("__float" in record) return formatFloat(record.__float as number);
    const keys = Object.keys(record);
    if (keys.length === 0) return "{}";
    const inner = indent + "  ";
    return `{\n${keys.map((key) => `${inner}${JSON.stringify(key)}: ${enc(record[key], inner)}`).join(",\n")}\n${indent}}`;
  }
  throw new Error(`unencodable ${String(value)}`);
}

export type Tree = {
  /** 📁️ Repo-relative path of the artifact's `🧬️mutations` directory. */
  readonly mutationsRoot: string;
  /** 📁️ Repo-relative path of the owning plugin's `📦️glue.rs`. */
  readonly glue: string;
  /** 🧵️ The `#[path]` prefix every sibling `mod` line in that glue block already uses. */
  readonly gluePrefix: string;
  /** 🦀️ Fully-qualified crate paths the emitted test imports. */
  readonly diffPath: string;
  readonly diffName: string;
  readonly snapshotPath: string;
  readonly snapshotName: string;
  readonly mutationsPath: string;
  readonly mutationName: string;
  /** ▶️ How this artifact is driven. `named`: it exposes its own `apply_*`/`inverse_*` free
   * functions next to the enum. `kernel`: it exposes none, so the test drives the very same kernel
   * entry point those wrappers delegate to (`protocol::apply_mutation` / `Mutation::inverse`). */
  readonly entry: "named" | "kernel";
  readonly applyFn?: string;
  readonly inverseFn?: string;
};

export type Case = {
  /** 📁️ Leaf directory name under `🧬️mutations/`, emoji prefix included. */
  readonly leafDir: string;
  /** 🏷️ Leaf name without the emoji — used verbatim in every assertion label. */
  readonly leafSlug: string;
  /** 🏷️ Kebab-case case directory name describing the change this fixture makes. */
  readonly caseName: string;
  /** 📝️ One sentence naming what this mutation's own diff builder does here. */
  readonly headline: string;
  readonly before: unknown;
  readonly after: unknown;
  readonly mutation: unknown;
  /** 🔺️ `null` only for a rejected case (which gets `🚫️component.absent` instead). */
  readonly diff: unknown | null;
  readonly outcome: unknown;
};

const write = (path: string, body: string): void => {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, body);
};

function testSource(tree: Tree, item: Case): string {
  const label = `${item.leafSlug}/${item.caseName}`;
  const named = tree.entry === "named";
  // ▶️ How this artifact is driven, spelled the way it actually exposes itself: its own
  // `apply_*`/`inverse_*` wrappers where it has them, otherwise the exact kernel entry point those
  // wrappers delegate to.
  const imports = named
    ? `use ${tree.diffPath}::${tree.diffName};\nuse ${tree.mutationsPath}::{${tree.applyFn}, ${tree.inverseFn}, ${tree.mutationName}};\nuse ${tree.snapshotPath}::${tree.snapshotName};`
    : `use ${tree.diffPath}::${tree.diffName};\nuse ${tree.mutationsPath}::${tree.mutationName};\nuse ${tree.snapshotPath}::${tree.snapshotName};`;
  const forwardOnce = named
    ? `    let mut snapshot = before();\n    ${tree.applyFn}(&mut snapshot, &mutation()).expect("${item.leafSlug} applies to its committed before-snapshot");`
    : `    let (snapshot, _) = protocol::apply_mutation(&before(), &mutation()).expect("${item.leafSlug} applies to its committed before-snapshot");`;
  const inverseChain = named
    ? `    let inverse = ${tree.inverseFn}(&base, &mutation);\n    let mut snapshot = base.clone();\n    ${tree.applyFn}(&mut snapshot, &mutation).expect("forward applies");\n    for step in &inverse {\n        ${tree.applyFn}(&mut snapshot, step).expect("inverse step applies");\n    }`
    : `    let inverse = <${tree.mutationName} as protocol::Mutation<${tree.snapshotName}>>::inverse(&mutation, &base);\n    let (mut snapshot, _) = protocol::apply_mutation(&base, &mutation).expect("forward applies");\n    for step in &inverse {\n        snapshot = protocol::apply_mutation(&snapshot, step).expect("inverse step applies").0;\n    }`;
  const outcomeApply = named
    ? `    let mut snapshot = before();\n    let applied = ${tree.applyFn}(&mut snapshot, &mutation()).is_ok();`
    : `    let attempt = protocol::apply_mutation(&before(), &mutation());\n    let applied = attempt.is_ok();\n    let snapshot = attempt.map(|(next, _)| next).unwrap_or_else(|_| before());`;
  return `//! 🧪️ \`${item.leafSlug}\` fixture — \`${item.caseName}\`.
//!
//! ${item.headline}
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! \`26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION\`). The \`.op.semio\`/\`.spr.semio\`/\`.dsl.semio\`/
//! \`.pack.semio\`/\`.patch.semio\` encodings are derived from it by \`fixtures generate\` and are
//! asserted by the shared codec-matrix harness, not here.

${imports}

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> ${tree.snapshotName} {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> ${tree.snapshotName} {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> ${tree.mutationName} {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries \`before\` to exactly the committed \`after\`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
${forwardOnce}
    assert_eq!(snapshot, expected_after(), "${label}: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores \`before\` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
${inverseChain}
    assert_eq!(snapshot, base, "${label}: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ${tree.snapshotName} = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "${label}: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "${label}: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| {
            rows.iter()
                .map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string()))
                .collect()
        })
        .unwrap_or_default();
    let raised = <${tree.mutationName} as protocol::Mutation<${tree.snapshotName}>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "${label}: raised diagnostics differ from the committed 🎯️outcome messages");
${outcomeApply}
    match status {
        "applied" if declared.iter().any(|(_, code)| code == "mutation.no-op") => {
            assert!(applied, "${label}: declared applied but the mutation was rejected");
            assert_eq!(snapshot, before(), "${label}: a no-op outcome is applied with an EMPTY diff — the snapshot must come back untouched");
        }
        "applied" => {
            assert!(applied, "${label}: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "${label}: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "${label}: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("${label}: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields \`${item.leafSlug}\` is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <${tree.mutationName} as protocol::Mutation<${tree.snapshotName}>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "${label}: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ${tree.diffName} = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "${label}: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to \`before\` yields the committed \`after\` — the diff is a
/// complete description of what \`${item.leafSlug}\` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ${tree.diffName} = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <${tree.diffName} as protocol::MutationDiff<${tree.snapshotName}>>::apply(&decoded, &before())
        .expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "${label}: committed diff did not carry before to after");
}
`;
}

/** 🪡️ Splices the test module line into the plugin glue directly after this leaf's own
 * `pub mod inverse;`, at the identical indentation. */
function wireGlue(tree: Tree, item: Case): void {
  // 🧵️ Two possible homes for a leaf's `pub mod inverse;`: the plugin `📦️glue.rs` (the usual case,
  // with the long `../../🗿️artifacts/…` prefix) or — for leaves that never got a pre-wired glue slot
  // — the artifact's own `🧬️mutations/🦀️component.rs`, which declares them inline with a `#[path]`
  // relative to the mutations directory. Try glue first, then fall back.
  const hosts = [
    { path: join(REPO, tree.glue), prefix: `${tree.gluePrefix}/` },
    { path: join(REPO, tree.mutationsRoot, "🦀️component.rs"), prefix: "" },
  ];
  for (const host of hosts) {
    const lines = readFileSync(host.path, "utf8").split("\n");
    const anchor = `"${host.prefix}${item.leafDir}/↩️inverse/🦀️component.rs"`;
    const at = lines.findIndex((line) => line.includes(anchor));
    if (at < 0) continue;
    const testPath = `"${host.prefix}${item.leafDir}/🧪️tests/${item.caseName}/🦀️component.rs"`;
    if (lines.some((line) => line.includes(testPath))) return;
    const modLine = lines[at + 1];
    if (!modLine.trim().startsWith("pub mod inverse;")) throw new Error(`unexpected shape after ${anchor} in ${host.path}: ${modLine}`);
    const pad = modLine.slice(0, modLine.length - modLine.trimStart().length);
    lines.splice(at + 2, 0, `${pad}#[cfg(test)]`, `${pad}#[path = ${testPath}]`, `${pad}mod tests_${item.caseName.replace(/-/g, "_")};`);
    writeFileSync(host.path, lines.join("\n"));
    return;
  }
  throw new Error(`no wiring anchor for ${item.leafDir}/↩️inverse in ${tree.glue} or ${tree.mutationsRoot}/🦀️component.rs`);
}

/** 📦️ Writes one handcrafted case to disk and wires it into the plugin glue. */
export function emit(tree: Tree, item: Case): void {
  const caseDir = join(REPO, tree.mutationsRoot, item.leafDir, "🧪️tests", item.caseName);
  write(join(caseDir, "📸️snapshot/⬅️before/🔣️component.json"), `${enc(item.before)}\n`);
  write(join(caseDir, "📸️snapshot/➡️after/🔣️component.json"), `${enc(item.after)}\n`);
  write(join(caseDir, "🦠️mutation/🔣️component.json"), `${enc(item.mutation)}\n`);
  write(join(caseDir, "🎯️outcome/🔣️component.json"), `${enc(item.outcome)}\n`);
  if (item.diff === null) write(join(caseDir, "🔺️diff/🚫️component.absent"), "");
  else write(join(caseDir, "🔺️diff/🔣️component.json"), `${enc(item.diff)}\n`);
  write(join(caseDir, "🦀️component.rs"), testSource(tree, item));
  wireGlue(tree, item);
}

export function emitAll(tree: Tree, cases: readonly Case[]): void {
  for (const item of cases) emit(tree, item);
  console.log(`✅️ ${cases.length} case(s) → ${tree.mutationsRoot}`);
}
