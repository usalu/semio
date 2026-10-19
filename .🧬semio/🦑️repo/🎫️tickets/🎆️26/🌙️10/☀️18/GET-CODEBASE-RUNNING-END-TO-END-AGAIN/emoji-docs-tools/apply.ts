import ts from "typescript";
import { statSync, readFileSync, writeFileSync } from "node:fs";
import { applyEdits, planFile } from "./plan.ts";
import { blocksOf } from "./docs.ts";

const root = "/Users/ueli/Documents/semio";
const out = process.argv[2];
const mode = process.argv[3] ?? "dry";
const list: string[] = JSON.parse(readFileSync(`${out}/${process.argv[4] ?? "targets.json"}`, "utf8"));
const WINDOW = 15 * 60 * 1000;
const applied: any[] = [];
const deferred: string[] = [];
const failed: any[] = [];
const skippedTotals: Record<string, number> = {};
const how: Record<string, number> = {};
const lineWise = (a: string, b: string, isDoc: (l: string) => boolean) => {
  const x = a.split("\n"), y = b.split("\n");
  if (x.length !== y.length) return false;
  for (let i = 0; i < x.length; i++) if (x[i] !== y[i] && !(isDoc(x[i]) || isInComment(x, i))) return false;
  return true;
};
const isInComment = (lines: string[], i: number) => /^\s*(\/\*\*|\*)/.test(lines[i]) || /\/\*\*/.test(lines[i]);
const tsCode = (p: string, t: string) => ts.transpileModule(t, { fileName: p, reportDiagnostics: false, compilerOptions: { removeComments: true, target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, jsx: ts.JsxEmit.Preserve, allowJs: true, isolatedModules: true } }).outputText;

for (const p of list) {
  const abs = `${root}/${p}`;
  let st;
  try { st = statSync(abs); } catch { continue; }
  if (Date.now() - st.mtimeMs < WINDOW) { deferred.push(p); continue; }
  const text = readFileSync(abs, "utf8");
  const plan = planFile(p, text);
  for (const [k, v] of Object.entries(plan.skipped)) skippedTotals[k] = (skippedTotals[k] ?? 0) + v;
  if (!plan.edits.length) continue;
  const next = applyEdits(text, plan.edits);
  let ok = true;
  let why = "";
  if (p.endsWith(".rs")) {
    ok = lineWise(text, next, (l) => /^\s*\/\/[\/!]/.test(l));
    why = "rs-lines";
  } else {
    ok = lineWise(text, next, () => false);
    why = "ts-lines";
    if (ok) {
      try { ok = tsCode(p, text) === tsCode(p, next); why = "ts-transpile"; } catch (e) { ok = false; why = "ts-throw"; }
    }
  }
  if (ok) {
    const after = blocksOf(p, next);
    const before = blocksOf(p, text);
    if (after.length !== before.length) { ok = false; why = "block-count"; }
  }
  if (!ok) { failed.push({ p, why }); continue; }
  if (mode === "apply") {
    const st2 = statSync(abs);
    if (st2.mtimeMs !== st.mtimeMs || Date.now() - st2.mtimeMs < WINDOW) { deferred.push(p); continue; }
    writeFileSync(abs, next);
  }
  for (const e of plan.edits) how[e.how] = (how[e.how] ?? 0) + 1;
  applied.push({ p, n: plan.edits.length, edits: plan.edits.map((e) => ({ line: e.line, emoji: e.emoji, how: e.how, name: e.name, head: e.head })) });
}
writeFileSync(`${out}/applied-${mode}.json`, JSON.stringify(applied));
writeFileSync(`${out}/deferred-${mode}.json`, JSON.stringify(deferred));
writeFileSync(`${out}/failed-${mode}.json`, JSON.stringify(failed));
console.log(mode, "files", applied.length, "edits", applied.reduce((a, b) => a + b.n, 0), "deferred", deferred.length, "failed", failed.length, "\nhow", how, "\nskipped", skippedTotals);
if (failed.length) console.log(failed.slice(0, 20));
