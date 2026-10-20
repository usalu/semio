import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
const R = "/Users/ueli/Documents/semio/";
const DIRECTIVE = /@ts-(expect-error|ignore|nocheck|check)|eslint-|biome-ignore|istanbul|prettier-ignore|#region|#endregion|^\/\/\/\s*<|@__PURE__|c8 ignore|v8 ignore/;
const DEF = (n) => ts.isFunctionLike(n) || ts.isClassLike(n) || ts.isInterfaceDeclaration(n) || ts.isEnumDeclaration(n) || ts.isTypeAliasDeclaration(n) || ts.isVariableStatement(n) || ts.isModuleDeclaration(n);
export function scan(text, name) {
  const sf = ts.createSourceFile(name, text, ts.ScriptTarget.Latest, true, name.endsWith(".mjs") ? ts.ScriptKind.JS : name.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const seen = new Map();
  const add = (r) => { if (r) for (const c of r) seen.set(c.pos, c); };
  const visit = (n) => { add(ts.getLeadingCommentRanges(text, n.pos)); add(ts.getTrailingCommentRanges(text, n.end)); for (const c of n.getChildren(sf)) visit(c); };
  visit(sf);
  add(ts.getLeadingCommentRanges(text, sf.endOfFileToken.pos));
  const defs = [];
  const collect = (n) => { if (DEF(n)) defs.push(n); ts.forEachChild(n, collect); };
  collect(sf);
  const attached = new Set();
  const att = (n) => { for (const d of n.jsDoc ?? []) attached.add(d.pos); ts.forEachChild(n, att); };
  att(sf);
  const out = [];
  for (const c of [...seen.values()].sort((a, b) => a.pos - b.pos)) {
    const body = text.slice(c.pos, c.end);
    if (body.startsWith("/**")) continue;
    if (c.pos === 0 && body.startsWith("#!")) continue;
    const container = defs.filter((d) => d.getStart(sf) < c.pos && c.end <= d.end).sort((a, b) => (b.getStart(sf) - a.getStart(sf)))[0];
    if (!container) continue;
    const isDoc = body.startsWith("/**") && attached.has(c.pos);
    out.push({ pos: c.pos, end: c.end, kind: c.kind, body, isDoc, directive: DIRECTIVE.test(body), container: container.getStart(sf), line: sf.getLineAndCharacterOfPosition(c.pos).line + 1 });
  }
  return out;
}
