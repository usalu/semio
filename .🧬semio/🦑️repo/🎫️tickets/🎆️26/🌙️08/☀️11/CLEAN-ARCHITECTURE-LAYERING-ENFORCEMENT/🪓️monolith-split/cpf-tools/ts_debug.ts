import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, writeFileSync } from "node:fs";
import { parse, kindOf } from "./ts_lib.ts";
const R = "/Users/ueli/Documents/semio/", G = process.env.G!;
const files: string[] = JSON.parse(readFileSync(process.argv[2], "utf8"));
const TAG = "[DEBUG] ";
type Occ = { path: string; start: number; end: number; tag: string; role: "emit" | "other"; stmt?: [number, number]; ctx?: string; line: number };
const out: Occ[] = [];
const EMIT = /^(console\.(log|error|warn|info|debug)|process\.(stdout|stderr)\.write|writeSync|(?:\w+\.)?writeSync)$/;
function litText(n: ts.Node): string | undefined {
  if (ts.isStringLiteral(n) || ts.isNoSubstitutionTemplateLiteral(n)) return n.text;
  if (ts.isTemplateHead(n)) return n.text;
  return undefined;
}
for (const p of files) {
  const text = readFileSync(R + p, "utf8");
  if (!text.includes(TAG)) continue;
  const sf = parse(p, text);
  const lineOf = (pos: number) => sf.getLineAndCharacterOfPosition(pos).line + 1;
  const visit = (n: ts.Node) => {
    const lt = litText(n);
    if (lt !== undefined && lt.includes(TAG)) {
      const tag = lt.slice(lt.indexOf(TAG) + TAG.length).replace(/\s*[{"`'].*$/s, "").trim().slice(0, 60);
      // climb: left-most operand of + chain, then argument of a call
      let cur: ts.Node = n;
      while (cur.parent && ts.isBinaryExpression(cur.parent) && cur.parent.operatorToken.kind === ts.SyntaxKind.PlusToken && cur.parent.left === cur) cur = cur.parent;
      if (cur.parent && ts.isTemplateExpression(cur.parent)) cur = cur.parent;
      let role: "emit" | "other" = "other"; let stmt: [number, number] | undefined; let ctx: string | undefined;
      const call = cur.parent;
      if (call && ts.isCallExpression(call) && call.arguments[0] === cur || (call && ts.isCallExpression(call) && call.arguments[1] === cur && /writeSync$/.test(call.expression.getText(sf)))) {
        if (EMIT.test(call.expression.getText(sf))) {
          role = "emit";
          const par = call.parent;
          if (par && ts.isExpressionStatement(par)) { stmt = [par.getStart(sf, false), par.end]; ctx = ts.SyntaxKind[par.parent.kind]; }
          else ctx = "nonstmt:" + (par ? ts.SyntaxKind[par.kind] : "?");
        }
      }
      out.push({ path: p, start: n.getStart(sf, false), end: n.end, tag, role, stmt, ctx, line: lineOf(n.getStart(sf, false)) });
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
writeFileSync(G + "/cpf-ts-debug-occ.json", JSON.stringify(out));
console.log("occurrences", out.length, "emit", out.filter(o => o.role === "emit").length);
