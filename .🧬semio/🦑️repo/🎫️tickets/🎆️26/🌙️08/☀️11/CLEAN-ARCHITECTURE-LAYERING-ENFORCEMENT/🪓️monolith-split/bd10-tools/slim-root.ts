// 🧰 bd10: moves the content items (fn, struct, enum, impl, const, ...) of a crate lib root into one child module so the root is wiring only.
// usage: bun slim-root.ts <root.rs> <childFolder> <childDoc> [--write]
import {readFileSync, writeFileSync, mkdirSync} from "node:fs";
import {dirname, join} from "node:path";
import {parseRustSourceTree, rustItemStartWithComments} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import {rustTokens, rustTokenPairs} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const [root, folder, doc, regionArg] = process.argv.slice(2).filter((a) => a !== "--write");
const write = process.argv.includes("--write");
const source = readFileSync(root!, "utf8");
const tree = parseRustSourceTree(source);
const regionRanges = (regionArg ?? "").split(",").filter(Boolean).map((name) => { const a = source.indexOf(`//#region ${name}`); const b = source.indexOf(`//#endregion ${name}`); if (a < 0 || b < a) throw new Error(`region ${name} not found`); return [a, b] as const; });
const moved = tree.items.filter((item) => !["use", "extern-crate"].includes(item.kind) && (!regionRanges.length || regionRanges.some(([a, b]) => item.start >= a && item.start <= b)));
if (!moved.length) { console.log("nothing to move"); process.exit(0); }
const macros = moved.filter((item) => item.kind === "macro");
if (macros.length) { console.log("MACROS present", macros.map((m) => m.name)); process.exit(1); }
const ranges = moved.map((item) => ({start: rustItemStartWithComments(source, item.start), end: item.end, item}));
const widen = (text: string, item: (typeof moved)[number]): string => {
  const head = /^((?:\s*(?:\/\/\/[^\n]*|#\[[^\n]*\]|\/\*\*[\s\S]*?\*\/)\n)*)(\s*)((?:pub(?:\([^)]*\))?\s+)?)/u;
  let out = text;
  if (item.kind !== "impl" && item.visibility === "") {
    const m = /(\n|^)(\s*)(?!pub)(?=(?:async\s+|unsafe\s+|const\s+(?=fn)|extern\s+"[^"]*"\s+)*(?:fn|struct|enum|type|const|static|trait|union|mod)\s)/u;
    out = out.replace(m, (all, nl, ws) => `${nl}${ws}pub(super) `);
  }
  if (item.kind === "struct") {
    out = out.replace(/(\{\s*)([\s\S]*)(\n\})/u, (all, open, body, close) => `${open}${body.replace(/^(\s*(?:#\[[^\n]*\]\s*)*)(?!pub)([a-z_][a-z0-9_]*\s*:)/gmu, "$1pub(super) $2")}${close}`);
  }
  if (item.kind === "impl" && !/^\s*(?:#\[[^\n]*\]\s*)*(?:unsafe\s+)?impl(?:<[^>]*>)?\s+[^{]*\bfor\b/u.test(text.replace(/\/\/\/[^\n]*\n/gu, ""))) {
    out = out.replace(/^(\s+)(?!pub)((?:async\s+|const\s+|unsafe\s+)*fn\s)/gmu, "$1pub(super) $2");
  }
  return out;
};
const body = ranges.map(({start, end, item}) => widen(source.slice(start, end), item)).join("\n\n");
let rootText = source;
for (const {start, end} of [...ranges].sort((a, b) => b.start - a.start)) rootText = rootText.slice(0, start) + rootText.slice(end);
rootText = rootText.replace(/\n{3,}/gu, "\n\n").trimEnd() + "\n";
const modName = folder!.replace(/^[^a-z]*/u, "").replace(/-/gu, "_");
const wiring = `\n#[path = "${folder}/🦀️.rs"]\nmod ${modName};\npub use ${modName}::*;\n`;
const childText = `//! ${doc}\n\nuse super::*;\n\n${body}\n`;
console.log(`moved ${moved.length} items (${body.split("\n").length} lines) -> ${folder}; root ${source.split("\n").length} -> ${(rootText + wiring).split("\n").length} lines`);
if (write) {
  mkdirSync(join(dirname(root!), folder!), {recursive: true});
  writeFileSync(join(dirname(root!), folder!, "🦀️.rs"), childText);
  writeFileSync(root!, rootText + wiring);
}
