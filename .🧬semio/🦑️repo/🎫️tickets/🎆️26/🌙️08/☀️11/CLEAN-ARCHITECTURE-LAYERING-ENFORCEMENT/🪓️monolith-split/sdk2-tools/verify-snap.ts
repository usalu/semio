/** ✅️ Checks the files of one snapshot JSON against the live tree: same line count, every changed line is an import/export/require/mock line. Usage: bun verify-snap.ts <snap.json>... */
import { readFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
let files = 0, bad = 0, changed = 0;
for (const p of process.argv.slice(2)) {
  const snap: Record<string, string> = JSON.parse(readFileSync(p, "utf8"));
  for (const [f, before] of Object.entries(snap)) {
    files++;
    const a = before.split("\n"), b = readFileSync(ROOT + f, "utf8").split("\n");
    if (a.length !== b.length) { bad++; console.log("LINECOUNT", f.slice(-80), a.length, b.length); continue; }
    for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) { changed++; if (!/(import|export|from|require|\bmock\b)/.test(a[i])) { bad++; console.log("NONIMPORT", f.slice(-80), i + 1, a[i].slice(0, 120)); } }
  }
}
console.log({ files, changed, bad });
