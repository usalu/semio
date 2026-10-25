import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync } from "node:fs";
const files = process.argv.slice(2).filter(f => /\.(ts|tsx|mjs|js)$/.test(f));
const sources = files.map(f => [f, readFileSync(f, "utf8")]);
const walk = node => { let n = 1; ts.forEachChild(node, child => { n += walk(child); }); return n; };
const time = fn => { const t = performance.now(); fn(); return +(performance.now() - t).toFixed(0); };
for (let round = 0; round < 2; round++) {
  const three = time(() => { for (const [f, s] of sources) for (let k = 0; k < 3; k++) walk(ts.createSourceFile(f, s, ts.ScriptTarget.Latest, false)); });
  const once = time(() => { for (const [f, s] of sources) { const tree = ts.createSourceFile(f, s, ts.ScriptTarget.Latest, false); for (let k = 0; k < 3; k++) walk(tree); } });
  console.log(JSON.stringify({ files: sources.length, bytes: sources.reduce((a, [, s]) => a + s.length, 0), threeParsesMs: three, oneParseMs: once }));
}
