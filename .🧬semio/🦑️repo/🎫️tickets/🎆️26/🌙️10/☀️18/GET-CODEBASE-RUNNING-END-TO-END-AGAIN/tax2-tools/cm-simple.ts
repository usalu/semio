// 🧹 Behaviour-preserving syntax normalizations the router law requires. usage: bun cm-simple.ts <list.txt> [--write]
import {readFileSync, writeFileSync} from "node:fs";
import {recordOwn, accepts, ageMinutes, ledger, read, syntaxOk} from "./lib.ts";
const write = process.argv.includes("--write");
const tally: Record<string, number> = {};
const count = (k: string) => (tally[k] = (tally[k] ?? 0) + 1);
const rules: [string, (s: string) => string][] = [
  ["throw-new", (s) => s.replace(/\bthrow Error\(/g, "throw new Error(")],
  ["default-command-spread", (s) => s.replace(/\.\.\.\(\{\s*defaultCommand:\s*("[^"]+")\s*\}\)/g, "defaultCommand: $1")],
];
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const src = read(p);
  let out = src; const hit: string[] = [];
  for (const [name, f] of rules) { const n = f(out); if (n !== out) { hit.push(name); out = n; } }
  if (out === src) continue;
  if (!syntaxOk(p, out)) { count("syntax-fail"); continue; }
  const acc = accepts(out);
  if (!write) { count(acc ? "ok-accepted" : "ok-still-rejected"); continue; }
  if (ageMinutes(p) < 15) { count("skip-recent"); continue; }
  if (read(p) !== src) { count("skip-changed"); continue; }
  writeFileSync(`/Users/ueli/Documents/semio/${p}`, out); recordOwn(p); count(acc ? "written-accepted" : "written-still-rejected"); ledger({tool: "cm-simple", p, rules: hit, result: acc ? "written-accepted" : "written-still-rejected"});
}
console.log(tally);
