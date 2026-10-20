import { planFile } from "./plan.ts";
import { statSync } from "node:fs";
const out = process.argv[2];
const list: string[] = JSON.parse(await Bun.file(`${out}/${process.argv[3]}`).text());
let files = 0, edits = 0;
const rows: any[] = [];
for (const p of list) {
  const text = await Bun.file(`/Users/ueli/Documents/semio/${p}`).text();
  const r = planFile(p, text);
  if (r.edits.length) { files++; edits += r.edits.length; rows.push({ p, n: r.edits.length, age: Math.round((Date.now() - statSync(`/Users/ueli/Documents/semio/${p}`).mtimeMs) / 60000) }); }
}
console.log({ files, edits });
console.log(rows.slice(0, 40));
