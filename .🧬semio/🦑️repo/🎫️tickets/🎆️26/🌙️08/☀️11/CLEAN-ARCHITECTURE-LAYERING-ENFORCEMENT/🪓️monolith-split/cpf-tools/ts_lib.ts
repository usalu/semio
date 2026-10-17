import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
export type C = { start: number; end: number; block: boolean; doc: boolean; fn: boolean; de: boolean; jsx: boolean; own: boolean; line: number; text: string };
export function kindOf(path: string) { return path.endsWith(".tsx") ? ts.ScriptKind.TSX : /\.(mjs|js)$/.test(path) ? ts.ScriptKind.JS : ts.ScriptKind.TS; }
export function parse(path: string, text: string) { return ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, kindOf(path)); }
export function comments(path: string, text: string): C[] {
  const sf = parse(path, text);
  const fnR: [number, number][] = [], deR: [number, number][] = [], jsxR: [number, number][] = [];
  const seen = new Map<number, C>();
  const lineStarts = sf.getLineStarts();
  const lineOf = (pos: number) => sf.getLineAndCharacterOfPosition(pos).line;
  const add = (r: ts.CommentRange) => {
    if (seen.has(r.pos)) return;
    const t = text.slice(r.pos, r.end);
    const block = r.kind === ts.SyntaxKind.MultiLineCommentTrivia;
    const doc = (block && t.startsWith("/**") && !t.startsWith("/**/") && !t.startsWith("/***")) || (!block && t.startsWith("///"));
    const line = lineOf(r.pos);
    const own = text.slice(lineStarts[line], r.pos).trim() === "";
    seen.set(r.pos, { start: r.pos, end: r.end, block, doc, fn: false, de: false, jsx: false, own, line, text: t });
  };
  const visit = (n: ts.Node) => {
    const lead = ts.getLeadingCommentRanges(text, n.pos); lead?.forEach(add);
    const trail = ts.getTrailingCommentRanges(text, n.end); trail?.forEach(add);
    if ((ts.isFunctionDeclaration(n) || ts.isFunctionExpression(n) || ts.isArrowFunction(n) || ts.isMethodDeclaration(n) || ts.isConstructorDeclaration(n) || ts.isGetAccessorDeclaration(n) || ts.isSetAccessorDeclaration(n)) && n.body) fnR.push([n.body.getStart(sf, false), n.body.end]);
    if (ts.isClassStaticBlockDeclaration(n)) fnR.push([n.getStart(sf, false), n.end]);
    if (ts.isClassDeclaration(n) || ts.isClassExpression(n) || ts.isInterfaceDeclaration(n) || ts.isEnumDeclaration(n)) {
      const open = text.indexOf("{", n.members.pos - 1 < 0 ? 0 : Math.max(n.getStart(sf, false), 0));
      deR.push([n.members.pos, n.end]);
    }
    if (ts.isTypeLiteralNode(n)) deR.push([n.members.pos, n.end]);
    if (ts.isJsxExpression(n) || ts.isJsxText(n)) jsxR.push([n.pos, n.end]);
    ts.forEachChild(n, visit);
  };
  visit(sf);
  const eof = sf.endOfFileToken;
  ts.getLeadingCommentRanges(text, eof.pos)?.forEach(add);
  const inR = (rs: [number, number][], p: number) => rs.some(([a, b]) => p > a && p < b);
  const out = [...seen.values()].sort((a, b) => a.start - b.start);
  for (const c of out) { c.fn = inR(fnR, c.start); c.de = inR(deR, c.start); c.jsx = inR(jsxR, c.start); }
  return out;
}
export function printNoComments(path: string, text: string): string {
  const sf = parse(path, text);
  const pr = ts.createPrinter({ removeComments: true });
  return pr.printFile(sf);
}
export function diagnostics(path: string, text: string): number {
  const sf = parse(path, text);
  return ((sf as any).parseDiagnostics as ts.Diagnostic[]).length;
}
