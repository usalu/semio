// 🔎 Classifies every reference to every list script: IMPORT, READ (source read), SPAWN, DATA (json/toml), OTHER. usage: bun ref-classify.ts [substring] [--kind READ]
import {readFileSync} from "node:fs";
import {refsOf} from "./refs-lib.ts";
import {LIB, REPO, TOOLS} from "./vfs.ts";
const key = process.argv[2] && !process.argv[2].startsWith("--") ? process.argv[2] : "";
const only = process.argv.includes("--kind") ? process.argv[process.argv.indexOf("--kind") + 1] : "";
const targets = readFileSync(`${TOOLS}/library-43.txt`, "utf8").split("\n").filter(Boolean).filter((p) => p.includes(key));
const tally: Record<string, number> = {};
for (const target of targets) {
  const rows: string[] = [];
  for (const r of refsOf(target)) {
    const text = readFileSync(`${REPO}/${r.file}`, "utf8").split("\n")[r.line - 1]!;
    const at = text.indexOf(r.spec), before = text.slice(Math.max(0, at - 160), at);
    const kind = /\.(json|toml|md|mjs)$/u.test(r.file) && !/\.mjs$/u.test(r.file) ? "DATA" : /(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s*)["'`]$/u.test(text.slice(Math.max(0, at - 24), at)) ? "IMPORT" : /readFile|Bun\.file|createSourceFile|\.text\(\)|statSync|existsSync/u.test(before) ? "READ" : /runOwnedCommand|captureOwned|spawn|execPath|runCmd|"bun"|Command|execute/u.test(before) ? "SPAWN" : "OTHER";
    tally[kind] = (tally[kind] ?? 0) + 1;
    if (only && kind !== only) continue;
    if (kind === "IMPORT") continue;
    rows.push(`  ${kind.padEnd(5)} ${r.file.replace(LIB, "L").slice(-80)}:${r.line} ${text.slice(Math.max(0, at - 70), at + r.spec.length + 25).trim()}`);
  }
  if (rows.length) console.log(`## ${target.replace(LIB, "L")}\n${rows.join("\n")}`);
}
console.log(JSON.stringify(tally));
