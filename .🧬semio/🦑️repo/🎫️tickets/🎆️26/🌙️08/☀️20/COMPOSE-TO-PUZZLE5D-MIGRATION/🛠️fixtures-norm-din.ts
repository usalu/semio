#!/usr/bin/env bun
/** 🛠️ Ticket-local fixture writer for the two DIN norm artifacts (`📗️din16798`, `📕️din4108`).
 *
 * Carries NO mutation knowledge of its own: every base value, every new value, every witness field
 * and every assertion sentence comes from the handcrafted `CASES` tables below, each row of which
 * was transcribed from that mutation's own `🔺️diff/🦀️component.rs` oracle. The writer only does the
 * mechanical parts — forcing a decimal point on `f64` literals so the canonical-JSON assertion holds
 * exactly, expanding serde's own `camelCase` rule, creating the case directories, and rewriting the
 * self-wiring `#[cfg(test)] mod fixture_tests` block in each artifact's OWN mutations-root
 * `🦀️component.rs` (the plugin `📦️glue.rs` is shared with concurrent agents and is never touched).
 */
import { mkdirSync, readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";

//#region 🔖️Encoding
/** 🔢️ Marks a JSON number that must be emitted as an `f64` literal (`1` ⇒ `1.0`). */
const f = (value: number) => ({ __float: value });
type Json = unknown;

const formatFloat = (value: number): string => {
  const text = String(value);
  if (text.includes("e") || text.includes("E")) throw new Error(`float ${text} would not round-trip as plain decimal`);
  return text.includes(".") ? text : `${text}.0`;
};

/** 🔣️ Canonical-JSON encoder: 2-space indent, `f()` wrappers become decimal-pointed literals. */
function enc(value: Json, indent = ""): string {
  if (value === null) return "null";
  if (typeof value === "boolean") return String(value);
  if (typeof value === "number") return String(value);
  if (typeof value === "string") return JSON.stringify(value);
  if (Array.isArray(value)) {
    if (value.length === 0) return "[]";
    const inner = `${indent}  `;
    return `[\n${value.map((item) => inner + enc(item, inner)).join(",\n")}\n${indent}]`;
  }
  const record = value as Record<string, Json>;
  if ("__float" in record) return formatFloat(record.__float as number);
  const keys = Object.keys(record);
  if (keys.length === 0) return "{}";
  const inner = `${indent}  `;
  return `{\n${keys.map((key) => `${inner}${JSON.stringify(key)}: ${enc(record[key], inner)}`).join(",\n")}\n${indent}}`;
}

/** 🐫️ serde's own `rename_all = "camelCase"`: PascalCase, then lowercase the first character. */
function camel(snake: string): string {
  let pascal = "";
  let capitalize = true;
  for (const ch of snake) {
    if (ch === "_") capitalize = true;
    else if (capitalize) {
      pascal += ch.toUpperCase();
      capitalize = false;
    } else pascal += ch;
  }
  return pascal.slice(0, 1).toLowerCase() + pascal.slice(1);
}
//#endregion 🔖️Encoding

//#region 🔖️Spec
/** 🧬️ A scalar field of the artifact's snapshot, with the value the fixture's base carries. */
type Kind = "f64" | "u32" | "u8" | "bool" | "string" | "enum";

type Field = { readonly rust: string; readonly kind: Kind; readonly base: any; readonly enumPath?: string };

/** 🧪️ One handcrafted case: which leaf, what it changes, to what, and how it reads in prose. */
type Case = {
  readonly leafDir: string;
  readonly leafSlug: string;
  readonly caseName: string;
  readonly headline: string;
  readonly modName: string;
  /** 🎯️ `null` for the layer-collection mutations, which carry their own bespoke extras. */
  readonly field: string | null;
  readonly next: any;
  readonly witness: string;
  /** 🦠️ The committed `🦠️mutation/🔣️component.json` body. */
  readonly mutation: Json;
  /** ➡️ Patch applied to the base snapshot to build `after`. */
  readonly afterPatch: Record<string, Json>;
  /** 🔺️ Patch applied to the all-null diff skeleton. */
  readonly diffPatch: Record<string, Json>;
  /** 🩹 Bespoke Rust assertion lines, keyed by test, for the layer-collection mutations. */
  readonly extras?: { apply: string[]; inverse: string[]; diff: string[] };
};

type Tree = {
  readonly label: string;
  readonly mutationsRoot: string;
  readonly artifactMod: string;
  readonly snapshotName: string;
  readonly diffName: string;
  readonly mutationName: string;
  readonly fields: Record<string, Field>;
  /** 📸️ Non-scalar base members (din4108's `layers`), spliced into the base snapshot in order. */
  readonly extraBase: Record<string, Json>;
  /** 🧾️ Snapshot field order for the emitted JSON. */
  readonly order: readonly string[];
  /** 🔺️ Diff struct field order (snake_case), `artifact` first, `selected_check_index` last. */
  readonly diffOrder: readonly string[];
  readonly wiringNote: string;
};
//#endregion 🔖️Spec

//#region 🔖️Rendering
const jsonOf = (field: Field, value: any): Json => (field.kind === "f64" ? f(value) : value);

function rustLiteral(field: Field, value: any): string {
  switch (field.kind) {
    case "f64":
      return formatFloat(value);
    case "u32":
    case "u8":
      return String(value);
    case "bool":
      return String(value);
    case "string":
      return JSON.stringify(value);
    case "enum":
      return `${field.enumPath}::${value}`;
  }
}

function snapshotAssert(tree: Tree, name: string, value: any, message: string): string {
  const field = tree.fields[name];
  if (field.kind === "bool") return `    assert!(${value ? "" : "!"}snapshot.${field.rust}, "${message}");`;
  return `    assert_eq!(snapshot.${field.rust}, ${rustLiteral(field, value)}, "${message}");`;
}

function diffAssert(tree: Tree, name: string, value: any, message: string): string {
  const field = tree.fields[name];
  if (field.kind === "string") return `    assert_eq!(raised.diff().${field.rust}.as_deref(), Some(${rustLiteral(field, value)}), "${message}");`;
  return `    assert_eq!(raised.diff().${field.rust}, Some(${rustLiteral(field, value)}), "${message}");`;
}

function scalarExtras(tree: Tree, item: Case): { apply: string[]; inverse: string[]; diff: string[] } {
  const name = item.field as string;
  const field = tree.fields[name];
  const witness = tree.fields[item.witness];
  const label = `${item.leafSlug}/${item.caseName}`;
  const wire = camel(name);
  const witnessWire = camel(item.witness);
  // 🧵️ Prose renderings live inside a Rust string literal, so a quoted value uses single quotes.
  const shown = field.kind === "string" ? `'${item.next}'` : field.kind === "enum" ? String(item.next) : formatOrPlain(field, item.next);
  const wasShown = field.kind === "string" ? `'${field.base}'` : field.kind === "enum" ? String(field.base) : formatOrPlain(field, field.base);
  return {
    apply: [
      snapshotAssert(tree, name, item.next, `${label}: ${wire} did not land on ${shown}`),
      `    assert_eq!(snapshot.${witness.rust}, before().${witness.rust}, "${label}: ${witnessWire} must stay exactly as the before-snapshot had it — ${item.leafSlug} owns ${wire} and nothing else");`,
    ],
    inverse: [
      `    assert_eq!(inverse.len(), 1, "${label}: undoing one ${wire} edit is exactly one step");`,
      snapshotAssert(tree, name, field.base, `${label}: the undo step must put ${wire} back to ${wasShown}`),
    ],
    diff: [
      diffAssert(tree, name, item.next, `${label}: the sparse delta must carry ${wire} = ${shown}`),
      `    assert!(raised.diff().${witness.rust}.is_none(), "${label}: the sparse delta must leave ${witnessWire} unset — a delta that rewrote it would be a bug this assertion exists to catch");`,
    ],
  };
}

const formatOrPlain = (field: Field, value: any): string => (field.kind === "f64" ? formatFloat(value) : String(value));

function testSource(tree: Tree, item: Case): string {
  const label = `${item.leafSlug}/${item.caseName}`;
  const extras = item.extras ?? scalarExtras(tree, item);
  return `//! 🧪️ \`${item.leafSlug}\` fixture — \`${item.caseName}\`.
//!
//! ${item.headline}
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! \`26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION\`), every value of which was transcribed from this
//! leaf's own \`🔺️diff/🦀️component.rs\` oracle. The \`.op.semio\`/\`.spr.semio\`/\`.dsl.semio\`/
//! \`.pack.semio\`/\`.patch.semio\` encodings are derived from it by \`fixtures generate\` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::${tree.artifactMod}::diff::${tree.diffName};
use crate::artifacts::${tree.artifactMod}::${tree.mutationName};
use crate::artifacts::${tree.artifactMod}::${tree.snapshotName};

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

/// ▶️ \`${item.leafSlug}\` carries \`before\` to exactly the committed \`after\`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let (snapshot, _) = protocol::apply_mutation(&before(), &mutation()).expect("${item.leafSlug} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "${label}: applied state differs from committed after-snapshot");
${extras.apply.join("\n")}
}

/// ↩️ Applying \`${item.leafSlug}\` and then its own inverse restores \`before\` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <${tree.mutationName} as protocol::Mutation<${tree.snapshotName}>>::inverse(&mutation, &base);
    let (mut snapshot, _) = protocol::apply_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = protocol::apply_mutation(&snapshot, step).expect("inverse step applies").0;
    }
    assert_eq!(snapshot, base, "${label}: inverse did not restore the before-snapshot");
${extras.inverse.join("\n")}
}

/// 🔣️ Both committed snapshots and this leaf's committed mutation payload are already canonical:
/// decode→encode is a fixed point.
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

/// 🎯️ The declared outcome — status AND every diagnostic \`${item.leafSlug}\`'s own diff builder
/// raises for this payload — matches what the mutation actually produces.
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
    let attempt = protocol::apply_mutation(&before(), &mutation());
    let applied = attempt.is_ok();
    let snapshot = attempt.map(|(next, _)| next).unwrap_or_else(|_| before());
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

/// 🔺️ The sparse delta \`${item.leafSlug}\` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields of \`${tree.snapshotName}\` this leaf is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <${tree.mutationName} as protocol::Mutation<${tree.snapshotName}>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "${label}: produced diff differs from the committed 🔺️diff/🔣️component.json");
${extras.diff.join("\n")}
}

/// 🔣️ The committed diff is itself canonical and decodes to \`${tree.diffName}\`. Its
/// \`selectedCheckIndex\` is an \`Option<Option<u32>>\` and so cannot distinguish \`None\` from
/// \`Some(None)\` across a JSON round trip — \`${item.leafSlug}\` never writes it, so the committed
/// \`null\` is unambiguously \`None\` here and the fixed point holds.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ${tree.diffName} = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.selected_check_index.is_none(), "${label}: ${item.leafSlug} is an artifact-lane edit and must never carry the presence-lane selectedCheckIndex");
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
//#endregion 🔖️Rendering

//#region 🔖️Emit
const write = (path: string, body: string): void => {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, body);
};

function baseSnapshot(tree: Tree): Record<string, Json> {
  const out: Record<string, Json> = {};
  for (const name of tree.order) {
    if (name in tree.extraBase) out[camel(name)] = tree.extraBase[name];
    else out[camel(name)] = jsonOf(tree.fields[name], tree.fields[name].base);
  }
  return out;
}

function diffSkeleton(tree: Tree): Record<string, Json> {
  const out: Record<string, Json> = {};
  for (const name of tree.diffOrder) out[camel(name)] = null;
  return out;
}

function emit(tree: Tree, cases: readonly Case[]): void {
  const base = baseSnapshot(tree);
  const skeleton = diffSkeleton(tree);
  const wiring: string[] = [];
  for (const item of cases) {
    const caseDir = join(REPO, tree.mutationsRoot, item.leafDir, "🧪️tests", item.caseName);
    const after = { ...base };
    for (const [key, value] of Object.entries(item.afterPatch)) after[camel(key)] = value;
    const diff = { ...skeleton };
    for (const [key, value] of Object.entries(item.diffPatch)) diff[camel(key)] = value;
    write(join(caseDir, "📸️snapshot/⬅️before/🔣️component.json"), `${enc(base)}\n`);
    write(join(caseDir, "📸️snapshot/➡️after/🔣️component.json"), `${enc(after)}\n`);
    write(join(caseDir, "🦠️mutation/🔣️component.json"), `${enc(item.mutation)}\n`);
    write(join(caseDir, "🔺️diff/🔣️component.json"), `${enc(diff)}\n`);
    write(join(caseDir, "🎯️outcome/🔣️component.json"), `${enc({ status: "applied" })}\n`);
    write(join(caseDir, "🦀️component.rs"), testSource(tree, item));
    wiring.push(`    #[path = "${item.leafDir}/🧪️tests/${item.caseName}/🦀️component.rs"]\n    mod tests_${item.modName}_${item.caseName.replace(/-/g, "_")};`);
  }
  wireSelf(tree, wiring);
  console.log(`✅️ ${cases.length} case(s) → ${tree.mutationsRoot}`);
}

/** 🪡️ Rewrites the self-wiring test region at the end of the artifact's own mutations-root
 * `🦀️component.rs`. `#[path = "."]` on the inline module makes every child `#[path]` resolve
 * against this file's own directory (verified against rustc's reported module path), so each line
 * below is exactly the leaf-relative path of one committed case. `📦️glue.rs` is NEVER edited: it is
 * shared with the agents migrating the other thirteen norm artifacts. */
function wireSelf(tree: Tree, lines: readonly string[]): void {
  const path = join(REPO, tree.mutationsRoot, "🦀️component.rs");
  const source = readFileSync(path, "utf8");
  const region = `//#region 🧪️FixtureTests
${tree.wiringNote}
#[cfg(test)]
#[path = "."]
mod fixture_tests {
${lines.join("\n")}
}
//#endregion 🧪️FixtureTests
`;
  const marker = "//#region 🧪️FixtureTests";
  const at = source.indexOf(marker);
  const head = at < 0 ? `${source.replace(/\s*$/, "")}\n\n` : source.slice(0, at);
  writeFileSync(path, head + region);
  for (const line of lines) {
    const rel = line.match(/#\[path = "([^"]+)"\]/)![1];
    if (!existsSync(join(REPO, tree.mutationsRoot, rel))) throw new Error(`dangling #[path]: ${rel}`);
  }
}
//#endregion 🔖️Emit

export { emit, f, camel, formatFloat, enc, REPO };
export type { Case, Field, Tree };
