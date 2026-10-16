/** ✅️ Checks every rewritten file against its pre-edit snapshot: same line count, changed lines are import/export/dynamic-import lines, package.json differs only in dependency sections. Usage: bun verify.ts <snapshot-dir> */
import { readFileSync, readdirSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const dir = process.argv[2];
let files = 0, bad = 0, changedLines = 0, manifests = 0;
for (const name of readdirSync(dir)) {
  if (!name.endsWith(".json")) continue;
  const snap: Record<string, string> = JSON.parse(readFileSync(dir + "/" + name, "utf8"));
  for (const [f, before] of Object.entries(snap)) {
    const now = readFileSync(ROOT + f, "utf8");
    if (f.endsWith("/package.json")) {
      manifests++;
      const a = JSON.parse(before), b = JSON.parse(now);
      for (const k of new Set([...Object.keys(a), ...Object.keys(b)])) if (k !== "dependencies" && k !== "devDependencies" && JSON.stringify(a[k]) !== JSON.stringify(b[k])) { bad++; console.log("MANIFEST-KEY", f.slice(-80), k); }
      for (const k of ["dependencies", "devDependencies"]) for (const [d, v] of Object.entries(a[k] ?? {})) if (b[k]?.[d] !== v) { bad++; console.log("MANIFEST-DEP-LOST", f.slice(-80), k, d); }
      continue;
    }
    files++;
    const x = before.split("\n"), y = now.split("\n");
    if (x.length !== y.length) { bad++; console.log("LINECOUNT", f.slice(-80), x.length, y.length); continue; }
    for (let i = 0; i < x.length; i++) if (x[i] !== y[i]) {
      changedLines++;
      if (!/(import|export|\bfrom\b|require)/u.test(x[i])) { bad++; console.log("NONIMPORT", f.slice(-80), i + 1, x[i].slice(0, 120)); }
    }
  }
}
console.log({ files, manifests, changedLines, bad });
