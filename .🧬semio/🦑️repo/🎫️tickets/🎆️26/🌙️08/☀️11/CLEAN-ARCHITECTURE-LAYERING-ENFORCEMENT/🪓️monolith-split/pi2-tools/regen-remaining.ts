/** 🧬️ Regenerates `remaining` of `📋️surface-carriers.json` from the cut builder-contract family (support module + non-extension parts), keeping the schema row shape. */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { resolve, dirname, basename } from "node:path";
import { createHash } from "node:crypto";
import Parser from "web-tree-sitter";
const root = "/Users/ueli/Documents/semio";
const units = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units");
const json = JSON.parse(readFileSync(resolve(units, "📋️surface-carriers.json"), "utf8"));
await Parser.init({ wasmBinary: readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm", root)) });
const p = new Parser();
p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", root)));
const dir = dirname(resolve(root, json.original)), parent = dirname(dir), base = basename(dir);
const texts = readdirSync(parent).filter(n => n === base || (n.startsWith(base + "-part-") && !n.endsWith("-x"))).sort().map(n => readFileSync(resolve(parent, n, "🦀️.rs"), "utf8"));
const rows: { name: string; sha256: string; assertions: number; law: boolean; bytes: number }[] = [];
for (const text of texts) {
  const tree = p.parse(text);
  for (const fn of tree.rootNode.descendantsOfType("function_item")) {
    let law = false;
    for (let attr = fn.previousNamedSibling; attr?.type === "attribute_item"; attr = attr.previousNamedSibling) if (/\btest\b|::async_test\b/.test(attr.text)) law = true;
    rows.push({ name: fn.childForFieldName("name")!.text, sha256: createHash("sha256").update(fn.text).digest("hex"), assertions: (fn.text.match(/\bassert(?:_eq|_ne)?!/g) ?? []).length, law, bytes: Buffer.byteLength(fn.text) });
  }
  tree.delete();
}
rows.sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : a.sha256 < b.sha256 ? -1 : a.sha256 > b.sha256 ? 1 : 0));
console.log("old", json.remaining.length, "new", rows.length, "oldLaws", json.remaining.filter((r: { law: boolean }) => r.law).length, "newLaws", rows.filter(r => r.law).length);
if (process.argv.includes("--apply")) {
  json.remaining = rows;
  writeFileSync(resolve(units, "📋️surface-carriers.json"), JSON.stringify(json, null, 2) + "\n");
  console.log("applied");
}
