import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, normalize, relative } from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const dir = process.argv[2]!;
const apply = process.argv[3] === "apply";
const plans = new Map<string, any>(), skipped: any[] = [];
for (const name of ["edges0.plan-applied.json", "e1.plan-applied.json"]) {
  const d = JSON.parse(readFileSync(join(dir, name), "utf8"));
  for (const p of d.plans) if (!p.reuse) plans.set(p.target, p);
  skipped.push(...d.skipped.filter((s: any) => /^key exists with other target/u.test(s.reason)));
}
const seen = new Set<string>();
for (const s of skipped) {
  const key = s.from + "\0" + s.to; if (seen.has(key)) continue; seen.add(key);
  const plan = plans.get(s.to); if (!plan) { console.log("NO PLAN", s.to); continue; }
  const glueAbs = join(root, plan.glue), specRaw = relative(dirname(glueAbs), join(root, s.to)).split("\\").join("/"), spec = specRaw.startsWith(".") ? specRaw : "./" + specRaw;
  const body = `/** 🧪️ Public \`${plan.pkg}${plan.sub.slice(1)}\` entry: loads the module for its side effect, nothing is exported. */\nimport "${spec}";\n`;
  const have = existsSync(glueAbs) ? readFileSync(glueAbs, "utf8") : "";
  if (apply && (have === "" || !/\bimport\b|\bexport\b/u.test(have.replace(/\/\*\*[\s\S]*?\*\//u, "")))) writeFileSync(glueAbs, body);
  const text = readFileSync(join(root, s.from), "utf8"), sf = ts.createSourceFile(s.from, text, ts.ScriptTarget.Latest, true);
  let out = text, count = 0;
  const edits: { start: number; end: number; text: string }[] = [];
  const visit = (n: ts.Node) => {
    let lit: ts.StringLiteralLike | undefined;
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) lit = n.moduleSpecifier;
    else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require"))) lit = n.arguments[0] as ts.StringLiteralLike;
    if (lit && lit.text.startsWith(".") && normalize(join(dirname(s.from), lit.text)) === s.to) { let r = relative(dirname(join(root, s.from)), glueAbs).split("\\").join("/"); if (!r.startsWith(".")) r = "./" + r; edits.push({ start: lit.getStart(sf) + 1, end: lit.getEnd() - 1, text: r }); }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  for (const e of edits.sort((a, b) => b.start - a.start)) { out = out.slice(0, e.start) + e.text + out.slice(e.end); count++; }
  console.log(apply ? "APPLY" : "DRY", count, s.from.slice(-60), "->", plan.glue.slice(-50));
  if (apply && count) writeFileSync(join(root, s.from), out);
}
