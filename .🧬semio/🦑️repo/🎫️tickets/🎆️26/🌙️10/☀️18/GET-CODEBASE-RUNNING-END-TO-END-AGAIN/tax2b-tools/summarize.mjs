// 📊 Collapses per-area dependency-cruiser outputs to the two rules. usage: node summarize.mjs <tag> <out-edges.json>
import {readFileSync, readdirSync, writeFileSync} from "node:fs";
const C = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const RUN = `${C}/🗑️generated/tax2b`, tag = process.argv[2];
const isRule = (n) => /^no-cross-technology/u.test(n) || n === "ui-foundation-closure";
const edges = new Map(), perArea = {};
for (const f of readdirSync(RUN).filter((f) => f.startsWith(`nct-${tag}-`) && f.endsWith(".json")).sort()) {
  const r = JSON.parse(readFileSync(`${RUN}/${f}`, "utf8"));
  const mods = new Map(r.modules.map((m) => [m.source, m]));
  const area = f.slice(`nct-${tag}-`.length, -5);
  const counts = {"no-cross-technology": 0, "ui-foundation-closure": 0};
  for (const v of r.summary.violations) {
    if (v.rule.severity !== "error" || !isRule(v.rule.name)) continue;
    const dep = mods.get(v.from)?.dependencies.find((d) => d.resolved === v.to);
    const key = [v.rule.name, v.from, v.to].join("|");
    if (edges.has(key)) continue;
    edges.set(key, {area, rule: v.rule.name, from: v.from, to: v.to, specifier: dep?.module ?? null, dependencyTypes: dep?.dependencyTypes ?? []});
    counts[v.rule.name === "ui-foundation-closure" ? "ui-foundation-closure" : "no-cross-technology"]++;
  }
  perArea[area] = counts;
}
const list = [...edges.values()];
if (process.argv[3]) writeFileSync(process.argv[3], JSON.stringify(list));
const total = {"no-cross-technology": 0, "ui-foundation-closure": 0};
for (const c of Object.values(perArea)) { total["no-cross-technology"] += c["no-cross-technology"]; total["ui-foundation-closure"] += c["ui-foundation-closure"]; }
console.log(JSON.stringify({perArea, total, files: new Set(list.map((e) => e.from)).size}, null, 1));
