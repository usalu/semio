import { planFile } from "./plan.ts";
const root = "/Users/ueli/Documents/semio";
const out = process.argv[2];
const scan = JSON.parse(await Bun.file(`${out}/scan.json`).text());
const files = Object.entries<any>(scan).filter(([p, v]) => v.missing && !p.endsWith(".d.ts")).map(([p]) => p);
const rows: any[] = [];
const skipped: Record<string, number> = {};
const how: Record<string, number> = {};
let genBlocks = 0;
for (const p of files) {
  const text = await Bun.file(`${root}/${p}`).text();
  const r = planFile(p, text);
  for (const [k, v] of Object.entries(r.skipped)) skipped[k] = (skipped[k] ?? 0) + v;
  for (const e of r.edits) { how[e.how] = (how[e.how] ?? 0) + 1; rows.push({ p, ...e }); }
}
await Bun.write(`${out}/plan.json`, JSON.stringify(rows));
console.log("edits", rows.length, "\nhow", how, "\nskipped", skipped);
