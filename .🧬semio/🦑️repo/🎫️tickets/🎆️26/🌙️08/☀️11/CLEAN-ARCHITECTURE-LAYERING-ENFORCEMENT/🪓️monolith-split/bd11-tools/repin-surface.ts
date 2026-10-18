import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { createHash } from "node:crypto";
import { resolve, dirname, basename } from "node:path";
import Parser from "web-tree-sitter";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const root = "/Users/ueli/Documents/semio", plugin = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const file = plugin + "/🧪️tests/📦️units/📋️surface-carriers.json";
const owner = JSON.parse(readFileSync(file, "utf8"));
await Parser.init({ wasmBinary: readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm", root)) });
const p = new Parser(); p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", root)));
const family = (path: string) => { const dir = dirname(resolve(root, path)), parent = dirname(dir), base = basename(dir); return readdirSync(parent).filter(n => n === base || (n.startsWith(base + "-part-") && !n.endsWith("-x"))).sort().map(n => readModuleTreeSource(resolve(parent, n, "🦀️.rs"))); };
const fresh = family(owner.original).flatMap(text => { const t = p.parse(text); return t.rootNode.descendantsOfType("function_item").map(fn => ({ name: fn.childForFieldName("name")?.text as string, sha256: createHash("sha256").update(fn.text).digest("hex") })); });
const byName = new Map<string, string[]>(); for (const f of fresh) byName.set(f.name, [...(byName.get(f.name) ?? []), f.sha256]);
let changed = 0;
for (const name of new Set(owner.remaining.map((r: any) => r.name))) {
  const olds = owner.remaining.filter((r: any) => r.name === name), news = byName.get(name as string) ?? [];
  if (olds.length !== news.length) { console.log("COUNT", name, olds.length, news.length); continue; }
  const unmatched = olds.filter((r: any) => !news.includes(r.sha256)), unused = news.filter(s => !olds.some((r: any) => r.sha256 === s));
  if (unmatched.length !== unused.length) { console.log("AMBIG", name); continue; }
  unmatched.forEach((r: any, i: number) => { r.sha256 = unused[i]; changed++; });
}
writeFileSync(file, JSON.stringify(owner, null, 2) + "\n");
console.log("changed", changed, "fresh", fresh.length, "pinned", owner.remaining.length);
