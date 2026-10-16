// usage: node titles.cjs <file> [--json]  : prints top-level describe/it/test call titles with line ranges and counts of it() per block; with --json prints the sorted full-title list
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const fs = require("fs");
const file = process.argv[2];
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const lineOf = (n) => sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1;
const endOf = (n) => sf.getLineAndCharacterOfPosition(n.getEnd()).line + 1;
const names = new Set(["describe", "it", "test", "suite"]);
function calleeName(e) {
  if (ts.isIdentifier(e)) return e.text;
  if (ts.isPropertyAccessExpression(e)) { const b = calleeName(e.expression); return b ? b : null; }
  if (ts.isCallExpression(e)) return calleeName(e.expression);
  return null;
}
const out = [];
function titleOf(arg) {
  if (!arg) return "<none>";
  if (ts.isStringLiteralLike(arg)) return arg.text;
  return "<expr:" + arg.getText(sf).replace(/\s+/g, " ").slice(0, 80) + ">";
}
function walk(n, prefix, depth) {
  if (ts.isCallExpression(n)) {
    const c = calleeName(n.expression);
    if (c && names.has(c)) {
      const t = titleOf(n.arguments[0]);
      const full = prefix.concat(t);
      out.push({ kind: c, depth, title: full.join(" > "), start: lineOf(n), end: endOf(n) });
      n.arguments.slice(1).forEach((a) => walk(a, c === "describe" || c === "suite" ? full : full, depth + 1));
      return;
    }
  }
  ts.forEachChild(n, (ch) => walk(ch, prefix, depth));
}
walk(sf, [], 0);
if (process.argv[3] === "--json") { console.log(JSON.stringify(out.map((o) => `${o.kind}:${o.title}`).sort(), null, 0)); process.exit(0); }
for (const o of out) if (o.depth <= (+process.env.DEPTH || 1)) console.log(`${o.start}-${o.end}\t${o.kind}\t${"  ".repeat(o.depth)}${o.title.slice(0, 150)}`);
console.error("total titled calls:", out.length, "it/test:", out.filter((o) => o.kind !== "describe").length);
