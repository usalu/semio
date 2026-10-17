/** 🧬️ Re-records the function body hashes of `📋️surface-fixture.json` after the owner crates of the fixture files changed their path spellings; names, targets, law flags and assertion counts must stay equal. */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import Parser from "web-tree-sitter";
const root = "/Users/ueli/Documents/semio";
const file = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/📋️surface-fixture.json");
const json = JSON.parse(readFileSync(file, "utf8"));
await Parser.init({ wasmBinary: readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm", root)) });
const p = new Parser();
p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", root)));
const actual = new Map<string, { name: string; sha: string; assertions: number }[]>();
for (const path of new Set<string>([...json.files.map((r: { path: string }) => r.path), json.metadataTarget])) {
  const tree = p.parse(readFileSync(resolve(root, path), "utf8"));
  actual.set(path, tree.rootNode.descendantsOfType("function_item").map(fn => ({ name: fn.childForFieldName("name")?.text ?? "", sha: createHash("sha256").update(fn.text).digest("hex"), assertions: (fn.text.match(/\bassert(?:_eq|_ne)?!/g) ?? []).length })));
  tree.delete();
}
let changed = 0, total = 0;
const used = new Map<string, Set<number>>();
for (const f of json.files) for (const fn of f.functions) {
  total++;
  const rows = actual.get(fn.target)!;
  const taken = used.get(fn.target) ?? new Set<number>();
  used.set(fn.target, taken);
  const exact = rows.findIndex((r, i) => !taken.has(i) && r.name === fn.name && r.sha === fn.sha256);
  if (exact >= 0) { taken.add(exact); continue; }
  const i = rows.findIndex((r, k) => !taken.has(k) && r.name === fn.name && r.assertions === fn.assertions);
  if (i < 0) throw Error("no row for " + fn.name + " in " + fn.target);
  taken.add(i); fn.sha256 = rows[i]!.sha; changed++;
}
console.log("functions", total, "rehashed", changed);
if (process.argv.includes("--apply")) { writeFileSync(file, JSON.stringify(json, null, 2) + "\n"); console.log("applied"); }
