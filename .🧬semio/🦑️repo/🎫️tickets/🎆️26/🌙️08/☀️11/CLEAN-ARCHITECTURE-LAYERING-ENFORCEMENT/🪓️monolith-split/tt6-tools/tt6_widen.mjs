// 🧭 For every TS2769 on an `expect(actual).matcher(expected)` call, types the comparison by the expected side: `expect<typeof expected>(actual).matcher(expected)`.
// `expected` that is not an entity name is hoisted into a `const` before its statement. Never swaps actual and expected, never uses unknown/any.
// usage: node tt6_widen.mjs <tsc-output> [--apply] [--code TS2769]
import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
const root = "/Users/ueli/Documents/semio";
const require = createRequire(root + "/package.json");
const ts = require("typescript");
const MATCHERS = new Set(["toBe", "toEqual", "toStrictEqual", "toContain", "toContainEqual"]);
const code = process.argv.includes("--code") ? process.argv[process.argv.indexOf("--code") + 1] : "TS2769";
const apply = process.argv.includes("--apply");
const fallback = process.argv.includes("--fallback");
const only = process.argv.includes("--only") ? process.argv[process.argv.indexOf("--only") + 1] : null;
const diags = new Map();
for (const line of readFileSync(process.argv[2], "utf8").split("\n")) {
  const m = line.match(new RegExp(`^(.+?)\\((\\d+),(\\d+)\\): error ${code}:`));
  if (!m) continue;
  if (only && !m[1].includes(only)) continue;
  if (!diags.has(m[1])) diags.set(m[1], []);
  diags.get(m[1]).push({ line: +m[2], col: +m[3] });
}
const entity = /^[A-Za-z_$][\w$]*(\.[A-Za-z_$][\w$]*)*$/;
let done = 0, skipped = 0;
for (const [file, list] of diags) {
  const full = root + "/" + file;
  let text = readFileSync(full, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, true, file.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits = [];
  const claimed = new Set();
  for (const d of list) {
    const pos = sf.getPositionOfLineAndCharacter(d.line - 1, d.col - 1);
    let found = null;
    const visit = (n) => {
      if (n.getStart() > pos || n.getEnd() <= pos) return;
      if (ts.isCallExpression(n) && ts.isPropertyAccessExpression(n.expression) && MATCHERS.has(n.expression.name.text)) {
        let recv = n.expression.expression;
        while (ts.isPropertyAccessExpression(recv) && ["not", "resolves", "rejects"].includes(recv.name.text)) recv = recv.expression;
        if (ts.isCallExpression(recv) && ts.isIdentifier(recv.expression) && recv.expression.text === "expect" && n.arguments.length === 1) found = { n, recv };
      }
      ts.forEachChild(n, visit);
    };
    visit(sf);
    if (!found || claimed.has(found.n.getStart())) { console.log(`SKIP ${file}:${d.line}:${d.col} no matcher call`); skipped++; continue; }
    claimed.add(found.n.getStart());
    const { n, recv } = found;
    if (recv.typeArguments?.length) {
      if (!fallback) { console.log(`SKIP ${file}:${d.line} already has a type argument`); skipped++; continue; }
      const typeText = recv.typeArguments[0].getText();
      const chain0 = n.expression.expression.getText().slice(recv.getText().length);
      edits.push({ start: n.getStart(), end: n.getEnd(), text: `expect(${recv.arguments[0].getText()})${chain0}.${n.expression.name.text}<${typeText}>(${n.arguments[0].getText()})` });
      done++;
      continue;
    }
    if (fallback) { console.log(`SKIP ${file}:${d.line} fallback needs a type argument`); skipped++; continue; }
    const exp = n.arguments[0];
    const expText = exp.getText();
    const actText = recv.arguments[0].getText();
    const chain = n.expression.expression.getText().slice(recv.getText().length);
    const m = n.expression.name.text;
    if (entity.test(expText)) {
      edits.push({ start: n.getStart(), end: n.getEnd(), text: `expect<typeof ${expText}>(${actText})${chain}.${m}(${expText})` });
      done++;
      continue;
    }
    let stmt = n;
    while (stmt && !(ts.isExpressionStatement(stmt))) stmt = stmt.parent;
    if (!stmt || !(ts.isBlock(stmt.parent) || ts.isSourceFile(stmt.parent))) { console.log(`SKIP ${file}:${d.line}:${d.col} cannot hoist (${expText.slice(0, 60)})`); skipped++; continue; }
    const name = `expectedAt${d.line}x${d.col}`;
    edits.push({ start: stmt.getStart(), end: stmt.getStart(), text: `const ${name} = ${expText}; `, hoist: true });
    edits.push({ start: n.getStart(), end: n.getEnd(), text: `expect<typeof ${name}>(${actText})${chain}.${m}(${name})` });
    done++;
  }
  if (apply && edits.length) {
    edits.sort((a, b) => b.start - a.start || (a.hoist ? 1 : 0) - (b.hoist ? 1 : 0));
    for (const e of edits) text = text.slice(0, e.start) + e.text + text.slice(e.end);
    writeFileSync(full, text);
  }
}
console.error(`edited=${done} skipped=${skipped} files=${diags.size} apply=${apply}`);
