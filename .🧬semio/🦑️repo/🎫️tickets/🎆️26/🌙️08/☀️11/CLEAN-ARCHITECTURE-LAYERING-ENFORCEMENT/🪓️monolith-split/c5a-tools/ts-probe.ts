// 🧪 c5a probe: web-tree-sitter parses a plugin test file under bun.
import {readFileSync} from "node:fs";
import Parser from "web-tree-sitter";
const root = "/Users/ueli/Documents/semio";
await Parser.init({wasmBinary: readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm", root))});
const p = new Parser();
p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", root)));
const tree = p.parse(readFileSync(process.argv[2]!, "utf8"));
console.log("hasError", tree.rootNode.hasError(), "functions", tree.rootNode.descendantsOfType("function_item").length);
