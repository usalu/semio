// 🔎 Lists unrewritten indexed references of lifted scripts whose line also reads or parses source text: candidates for a manual source-read re-point. usage: bun review-reads.ts
import {readFileSync} from "node:fs";
import {classifyRef, refsOf} from "./refs-lib.ts";
import {LIB, REPO, TOOLS} from "./vfs.ts";
import {liftList} from "./plans/lifts.ts";
const done = JSON.parse(readFileSync(`${TOOLS}/staged/ops.json`, "utf8")).ops as {path: string; kind: string; sha?: string}[];
void done;
for (const target of liftList()) {
  for (const r of refsOf(target)) {
    const kind = classifyRef(r);
    if (kind === "IMPORT" || kind === "SPAWN" && false) continue;
    if (!/\.(ts|tsx|mjs)$/u.test(r.file)) continue;
    const line = readFileSync(`${REPO}/${r.file}`, "utf8").split("\n")[r.line - 1] ?? "";
    if (kind !== "OTHER" && kind !== "READ?" && kind !== "SPAWN") continue;
    const reads = /readFile|createSourceFile|Bun\.file|\.text\(\)|readFileSync|read\(/u.test(line);
    if (!reads && kind !== "OTHER") continue;
    const at = line.indexOf(r.spec);
    console.log(`${kind.padEnd(5)} ${target.replace(LIB, "L").slice(-48)} <- ${r.file.replace(LIB, "L").slice(-70)}:${r.line} ${reads ? "[reads]" : ""} ${line.slice(Math.max(0, at - 60), at + r.spec.length + 20).trim()}`);
  }
}
