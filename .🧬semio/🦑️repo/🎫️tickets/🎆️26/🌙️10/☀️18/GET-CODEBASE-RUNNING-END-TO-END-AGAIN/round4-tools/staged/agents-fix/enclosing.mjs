import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import { scan } from "./scan-comments.mjs";
const R = "/Users/ueli/Documents/semio/";
export function enclosing(path) {
  const text = fs.readFileSync(R + path, "utf8");
  const sf = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, path.endsWith(".mjs") ? ts.ScriptKind.JS : ts.ScriptKind.TS);
  const hits = scan(text, path).filter((h) => !h.directive);
  const out = [];
  for (const h of hits) {
    const chain = [];
    const visit = (n) => { if (n.getStart(sf) <= h.pos && h.end <= n.end) { if (ts.isFunctionLike(n) || ts.isClassLike(n) || ts.isVariableStatement(n) || ts.isMethodDeclaration(n)) chain.push(n); ts.forEachChild(n, visit); } };
    visit(sf);
    const names = chain.map((n) => { const d = n.name?.getText(sf) ?? (ts.isVariableStatement(n) ? n.declarationList.declarations[0].name.getText(sf) : "<anon>"); const doc = (n.jsDoc ?? []).map((j) => j.getText(sf).slice(0, 60)).join("|"); return `${d}@${sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1}[${doc}]`; });
    out.push({ line: h.line, chain: names });
  }
  return out;
}
const f = process.argv[2];
for (const e of enclosing(f)) console.log(e.line, e.chain.join(" > "));
