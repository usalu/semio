// 🧮 c5c8 function-row dump with the plugin units law grammar (web-tree-sitter 0.20.8 + tree-sitter-wasms 0.1.13): every function_item of each plain file as {name, sha256, assertions, text}, the file regex assertion count, hasError.
// usage: bun fnrows-text.ts <out.json> <file>...   (run from a directory whose node_modules holds web-tree-sitter and tree-sitter-wasms)
import {readFileSync, writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import Parser from "web-tree-sitter";
const [out, ...files] = process.argv.slice(2);
const base = process.env.C5C8_NM ?? "/Users/ueli/.cache/c5c8/nm";
await Parser.init({wasmBinary: readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm", base))});
const p = new Parser();
p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", base)));
const result: Record<string, unknown> = {};
for (const file of files) {
  const source = readFileSync(file, "utf8");
  const tree = p.parse(source);
  try {
    const fns = tree.rootNode.descendantsOfType("function_item").map(fn => ({name: fn.childForFieldName("name")?.text ?? "", sha256: createHash("sha256").update(fn.text).digest("hex"), assertions: (fn.text.match(/\bassert(?:_eq|_ne)?!/g) ?? []).length, text: fn.text}));
    result[file] = {hasError: tree.rootNode.hasError(), assertions: (source.match(/\bassert(?:_eq|_ne)?!/g) ?? []).length, bytes: Buffer.byteLength(source), sha256: createHash("sha256").update(source).digest("hex"), fns};
  } finally { tree.delete(); }
}
writeFileSync(out, JSON.stringify(result));
console.log("rows", Object.entries(result).map(([f, v]) => `${f.split("/").slice(-2).join("/")}:${(v as {fns: unknown[]}).fns.length}`).join(" "));
