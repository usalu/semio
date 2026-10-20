import Parser from "/Users/ueli/Documents/semio/node_modules/web-tree-sitter/tree-sitter.js";
import { findWasm } from "./parse.ts";
await Parser.init();
const p = new Parser(); p.setLanguage(await Parser.Language.load(findWasm("/Users/ueli/Documents/semio")!));
for (const f of process.argv.slice(2)) {
  const src = await Bun.file(f).text();
  const t = p.parse(src);
  const walk = (n: any, depth: number) => { if (n.type === "ERROR" || n.isMissing()) { console.log(f.slice(-50), n.startPosition.row + 1, n.type, JSON.stringify(src.slice(n.startIndex, Math.min(n.endIndex, n.startIndex + 90)))); return; } for (const c of n.children) walk(c, depth + 1); };
  walk(t.rootNode, 0);
}
