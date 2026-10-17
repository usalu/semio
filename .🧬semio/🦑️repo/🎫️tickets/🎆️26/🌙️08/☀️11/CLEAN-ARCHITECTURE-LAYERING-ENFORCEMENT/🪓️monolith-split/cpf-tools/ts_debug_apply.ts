import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, writeFileSync, renameSync } from "node:fs";
import { parse, diagnostics } from "./ts_lib.ts";
const R = "/Users/ueli/Documents/semio/", G = process.env.G!;
const APPLY = process.argv.includes("--apply");
const occ: any[] = JSON.parse(readFileSync(G + "/cpf-ts-debug-occ2.json", "utf8"));
const EXTRA_CHANNELS = new Set<string>();
const DEFERRED = new Set(["Fixed UDRW image"]);
const channel = new Set<string>([...EXTRA_CHANNELS]);
for (const o of occ) if (o.role === "other" && !DEFERRED.has(o.tag)) channel.add(o.tag);
const newTags = new Set<string>([...EXTRA_CHANNELS]);
for (const o of occ) if (o.new) newTags.add(o.tag);
const PURE_CALLS = new Set(["stringify", "String", "Number", "Boolean", "slice", "toString", "join", "map", "filter", "isArray", "is", "keys", "values", "entries", "from", "at", "trim", "split", "includes", "startsWith", "endsWith", "toFixed", "padStart", "padEnd", "isInteger", "sort", "concat", "flatMap", "reduce", "some", "every", "find", "indexOf", "length", "get", "has", "toLowerCase", "toUpperCase", "replace", "replaceAll", "hex", "digest", "update", "createHash", "readFileSync", "basename", "relative", "resolve", "join", "stat", "lstatSync", "statSync", "now", "remainingMilliseconds", "selectedRustGraphAssemblyFactsV1", "frozen"]);
function impure(n: ts.Node, sf: ts.SourceFile): string | undefined {
  let bad: string | undefined;
  const v = (x: ts.Node) => {
    if (bad) return;
    if (ts.isAwaitExpression(x) || ts.isYieldExpression(x) || ts.isDeleteExpression(x) || ts.isNewExpression(x)) { bad = ts.SyntaxKind[x.kind]; return; }
    if (ts.isBinaryExpression(x) && x.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && x.operatorToken.kind <= ts.SyntaxKind.LastAssignment) { bad = "assign"; return; }
    if ((ts.isPrefixUnaryExpression(x) || ts.isPostfixUnaryExpression(x)) && (x.operator === ts.SyntaxKind.PlusPlusToken || x.operator === ts.SyntaxKind.MinusMinusToken)) { bad = "incdec"; return; }
    if (ts.isCallExpression(x)) {
      const e = x.expression; const name = ts.isPropertyAccessExpression(e) ? e.name.text : ts.isIdentifier(e) ? e.text : "?";
      if (!PURE_CALLS.has(name)) { bad = "call:" + name; return; }
    }
    ts.forEachChild(x, v);
  };
  v(n); return bad;
}
type Edit = { start: number; end: number; text: string; why: string };
const byFile = new Map<string, any[]>();
for (const o of occ) { if (!byFile.has(o.path)) byFile.set(o.path, []); byFile.get(o.path)!.push(o); }
let nRetag = 0, nDel = 0; const problems: string[] = [];
for (const [p, os] of byFile) {
  let text = readFileSync(R + p, "utf8");
  const sf = parse(p, text);
  const edits: Edit[] = [];
  const stmtEdits = new Map<number, Edit>();
  for (const o of os) {
    // re-locate occurrence by text to survive concurrent edits: verify the literal still contains the tag at the recorded range
    const tok = text.slice(o.start, o.end);
    if (!tok.includes("[DEBUG] ")) { problems.push(`${p}:${o.line} range drift`); continue; }
    if (DEFERRED.has(o.tag)) continue;
    if (channel.has(o.tag)) {
      if (!newTags.has(o.tag)) continue;
      const i = o.start + tok.indexOf("[DEBUG] ");
      edits.push({ start: i, end: i + "[DEBUG] ".length, text: "[DATA] ", why: "retag:" + o.tag }); nRetag++;
    } else if (o.role === "emit" && o.new) {
      if (!o.stmt) { problems.push(`${p}:${o.line} nonstmt emitter ${o.tag} ${o.ctx}`); continue; }
      // locate stmt node
      let node: ts.Node | undefined;
      const find = (n: ts.Node) => { if (node) return; if (n.getStart(sf, false) === o.stmt[0] && n.end === o.stmt[1] && ts.isExpressionStatement(n)) { node = n; return; } if (n.pos <= o.stmt[0] && n.end >= o.stmt[1]) ts.forEachChild(n, find); };
      find(sf);
      if (!node) { problems.push(`${p}:${o.line} stmt not found`); continue; }
      const st = node as ts.ExpressionStatement;
      const call = st.expression as ts.CallExpression; const why = ts.isCallExpression(call) ? call.arguments.map(a => impure(a, sf)).find(Boolean) : "notcall";
      if (why) { problems.push(`${p}:${o.line} impure args ${o.tag} (${why})`); continue; }
      let range: [number, number] = [st.getStart(sf, false), st.end];
      const par = st.parent;
      let repl = "";
      if (ts.isIfStatement(par)) {
        if (par.thenStatement === st && !par.elseStatement) {
          const c = impure(par.expression, sf);
          if (c) { problems.push(`${p}:${o.line} impure if-cond ${o.tag} (${c})`); continue; }
          range = [par.getStart(sf, false), par.end];
        } else if (par.elseStatement === st) {
          range = [par.thenStatement.end, st.end];
        } else { repl = "{}"; }
      } else if (ts.isBlock(par) || ts.isSourceFile(par) || ts.isCaseClause(par) || ts.isDefaultClause(par) || ts.isModuleBlock(par)) {
        // fine
      } else { repl = "{}"; }
      // whole-line removal when the statement is alone on its line
      let [s, e] = range;
      if (!repl) {
        const ls = text.lastIndexOf("\n", s - 1) + 1; let le = text.indexOf("\n", e); if (le < 0) le = text.length;
        if (text.slice(ls, s).trim() === "" && text.slice(e, le).trim() === "") { s = ls; e = Math.min(le + 1, text.length); }
      }
      stmtEdits.set(s, { start: s, end: e, text: repl, why: "delete:" + o.tag }); nDel++;
    }
  }
  const all = [...edits, ...stmtEdits.values()].sort((a, b) => b.start - a.start);
  for (let i = 1; i < all.length; i++) if (all[i].end > all[i - 1].start) problems.push(`${p} overlapping edits ${all[i].why} / ${all[i - 1].why}`);
  if (!all.length) continue;
  let out = text;
  for (const e of all) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  const d = diagnostics(p, out);
  if (d) problems.push(`${p} PARSE ERRORS ${d}`);
  console.log(p.slice(-60), "edits", all.length, "retag", edits.length, "del", stmtEdits.size);
  if (APPLY && !d) {
    if (readFileSync(R + p, "utf8") !== text) { problems.push(p + " changed concurrently"); continue; }
    writeFileSync(R + p + ".cpf.tmp", out); renameSync(R + p + ".cpf.tmp", R + p);
  }
}
console.log("retag", nRetag, "delete", nDel);
for (const p of problems) console.log("PROBLEM", p);
