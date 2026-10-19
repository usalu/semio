import Parser from "web-tree-sitter";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
await Parser.init({ locateFile: () => resolve(dirname(Bun.resolveSync("web-tree-sitter", process.cwd())), "tree-sitter.wasm") });
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm", process.cwd())));
for (const file of process.argv.slice(2)) {
  const text = readFileSync(file, "utf8"), tree = parser.parse(text)!;
  console.log(file.slice(-90), "hasError", tree.rootNode.hasError());
  const walk = (node: Parser.SyntaxNode): void => {
    if (node.type === "ERROR" || node.isMissing) console.log("  ", node.type, node.isMissing ? "MISSING" : "", "line", node.startPosition.row + 1, JSON.stringify(text.split("\n")[node.startPosition.row]!.slice(0, 160)));
    for (const child of node.children) if (child.hasError || child.isMissing) walk(child);
  };
  walk(tree.rootNode);
  tree.delete();
}
