import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
const root = "/Users/ueli/Documents/semio";
const dir = process.argv[2]!;
const barrels = new Set<string>((JSON.parse(readFileSync(join(dir, "barrels.json"), "utf8")) as any[]).map((b) => b.glue));
const reverted: { from: string; to: string }[] = [];
for (const n of ["edges0.plan-applied.json", "e1.plan-applied.json"]) {
  const d = JSON.parse(readFileSync(join(dir, n), "utf8"));
  for (const e of d.edits) {
    if (!e.after || e.after === "") continue;
    const glue = normalize(join(dirname(e.file), e.after));
    if (!barrels.has(glue)) continue;
    const path = join(root, e.file); if (!existsSync(path)) continue;
    let text = readFileSync(path, "utf8"), changed = false;
    for (const q of ['"', "'"]) { const needle = q + e.after + q; if (text.includes(needle)) { text = text.split(needle).join(q + e.before + q); changed = true; } }
    if (changed) { writeFileSync(path, text); reverted.push({ from: e.file, to: normalize(join(dirname(e.file), e.before)) }); }
  }
}
const uniq = [...new Map(reverted.map((r) => [r.from + "\0" + r.to, r])).values()];
writeFileSync(join(dir, "reverted.json"), JSON.stringify(uniq, null, 1));
console.log("reverted pairs", uniq.length);
