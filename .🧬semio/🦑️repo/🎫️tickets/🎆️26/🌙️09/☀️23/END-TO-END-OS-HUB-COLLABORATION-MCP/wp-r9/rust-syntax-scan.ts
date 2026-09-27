#!/usr/bin/env bun
/** 🩺️ R9: tree-sitter-rust syntax scan of a list of Rust files (one repo-relative path per line); prints every ERROR/MISSING node. */
import { readFileSync } from "node:fs";
const REPO = "/Users/ueli/Documents/semio";
const Parser = (await import(`${REPO}/node_modules/web-tree-sitter/tree-sitter.js`)).default;
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(`${REPO}/node_modules/tree-sitter-wasms/out/tree-sitter-rust.wasm`));
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
let bad = 0;
for (const file of files) {
  let source: string;
  try { source = readFileSync(`${REPO}/${file}`, "utf8"); } catch { continue; }
  const tree = parser.parse(source);
  const errors: string[] = [];
  const walk = (node: any): void => {
    if (node.type === "ERROR" || node.isMissing()) errors.push(`${node.startPosition.row + 1}:${node.startPosition.column + 1} ${node.isMissing() ? "MISSING " + node.type : "ERROR"} ${JSON.stringify(node.text.slice(0, 80))}`);
    else if (node.hasError()) for (const child of node.children) walk(child);
  };
  walk(tree.rootNode);
  tree.delete();
  if (errors.length) { bad++; console.log(file); for (const e of errors.slice(0, 5)) console.log("  " + e); }
}
console.log(`files=${files.length} with-syntax-errors=${bad}`);
