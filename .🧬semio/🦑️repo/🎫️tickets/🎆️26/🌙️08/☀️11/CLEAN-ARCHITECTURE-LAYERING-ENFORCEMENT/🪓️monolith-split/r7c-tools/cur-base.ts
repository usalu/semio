// 🧱️ Lowers the shared helpers of the sourcing editor root (io, ids, table and action factories, UI value factories, document reset effect) into the new layer `🧱️base`, makes the file children of the root import them explicitly and re-points every user. usage: bun cur-base.ts [--write]
import { readFileSync, writeFileSync, mkdirSync, existsSync, readdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { parseRustSourceTree, bindingsOf, usedNames, renderUses, type Binding } from "./lib-imports.ts";
const write = process.argv.includes("--write");
const REPO = "/Users/ueli/Documents/semio";
const PLUGIN = `${REPO}/✏️s/🔌️plugins/🪵️sourcing`;
const ART = `${PLUGIN}/🗿️artifacts/🗂️curation`;
const E = `${ART}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`;
const ORIG = `${REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/r7c-tools/orig-cur`;
const M = "crate::editor::sourcing";
const rootPath = `${E}/🦀️.rs`;
let root = readFileSync(rootPath, "utf8");
const cutRegion = (name: string): string => {
  const open = `//#region ${name}\n`, close = `//#endregion ${name}\n`;
  const a = root.indexOf(open);
  if (a < 0 || root.indexOf(open, a + 1) >= 0) throw new Error(`region ${name}`);
  const b = root.indexOf(close, a);
  if (b < 0) throw new Error(`end ${name}`);
  const text = root.slice(a, b + close.length);
  root = root.slice(0, a) + root.slice(b + close.length);
  return text;
};
const regions = ["🔖️Io", "🔖️Constants", "🔖️ResetDocument"].map(cutRegion);
const baseBody = regions.join("\n");
const origRoot = readFileSync(rootPath, "utf8");
const origTree = parseRustSourceTree(origRoot);
const rootUses = new Map<string, Binding>();
for (const item of origTree.items) if (item.kind === "use" && item.visibility === "private") for (const b of bindingsOf(origRoot.slice(item.declStart, item.end), false)) rootUses.set(b.name, b);
const baseTree = parseRustSourceTree(baseBody);
const BASE = baseTree.items.filter((i: any) => i.name && i.kind !== "use" && i.kind !== "impl").map((i: any) => i.name as string);
const baseBindings = [...usedNames(baseBody, rootUses.keys())].sort().map((n) => rootUses.get(n)!);
const baseText = `//! 🧱️ Sourcing editor core: the typed media I/O surface, the controller and interaction ids, the action and table factories, the UI value factories and the whole-document reset effect every other layer of the editor builds on.\n\n${renderUses(baseBindings).join("\n")}\n\n${baseBody}`;
const rootRest = root.replace(/\n{4,}/gu, "\n\n\n");
const baseImportFor = (text: string): string => { const names = BASE.filter((n) => usedNames(text, [n]).has(n)).sort(); return names.length ? `use ${M}::base::{${names.join(", ")}};\n` : ""; };
console.log(`base items: ${BASE.join(", ")}`);

const edits = new Map<string, string>(), orig = new Map<string, string>();
const set = (p: string, t: string) => { if (!orig.has(p)) orig.set(p, readFileSync(p, "utf8")); edits.set(p, t); };
const get = (p: string) => edits.get(p) ?? readFileSync(p, "utf8");
const marker = "use crate::editor::sourcing::modes::edit;\n";
if (rootRest.split(marker).length !== 2) throw new Error("root import anchor");
set(rootPath, rootRest.replace(marker, baseImportFor(rootRest) + marker));
for (const dir of ["🧰️retained-commands", "📬️config-store-preparation", "🗄️artifact-store-preparation", "🧩️sourcing-app", "📋️manifest"]) {
  const p = `${E}/${dir}/🦀️.rs`;
  const t = readFileSync(p, "utf8");
  if (t.split("use super::*;\n").length !== 2) throw new Error(`glob ${dir}`);
  const imp = baseImportFor(t.replace("use super::*;\n", ""));
  if (imp) set(p, t.replace("use super::*;\n", `use super::*;\n${imp}`));
}
const GROUP = /\bcrate::editor::sourcing::\{/gu;
const PATH = /\bcrate::editor::sourcing::([A-Za-z_][A-Za-z0-9_]*)\b(?!::\{)/gu;
const isBase = (n: string) => BASE.includes(n);
const splitTop = (s: string): string[] => { const parts: string[] = []; let d = 0, cur = ""; for (const ch of s) { if (ch === "{") d++; if (ch === "}") d--; if (ch === "," && d === 0) { parts.push(cur.trim()); cur = ""; } else cur += ch; } if (cur.trim()) parts.push(cur.trim()); return parts.filter(Boolean); };
const rewrite = (text: string): string => {
  let out = "", last = 0, m: RegExpExecArray | null;
  GROUP.lastIndex = 0;
  const reps: { s: number; e: number; r: string }[] = [];
  while ((m = GROUP.exec(text))) {
    const open = m.index + m[0].length - 1;
    let depth = 0, close = -1;
    for (let k = open; k < text.length; k++) { if (text[k] === "{") depth++; else if (text[k] === "}") { depth--; if (depth === 0) { close = k; break; } } }
    const entries = splitTop(text.slice(open + 1, close));
    const moved = entries.filter((e) => !e.includes("::") && isBase(e)), keep = entries.filter((e) => !moved.includes(e));
    if (!moved.length) continue;
    const ls = text.lastIndexOf("\n", m.index) + 1, le = text.indexOf(";", close) + 1;
    const head = text.slice(ls, m.index);
    if (!/^\s*(?:pub\s+)?use\s+$/u.test(head)) throw new Error(`group outside use: ${text.slice(ls, close + 1).slice(0, 100)}`);
    const indent = /^\s*/u.exec(head)![0];
    const lines: string[] = [];
    if (keep.length) lines.push(`${head}${M}::${keep.length === 1 && !keep[0]!.includes("{") ? keep[0] : `{${keep.join(", ")}}`};`);
    lines.push(`${indent}use ${M}::base::${moved.length === 1 ? moved[0] : `{${moved.join(", ")}}`};`);
    reps.push({ s: ls, e: le, r: lines.join("\n") });
  }
  for (const r of reps) { out += text.slice(last, r.s) + r.r; last = r.e; }
  out += text.slice(last);
  return out.replace(PATH, (w, n: string) => (isBase(n) ? `${M}::base::${n}` : w));
};
const walk = (dir: string, acc: string[] = []): string[] => { for (const e of readdirSync(dir)) { if (e === "node_modules" || e === "target") continue; const p = join(dir, e); if (statSync(p).isDirectory()) walk(p, acc); else if (e.endsWith(".rs")) acc.push(p); } return acc; };
let n = 0;
for (const f of [...walk(ART), ...walk(`${PLUGIN}/🏭️bridge`)]) {
  if (f === rootPath) continue;
  const before = get(f);
  if (!before.includes("editor::sourcing")) continue;
  const after = rewrite(before);
  if (after !== before) { set(f, after); n++; }
}
const nestPath = `${ART}/🦀️.rs`;
const nest = get(nestPath);
const anchor = `        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]\n        mod component;\n        pub use component::*;\n`;
if (nest.split(anchor).length !== 2) throw new Error("nest anchor");
set(nestPath, nest.replace(anchor, `${anchor}\n        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧱️base/🦀️.rs"]\n        pub mod base;\n`));
console.log(`repointed ${n} files; edited ${edits.size}`);
if (write) {
  for (const [p, t] of edits) { const bak = join(ORIG, p.slice(REPO.length)); if (!existsSync(bak)) { mkdirSync(dirname(bak), { recursive: true }); writeFileSync(bak, orig.get(p)!); } writeFileSync(p, t); }
  mkdirSync(`${E}/🧱️base`, { recursive: true });
  writeFileSync(`${E}/🧱️base/🦀️.rs`, baseText);
  console.log("written");
}
