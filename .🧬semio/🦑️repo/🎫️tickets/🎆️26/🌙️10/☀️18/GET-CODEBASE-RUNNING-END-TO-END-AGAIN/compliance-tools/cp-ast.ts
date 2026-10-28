/** 🔎️ Compliance AST scan (lane compliance): bun cp-ast.ts <list file> <out jsonl>. Streams one JSON row per file. */
import { appendFileSync, readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
const TS_ROOT = "/Users/ueli/Documents/semio";
const ROOT = process.env.CP_ROOT ?? TS_ROOT;
const ts = createRequire(`${TS_ROOT}/package.json`)("typescript") as typeof import("typescript");
const [listFile, out] = process.argv.slice(2) as [string, string];
writeFileSync(out, "");
const files = readFileSync(listFile, "utf8").split("\n").filter(Boolean);
const LEAD = /^(?:\p{Extended_Pictographic}(?:️|\p{Emoji_Modifier})?(?:‍\p{Extended_Pictographic}️?)*|\p{Regional_Indicator}{2}|[0-9#*]️?⃣|\p{So}|\p{Sm})/u;
const BUILTIN = new Set(["fs", "path", "os", "child_process", "crypto", "url", "util", "assert", "events", "stream", "net", "http", "https", "zlib", "readline", "worker_threads", "perf_hooks", "buffer", "process", "module", "v8", "vm", "tty", "dns", "dgram", "cluster", "timers", "string_decoder", "querystring", "inspector", "async_hooks", "diagnostics_channel", "constants", "punycode", "repl", "tls", "http2", "wasi", "trace_events"]);
const LEGACY = /\b(legacy|deprecated|deprecation|compat|compatibility|backward|backwards|back-compat|shim|polyfill|workaround|hack|fixme|todo|xxx)\b/iu;
const DIRECTIVE = /^\/\/\s*(@ts-|eslint|prettier|biome|istanbul|c8|v8 ignore|#region|#endregion|region|endregion|\/\s*<reference)|^\/\/#(end)?region|^#!/u;

type Row = { rule: string; line: number; detail: string };
function leadOf(text: string): { key: string | null; first: string } {
  const body = text.replace(/^\/\*\*+/u, "").replace(/\*+\/$/u, "").split("\n").map((l) => l.replace(/^\s*\*?\s?/u, "")).join("\n").trimStart();
  const m = LEAD.exec(body);
  return { key: m ? m[0].replace(/️/gu, "") : null, first: body.slice(0, 24).replace(/\n/g, " ") };
}
for (const file of files) {
  const rows: Row[] = [];
  let text: string;
  try { text = readFileSync(`${ROOT}/${file}`, "utf8"); } catch { appendFileSync(out, JSON.stringify({ file, error: "unreadable" }) + "\n"); continue; }
  const kind = file.endsWith(".tsx") ? ts.ScriptKind.TSX : /\.m?ts$/u.test(file) ? ts.ScriptKind.TS : ts.ScriptKind.JS;
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, kind);
  const lineOf = (pos: number): number => sf.getLineAndCharacterOfPosition(pos).line + 1;
  const bodies: [number, number][] = [];
  const seen = new Set<number>();
  const comments: { pos: number; end: number; text: string; doc: boolean; line: boolean }[] = [];
  const addComments = (ranges: readonly ts.CommentRange[] | undefined): void => {
    for (const r of ranges ?? []) {
      if (seen.has(r.pos)) continue;
      seen.add(r.pos);
      const t = text.slice(r.pos, r.end);
      comments.push({ pos: r.pos, end: r.end, text: t, doc: t.startsWith("/**") && !t.startsWith("/**/") && !t.startsWith("/***"), line: r.kind === ts.SyntaxKind.SingleLineCommentTrivia });
    }
  };
  const walk = (node: ts.Node): void => {
    for (const child of node.getChildren(sf)) {
      if (ts.isFunctionLike(child) && (child as ts.FunctionLikeDeclaration).body) {
        const b = (child as ts.FunctionLikeDeclaration).body!;
        bodies.push([b.getStart(sf), b.end]);
      }
      if (child.getChildCount(sf) === 0) { addComments(ts.getLeadingCommentRanges(text, child.getFullStart())); addComments(ts.getTrailingCommentRanges(text, child.getFullStart())); addComments(ts.getTrailingCommentRanges(text, child.end)); }
      else walk(child);
    }
  };
  walk(sf);
  const inBody = (pos: number): boolean => bodies.some(([a, b]) => pos > a && pos < b);
  const keys = new Map<string, number[]>();
  for (const c of comments.sort((a, b) => a.pos - b.pos)) {
    const line = lineOf(c.pos);
    if (c.doc) {
      const lead = leadOf(c.text);
      if (lead.key === null) rows.push({ rule: "doc-not-emoji", line, detail: lead.first });
      else keys.set(lead.key, [...(keys.get(lead.key) ?? []), line]);
      if (inBody(c.pos)) rows.push({ rule: "comment-in-body", line, detail: `docstring in a function body: ${lead.first}` });
    } else if (inBody(c.pos)) {
      const t = c.text.trim();
      rows.push({ rule: DIRECTIVE.test(t) ? "directive-in-body" : "comment-in-body", line, detail: t.slice(0, 100) });
    }
  }
  for (const [key, lines] of keys) if (lines.length > 1) for (const line of lines) rows.push({ rule: "dup-emoji", line, detail: `${key} x${lines.length} (${lines.join(",")})` });
  {
    for (const st of sf.statements) {
      const mods = ts.canHaveModifiers(st) ? ts.getModifiers(st) : undefined;
      const exported = mods?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword) ?? false;
      const isExp = exported || ts.isExportAssignment(st);
      if (!isExp) continue;
      const hasDoc = ((st as any).jsDoc?.length ?? 0) > 0;
      if (!hasDoc) {
        const name = (st as any).name?.getText?.(sf) ?? (ts.isVariableStatement(st) ? st.declarationList.declarations.map((d) => d.name.getText(sf)).join(",") : ts.SyntaxKind[st.kind]);
        rows.push({ rule: "export-no-doc", line: lineOf(st.getStart(sf)), detail: String(name).slice(0, 60) });
      }
    }
  }
  const imports = new Set<string>();
  const collect = (node: ts.Node): void => {
    if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier)) imports.add(node.moduleSpecifier.text);
    if (ts.isCallExpression(node) && node.arguments.length >= 1 && ts.isStringLiteralLike(node.arguments[0]!)) {
      const callee = node.expression;
      if (callee.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(callee) && callee.text === "require")) imports.add((node.arguments[0] as ts.StringLiteral).text);
    }
    ts.forEachChild(node, collect);
  };
  collect(sf);
  for (const spec of imports) {
    if (spec.startsWith(".") || spec.startsWith("/")) continue;
    if (spec.startsWith("node:") || spec.startsWith("bun:")) continue;
    if (BUILTIN.has(spec.split("/")[0]!)) { rows.push({ rule: "builtin-without-node-prefix", line: 0, detail: spec }); continue; }
    rows.push({ rule: "external-import", line: 0, detail: spec });
  }
  const lines = text.split("\n");
  const commentLines = new Set<number>();
  for (const c of comments) { const a = lineOf(c.pos), b = lineOf(c.end); for (let i = a; i <= b; i++) commentLines.add(i); }
  lines.forEach((raw, index) => {
    const n = index + 1;
    if (raw.includes("[DEBUG]")) rows.push({ rule: "debug-tag", line: n, detail: raw.trim().slice(0, 100) });
    if (!commentLines.has(n) && LEGACY.test(raw) && !/^\s*import\b/u.test(raw)) rows.push({ rule: "legacy-word", line: n, detail: raw.trim().slice(0, 140) });
  });
  appendFileSync(out, JSON.stringify({ file, rows, lines: lines.length, bytes: text.length }) + "\n");
}
console.log("[DEBUG] scanned", files.length, "rss MB", Math.round(process.memoryUsage().rss / 1048576));
