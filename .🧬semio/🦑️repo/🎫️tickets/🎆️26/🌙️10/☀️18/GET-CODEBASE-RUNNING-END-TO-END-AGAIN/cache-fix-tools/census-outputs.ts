import { readFileSync } from "node:fs";
import { load } from "./overlay.ts";
const WS = "/Users/ueli/Documents/semio", C = process.argv[2]!;
const list = readFileSync(`${C}/🗑️generated/cachefix-x/pj-list.txt`, "utf8").split("\n").filter(Boolean);
const p = await load("staged");
let missing = 0; const rows: string[] = [];
for (const file of list) { let j: any; try { j = JSON.parse(readFileSync(`${WS}/${file}`, "utf8")); } catch { continue; }
  for (const [name, t] of Object.entries<any>(j.targets ?? {})) {
    const base = { ...(p.POLICY.targetDefaults?.[name] ?? {}), ...t, options: t.options ?? {} };
    const n = p.module.cachedOutputsAdmittedV1(p.module.targetPolicy(name, structuredClone(base), p.POLICY));
    if (n.cache === true && !Array.isArray(t.outputs)) { missing++; rows.push(`${j.name}:${name}`); }
  } }
console.log("cacheable without outputs array in the manifest:", missing); console.log(rows.join("\n"));
