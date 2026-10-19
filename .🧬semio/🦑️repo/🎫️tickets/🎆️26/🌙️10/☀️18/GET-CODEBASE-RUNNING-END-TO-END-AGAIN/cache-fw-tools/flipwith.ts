import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, projectFiles, readJson } from "./lib.ts";
const plan = JSON.parse(readFileSync(join(import.meta.dir, "../🗑️generated/cache-fw-plan.json"), "utf8"));
const flips = new Set(plan.changed.filter((c: any) => c[1] !== "generator-contract" && !/inputs/.test(c[2])).map((c: any) => c[0]));
let n = 0; const uniq = new Map<string, string[]>();
for (const f of projectFiles) { const j = readJson(f); for (const [name, t] of Object.entries<any>(j.targets ?? {})) { const id = j.name + ":" + name; if (flips.has(id) && t.inputs) { n++; const k = JSON.stringify(t.inputs); (uniq.get(k) ?? uniq.set(k, []).get(k)!).push(id); } } }
console.log(n, "flips with owner inputs;", uniq.size, "distinct input lists");
for (const [k, v] of [...uniq].sort((a, b) => b[1].length - a[1].length).slice(0, 25)) console.log(v.length, k.slice(0, 260), "|", v[0]);
