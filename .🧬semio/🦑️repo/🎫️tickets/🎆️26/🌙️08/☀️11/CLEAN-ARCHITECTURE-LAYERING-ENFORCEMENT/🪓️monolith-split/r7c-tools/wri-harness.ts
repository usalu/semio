// 🧪️ Moves the writer mounted-app harness (`unit_tests`) from the editor root to the artifact root as `editor_unit_tests`, makes its two globs of the root explicit and re-points its users. usage: bun wri-harness.ts [--write]
import { readFileSync, writeFileSync, mkdirSync, existsSync, readdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { parseRustSourceTree, bindingsOf, usedNames, renderUses, type Binding } from "./lib-imports.ts";
const write = process.argv.includes("--write");
const REPO = "/Users/ueli/Documents/semio";
const ART = `${REPO}/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer`;
const E = `${ART}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`;
const ORIG = `${REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/r7c-tools/orig-wri-harness`;
const rootPath = `${E}/🦀️.rs`, unitPath = `${E}/🧪️tests/🔬️unit/🦀️.rs`, nestPath = `${ART}/🦀️.rs`;
const rootText = readFileSync(rootPath, "utf8");
const tree = parseRustSourceTree(rootText);
const rootBindings = new Map<string, Binding>();
const provided = new Map<string, "item">();
for (const item of tree.items) {
  if (item.kind === "use") { if (item.visibility === "private") for (const b of bindingsOf(rootText.slice(item.declStart, item.end), item.attributeTexts.some((a: string) => /cfg\s*\(\s*test\s*\)/u.test(a)))) rootBindings.set(b.name, b); }
  else if (item.name && item.kind !== "impl" && item.visibility !== "private") provided.set(item.name, "item");
}
provided.set("WriterCommand", "item");
const all = [...rootBindings.keys(), ...provided.keys()];
const explicit = (text: string, indent: string): string => {
  const used = usedNames(text, all);
  const own = new Set<string>();
  for (const item of parseRustSourceTree(text).items) { if (item.name && item.kind !== "impl") own.add(item.name); if (item.kind === "use") for (const b of bindingsOf(text.slice(item.declStart, item.end), false)) own.add(b.name); }
  const roots: Binding[] = [], items: string[] = [];
  for (const n of [...used].sort()) { if (own.has(n)) continue; if (rootBindings.has(n)) roots.push(rootBindings.get(n)!); else if (provided.has(n)) items.push(n); }
  return [...renderUses(roots, indent), ...(items.length ? [`${indent}use crate::editor::writer::{${items.join(", ")}};`] : [])].join("\n");
};
const orig = new Map<string, string>(), edits = new Map<string, string>();
const set = (p: string, t: string) => { if (!orig.has(p)) orig.set(p, readFileSync(p, "utf8")); edits.set(p, t); };
const unit = readFileSync(unitPath, "utf8");
const lines = unit.split("\n");
const ctxIdx = lines.findIndex((l) => l.trim() === "use super::super::*;"), topIdx = lines.findIndex((l) => l.trim() === "use super::*;");
if (ctxIdx < 0 || topIdx < 0) throw new Error("globs");
const ctxText = lines.slice(0, topIdx).filter((_, i) => i !== ctxIdx).join("\n");
const topText = lines.slice(topIdx + 1).join("\n") + "\n" + lines.slice(0, topIdx).join("\n");
const ctxImports = explicit(ctxText, "    "), topImports = explicit(lines.filter((_, i) => i !== topIdx && i !== ctxIdx).join("\n"), "");
void topText;
lines[topIdx] = topImports;
lines[ctxIdx] = ctxImports;
set(unitPath, lines.join("\n").replaceAll("crate::editor::writer::unit_tests::", "crate::editor_unit_tests::"));
const mount = `//#region 🧪️UnitTests\n/// 🧪️ Shared test scaffolding for every taxonomy node's own \`🧪️Tests\` region — a component file must be\n/// able to drive the whole app without re-deriving the harness.\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\npub(crate) mod unit_tests;\n`;
if (rootText.split(mount).length !== 2) throw new Error("mount anchor");
set(rootPath, rootText.replace(mount, `//#region 🧪️UnitTests\n`));
const nest = readFileSync(nestPath, "utf8");
const tAnchor = `#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n`;
if (nest.split(tAnchor).length !== 2) throw new Error("nest tests anchor");
set(nestPath, nest.replace(tAnchor, `${tAnchor}/// 🧪️ The mounted-app harness (\`context\`) of the writer editor: it drives the whole app, so it sits above every layer and the tests of commands, windows and panels reach it by this path.\n#[cfg(test)]\n#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]\npub mod editor_unit_tests;\n`));
const walk = (dir: string, acc: string[] = []): string[] => { for (const e of readdirSync(dir)) { if (e === "node_modules" || e === "target") continue; const p = join(dir, e); if (statSync(p).isDirectory()) walk(p, acc); else if (e.endsWith(".rs")) acc.push(p); } return acc; };
let n = 0;
for (const f of walk(ART)) {
  if (f === unitPath || edits.has(f)) continue;
  const t = readFileSync(f, "utf8");
  if (t.includes("editor::writer::unit_tests::")) { set(f, t.replaceAll("crate::editor::writer::unit_tests::", "crate::editor_unit_tests::")); n++; }
}
console.log(`unit imports: ctx ${ctxImports.split("\n").length} top ${topImports.split("\n").length}; repointed ${n} files`);
if (write) for (const [p, t] of edits) { const bak = join(ORIG, p.slice(REPO.length)); if (!existsSync(bak)) { mkdirSync(dirname(bak), { recursive: true }); writeFileSync(bak, orig.get(p)!); } writeFileSync(p, t); }
