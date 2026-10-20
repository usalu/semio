import { readFileSync } from "node:fs";
const WS = "/Users/ueli/Documents/semio", C = process.argv[2]!;
const list = readFileSync(`${C}/🗑️generated/cachefix-x/pj-list.txt`, "utf8").split("\n").filter(Boolean);
const nx = JSON.parse(readFileSync(`${WS}/nx.json`, "utf8")).namedInputs;
const policy = JSON.parse(readFileSync(`${WS}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`, "utf8"));
const wsPolicy = JSON.parse(readFileSync(`${WS}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`, "utf8"));
console.log("policy.targetDefaults keys", Object.keys(policy.targetDefaults), "wsPolicy keys", Object.keys(wsPolicy));
const used: Record<string, Set<string>> = {};
for (const file of list) { let j: any; try { j = JSON.parse(readFileSync(`${WS}/${file}`, "utf8")); } catch { continue; }
  const declared = new Set([...Object.keys(nx), ...Object.keys(j.namedInputs ?? {}), ...Object.keys(j.metadata?.sourceInputs ?? {}), ...Object.keys(wsPolicy.namedInputs ?? {})]);
  for (const [name, t] of Object.entries<any>(j.targets ?? {})) for (const i of t.inputs ?? []) if (typeof i === "string" && /^\^?[A-Za-z][A-Za-z0-9_-]*$/.test(i)) { const w = i.replace(/^\^/, ""); if (!declared.has(w)) (used[w] ??= new Set()).add(`${j.name}:${name}`); } }
for (const [w, s] of Object.entries(used)) console.log(w, s.size, [...s].slice(0, 3).join(","));
