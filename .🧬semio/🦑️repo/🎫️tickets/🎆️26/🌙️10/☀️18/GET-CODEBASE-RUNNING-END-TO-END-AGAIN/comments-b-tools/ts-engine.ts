import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { keepText, type Removal } from "./keep.ts";

export type Plan = { removals: Removal[]; kept: number; error?: string };

const KIND: Record<string, ts.ScriptKind> = {
  ".ts": ts.ScriptKind.TS,
  ".mts": ts.ScriptKind.TS,
  ".cts": ts.ScriptKind.TS,
  ".tsx": ts.ScriptKind.TSX,
  ".js": ts.ScriptKind.JS,
  ".mjs": ts.ScriptKind.JS,
  ".cjs": ts.ScriptKind.JS,
  ".jsx": ts.ScriptKind.JSX,
};

export function tsKind(path: string): ts.ScriptKind | undefined {
  const m = /\.[a-z]+$/.exec(path);
  return m ? KIND[m[0]] : undefined;
}

function parse(path: string, text: string): ts.SourceFile | undefined {
  const k = tsKind(path);
  if (k === undefined) return undefined;
  const sf = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, k);
  const diags = (sf as unknown as { parseDiagnostics: unknown[] }).parseDiagnostics;
  return diags.length ? undefined : sf;
}

function isJsDoc(n: ts.Node): boolean {
  return n.kind >= ts.SyntaxKind.FirstJSDocNode && n.kind <= ts.SyntaxKind.LastJSDocNode;
}

type Found = { pos: number; end: number; jsx?: ts.Node };

function collect(sf: ts.SourceFile, text: string): { found: Found[]; jsxText: [number, number][] } {
  const found = new Map<number, Found>();
  const jsxText: [number, number][] = [];
  const add = (ranges: ts.CommentRange[] | undefined, jsx?: ts.Node) => {
    if (!ranges) return;
    for (const r of ranges) if (!found.has(r.pos)) found.set(r.pos, { pos: r.pos, end: r.end, jsx });
  };
  const visit = (n: ts.Node, jsx?: ts.Node) => {
    if (isJsDoc(n)) return;
    if (n.kind === ts.SyntaxKind.JsxText) {
      jsxText.push([n.pos, n.end]);
      return;
    }
    const ctx = n.kind === ts.SyntaxKind.JsxExpression && !(n as ts.JsxExpression).expression ? n : jsx;
    const kids = n.getChildren(sf);
    if (kids.length === 0) {
      add(ts.getLeadingCommentRanges(text, n.pos), ctx);
      add(ts.getTrailingCommentRanges(text, n.end), ctx);
      return;
    }
    for (const k of kids) visit(k, ctx);
  };
  visit(sf);
  return { found: [...found.values()], jsxText };
}

function topStatements(sf: ts.SourceFile): ts.Statement[] {
  const out: ts.Statement[] = [];
  const walk = (stmts: readonly ts.Statement[]) => {
    for (const s of stmts) {
      if (ts.isModuleDeclaration(s) && s.body && ts.isModuleBlock(s.body)) walk(s.body.statements);
      else out.push(s);
    }
  };
  walk(sf.statements);
  return out;
}

/** 🔎 Plans the removal of every non-docstring comment strictly inside a top-level definition; kept comments are counted. */
export function planTs(path: string, text: string): Plan {
  const sf = parse(path, text);
  if (!sf) return { removals: [], kept: 0, error: "parse" };
  const { found, jsxText } = collect(sf, text);
  const stmts = topStatements(sf)
    .filter((s) => !ts.isImportDeclaration(s) && !ts.isExportDeclaration(s) && !ts.isImportEqualsDeclaration(s))
    .map((s) => [s.getStart(sf), s.end] as const);
  stmts.sort((a, b) => a[0] - b[0]);
  const inside = (pos: number, end: number): boolean => {
    let lo = 0;
    let hi = stmts.length - 1;
    while (lo <= hi) {
      const mid = (lo + hi) >> 1;
      const [s, e] = stmts[mid];
      if (pos >= e) lo = mid + 1;
      else if (pos < s) hi = mid - 1;
      else return pos > s && end <= e;
    }
    return false;
  };
  let kept = 0;
  const removable: Found[] = [];
  const blocked = new Set<ts.Node>();
  for (const f of found) {
    if (jsxText.some(([s, e]) => f.pos >= s && f.pos < e)) continue;
    const raw = text.slice(f.pos, f.end);
    if (!inside(f.pos, f.end)) continue;
    if (keepText(raw)) {
      kept++;
      if (f.jsx) blocked.add(f.jsx);
      continue;
    }
    removable.push(f);
  }
  const removals = new Map<number, Removal>();
  for (const f of removable) {
    if (f.jsx) {
      if (blocked.has(f.jsx)) continue;
      const s = f.jsx.getStart(sf);
      removals.set(s, { pos: s, end: f.jsx.end, exact: true });
    } else removals.set(f.pos, { pos: f.pos, end: lineEnd(text, f) });
  }
  return { removals: [...removals.values()], kept };
}

function lineEnd(text: string, f: Found): number {
  let e = f.end;
  while (e > f.pos && (text[e - 1] === "\r" || text[e - 1] === "\n")) e--;
  return e;
}

/** 🧾 Comment-free token signature used to prove that only comments changed. */
export function tsSignature(path: string, text: string): string[] | undefined {
  const sf = parse(path, text);
  if (!sf) return undefined;
  const out: string[] = [];
  let jsx = "";
  const flush = () => {
    const c = jsxClean(jsx);
    if (c !== "") out.push(`J:${c}`);
    jsx = "";
  };
  const visit = (n: ts.Node) => {
    if (isJsDoc(n)) return;
    if (n.kind === ts.SyntaxKind.JsxText) {
      jsx += text.slice(n.pos, n.end);
      return;
    }
    if (n.kind === ts.SyntaxKind.JsxExpression && !(n as ts.JsxExpression).expression) return;
    const kids = n.getChildren(sf);
    if (kids.length > 0) out.push(`N${n.kind}`);
    if (kids.length === 0) {
      flush();
      if (n.kind !== ts.SyntaxKind.EndOfFileToken) out.push(`${n.kind}:${n.getText(sf)}`);
      return;
    }
    for (const k of kids) visit(k);
  };
  visit(sf);
  flush();
  return out;
}

function jsxClean(t: string): string {
  if (!/[\r\n]/.test(t)) return t;
  const lines = t.split(/\r\n|\n|\r/);
  const out: string[] = [];
  lines.forEach((l, i) => {
    let x = l.replace(/\t/g, " ");
    if (i > 0) x = x.replace(/^ +/, "");
    if (i < lines.length - 1) x = x.replace(/ +$/, "");
    if (x !== "") out.push(x);
  });
  return out.join(" ");
}
