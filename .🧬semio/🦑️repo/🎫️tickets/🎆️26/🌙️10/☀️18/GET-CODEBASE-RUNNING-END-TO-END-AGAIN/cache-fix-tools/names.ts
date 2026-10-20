import { readFileSync } from "node:fs";
const WS = "/Users/ueli/Documents/semio", C = process.argv[2];
const list = readFileSync(`${C}/🗑️generated/cachefix-x/pj-list.txt`, "utf8").split("\n").filter(Boolean);
const out: Record<string, { n: number; cacheFalse: number; projects: string[] }> = {};
for (const file of list) { let j: any; try { j = JSON.parse(readFileSync(`${WS}/${file}`, "utf8")); } catch { continue; }
  for (const [name, t] of Object.entries<any>(j.targets ?? {})) { const e = (out[name] ??= { n: 0, cacheFalse: 0, projects: [] }); e.n++; if (t.cache === false) e.cacheFalse++; if (e.projects.length < 3) e.projects.push(file.slice(0, 60)); } }
await Bun.write(`${C}/🗑️generated/cachefix-x/names.json`, JSON.stringify(out, null, 1));
console.log(Object.keys(out).length, "distinct names");
