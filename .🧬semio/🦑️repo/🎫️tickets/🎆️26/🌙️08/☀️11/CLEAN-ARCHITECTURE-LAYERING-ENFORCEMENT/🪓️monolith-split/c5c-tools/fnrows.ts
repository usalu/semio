// 🧮 c5c function-row dump: parses Rust files (plain text, or the module tree with --tree) with the same tree-sitter grammar as the plugin units law and writes every function_item as {name, sha256, assertions, test, bytes} (+ file hasError and ERROR node texts).
// usage: bun fnrows.ts <out.json> [--tree] [--text] <file>...
import {readFileSync, writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import Parser from "web-tree-sitter";
import {readModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const args = process.argv.slice(2);
const out = args.shift()!;
let tree = false, withText = false;
while (args[0]?.startsWith("--")) { const flag = args.shift(); if (flag === "--tree") tree = true; if (flag === "--text") withText = true; }
await Parser.init({wasmBinary: readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm", root))});
const p = new Parser();
p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", root)));
const result: Record<string, unknown> = {};
for (const file of args) {
  const text = tree ? readModuleTreeSource(file) : readFileSync(file, "utf8");
  const t = p.parse(text);
  try {
    const errors: string[] = [];
    const inspect = (node: Parser.SyntaxNode) => { if (node.type === "ERROR") errors.push(node.text); for (const child of node.children) inspect(child); };
    inspect(t.rootNode);
    const fns = t.rootNode.descendantsOfType("function_item").map(fn => {
      let test = false;
      for (let attr = fn.previousNamedSibling; attr?.type === "attribute_item"; attr = attr.previousNamedSibling) if (/\btest\b|::async_test\b/.test(attr.text)) test = true;
      return {name: fn.childForFieldName("name")?.text, sha256: createHash("sha256").update(fn.text).digest("hex"), assertions: (fn.text.match(/\bassert(?:_eq|_ne)?!/g) ?? []).length, test, bytes: Buffer.byteLength(fn.text), ...(withText ? {text: fn.text} : {})};
    });
    result[file] = {hasError: t.rootNode.hasError(), errors, assertions: (text.match(/\bassert(?:_eq|_ne)?!/g) ?? []).length, fns};
  } finally { t.delete(); }
}
writeFileSync(out, JSON.stringify(result));
console.log("wrote", out, Object.keys(result).length, "files");
