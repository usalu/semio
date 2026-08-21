/** 🧪️ Writes the handcrafted `🗒️note` fixture quartets + their Rust test files, and wires each case
 * into the plugin's `📦️glue.rs`. Authoring aid only — the emitted files are the artefact. */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { base, json } from "./note-fixtures-base.ts";
import { cases } from "./note-fixtures-cases.ts";

const REPO = "/Users/ueli/Documents/semio";
const MUTATIONS = join(REPO, "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
const GLUE = join(REPO, "✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📦️glue.rs");

/** 📥️ Renders one case's `use` block, folding the optional `NoteBlockNode` import into the shared
 * `crate::artifacts::note::{..}` line so the emitted header stays in rustfmt's own order. */
const useBlock = (extra: string | undefined): string => {
  const wantsNode = (extra ?? "").includes("NoteBlockNode");
  const schema = (extra ?? "").split("\n").find((line) => line.startsWith("use crate::artifacts::note::schema::{"));
  const lines = ["use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};"];
  if (schema) lines.push(schema);
  lines.push(wantsNode ? "use crate::artifacts::note::{NoteBlockNode, NoteDiff, NoteSnapshot};" : "use crate::artifacts::note::{NoteDiff, NoteSnapshot};");
  lines.push("use protocol::Mutation;");
  return `${lines.join("\n")}\n`;
};

const testFile = (c: (typeof cases)[number]): string => `//! 🧪️ \`${c.slug}\` fixture — \`${c.caseName}\`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! \`26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION\`). The \`.op.semio\`/\`.spr.semio\`/\`.dsl.semio\`/
//! \`.pack.semio\`/\`.patch.semio\` encodings are derived from it by \`fixtures generate\` and are
//! asserted by the shared codec-matrix harness, not here.

${useBlock(c.extraUse)}
const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> NoteSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> NoteSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> NoteMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ ${c.applyDoc}
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("${c.slug} applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "${c.slug}/${c.caseName}: applied state differs from committed after-snapshot");
}

/// ↩️ ${c.inverseDoc}
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("${c.slug} applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("${c.slug} inverse step applies");
    }
    assert_eq!(snapshot, base, "${c.slug}/${c.caseName}: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "${c.slug}/${c.caseName}: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "${c.slug}/${c.caseName}: committed mutation JSON is not canonical");
}

/// 🎯️ ${c.outcomeDoc}
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "${c.slug}/${c.caseName}: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "${c.slug}/${c.caseName}: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("${c.slug}/${c.caseName}: declared applied but the diff would not apply");
}

/// 🔺️ ${c.diffDoc}
///
/// The single most load-bearing assertion in the fixture: \`before\`+\`after\` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "${c.slug}/${c.caseName}: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own \`NoteDiff\`: its container is
/// \`#[serde(default)]\` with no \`skip_serializing_if\`, so all 23 fields must be present, \`null\` for
/// every slot \`${c.slug}\` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "${c.slug}/${c.caseName}: committed diff JSON is not canonical");
}

/// 🩹 ${c.applyDiffDoc}
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "${c.slug}/${c.caseName}: committed diff did not carry before to after");
}

/// ${c.extraDoc}
#[semio_framework_async_macros::async_test]
async fn ${c.extraName}() {
${c.extraBody}
}
`;

const seen = new Set<string>();
for (const c of cases) {
  const leaf = join(MUTATIONS, c.dir);
  if (!existsSync(join(leaf, "🦠️mutation/🦀️component.rs"))) throw new Error(`no such mutation leaf: ${c.dir}`);
  if (seen.has(c.dir)) throw new Error(`duplicate leaf ${c.dir}`);
  seen.add(c.dir);
  const dir = join(leaf, "🧪️tests", c.caseName);
  mkdirSync(join(dir, "📸️snapshot/⬅️before"), { recursive: true });
  mkdirSync(join(dir, "📸️snapshot/➡️after"), { recursive: true });
  mkdirSync(join(dir, "🦠️mutation"), { recursive: true });
  mkdirSync(join(dir, "🎯️outcome"), { recursive: true });
  writeFileSync(join(dir, "📸️snapshot/⬅️before/🔣️component.json"), json(base()));
  writeFileSync(join(dir, "📸️snapshot/➡️after/🔣️component.json"), json(c.after()));
  writeFileSync(join(dir, "🦠️mutation/🔣️component.json"), json(c.mutation));
  mkdirSync(join(dir, "🔺️diff"), { recursive: true });
  writeFileSync(join(dir, "🔺️diff/🔣️component.json"), json(c.diff(c.after() as ReturnType<typeof base>)));
  writeFileSync(join(dir, "🎯️outcome/🔣️component.json"), json({ status: "applied" }));
  writeFileSync(join(dir, "🦀️component.rs"), testFile(c));
}

// 🔌️ Wire each case into glue.rs immediately after that leaf's own `pub mod inverse;` line.
let glue = readFileSync(GLUE, "utf8");
for (const c of cases) {
  const anchor = `🧬️mutations/${c.dir}/↩️inverse/🦀️component.rs"]`;
  const lines = glue.split("\n");
  const anchorIndex = lines.findIndex((line) => line.includes(anchor));
  if (anchorIndex < 0) throw new Error(`no glue anchor for ${c.dir}`);
  if (!lines[anchorIndex + 1].includes("pub mod inverse;")) throw new Error(`unexpected glue shape for ${c.dir}`);
  const indent = lines[anchorIndex + 1].match(/^\s*/)![0];
  const testMod = `tests_${c.caseName.replace(/-/g, "_")}`;
  if (glue.includes(testMod)) continue;
  lines.splice(anchorIndex + 2, 0,
    `${indent}#[cfg(test)]`,
    `${indent}#[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/${c.dir}/🧪️tests/${c.caseName}/🦀️component.rs"]`,
    `${indent}mod ${testMod};`,
  );
  glue = lines.join("\n");
}
writeFileSync(GLUE, glue);
console.log(`✅️ emitted ${cases.length} case(s) and wired glue.rs`);
