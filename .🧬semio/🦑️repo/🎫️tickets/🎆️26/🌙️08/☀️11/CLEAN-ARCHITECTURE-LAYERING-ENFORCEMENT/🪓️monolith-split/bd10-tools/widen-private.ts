// 🧰 bd10: gives every private top-level fn/struct/enum/type/const/static/trait/mod of a moved child file `pub(super)` visibility (private struct fields too), so sibling modules reach it through the parent glob.
// usage: bun widen-private.ts <file.rs> [--write] [--pub=name,name]
import {readFileSync, writeFileSync} from "node:fs";
import {parseRustSourceTree} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";

const file = process.argv[2]!;
const write = process.argv.includes("--write");
const makePub = new Set((process.argv.find((a) => a.startsWith("--pub=")) ?? "").slice(6).split(",").filter(Boolean));
let text = readFileSync(file, "utf8");
const tree = parseRustSourceTree(text);
const edits: {start: number; end: number; text: string}[] = [];
const kinds = new Set(["fn", "struct", "enum", "type", "const", "static", "trait", "union", "mod"]);
const names: string[] = [];
for (const item of tree.items) {
  if (!kinds.has(item.kind) || item.visibility !== "private") continue;
  const vis = makePub.has(item.name ?? "") ? "pub " : "pub(super) ";
  edits.push({start: item.declStart, end: item.declStart, text: vis});
  names.push(item.name ?? item.kind);
  if (item.kind === "struct") {
    const seg = text.slice(item.declStart, item.end);
    const widened = seg.replace(/(\{\s*)([\s\S]*)(\n\s*\})/u, (_a, open: string, body: string, close: string) => `${open}${body.replace(/^(\s*(?:#\[[^\n]*\]\s*)*)(?!pub)([a-z_][a-z0-9_]*\s*:)/gmu, "$1pub(super) $2")}${close}`);
    if (widened !== seg) edits.push({start: item.declStart, end: item.end, text: (vis) + widened});
    if (widened !== seg) edits.splice(edits.length - 2, 1);
  }
}
for (const edit of edits.sort((a, b) => b.start - a.start || b.end - a.end)) text = text.slice(0, edit.start) + edit.text + text.slice(edit.end);
console.log(`${file.split("✳️any/")[1]}: widened ${names.length}: ${names.join(", ")}`);
if (write) writeFileSync(file, text);
