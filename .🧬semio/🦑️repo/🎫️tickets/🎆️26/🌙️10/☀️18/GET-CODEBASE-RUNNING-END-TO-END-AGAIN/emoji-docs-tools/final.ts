import { blocksOf } from "./docs.ts";
import { planFile } from "./plan.ts";
const root = "/Users/ueli/Documents/semio";
const out = process.argv[2];
let total = 0, withEmoji = 0, files = 0;
const cat: Record<string, number> = {};
const perTree: Record<string, number> = {};
const left: string[] = [];
for (const s of ["s1", "s2"]) {
  const scan = JSON.parse(await Bun.file(`${out}/${s}/scan.json`).text());
  for (const [p, v] of Object.entries<any>(scan)) {
    total += v.total; withEmoji += v.total - v.missing; files++;
    if (!v.missing) continue;
    const text = await Bun.file(`${root}/${p}`).text();
    const gen = planFile(p, text);
    const blocks = blocksOf(p, text).filter((b) => !b.startEmoji);
    const plannable = gen.edits.length;
    const k = gen.generated ? "generated-skipped" : "other";
    cat[k] = (cat[k] ?? 0) + blocks.length;
    for (const [kk, vv] of Object.entries(gen.skipped)) cat["skip:" + kk] = (cat["skip:" + kk] ?? 0) + vv;
    if (plannable) { cat["PLANNABLE"] = (cat["PLANNABLE"] ?? 0) + plannable; left.push(p + " " + plannable); }
    const tree = p.split("/").slice(0, 2).join("/");
    perTree[tree] = (perTree[tree] ?? 0) + blocks.length;
  }
}
console.log({ files, total, withEmoji, missing: total - withEmoji });
console.log(cat);
console.log(perTree);
console.log(left.slice(0, 20));
