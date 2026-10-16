import fs from "node:fs";
const plan = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const org = JSON.parse(fs.readFileSync(process.argv[3], "utf8"));
const sdk = JSON.parse(fs.readFileSync(process.argv[4], "utf8"));
const sdkFiles = new Map(sdk.map(([sub, f]) => [f, sub]));
const res = [];
for (const o of plan) {
  const tgt = org[o.to] ?? {};
  const need = o.names.filter(n => n.name !== "default" && n.name !== "*");
  const covering = [];
  for (const [f, sub] of sdkFiles) {
    const e = org[f]; if (!e) continue;
    if (need.every(n => e[n.name] !== undefined && e[n.name] === tgt[n.name])) covering.push(sub);
  }
  const partial = new Map();
  for (const n of need) for (const [f, sub] of sdkFiles) { const e = org[f]; if (e && e[n.name] !== undefined && e[n.name] === tgt[n.name]) { const a = partial.get(n.name) ?? []; a.push(sub); partial.set(n.name, a); } }
  res.push({ to: o.to, need: need.map(n => n.name), covering, uncovered: need.filter(n => !partial.has(n.name)).map(n => n.name), missingInTarget: need.filter(n => tgt[n.name] === undefined).map(n => n.name) });
}
for (const r of res) console.log((r.covering.length ? "OK  " : (r.uncovered.length === r.need.length ? "NEW " : "PART")), r.to.replace("🧰️framework/🔨️modules/", "F/").slice(0, 100), "|", r.covering.slice(0, 3).join(" "), r.uncovered.length ? "| uncovered: " + r.uncovered.join(",") : "", r.missingInTarget.length ? "| NOT-EXPORTED-BY-TARGET: " + r.missingInTarget.join(",") : "");
fs.writeFileSync(process.argv[5], JSON.stringify(res, null, 1));
