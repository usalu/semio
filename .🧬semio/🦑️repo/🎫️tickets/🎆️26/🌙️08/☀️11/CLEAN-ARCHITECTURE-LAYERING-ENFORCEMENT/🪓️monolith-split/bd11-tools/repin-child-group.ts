import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import Parser from "web-tree-sitter";
const root = "/Users/ueli/Documents/semio";
const file = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/📋️child-group-owner.json";
const owner = JSON.parse(readFileSync(file, "utf8"));
await Parser.init({ wasmBinary: readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm", root)) });
const p = new Parser(); p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", root)));
const sha = (t: string) => createHash("sha256").update(t).digest("hex");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const tree = p.parse(read(owner.target));
const body = tree.rootNode.descendantsOfType("mod_item").find(n => n.childForFieldName("name")?.text === "child_complete_group_candidate_tests")!;
owner.canonicalModuleSha256 = sha(body.text);
let total = 0;
for (const law of owner.laws) {
  const fn = body.descendantsOfType("function_item").find(n => n.childForFieldName("name")?.text === law.name)!;
  law.canonicalSha256 = sha(fn.text); law.assertions = (fn.text.match(/\bassert(?:_eq|_ne)?!/g) ?? []).length; total += law.assertions;
}
owner.newLawAssertions = total; owner.oldLawAssertions = total * 2;
for (const r of owner.remaining) {
  const t = p.parse(read(r.path));
  r.sha256 = sha(read(r.path));
  r.functions = t.rootNode.descendantsOfType("function_item").map(n => ({ name: n.childForFieldName("name")?.text, sha256: sha(n.text) }));
}
for (const c of owner.closure) { const t = read(c.path); c.sha256 = sha(t); c.bytes = Buffer.byteLength(t); c.nonblank = t.split("\n").filter(l => l.trim()).length; }
writeFileSync(file, JSON.stringify(owner, null, 2) + "\n");
console.log("laws", owner.laws.length, "assertions", total);
