import { readFileSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
const C = process.argv[2];
const rows = JSON.parse(readFileSync(`${C}/🗑️generated/cachefix-x/fallthrough.json`, "utf8"));
const want = new Map<string, { project: string; root: string; targets: Set<string> }>();
for (const r of rows) { if (!/-check$/.test(r.name)) continue; const e = want.get(r.file) ?? { project: r.project, root: dirname(r.file), targets: new Set() }; e.targets.add(r.name); want.set(r.file, e); }
const out = [...want.values()].map((e) => ({ project: e.project, root: e.root, targets: [...e.targets] }));
writeFileSync(`${C}/🗑️generated/cachefix-x/targets-check.json`, JSON.stringify(out, null, 1));
console.log(out.length, out.reduce((a, e) => a + e.targets.length, 0));
