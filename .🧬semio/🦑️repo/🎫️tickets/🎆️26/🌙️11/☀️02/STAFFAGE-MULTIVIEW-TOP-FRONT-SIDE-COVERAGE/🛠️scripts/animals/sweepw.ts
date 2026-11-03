import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
const root = join(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const ids = readFileSync(process.argv[2], "utf8").trim().split(/\s+/);
let total = 0;
for (const cat of readdirSync(root, { withFileTypes: true }).filter((d) => d.isDirectory())) {
  for (const id of ids) {
    const f = join(root, cat.name, id, "🔣️.json");
    let t: string; try { t = readFileSync(f, "utf8"); } catch { continue; }
    let n = 0;
    const r = t.replace(/^(\s*)"kind": "extruded",\r?\n(?!\s*"sweep")/gm, (m, ind) => { n++; return `${ind}"kind": "extruded",\n${ind}"sweep": "w",\n`; });
    if (n) writeFileSync(f, r); total += n; console.log(id, n);
  }
}
console.log("TOTAL", total);
