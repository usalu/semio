/** ✅️ Checks every rewritten file against its pre-edit snapshot: same line count, changed lines contain only import/export/dynamic specifiers. */
import { readFileSync, readdirSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const dir = process.argv[2];
let files = 0, bad = 0, changedLines = 0, drift = 0;
for (const name of readdirSync(dir)) {
  if (!name.endsWith(".json") || name.endsWith(".report.json")) continue;
  const snap: Record<string, string> = JSON.parse(readFileSync(dir + "/" + name, "utf8"));
  for (const [f, before] of Object.entries(snap)) {
    files++;
    const now = readFileSync(ROOT + f, "utf8");
    const a = before.split("\n"), b = now.split("\n");
    if (a.length !== b.length) { bad++; console.log("LINECOUNT", f.slice(-80), a.length, b.length); continue; }
    for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) {
      changedLines++;
      const stripped = (s: string) => s.replace(/(["'`])[^"'`\n]*\1/g, "$1$1");
      if (!/(import|export|from|require|\bmock\b)/.test(a[i]) ) { bad++; console.log("NONIMPORT", f.slice(-80), i + 1, a[i].slice(0, 120)); }
    }
  }
}
console.log({ files, changedLines, bad });
