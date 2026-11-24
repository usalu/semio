import { readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, resolve } from "node:path";
import Parser from "web-tree-sitter";

/** 🧾️ Recomputes `remainingFunctions` of `📋️feature-owner.json` exactly as the units test derives it (function items of the facade and its mounted law files).
 * @see ../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/🟦️.ts */
const root = process.cwd();
const plugin = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
const registry = resolve(plugin, "🧪️tests/📦️units/📋️feature-owner.json");
const read = (path: string) => readFileSync(path, "utf8");
await Parser.init({ wasmBinary: readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm", root)) });
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", root)));
const mounted = (start: string) => {
  const files: string[] = [];
  const walk = (path: string) => {
    if (files.includes(path)) return;
    files.push(path);
    for (const match of read(path).matchAll(/#\[path\s*=\s*"([^"\n]+)"\]|include!\(\s*"([^"\n]+)"\s*\)/g)) walk(resolve(dirname(path), (match[1] ?? match[2])!));
  };
  walk(start);
  return files;
};
const lawFiles = [resolve(plugin, "🧪️testing/🦀️.rs"), ...mounted(resolve(plugin, "🧪️testing/🧪️artifact-app-laws/🦀️.rs"))];
const remaining: { name: string | undefined; sha256: string }[] = [];
for (const file of lawFiles) {
  const tree = parser.parse(read(file).replaceAll("= () where 's: 't;", "= ();"));
  remaining.push(...tree.rootNode.descendantsOfType("function_item").map(node => ({ name: node.childForFieldName("name")?.text, sha256: createHash("sha256").update(node.text).digest("hex") })));
  tree.delete();
}
const value = JSON.parse(read(registry));
const before = new Map<string, string>(value.remainingFunctions.map((row: { name: string; sha256: string }) => [row.name + row.sha256, row.name]));
value.remainingFunctions = remaining;
if (process.argv[2] === "write") writeFileSync(registry, `${JSON.stringify(value, null, 2)}\n`);
console.log(JSON.stringify({ count: remaining.length, changedOrNew: remaining.filter(row => !before.has(row.name + row.sha256)).map(row => row.name) }));
