import { readdirSync, readFileSync, writeFileSync } from "fs";
import { join } from "path";
const root = process.cwd();
const s = readdirSync(root).find((n) => n.includes("✏️"));
function walk(d, depth = 0, acc = []) {
  if (depth > 10) return acc;
  let ents; try { ents = readdirSync(d, { withFileTypes: true }); } catch { return acc; }
  for (const e of ents) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = join(d, e.name);
    if (e.isFile() && (e.name.endsWith(".rs")) && /store|component|engine|kernel/i.test(e.name)) {
      const src = readFileSync(p, "utf8");
      if (/fn dispatch_text\b/.test(src)) {
        const lines = src.split("\n");
        for (let i = 0; i < lines.length; i++) {
          if (/fn dispatch_text\b/.test(lines[i])) {
            acc.push({ p, sig: lines.slice(i, i + 3).join("\n") });
          }
        }
      }
    }
    if (e.isDirectory()) walk(p, depth + 1, acc);
  }
  return acc;
}
const all = walk(join(root, s));
const fem = all.filter((x) => x.p.includes("fem"));
const puzzle = all.filter((x) => x.p.includes("puzzle"));
const lowpoly = all.filter((x) => x.p.includes("lowpoly") || x.p.includes("procedural"));
console.log("FEM", JSON.stringify(fem, null, 2));
console.log("PUZZLE", JSON.stringify(puzzle, null, 2));
writeFileSync(join(process.argv[2], "dispatch-sigs.json"), JSON.stringify({ fem, puzzle, allCount: all.length, sample: all.slice(0, 30) }, null, 2));
