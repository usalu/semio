// 🪜️ Lowers the helpers of the sequence editor root into their owning child modules: every `use super::*;` of a child becomes explicit imports, every root path (`editor::sequence::Name`) names the owning child, the children are mounted by the nest instead of the root.
// usage: bun seq-lower-root.ts [--write]
import { readFileSync, writeFileSync, mkdirSync, existsSync, readdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { parseRustSourceTree, bindingsOf, usedNames, renderUses, expandUse, type Binding } from "./lib-imports.ts";

const write = process.argv.includes("--write");
const ART = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence";
const E = `${ART}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`;
const ORIG = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/r7c-tools/orig-seq";
const kids: Record<string, string> = { base: "🧱️base", host: "🖥️host", retained: "🧵️retained", persistent: "🔁️persistent", import: "🎞️import", app: "🎹️app" };
const EXT: Record<string, string> = {
  SEQUENCE_PLAY_BODY_CATALOGUE: "crate::editor::sequence::panels::catalogue::SEQUENCE_PLAY_BODY_CATALOGUE",
  SEQUENCE_PLAY_BODY_COMPILED: "crate::editor::sequence::modes::edit::windows::compiled::SEQUENCE_PLAY_BODY_COMPILED",
  SEQUENCE_PLAY_BODY_ARTIFACT: "crate::editor::sequence::panels::document::SEQUENCE_PLAY_BODY_ARTIFACT",
  SEQUENCE_PLAY_BODY_INSPECTOR: "crate::editor::sequence::panels::inspection::SEQUENCE_PLAY_BODY_INSPECTOR",
  SEQUENCE_PLAY_BODY_MAIN: "crate::editor::sequence::modes::edit::windows::main::SEQUENCE_PLAY_BODY_MAIN",
  SEQUENCE_PLAY_BODY_SCRIPT: "crate::editor::sequence::modes::edit::windows::script::SEQUENCE_PLAY_BODY_SCRIPT",
  ui_node_list: "semio_framework_plugin::ui_node_list",
};

const rootText = readFileSync(`${E}/🦀️.rs`, "utf8");
const rootTree = parseRustSourceTree(rootText);
const rootBindings = new Map<string, Binding>();
for (const item of rootTree.items) {
  if (item.kind !== "use" || item.visibility !== "private") continue;
  const stmt = rootText.slice(item.declStart, item.end);
  const cfgTest = item.attributeTexts.some((a: string) => /cfg\s*\(\s*test\s*\)/u.test(a));
  for (const b of bindingsOf(stmt, cfgTest)) rootBindings.set(b.name, b);
}
for (const b of rootBindings.values()) {
  const head = b.path.split("::")[0]!;
  if (head !== b.name && rootBindings.has(head) && !["crate", "self", "super"].includes(head)) b.path = b.path.replace(head, rootBindings.get(head)!.path);
}

const childOf = new Map<string, string>();
for (const [name, dir] of Object.entries(kids)) {
  const text = readFileSync(`${E}/${dir}/🦀️.rs`, "utf8");
  const tree = parseRustSourceTree(text);
  for (const item of tree.items) {
    if (item.kind === "use" || item.kind === "impl" || !item.name) continue;
    if (item.visibility !== "private") {
      if (childOf.has(item.name)) console.log(`dup provided ${item.name}: ${childOf.get(item.name)} and ${name}`);
      childOf.set(item.name, name);
    }
  }
}
childOf.set("SequenceCommand", "retained");
for (const n of Object.keys(EXT)) childOf.delete(n);

const allNames = new Set<string>([...childOf.keys(), ...Object.keys(EXT), ...rootBindings.keys()]);

const ownNames = (text: string): Set<string> => {
  const own = new Set<string>();
  const tree = parseRustSourceTree(text);
  for (const item of tree.items) {
    if (item.name && item.kind !== "impl") own.add(item.name);
    if (item.kind === "use") for (const b of bindingsOf(text.slice(item.declStart, item.end), false)) own.add(b.name);
  }
  for (const m of tree.modules) own.add(m.name);
  return own;
};

const explicitFor = (text: string, ownChild: string | null, indent = ""): string => {
  const own = ownNames(text);
  const used = usedNames(text, allNames);
  const ext: Binding[] = [], roots: Binding[] = [];
  const byChild = new Map<string, string[]>();
  for (const name of [...used].sort()) {
    if (own.has(name)) continue;
    if (EXT[name]) ext.push({ name, path: EXT[name]!, alias: false, cfgTest: false });
    else if (childOf.has(name)) {
      const c = childOf.get(name)!;
      if (c === ownChild) continue;
      byChild.set(c, [...(byChild.get(c) ?? []), name]);
    } else if (rootBindings.has(name)) roots.push(rootBindings.get(name)!);
  }
  const lines = [...renderUses(roots, indent), ...renderUses(ext, indent)];
  for (const [c, names] of byChild) lines.push(...renderUses(names.map((n) => ({ name: n, path: `crate::editor::sequence::${c}::${n}`, alias: false, cfgTest: false })), indent));
  return lines.join("\n");
};

const edits = new Map<string, string>();
const original = new Map<string, string>();
const setText = (path: string, text: string) => { if (!original.has(path)) original.set(path, readFileSync(path, "utf8")); edits.set(path, text); };
const getText = (path: string) => edits.get(path) ?? readFileSync(path, "utf8");

for (const [name, dir] of Object.entries(kids)) {
  const path = `${E}/${dir}/🦀️.rs`;
  const text = getText(path);
  const globs = text.split("\n").filter((l) => l.trim() === "use super::*;");
  if (globs.length !== 1) throw new Error(`${name}: glob count ${globs.length}`);
  const body = text.replace(/^use super::\*;$/mu, "@@EXPLICIT@@");
  const rendered = explicitFor(body.replace("@@EXPLICIT@@", ""), name);
  setText(path, body.replace("@@EXPLICIT@@", rendered));
  console.log(`${name}: ${rendered.split("\n").length} import line(s)`);
}

const unitPath = `${E}/🧪️tests/🔬️unit/🦀️.rs`;
{
  const text = getText(unitPath);
  const pre = text.replace(/^(\s*)use super::super::\*;$/mu, "$1@@CTX@@").replace(/^use super::\*;$/mu, "@@TOP@@");
  if (!pre.includes("@@CTX@@") || !pre.includes("@@TOP@@")) throw new Error("unit globs missing");
  const all = explicitFor(pre.replace("@@CTX@@", "").replace("@@TOP@@", ""), null);
  const ctxStart = pre.indexOf("@@CTX@@"), topAt = pre.indexOf("@@TOP@@");
  const ctxText = pre.slice(0, topAt).replace("@@CTX@@", "");
  const topText = pre.slice(topAt).replace("@@TOP@@", "");
  const ctxImports = explicitFor(ctxText, null, "    ");
  const topImports = explicitFor(topText, null);
  setText(unitPath, pre.replace("@@CTX@@", ctxImports.trimStart() ? ctxImports.replace(/^ {4}/u, "") : "").replace("@@TOP@@", topImports));
  void all; void ctxStart;
}

const GROUP = /(crate|semio_s_artifact_sequence_sequence)::editor::sequence::\{/gu;
const PATH = /\b(crate|semio_s_artifact_sequence_sequence)::editor::sequence::([A-Za-z_][A-Za-z0-9_]*)\b(?!::\{)/gu;
const splitTop = (s: string): string[] => {
  const parts: string[] = []; let depth = 0, cur = "";
  for (const ch of s) {
    if (ch === "{") depth += 1; if (ch === "}") depth -= 1;
    if (ch === "," && depth === 0) { parts.push(cur.trim()); cur = ""; } else cur += ch;
  }
  if (cur.trim()) parts.push(cur.trim());
  return parts.filter(Boolean);
};
const mapName = (prefix: string, name: string): string | null => {
  if (EXT[name]) return EXT[name]!.replace(/^crate::/u, `${prefix}::`);
  if (childOf.has(name)) return `${prefix}::editor::sequence::${childOf.get(name)}::${name}`;
  return null;
};
const rewritePaths = (text: string): string => {
  let out = "";
  let last = 0;
  GROUP.lastIndex = 0;
  let m: RegExpExecArray | null;
  const matches: { start: number; end: number; repl: string }[] = [];
  while ((m = GROUP.exec(text))) {
    const open = m.index + m[0].length - 1;
    let depth = 0, close = -1;
    for (let k = open; k < text.length; k += 1) { if (text[k] === "{") depth += 1; else if (text[k] === "}") { depth -= 1; if (depth === 0) { close = k; break; } } }
    if (close < 0) throw new Error("unbalanced group");
    const inner = text.slice(open + 1, close);
    const entries = splitTop(inner);
    const prefix = m[1]!;
    const keep: string[] = [], moved: string[] = [];
    for (const e of entries) { const head = e.split("::")[0]!.trim(); if (!e.includes("::") && (EXT[head] || childOf.has(head))) moved.push(head); else keep.push(e); }
    if (!moved.length) continue;
    const lineStart = text.lastIndexOf("\n", m.index) + 1;
    const lineEnd = text.indexOf(";", close) + 1;
    const head = text.slice(lineStart, m.index);
    if (!/^\s*(?:pub\s+)?use\s+$/u.test(head)) throw new Error(`group outside use: ${text.slice(lineStart, close + 1).slice(0, 120)}`);
    const indent = /^\s*/u.exec(head)![0];
    const lines: string[] = [];
    if (keep.length) lines.push(`${head}${prefix}::editor::sequence::${keep.length === 1 && !keep[0]!.includes("{") ? keep[0] : `{${keep.join(", ")}}`};`);
    const byTarget = new Map<string, string[]>();
    for (const n of moved) { const full = mapName(prefix, n)!; const idx = full.lastIndexOf("::"); byTarget.set(full.slice(0, idx), [...(byTarget.get(full.slice(0, idx)) ?? []), full.slice(idx + 2)]); }
    for (const [parent, names] of byTarget) lines.push(`${indent}use ${parent}::${names.length === 1 ? names[0] : `{${names.join(", ")}}`};`);
    matches.push({ start: lineStart, end: lineEnd, repl: lines.join("\n") });
  }
  for (const r of matches) { out += text.slice(last, r.start) + r.repl; last = r.end; }
  out += text.slice(last);
  return out.replace(PATH, (whole, prefix: string, name: string) => mapName(prefix, name) ?? whole);
};

const walk = (dir: string, acc: string[] = []): string[] => {
  for (const e of readdirSync(dir)) {
    if (e === "node_modules" || e === "target") continue;
    const p = join(dir, e);
    const st = statSync(p);
    if (st.isDirectory()) walk(p, acc); else if (e.endsWith(".rs")) acc.push(p);
  }
  return acc;
};
const REPO = "/Users/ueli/Documents/semio";
const files = [
  ...walk(ART),
  `${REPO}/✏️s/🔌️plugins/🎬️sequence/🏭️bridge/🦀️.rs`,
  `${REPO}/🌎️hub/🧩️compositions/🎬️sequence/🦀️.rs`,
  `${REPO}/🌎️hub/🧩️compositions/🎬️sequence/🧪️tests/🔬️surface/🦀️.rs`,
  `${REPO}/✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🎬️sequence-runtime/🦀️.rs`,
  `${REPO}/✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🎬️sequence-runtime/🏃️playback/🦀️.rs`,
  `${REPO}/✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🎬️sequence-runtime/🪟️windows/🦀️.rs`,
];
let changed = 0;
for (const f of files) {
  if (!existsSync(f)) { console.log(`missing ${f}`); continue; }
  const before = getText(f);
  if (!before.includes("editor::sequence")) continue;
  const after = rewritePaths(before);
  if (after !== before) { setText(f, after); changed += 1; console.log(`repoint ${f.slice(REPO.length + 1).slice(-90)}`); }
}
console.log(`files changed by repoint: ${changed}; total edited: ${edits.size}`);
if (write) {
  for (const [path, text] of edits) {
    const bak = join(ORIG, path.slice(REPO.length));
    if (!existsSync(bak)) { mkdirSync(dirname(bak), { recursive: true }); writeFileSync(bak, original.get(path)!); }
    writeFileSync(path, text);
  }
  console.log("written");
}
