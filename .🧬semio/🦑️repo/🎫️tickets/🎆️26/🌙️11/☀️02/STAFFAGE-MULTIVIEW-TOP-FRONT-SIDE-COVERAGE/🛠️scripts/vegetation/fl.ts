import { readFileSync } from "node:fs";
const f = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const rows = Object.entries<any>(f.assets ?? f).filter(([k]) => k.startsWith("vegetation"));
for (const [k, v] of rows) { console.log(k, JSON.stringify(v.codes ?? {})); }
const seen = new Set<string>();
for (const [k, v] of rows) for (const i of v.issues ?? []) { const key = i.code; if (seen.has(key + k.slice(0, 14))) continue; seen.add(key + k.slice(0, 14)); console.log("  ", k, i.code, JSON.stringify(i).slice(0, 330)); }
