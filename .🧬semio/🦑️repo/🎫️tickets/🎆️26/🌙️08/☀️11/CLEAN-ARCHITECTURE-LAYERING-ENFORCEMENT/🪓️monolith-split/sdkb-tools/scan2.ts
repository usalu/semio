/** 🔭️ AST scan: every plugin import that resolves into 🧰️framework and is not an SDK export target. Read-only.
 * Usage: bun scan2.ts <files.txt> <out.json>
 */
import { readFileSync, writeFileSync } from "node:fs";
import { ROOT, scanSource, sdkExportTargets, isFrameworkTarget, readLines } from "./lib.ts";
const [list, out] = process.argv.slice(2);
const allowed = sdkExportTargets();
const files = readLines(list).map(f => f.startsWith("/") ? f.slice(ROOT.length + 1) : f);
const recs: any[] = [];
let total = 0, ok = 0;
for (const f of files) {
  let text: string;
  try { text = readFileSync(ROOT + "/" + f, "utf8"); } catch { continue; }
  for (const r of scanSource(f, text)) {
    if (!isFrameworkTarget(r.resolved)) continue;
    total++;
    if (allowed.has(r.resolved!)) { ok++; continue; }
    recs.push(r);
  }
}
writeFileSync(out, JSON.stringify(recs));
console.log(`files=${files.length} frameworkImports=${total} allowedByExisting=${ok} violating=${recs.length}`);
