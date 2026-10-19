import { readFileSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
import { createHash } from "node:crypto";
import Parser from "web-tree-sitter";
const root = "/Users/ueli/Documents/semio";
const U = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units";
await Parser.init(); const p = new Parser();
p.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", "/Users/ueli/Documents/semio/✏️s")), "out/tree-sitter-rust.wasm")));
const d = JSON.parse(readFileSync(U + "/📋️child-group-owner.json", "utf8"));
for (const o of d.remaining) {
  const t = p.parse(readFileSync(resolve(root, o.path), "utf8"));
  const act = t.rootNode.descendantsOfType("function_item").map((n: any) => ({ name: n.childForFieldName("name")?.text, sha256: createHash("sha256").update(n.text).digest("hex") }));
  const diff = JSON.stringify(act) !== JSON.stringify(o.functions);
  console.log(diff ? "DIFF" : "same", o.path.slice(-70), act.length, o.functions.length, t.rootNode.hasError());
  if (diff) for (let i = 0; i < Math.max(act.length, o.functions.length); i++) if (JSON.stringify(act[i]) !== JSON.stringify(o.functions[i])) console.log("  ", i, act[i]?.name, o.functions[i]?.name);
}
