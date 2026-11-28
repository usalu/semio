/** 🔎️ H116 static census of bun tests: zero-assertion tests, conditional-only assertions, dangling expect(), source-text greps. Usage: bun observed-census.ts <out.json> <root>... */
import ts from "typescript";
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const SKIP = new Set(["node_modules", "target", ".cache", "dist", ".nx", "🗑️generated", "generated", "bazel-out"]);
const TEST_FNS = new Set(["test", "it"]);
const TEXT_MATCHERS = new Set(["toContain", "toMatch", "toInclude", "toStartWith", "toEndWith", "toContainEqual"]);
const READERS = /readFileSync|readText|readSource|\.text\(\)|readUtf8|readRepoFile|sourceOf|readTextFile/;
const HELPER = /^(assert|expect|verify|check|must|ensure|require)/i;

interface Row { file: string; name: string; line: number; kind: string; detail?: string }

function walk(dir: string, out: string[]): void {
  for (const e of readdirSync(dir)) {
    if (SKIP.has(e)) continue;
    const p = join(dir, e);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walk(p, out);
    else if (/\.(ts|mts)$/.test(e) && !e.endsWith(".d.ts")) out.push(p);
  }
}

function calleeName(c: ts.CallExpression): string {
  let e: ts.Expression = c.expression;
  while (ts.isPropertyAccessExpression(e) || ts.isCallExpression(e)) e = ts.isPropertyAccessExpression(e) ? e.expression : e.expression;
  return ts.isIdentifier(e) ? e.text : "";
}

function testNameCall(c: ts.CallExpression): boolean {
  let e: ts.Expression = c.expression;
  if (ts.isCallExpression(e)) e = e.expression;
  while (ts.isPropertyAccessExpression(e)) {
    if (!["each", "skip", "only", "todo", "serial", "concurrent", "failing", "if", "skipIf", "todoIf"].includes(e.name.text)) return false;
    e = e.expression;
  }
  return ts.isIdentifier(e) && TEST_FNS.has(e.text);
}

function isSkipped(c: ts.CallExpression): boolean {
  const s = c.expression.getText();
  return /\.(skip|todo)\b/.test(s);
}

function analyse(file: string, rows: Row[], stat: { files: number; tests: number; sourceGrepFiles: number }): void {
  const text = readFileSync(file, "utf8");
  if (!/from\s+["'](bun:test|vitest)["']/.test(text)) return;
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true);
  stat.files++;
  const tainted = new Set<string>();
  let textAsserts = 0;
  let allAsserts = 0;
  const visitTaint = (n: ts.Node): void => {
    if (ts.isVariableDeclaration(n) && ts.isIdentifier(n.name) && n.initializer && READERS.test(n.initializer.getText().slice(0, 400))) tainted.add(n.name.text);
    ts.forEachChild(n, visitTaint);
  };
  visitTaint(sf);

  const visitTests = (n: ts.Node): void => {
    if (ts.isCallExpression(n) && testNameCall(n)) {
      const fn = [...n.arguments].reverse().find((a) => ts.isArrowFunction(a) || ts.isFunctionExpression(a)) as ts.ArrowFunction | undefined;
      const nameArg = n.arguments[0];
      const name = nameArg && (ts.isStringLiteralLike(nameArg) || ts.isTemplateExpression(nameArg)) ? nameArg.getText().slice(0, 100) : "<dynamic>";
      const line = sf.getLineAndCharacterOfPosition(n.getStart()).line + 1;
      if (!fn || isSkipped(n)) { ts.forEachChild(n, visitTests); return; }
      stat.tests++;
      let direct = 0, helper = 0, conditional = 0, dangling = 0, textHits = 0, asserts = 0;
      const visit = (m: ts.Node, cond: boolean): void => {
        if (ts.isCallExpression(m)) {
          const cn = calleeName(m);
          const full = m.expression.getText();
          const rootIdent = ts.isIdentifier(m.expression) ? m.expression.text : "";
          const isAssertCall = rootIdent === "expect" || rootIdent === "assert" || /^assert\.\w+$/.test(full) || /^expect\.(assertions|hasAssertions)$/.test(full);
          if (isAssertCall) {
            direct++;
            if (cond) conditional++;
            if (rootIdent === "expect") {
              const p = m.parent;
              if (ts.isExpressionStatement(p)) dangling++;
              const subj = m.arguments[0];
              let outer: ts.Node = m;
              const names: string[] = [];
              while (ts.isPropertyAccessExpression(outer.parent) || ts.isCallExpression(outer.parent)) {
                outer = outer.parent;
                if (ts.isPropertyAccessExpression(outer)) names.push(outer.name.text);
              }
              if (subj) {
                const subjText = subj.getText();
                const fromSource = (ts.isIdentifier(subj) && tainted.has(subj.text)) || READERS.test(subjText);
                if (fromSource && names.some((x) => TEXT_MATCHERS.has(x))) textHits++;
              }
            }
          } else if (HELPER.test(cn) || HELPER.test(full.split(".").pop() ?? "")) helper++;
        }
        const isCondNode = ts.isIfStatement(m) || ts.isForStatement(m) || ts.isForOfStatement(m) || ts.isForInStatement(m) || ts.isWhileStatement(m) || ts.isCatchClause(m) || ts.isConditionalExpression(m) || (ts.isCallExpression(m) && /\.(forEach|map|filter|some|every)$/.test(m.expression.getText()) && true);
        ts.forEachChild(m, (c) => visit(c, cond || isCondNode));
      };
      visit(fn.body, false);
      asserts = direct;
      allAsserts += asserts;
      textAsserts += textHits;
      const base = { file, name, line };
      if (direct === 0 && helper === 0) rows.push({ ...base, kind: "zero-assertions" });
      else if (direct === 0) rows.push({ ...base, kind: "helper-only", detail: `${helper} helper calls` });
      else if (conditional === direct) rows.push({ ...base, kind: "conditional-only", detail: `${direct} expects all inside if/loop/callback` });
      if (dangling) rows.push({ ...base, kind: "dangling-expect", detail: `${dangling} expect() statements without matcher` });
      if (textHits > 0 && textHits === direct) rows.push({ ...base, kind: "source-text-only", detail: `${textHits}/${direct} expects are text matchers on source read from disk` });
    }
    ts.forEachChild(n, visitTests);
  };
  visitTests(sf);
  if (textAsserts > 0) {
    stat.sourceGrepFiles++;
    rows.push({ file, name: "<file>", line: 0, kind: "source-text-file", detail: `${textAsserts}/${allAsserts} expects` });
  }
}

const [out, ...roots] = process.argv.slice(2);
const files: string[] = [];
for (const r of roots) walk(r, files);
const rows: Row[] = [];
const stat = { files: 0, tests: 0, sourceGrepFiles: 0 };
for (const f of files) {
  try { analyse(f, rows, stat); } catch (e) { rows.push({ file: f, name: "<parse>", line: 0, kind: "error", detail: String(e).slice(0, 120) }); }
}
writeFileSync(out, JSON.stringify({ stat, rows }, null, 1));
const by: Record<string, number> = {};
for (const r of rows) by[r.kind] = (by[r.kind] ?? 0) + 1;
console.log(JSON.stringify({ scanned: files.length, ...stat, by }));
