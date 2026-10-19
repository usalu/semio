import Parser from "web-tree-sitter";
import { dirname, join } from "node:path";
import { readFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio";
await Parser.init(); const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", root)), "out/tree-sitter-rust.wasm")));
for (const file of process.argv.slice(2)) {
  const source = readFileSync(file, "utf8"); const tree = parser.parse(source); const errors: string[] = [];
  const walk = (n: Parser.SyntaxNode) => { if (n.type === "ERROR" || n.isMissing()) errors.push(`${n.startPosition.row + 1}:${n.startPosition.column} ${n.type} ${JSON.stringify(source.slice(Math.max(0, n.startIndex - 40), n.startIndex + 60))}`); for (const c of n.children) walk(c); };
  walk(tree.rootNode); console.log(file.slice(-60), tree.rootNode.hasError(), errors.length); for (const e of errors.slice(0, 5)) console.log("  ", e);
}
