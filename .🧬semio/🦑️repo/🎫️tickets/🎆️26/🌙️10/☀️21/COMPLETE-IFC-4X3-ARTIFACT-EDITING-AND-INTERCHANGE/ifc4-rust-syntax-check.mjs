/** 🦀️ Parse owned native geometry sources with the existing independent Rust grammar. */
import fs from "node:fs/promises";
import path from "node:path";
import {createRequire} from "node:module";
let parserConstructor;
export async function checkRustSyntax(root, files, initializedParser) {
  const require = createRequire(path.join(root, "package.json"));
  let Parser = initializedParser ?? parserConstructor ?? require("web-tree-sitter");
  if (typeof Parser.init !== "function") {
    delete require.cache[require.resolve("web-tree-sitter")];
    Parser = require("web-tree-sitter");
  }
  parserConstructor = Parser;
  await Parser.init();
  let wasm;
  try { wasm = require.resolve("tree-sitter-wasms/out/tree-sitter-rust.wasm"); } catch {
    const cache = path.join(root, "node_modules/.bun"), packages = await fs.readdir(cache), name = packages.find(name => name.startsWith("tree-sitter-wasms@"));
    if (!name) throw new Error("Existing Rust parser grammar is unavailable");
    wasm = path.join(cache, name, "node_modules/tree-sitter-wasms/out/tree-sitter-rust.wasm");
  }
  const parser = new Parser(); parser.setLanguage(await Parser.Language.load(wasm));
  const results = [];
  for (const file of files) {
    const tree = parser.parse(await fs.readFile(file, "utf8")), failures = [];
    const visit = node => {if (node.type === "ERROR" || node.isMissing()) failures.push({type: node.type, start: node.startPosition, end: node.endPosition, text: node.text.slice(0, 160)}); for (const child of node.children) visit(child);};
    visit(tree.rootNode); results.push({file, failures}); tree.delete();
  }
  parser.delete();
  return results;
}
