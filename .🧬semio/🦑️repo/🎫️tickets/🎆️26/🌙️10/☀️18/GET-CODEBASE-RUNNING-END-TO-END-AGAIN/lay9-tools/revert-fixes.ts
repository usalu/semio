import { readFileSync, writeFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio/";
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const needle = process.argv[3]!;
const keep: any[] = [];
const byFile = new Map<string, { specifier: string; replacement: string }[]>();
for (const f of plan.fixes) {
  if (f.from.includes(needle)) { const l = byFile.get(f.from) ?? []; l.push(f); byFile.set(f.from, l); } else keep.push(f);
}
let n = 0;
for (const [file, list] of byFile) {
  let s = readFileSync(ROOT + file, "utf8");
  for (const f of list) {
    for (const q of ['"', "'", "`"]) {
      const from = q + f.replacement + q, to = q + f.specifier + q;
      if (s.includes(from)) { s = s.split(from).join(to); n++; }
    }
  }
  writeFileSync(ROOT + file, s);
}
plan.fixes = keep;
writeFileSync(process.argv[2]!, JSON.stringify(plan, null, 1));
console.log("reverted", n, "in", byFile.size, "files; remaining fixes", keep.length);
