import { test, expect } from "bun:test";
import * as fs from "node:fs";
import * as path from "node:path";
import ts from "typescript";

const mode = process.env.TJC_MODE!;
const realDir = process.env.TJC_REAL_DIR!;
const oldFile = process.env.TJC_OLD_FILE!;
const out = process.env.TJC_OUT!;
const trace: string[] = [];
(globalThis as any).__t = (value: unknown) => { trace.push(value ? "T" : "F"); return value; };
(globalThis as any).__c = (name: string, run: () => number) => { try { const value = run(); trace.push(`CALL ${name} ${value}`); return value; } catch (e) { trace.push(`CALLERR ${name} ${e instanceof Error ? e.message : String(e)}`); return 0; } };
(globalThis as any).__f = (message: unknown) => { trace.push("THROW " + String(message)); };
const instrument = (source: string, name: string) => {
  const sf = ts.createSourceFile(name, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const edits: { s: number; e: number; t: string }[] = [];
  const throwsIn = (n: ts.Node): boolean => ts.isThrowStatement(n) || (ts.isBlock(n) && n.statements.some(throwsIn));
  const visit = (n: ts.Node) => {
    if (ts.isIfStatement(n) && throwsIn(n.thenStatement)) edits.push({ s: n.expression.getStart(sf), e: n.expression.getEnd(), t: `__t(${n.expression.getText(sf)})` });
    if (ts.isThrowStatement(n) && n.expression && ts.isNewExpression(n.expression) && n.expression.arguments?.length === 1) edits.push({ s: n.getStart(sf), e: n.getEnd(), t: `__f(${n.expression.arguments[0]!.getText(sf)});` });
    if (ts.isCallExpression(n) && ts.isIdentifier(n.expression) && /SelfTests$/.test(n.expression.text)) edits.push({ s: n.getStart(sf), e: n.getEnd(), t: `__c(${JSON.stringify(n.expression.text)}, () => ${n.getText(sf)})` });
    ts.forEachChild(n, visit);
  };
  visit(sf);
  let outText = source;
  const nested = edits.filter((e, i) => !edits.some((o, j) => j !== i && o.s <= e.s && o.e >= e.e && (o.s !== e.s || o.e !== e.e)));
  for (const e of nested.sort((a, b) => b.s - a.s)) outText = outText.slice(0, e.s) + e.t + outText.slice(e.e);
  return outText;
};
const sourceMarker = mode === "old" ? oldFile : realDir;
Bun.plugin({
  name: "tjc-instrument",
  setup(build) {
    build.onResolve({ filter: /^\.\.?\// }, (args) => {
      if (mode === "old" && args.importer === oldFile) return { path: path.resolve(realDir, args.path) };
      return undefined;
    });
    const esc = (s: string) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    build.onLoad({ filter: new RegExp(`^(${esc(oldFile)}|${esc(realDir)}/.*)$`) }, (args) => {
      let source = fs.readFileSync(args.path, "utf8");
      if (process.env.TJC_MUTATE) { const [from, to] = process.env.TJC_MUTATE.split("=>"); source = source.split(from!).join(to!); }
      return { contents: instrument(source, args.path), loader: "ts" };
    });
  },
});
let result: unknown;
let error: string | undefined;
const entry = mode === "old" ? oldFile : path.join(realDir, "🟦️.ts");
const m = await import(entry);
const t0 = performance.now();
try { result = m.toolJobCoverageSelfTests(); } catch (e) { error = e instanceof Error ? e.message : String(e); }
fs.writeFileSync(out, JSON.stringify({ mode, result: String(result), error, count: trace.length, trace }, null, 0));
console.log("RESULT", mode, result, error ?? "", trace.length, "events", Math.round(performance.now() - t0), "ms");
test("tjc differential trace recorded", () => { expect(trace.length).toBeGreaterThan(0); });
