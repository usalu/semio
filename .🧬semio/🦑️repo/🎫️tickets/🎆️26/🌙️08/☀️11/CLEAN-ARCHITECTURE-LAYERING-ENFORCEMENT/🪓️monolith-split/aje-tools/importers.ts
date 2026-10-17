/** 🔭️ Lists every static or dynamic import whose resolved target starts with one of the given prefixes. Usage: bun importers.ts <prefix>... */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, scanSource } from "../sdkb-tools/lib.ts";
import { listSourceFiles, nfc } from "../psc3-tools/model.ts";
const prefixes = process.argv.slice(2).map(nfc);
const files = listSourceFiles();
let n = 0;
for (const f of files) {
  let text: string;
  try { text = readFileSync(join(ROOT, f), "utf8"); } catch { continue; }
  if (!/(?:from|import|require)\s*\(?\s*["'`]/u.test(text)) continue;
  for (const r of scanSource(f, text)) {
    if (!r.resolved) continue;
    const res = nfc(r.resolved);
    if (prefixes.some(p => res.startsWith(p))) { n++; console.log(`${f}:${r.line}\t${r.spec}\t${res}`); }
  }
}
console.error(`files=${files.length} hits=${n}`);
