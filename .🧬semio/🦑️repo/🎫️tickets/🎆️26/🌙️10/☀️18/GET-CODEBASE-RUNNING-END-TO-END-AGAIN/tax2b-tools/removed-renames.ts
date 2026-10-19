// 🔎 Finds HEAD-tracked paths that vanished through tax2's two rename shapes and have a successor on disk. usage: bun removed-renames.ts <head-files.txt> <out.tsv>
import {existsSync, readFileSync, writeFileSync} from "node:fs";
const R = "/Users/ueli/Documents/semio/";
const rows: string[] = [];
for (const p of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  if (p.startsWith(".🧬semio/") || existsSync(R + p)) continue;
  let m = /^(.*\/)🟦️\.test\.(tsx?)$/u.exec(p);
  if (m && existsSync(R + m[1] + "🟦️." + m[2])) { rows.push(`${p}\t${m[1]}🟦️.${m[2]}`); continue; }
  m = /^(.*\/)([^/]+)\.(tsx?)$/u.exec(p);
  if (m && !m[2]!.startsWith("🟦️") && existsSync(R + m[1] + m[2] + "/🟦️." + m[3])) rows.push(`${p}\t${m[1]}${m[2]}/🟦️.${m[3]}`);
}
writeFileSync(process.argv[3]!, rows.join("\n") + "\n");
console.log(rows.length);
