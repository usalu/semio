/** 🧭 Classify the syntactic context of each hit so the rewriter handles only shapes it understands. */
import { readFileSync } from "node:fs";
import ts from "typescript";
import { collect, type Hit } from "./work.ts";

const astCache = new Map<string, ts.SourceFile>();
const READERS = new Set(["readFileSync", "readFile", "existsSync", "statSync", "readTextFile"]);
const PATH_BUILDERS = new Set(["resolve", "join"]);
const callee = (call: ts.CallExpression): string => {
  const e = call.expression;
  return ts.isIdentifier(e) ? e.text : ts.isPropertyAccessExpression(e) ? e.name.text : "";
};
const calleeFull = (call: ts.CallExpression): string => {
  const e = call.expression;
  return ts.isPropertyAccessExpression(e) ? `${e.expression.getText()}.${e.name.text}` : ts.isIdentifier(e) ? e.text : "";
};
function readerPath(h: Hit): Form | null {
  let source = astCache.get(h.text);
  if (!source) {
    source = ts.createSourceFile("x.tsx", h.text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
    if (astCache.size > 4) astCache.clear();
    astCache.set(h.text, source);
  }
  let found: ts.Node | undefined;
  const visit = (node: ts.Node): void => {
    if (found) return;
    if ((ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node)) && node.getStart(source) === h.start) {
      found = node;
      return;
    }
    if (node.end < h.start || node.getStart(source) > h.end) return;
    ts.forEachChild(node, visit);
  };
  visit(source);
  if (!found) return null;
  const parent = found.parent;
  if (!ts.isCallExpression(parent) && !ts.isNewExpression(parent)) return null;
  const isUrl = ts.isNewExpression(parent) && parent.expression.getText() === "URL" && parent.arguments?.[0] === found && (parent.arguments?.length ?? 0) === 2;
  const isBuilder = ts.isCallExpression(parent) && PATH_BUILDERS.has(callee(parent)) && parent.arguments[parent.arguments.length - 1] === found && parent.arguments.length >= 2;
  if (!isUrl && !isBuilder) return null;
  const outer = parent.parent;
  if (!ts.isCallExpression(outer) || outer.arguments[0] !== parent) return null;
  const name = calleeFull(outer);
  if (!READERS.has(callee(outer)) && name !== "Bun.file") return null;
  return { kind: "reader-path", from: parent.getStart(source), to: parent.end };
}

export type Form =
  | { kind: "static"; from: number; to: number; binding: string }
  | { kind: "dyn-default-destructure"; from: number; to: number; binding: string }
  | { kind: "dyn-paren-default"; from: number; to: number }
  | { kind: "dyn-bare"; from: number; to: number }
  | { kind: "url"; from: number; to: number }
  | { kind: "reader-path"; from: number; to: number }
  | { kind: "other" };

export function classify(h: Hit): Form {
  const t = h.text;
  const before = t.slice(Math.max(0, h.start - 120), h.start);
  const after = t.slice(h.end, h.end + 80);
  let m = /(?:^|[\n;])[ \t]*import[ \t]+([A-Za-z_$][\w$]*)[ \t]+from[ \t]*$/.exec(before);
  if (m) {
    const tail = /^[ \t]*(?:with[ \t]*\{[ \t]*type:[ \t]*"json"[ \t]*\})?[ \t]*;?/.exec(after)!;
    const lineStart = h.start - before.length + before.lastIndexOf("import", before.length) - (before.length - before.length);
    const idx = before.lastIndexOf("import");
    return { kind: "static", from: h.start - before.length + idx, to: h.end + tail[0].length, binding: m[1]! };
  }
  m = /\{[ \t]*default:[ \t]*([A-Za-z_$][\w$]*)[ \t]*\}[ \t]*=[ \t]*await[ \t]+import\([ \t]*$/.exec(before);
  if (m && /^[ \t]*\)/.test(after)) {
    const open = before.lastIndexOf("{");
    const close = after.indexOf(")") + 1;
    return { kind: "dyn-default-destructure", from: h.start - before.length + before.lastIndexOf("{", before.lastIndexOf("default")), to: h.end + close, binding: m[1]! };
  }
  m = /\([ \t]*await[ \t]+import\([ \t]*$/.exec(before);
  if (m && /^[ \t]*\)[ \t]*\)[ \t]*\.default\b/.test(after)) {
    const k = /^[ \t]*\)[ \t]*\)[ \t]*\.default\b/.exec(after)![0];
    return { kind: "dyn-paren-default", from: h.start - m[0].length, to: h.end + k.length };
  }
  m = /await[ \t]+import\([ \t]*$/.exec(before);
  if (m && /^[ \t]*\)/.test(after)) return { kind: "dyn-bare", from: h.start - m[0].length, to: h.end + after.indexOf(")") + 1 };
  m = /new[ \t]+URL\([ \t]*$/.exec(before);
  if (m) {
    const k = /^[ \t]*,[ \t]*import\.meta\.url[ \t]*\)/.exec(after);
    if (k) return { kind: "url", from: h.start - m[0].length, to: h.end + k[0].length };
  }
  const viaReader = readerPath(h);
  return viaReader ?? { kind: "other" };
}

if (import.meta.main) {
  const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
  const { hits } = collect(files);
  const counts: Record<string, number> = {};
  const others: string[] = [];
  for (const h of hits) {
    const f = classify(h);
    counts[f.kind] = (counts[f.kind] ?? 0) + 1;
    if (f.kind === "other") {
      const line = h.text.slice(0, h.start).split("\n").length;
      others.push(`${h.file}:${line}\t${h.text.split("\n")[line - 1]!.trim().slice(0, 220)}`);
    }
  }
  console.log(counts);
  console.log(others.join("\n"));
}
