#!/usr/bin/env bun
/** 🦴️ One-shot spine decomposition: splits the old 🦴️spine crate's 14 top-level `mod` blocks into the
 *  new taxonomy component files under `🗿️artifacts/🏛️program/`, dedenting and rewriting `crate::<mod>::`
 *  paths onto the new module tree. Scratch tool for ticket
 *  `26/08/05/ARCHITECT-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const pluginRoot = join(repoRoot, "✏️s/🔌️plugins/🏛️architect");
const spine = join(pluginRoot, "🔨️modules/🦴️spine/⚡️implementations/🦀️rust/📦️lib.rs");

const text = readFileSync(spine, "utf8");
const lines = text.split("\n");

/** ✂️ Slices each top-level `mod <name> {` … matching `}` at column 0. */
function extractModules(): Map<string, string[]> {
  const out = new Map<string, string[]>();
  for (let i = 0; i < lines.length; i++) {
    const match = /^mod ([a-z_]+) \{$/.exec(lines[i]!);
    if (!match) continue;
    let end = i + 1;
    while (end < lines.length && lines[end] !== "}") end++;
    out.set(match[1]!, lines.slice(i + 1, end));
    i = end;
  }
  return out;
}

const MODULE_PATHS: Record<string, string> = {
  kernel: "crate::artifacts::program::kernel",
  program: "crate::artifacts::program",
  registers: "crate::artifacts::program::registers",
  operations: "crate::artifacts::program::op",
  adjacency: "crate::artifacts::program::engine::adjacency",
  analyze: "crate::artifacts::program::engine::analyze",
  exchange: "crate::artifacts::program::engine::exchange",
  outputs: "crate::artifacts::program::engine::outputs",
  report: "crate::artifacts::program::engine::report",
  search: "crate::artifacts::program::engine::search",
  status_summary: "crate::artifacts::program::engine::status_summary",
  template: "crate::artifacts::program::engine::template",
  trace: "crate::artifacts::program::engine::trace",
  validate: "crate::artifacts::program::engine::validate",
};

const CRATE_REF_RE = new RegExp(`crate::(${Object.keys(MODULE_PATHS).join("|")})::`, "g");

/** 🧹 Dedents one level and repoints every `crate::<old-mod>::` at its new taxonomy module path. */
function normalize(body: string[]): string {
  return body
    .map((line) => (line.startsWith("    ") ? line.slice(4) : line))
    .join("\n")
    .replace(CRATE_REF_RE, (_all, name: string) => `${MODULE_PATHS[name]}::`)
    .replace(/^\n+/, "")
    .replace(/\n+$/, "\n");
}

/** 🩻 Splits a module body at a `// #region <name>` … `// #endregion` marker pair. */
function cutRegion(body: string[], marker: string): { inside: string[]; rest: string[] } {
  const start = body.findIndex((line) => line.trim() === `// #region ${marker}`);
  if (start < 0) throw new Error(`region ${marker} not found`);
  let end = start + 1;
  while (end < body.length && body[end]!.trim() !== "// #endregion") end++;
  return { inside: body.slice(start + 1, end), rest: [...body.slice(0, start), ...body.slice(end + 1)] };
}

function emit(relPath: string, header: string, body: string) {
  const abs = join(pluginRoot, relPath);
  mkdirSync(dirname(abs), { recursive: true });
  writeFileSync(abs, `${header}\n${body}`);
  console.log(`wrote ${relPath} (${body.split("\n").length} lines)`);
}

const mods = extractModules();
console.log("modules:", [...mods.keys()].join(", "));

const ARTIFACT = "🗿️artifacts/🏛️program";

// 🧱️ Document model — kernel + registers as artifact-root sibling topic files.
emit(`${ARTIFACT}/🦀️kernel.rs`, "//! 🧱️ Architect program artifact — shared kernel types for program entities: ids, headers,\n//! quantities, traces, and diagnostics.\n", normalize(mods.get("kernel")!));
emit(`${ARTIFACT}/🦀️registers.rs`, "//! 🏛️ Architect program artifact — the typed register entities for all 65 feature areas (the\n//! document model's row types; the `Program` document that holds them lives in `🦀️component.rs`).\n", normalize(mods.get("registers")!));

// 🔧️ op / 🔺️diff — the operations module minus its ProgramDiff region.
const operations = cutRegion(mods.get("operations")!, "🔖️ProgramDiff");
emit(`${ARTIFACT}/🔧️op/🦀️component.rs`, "//! 🔁️ Architect program artifact — the typed document operation surface: `ProgramOperation`,\n//! its apply/invert kernel and its `OpText`/`OpBinary` wire codecs (constitutional: op).\n", normalize(operations.rest));
emit(`${ARTIFACT}/🔺️diff/🦀️component.rs`, "//! 📦️ Architect program artifact — the operation-diff carrier (constitutional: diff).\n", normalize(operations.inside));

// ⚙️ engine topic files.
for (const topic of ["adjacency", "analyze", "exchange", "outputs", "report", "search", "status_summary", "template", "trace", "validate"]) {
  emit(`${ARTIFACT}/⚙️engine/🦀️${topic}.rs`, `//! ⚙️ Architect program artifact engine — the \`${topic}\` topic.\n`, normalize(mods.get(topic)!));
}

// 🩻 The document itself.
emit(`${ARTIFACT}/🦀️component.rs`, "//! 🏛️ Architect program artifact — the root program document: all 65 feature-area registers plus\n//! meta, project, and governance (constitutional: general).\n", normalize(mods.get("program")!));
