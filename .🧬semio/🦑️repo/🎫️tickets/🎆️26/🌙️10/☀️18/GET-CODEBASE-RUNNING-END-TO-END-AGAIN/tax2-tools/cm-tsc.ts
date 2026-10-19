// 🧮 Replaces inline Bun.resolveSync of the TypeScript compiler by the shared resolver. usage: bun cm-tsc.ts <list.txt> [--write]
import {readFileSync, writeFileSync} from "node:fs";
import {recordOwn, accepts, addImport, ageMinutes, EXECUTION, ledger, read, rel, syntaxOk} from "./lib.ts";
const write = process.argv.includes("--write");
const tally: Record<string, number> = {};
const count = (k: string) => (tally[k] = (tally[k] ?? 0) + 1);
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const src = read(p);
  const re = /Bun\.resolveSync\(\s*(["'])typescript\/bin\/tsc\1\s*,\s*([^)]+?)\s*\)/gu;
  if (!re.test(src)) continue;
  re.lastIndex = 0;
  let out = src.replace(re, (_m, _q, root) => `resolveTypescriptCompilerV1(${root})`);
  const exec = /import\s*\{[^}]*\}\s*from\s*"([^"]*🎛️execution\/🟦️\.ts)";/u.exec(out);
  const canonical = rel(p, EXECUTION);
  out = addImport(out, ["resolveTypescriptCompilerV1"], canonical);
  if (!syntaxOk(p, out)) { count("syntax-fail"); ledger({tool: "cm-tsc", p, result: "syntax-fail"}); continue; }
  const acc = accepts(out);
  if (!write) { count(acc ? "ok-accepted" : "ok-still-rejected"); continue; }
  if (ageMinutes(p) < 15) { count("skip-recent"); continue; }
  if (read(p) !== src) { count("skip-changed"); continue; }
  writeFileSync(`/Users/ueli/Documents/semio/${p}`, out); recordOwn(p); count(acc ? "written-accepted" : "written-still-rejected"); ledger({tool: "cm-tsc", p, result: acc ? "written-accepted" : "written-still-rejected"});
}
console.log(tally);
