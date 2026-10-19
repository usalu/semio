// 🔎 Prints every reference to a script with its code context so that non-import references (source reads, spawns) can be classified. usage: bun ref-report.ts <substring>
import {readFileSync} from "node:fs";
import {refsOf} from "./refs-lib.ts";
import {LIB, REPO, TOOLS} from "./vfs.ts";
const key = process.argv[2]!;
const target = readFileSync(`${TOOLS}/library-43.txt`, "utf8").split("\n").find((p) => p.includes(key))!;
console.log("TARGET", target.replace(LIB, "L"));
for (const r of refsOf(target)) {
  const line = readFileSync(`${REPO}/${r.file}`, "utf8").split("\n")[r.line - 1]!;
  const at = line.indexOf(r.spec), ctx = line.slice(Math.max(0, at - 110), at + r.spec.length + 60).trim();
  const kind = /(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s*)["'`]$/u.test(line.slice(Math.max(0, at - 24), at)) ? "IMPORT" : "other ";
  console.log(`${kind} ${r.file.replace(LIB, "L").slice(-95)}:${r.line}  ${ctx}`);
}
