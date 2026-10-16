/** 🧮️ Per open target owner: TS package, link status in node_modules, existing exports, other-language packages. Usage: bun groups.ts <policy.json> <scan.json> */
import { readFileSync, existsSync, lstatSync } from "node:fs";
import { loadPolicy, groupsOf } from "./model.ts";
import { edgeSkip } from "./skip.ts";
const policy = loadPolicy(process.argv[2]);
const edges = JSON.parse(readFileSync(process.argv[3], "utf8")).filter((e: any) => !edgeSkip(e));
const groups = new Map(groupsOf(policy).map(g => [g.sourceOwner, g]));
const cnt = new Map<string, number[]>();
for (const e of edges) { const v = cnt.get(e.sourceOwner) ?? [0, 0]; v[e.test ? 1 : 0]++; cnt.set(e.sourceOwner, v); }
const R = "/Users/ueli/Documents/semio/";
for (const [k, v] of [...cnt].sort((a, b) => b[1][0] + b[1][1] - a[1][0] - a[1][1])) {
  const g = groups.get(k)!;
  const ts = g.tsPackage;
  const link = ts ? (existsSync(R + "node_modules/" + ts.name) ? "linked" : "UNLINKED") : "-";
  console.log(`${v.join("/")}\t${k.replace(/🧰️framework\/🔨️modules\//g, "F/M/").replace(/🧰️framework\/🛍️products\//g, "F/P/")}\tpkgs=${g.packages.map(p => p.name + "@" + p.owner.slice(k.length) ).join(",")}\tts=${ts ? ts.name : "NONE"} ${link} exports=${ts ? ts.exports.length : 0}`);
}
