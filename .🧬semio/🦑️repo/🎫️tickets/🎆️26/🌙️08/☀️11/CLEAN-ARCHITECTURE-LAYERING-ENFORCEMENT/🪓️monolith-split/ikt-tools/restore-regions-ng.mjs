import fs from "node:fs";
import path from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { analyze } from "./analyze.mjs";
import cfg from "./cfg-nodegraph.mjs";
import { hooks } from "./fg-config.mjs";

const mode = process.argv[2] ?? "dry";
const ORIG = path.resolve("orig-NodeGraph.tsx");
process.env.SPLIT_SOURCE = ORIG;
const a = analyze(cfg.file);
const P = path.dirname(cfg.file);
const origLines = fs.readFileSync(ORIG, "utf8").split("\n");
const dirOf = new Map(cfg.modules.map((m) => [m.key, m.dir]));
const fileOfKey = (k) => {
  if (k === "parent") return cfg.file;
  const b = path.join(P, dirOf.get(k), "🟦️");
  return fs.existsSync(b + ".ts") ? b + ".ts" : b + ".tsx";
};
const hookFiles = hooks.map((h) => { const b = path.join(P, h.dir, "🟦️"); return fs.existsSync(b + ".ts") ? b + ".ts" : b + ".tsx"; });
const units = a.units.map((u, i) => ({ i, key: cfg.assign(u, a), sl: a.lineOf(u.start), el: a.lineOf(u.end), names: u.names, exportDecl: u.exportDecl }));
const unitOf156 = units.find((u) => u.names.includes("FlowGraphCanvasHost"));

const REG = /^[ \t]*\/\/\s*#(end)?region\b\s*(.*)$/;
const marks = [];
origLines.forEach((l, i) => { const m = REG.exec(l); if (m) marks.push({ line: i + 1, end: !!m[1], name: m[2].trim(), raw: l }); });
const stack = [], pairs = [];
for (const k of marks) {
  if (k.name === "🧲️Header") continue;
  if (!k.end) stack.push(k);
  else { const j = stack.map((s) => s.name).lastIndexOf(k.name); if (j < 0) throw new Error("unpaired end " + k.name); pairs.push({ b: stack[j], e: k }); stack.splice(j, 1); }
}
if (stack.length) throw new Error("unpaired begin " + stack.map((s) => s.name));

const texts = new Map();
const textOf = (f) => { if (!texts.has(f)) texts.set(f, fs.readFileSync(f, "utf8")); return texts.get(f); };
const stmtIndex = new Map();
const stmts = (f) => {
  if (stmtIndex.has(f)) return stmtIndex.get(f);
  const t = textOf(f);
  const sf = ts.createSourceFile(f, t, ts.ScriptTarget.ESNext, true, ts.ScriptKind.TSX);
  const rows = [];
  const bindNames = (b, out) => { if (ts.isIdentifier(b)) out.push(b.text); else for (const el of b.elements) if (!ts.isOmittedExpression(el)) bindNames(el.name, out); };
  for (const st of sf.statements) {
    if (ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) continue;
    const names = [];
    if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) bindNames(d.name, names);
    else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isEnumDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st)) && st.name) names.push(st.name.text);
    rows.push({ names, start: st.getStart(sf), end: st.end });
  }
  const imports = sf.statements.filter((s) => ts.isImportDeclaration(s));
  const info = { rows, lastImportEnd: imports.length ? imports[imports.length - 1].end : 0, firstImportStart: imports.length ? imports[0].getStart(sf) : 0 };
  stmtIndex.set(f, info);
  return info;
};
const lineStart = (t, p) => t.lastIndexOf("\n", p - 1) + 1;
const spanOf = (f, unit) => {
  const t = textOf(f);
  const rows = stmts(f).rows.filter((r) => r.names.some((n) => unit.names.includes(n)));
  if (!rows.length) return null;
  let s = Math.min(...rows.map((r) => r.start)), e = Math.max(...rows.map((r) => r.end));
  s = lineStart(t, s);
  for (;;) {
    if (s === 0) break;
    const prevStart = lineStart(t, s - 1);
    const line = t.slice(prevStart, s - 1);
    if (/^\s*(\/\*\*?|\*|\*\/|\/\/)/.test(line) && !REG.test(line)) s = prevStart; else break;
  }
  const nl = t.indexOf("\n", e);
  return { start: s, end: nl < 0 ? t.length : nl + 1 };
};

const ops = new Map();
const addOp = (f, op) => { if (!ops.has(f)) ops.set(f, []); ops.get(f).push(op); };
const report = [];
const perRegion = [];
for (const { b, e } of pairs) {
  const members = units.filter((u) => u.sl > b.line && u.el < e.line && !u.exportDecl);
  const spans = new Map();
  for (const u of members) {
    const f = fileOfKey(u.key);
    const sp = spanOf(f, u);
    if (!sp) throw new Error(`unit not found ${u.names} in ${f}`);
    const cur = spans.get(f);
    spans.set(f, cur ? { start: Math.min(cur.start, sp.start), end: Math.max(cur.end, sp.end) } : sp);
  }
  if (unitOf156 && members.includes(unitOf156)) {
    for (const hf of hookFiles) {
      const info = stmts(hf);
      const t = textOf(hf);
      const nl = t.indexOf("\n", info.lastImportEnd);
      spans.set(hf, { start: nl + 1, end: t.length });
    }
  }
  if (!members.length && b.name === "🔌️Adapters") {
    const info = stmts(cfg.file);
    spans.set(cfg.file, { start: lineStart(textOf(cfg.file), info.firstImportStart), end: textOf(cfg.file).indexOf("\n", info.lastImportEnd) + 1 });
  }
  if (!spans.size) throw new Error("region without any member: " + b.name);
  perRegion.push(`${b.name}: ${[...spans.keys()].map((f) => path.basename(path.dirname(f))).join(", ") || "parent"}`);
  for (const [f, sp] of spans) {
    addOp(f, { off: sp.start, kind: 0, order: b.line, str: b.raw + "\n" });
    addOp(f, { off: sp.end, kind: 1, order: -b.line, str: e.raw + "\n" });
  }
}
console.log(perRegion.join("\n"));
const out = new Map();
for (const [f, list] of ops) {
  const t = textOf(f);
  list.sort((x, y) => x.off - y.off || y.kind - x.kind || x.order - y.order);
  let res = "", cur = 0;
  for (const op of list) {
    res += t.slice(cur, op.off);
    if (op.off > 0 && op.off <= t.length && res.length > 0 && !res.endsWith("\n")) res += "\n";
    res += op.str;
    cur = op.off;
  }
  res += t.slice(cur);
  out.set(f, res);
}
if (mode === "dry") {
  const dir = process.argv[3];
  for (const [f, t] of out) { const d = path.join(dir, path.relative(P, f)); fs.mkdirSync(path.dirname(d), { recursive: true }); fs.writeFileSync(d, t); }
  console.log("dry files", out.size);
} else if (mode === "write") {
  for (const [f, t] of out) { if (fs.readFileSync(f, "utf8") !== texts.get(f)) throw new Error("changed on disk " + f); }
  for (const [f, t] of out) fs.writeFileSync(f, t);
  console.log("written", out.size);
}
