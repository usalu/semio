import { readFileSync, writeFileSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
import { createHash } from "node:crypto";
import Parser from "web-tree-sitter";
const root = "/Users/ueli/Documents/semio";
const U = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units";
await Parser.init(); const p = new Parser();
p.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", "/Users/ueli/Documents/semio/✏️s")), "out/tree-sitter-rust.wasm")));
const file = U + "/📋️child-group-owner.json";
const d = JSON.parse(readFileSync(file, "utf8"));
let changed = 0;
for (const o of d.remaining) {
  const t = p.parse(readFileSync(resolve(root, o.path), "utf8"));
  if (t.rootNode.hasError()) throw Error("parse " + o.path);
  const act = t.rootNode.descendantsOfType("function_item").map((n: any) => ({ name: n.childForFieldName("name")?.text, sha256: createHash("sha256").update(n.text).digest("hex") }));
  if (JSON.stringify(act) !== JSON.stringify(o.functions)) { if (act.length !== o.functions.length || act.some((a: any, i: number) => a.name !== o.functions[i].name)) throw Error("names changed " + o.path); o.functions = act; changed++; }
}
console.log("changed owners", changed);
if (process.argv[2] === "write") writeFileSync(file, JSON.stringify(d, null, 2) + "\n");
