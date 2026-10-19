import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
const [planPath, ...revPaths] = process.argv.slice(2) as string[];
const R = "/Users/ueli/Documents/semio/";
const plan = JSON.parse(readFileSync(planPath!, "utf8"));
const rev = new Set<string>();
for (const p of revPaths) for (const r of JSON.parse(readFileSync(p, "utf8")).reverted) rev.add(r.file + "\0" + r.after);
const edits = (plan.edits as any[]).filter(e => e.to.endsWith(".json") && !rev.has(e.file + "\0" + e.after));
let n = 0;
for (const e of edits) {
  const p = R + e.file;
  let t = readFileSync(p, "utf8");
  const sf = ts.createSourceFile(p, t, ts.ScriptTarget.Latest, true, e.file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const cuts: [number, number][] = [];
  const v = (node: ts.Node): void => {
    if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier) && node.moduleSpecifier.text === e.after && node.attributes) cuts.push([node.moduleSpecifier.getEnd(), node.attributes.getEnd()]);
    ts.forEachChild(node, v);
  };
  v(sf);
  for (const [s, en] of cuts.sort((a, b) => b[0] - a[0])) { t = t.slice(0, s) + t.slice(en); n++; }
  if (cuts.length) writeFileSync(p, t);
}
console.log("attributes removed", n);
