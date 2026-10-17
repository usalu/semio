// 🧱️ Lowers the shared helpers of the writer editor root (ids, UI value factories, interaction ids, typed media I/O, document reset effect) into the new layer `🧱️base`, and re-points every user. usage: bun wri-base.ts [--write]
import { readFileSync, writeFileSync, mkdirSync, existsSync, readdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { parseRustSourceTree, bindingsOf, usedNames, renderUses, type Binding } from "./lib-imports.ts";

const write = process.argv.includes("--write");
const REPO = "/Users/ueli/Documents/semio";
const ART = `${REPO}/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer`;
const E = `${ART}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`;
const ORIG = `${REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/r7c-tools/orig-wri`;
const BASE = ["WRITER_PLAY_APP_ID", "writer_action", "ui_label", "ui_value_text", "ui_value_bool", "ui_value_number", "ui_value_list", "ui_value_map", "ui_node_list", "WRITER_INTERACTION_AST", "WRITER_INTERACTION_GRANULARITY", "writer_io", "WriterChapterPayload", "writer_chapter_payload", "reset_document_effect"];
const rootPath = `${E}/🦀️.rs`;
let root = readFileSync(rootPath, "utf8");
const cut = (from: string, to: string, inclusiveTo = true): string => {
  const a = root.indexOf(from);
  if (a < 0 || root.indexOf(from, a + 1) >= 0) throw new Error(`anchor ${from.slice(0, 40)}`);
  const b = root.indexOf(to, a);
  if (b < 0) throw new Error(`end anchor ${to.slice(0, 40)}`);
  const end = inclusiveTo ? b + to.length : b;
  const text = root.slice(a, end);
  root = root.slice(0, a) + root.slice(end);
  return text;
};
const idLine = cut("pub const WRITER_PLAY_APP_ID: &str = \"s.writer.writer@1/*#editor\";\n", "\n");
const seg1 = cut("/// 🎯️ An `ActionDescriptor` addressed at this app", "pub use semio_framework_plugin::ui_node_list;\n");
const seg2 = cut("/// 🕹️ The `ast` interaction domain this app declares", "pub const WRITER_INTERACTION_GRANULARITY: &str = \"node\";\n");
const seg3 = cut("//#region 🔖️Io\n", "//#endregion 🔖️Io\n");
const rootNew = root.replace(/\n{4,}/gu, "\n\n\n");
const rootUses = new Map<string, Binding>();
const tree = parseRustSourceTree(readFileSync(rootPath, "utf8"));
const src = readFileSync(rootPath, "utf8");
for (const item of tree.items) if (item.kind === "use" && item.visibility === "private") for (const b of bindingsOf(src.slice(item.declStart, item.end), false)) rootUses.set(b.name, b);
const baseBody = `${idLine}\n${seg1}\n${seg2}\n${seg3}`;
const baseUsed = usedNames(baseBody, rootUses.keys());
const baseBindings = [...baseUsed].filter((n) => rootUses.has(n)).sort().map((n) => rootUses.get(n)!);
const baseText = `//! 🧱️ Writer editor core: the app id, the UI value factories, the interaction ids, the typed media I/O surface and the whole-document reset effect every other layer of the editor builds on.\n\n${renderUses(baseBindings).join("\n")}\nuse serde::{Deserialize, Serialize};\n\n//#region 🔖️Constants\n${baseBody.replace(/^\/\/#region 🔖️Io\n/mu, "//#endregion 🔖️Constants\n\n//#region 🔖️Io\n")}`;
const remaining = rootNew;
const rootNeeds = BASE.filter((n) => usedNames(remaining, [n]).has(n));
const baseImport = `use crate::editor::writer::base::{${rootNeeds.sort().join(", ")}};\n`;
const mark = "use crate::editor::writer::commands::set_camera;\n";
if (remaining.split(mark).length !== 2) throw new Error("import anchor");
const rootOut = remaining.replace(mark, baseImport + mark);
console.log(`root needs from base: ${rootNeeds.join(", ")}; base imports ${baseBindings.length}`);

const edits = new Map<string, string>();
const orig = new Map<string, string>();
const set = (p: string, t: string) => { if (!orig.has(p)) orig.set(p, readFileSync(p, "utf8")); edits.set(p, t); };
const get = (p: string) => edits.get(p) ?? readFileSync(p, "utf8");
set(rootPath, rootOut);
const GROUP = /(crate)::editor::writer::\{/gu;
const PATH = /\bcrate::editor::writer::([A-Za-z_][A-Za-z0-9_]*)\b(?!::\{)/gu;
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
    if (keep.length) lines.push(`${head}crate::editor::writer::${keep.length === 1 && !keep[0]!.includes("{") ? keep[0] : `{${keep.join(", ")}}`};`);
    lines.push(`${indent}use crate::editor::writer::base::${moved.length === 1 ? moved[0] : `{${moved.join(", ")}}`};`);
    reps.push({ s: ls, e: le, r: lines.join("\n") });
  }
  for (const r of reps) { out += text.slice(last, r.s) + r.r; last = r.e; }
  out += text.slice(last);
  return out.replace(PATH, (w, n: string) => (isBase(n) ? `crate::editor::writer::base::${n}` : w));
};
const walk = (dir: string, acc: string[] = []): string[] => { for (const e of readdirSync(dir)) { if (e === "node_modules" || e === "target") continue; const p = join(dir, e); if (statSync(p).isDirectory()) walk(p, acc); else if (e.endsWith(".rs")) acc.push(p); } return acc; };
let n = 0;
for (const f of [...walk(ART), ...walk(`${REPO}/✏️s/🔌️plugins/✒️writer/🏭️bridge`), ...walk(`${REPO}/🌎️hub/🧩️compositions/✒️writer`)]) {
  if (f === rootPath) continue;
  const before = get(f);
  if (!before.includes("editor::writer")) continue;
  const after = rewrite(before);
  if (after !== before) { set(f, after); n++; }
}
const nestPath = `${ART}/🦀️.rs`;
const nest = get(nestPath);
const anchor = `        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]\n        pub mod terminology;\n`;
if (nest.split(anchor).length !== 2) throw new Error("nest anchor");
set(nestPath, nest.replace(anchor, `        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧱️base/🦀️.rs"]\n        pub mod base;\n${anchor}`));
console.log(`repointed ${n} files; edited ${edits.size}`);
if (write) {
  for (const [p, t] of edits) { const bak = join(ORIG, p.slice(REPO.length)); if (!existsSync(bak)) { mkdirSync(dirname(bak), { recursive: true }); writeFileSync(bak, orig.get(p)!); } writeFileSync(p, t); }
  mkdirSync(`${E}/🧱️base`, { recursive: true });
  writeFileSync(`${E}/🧱️base/🦀️.rs`, baseText);
  console.log("written");
}
