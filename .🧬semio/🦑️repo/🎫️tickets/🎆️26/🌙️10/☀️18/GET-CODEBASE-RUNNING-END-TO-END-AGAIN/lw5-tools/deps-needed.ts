import { readFileSync, existsSync } from "node:fs";
import { dirname } from "node:path";
const R = "/Users/ueli/Documents/semio/";
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const names = (spec: string) => (spec.startsWith("@") ? spec.split("/").slice(0, 2).join("/") : spec.split("/")[0]!);
const out = new Map<string, Set<string>>();
for (const f of plan.fixes) {
  let d = dirname(R + f.from), pj = "";
  while (d.length >= R.length) { if (existsSync(d + "/package.json")) { pj = d + "/package.json"; break; } d = dirname(d); }
  const j = pj ? JSON.parse(readFileSync(pj, "utf8")) : {};
  const have = { ...j.dependencies, ...j.devDependencies, ...j.peerDependencies, ...j.optionalDependencies };
  const need = names(f.replacement);
  if (j.name === need || have[need]) continue;
  const key = pj.slice(R.length) || "(none)";
  (out.get(key) ?? out.set(key, new Set()).get(key)!).add(need);
}
console.log(JSON.stringify(Object.fromEntries([...out].map(([k, v]) => [k, [...v]])), null, 1));
