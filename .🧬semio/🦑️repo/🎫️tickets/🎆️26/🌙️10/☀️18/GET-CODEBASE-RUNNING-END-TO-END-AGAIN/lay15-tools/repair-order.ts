import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const [planPath, ...files] = process.argv.slice(2) as string[];
const ROOT = "/Users/ueli/Documents/semio";
const plan = JSON.parse(readFileSync(planPath!, "utf8"));
const apply = process.env.APPLY === "1";
for (const f of files) {
  const edits = (plan.edits as any[]).filter(e => e.file === f).sort((a, b) => a.start - b.start);
  const dupAfter = new Set<string>();
  const seen = new Map<string, number>();
  for (const e of edits) seen.set(e.after, (seen.get(e.after) ?? 0) + 1);
  for (const [a, n] of seen) if (n > 1) dupAfter.add(a);
  const group = edits.filter(e => dupAfter.has(e.after));
  const p = resolve(ROOT, f);
  const text = readFileSync(p, "utf8");
  const sf = ts.createSourceFile(p, text, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const wanted = new Set(group.map(e => e.before));
  const nodes: ts.StringLiteralLike[] = [];
  const visit = (n: ts.Node): void => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier) && wanted.has(n.moduleSpecifier.text)) nodes.push(n.moduleSpecifier);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  console.log(f.slice(-60), "group", group.length, "nodes", nodes.length);
  if (nodes.length !== group.length) { console.log("  MISMATCH, skipping"); continue; }
  let out = text;
  const reps = nodes.map((n, i) => ({ s: n.getStart(sf) + 1, e: n.getEnd() - 1, v: group[i]!.before, cur: n.text })).sort((a, b) => b.s - a.s);
  for (const r of reps) { console.log("  ", r.cur.slice(-40), "=>", r.v.slice(-40)); out = out.slice(0, r.s) + r.v + out.slice(r.e); }
  if (apply) writeFileSync(p, out);
}
