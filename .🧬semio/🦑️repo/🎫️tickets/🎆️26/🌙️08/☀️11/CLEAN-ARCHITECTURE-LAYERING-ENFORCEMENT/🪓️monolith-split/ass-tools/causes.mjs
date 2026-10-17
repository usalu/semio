/** 🧮️ Groups load-failure messages of a vitest summary: `node causes.mjs <phase>`. */
import { readFileSync } from "node:fs";
const s = JSON.parse(readFileSync(`${process.env.G}/ass-sum-${process.argv[2]}.json`, "utf8"));
for (const [label, v] of Object.entries(s)) {
  if (!v.report) continue;
  const groups = {};
  for (const [f, r] of Object.entries(v.perFile)) if (!r.loaded) {
    const k = (r.loadError ?? "").replace(/from ".*"/u, "").slice(0, 140);
    (groups[k] ??= []).push(f.split("/").slice(-2).join("/"));
  }
  const keys = Object.keys(groups);
  if (!keys.length) continue;
  console.log(`## ${label}`);
  for (const k of keys) console.log(`  ${String(groups[k].length).padStart(3)}  ${k}`);
}
