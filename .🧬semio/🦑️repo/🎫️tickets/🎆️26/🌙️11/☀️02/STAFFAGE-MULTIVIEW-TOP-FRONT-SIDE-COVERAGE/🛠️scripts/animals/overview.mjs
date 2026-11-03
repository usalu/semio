import fs from "node:fs";
const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/";
const cat = process.argv[2];
for (const slug of process.argv.slice(3)) {
  const a = JSON.parse(fs.readFileSync(`${root}${cat}/${slug}/🔣️.json`, "utf8")).asset;
  console.log("==", slug, JSON.stringify(a.dimensions), "bones", a.rig.bones.length, "poses", a.poses.map(p=>p.id).join(","), "clips", a.clips.map(c=>c.id+":"+c.keys.length).join(","));
  console.log(" symbols", a.symbols.map(s=>s.id).join(","));
  for (const [k, r] of Object.entries(a.representations)) {
    if (r.status === "authored") console.log(" ", k, "authored", r.validRange.minDenominator + "-" + r.validRange.maxDenominator, "el", r.geometry.elevation.length, "fr", r.geometry.front.length, "pl", r.geometry.plan.length, "solid", ["elevation","front","plan"].map(v=>r.geometry[v].filter(s=>s.solid).length).join("/"), "follow", ["front","plan"].map(v=>r.geometry[v].filter(s=>s.followTip).length).join("/"));
    else console.log(" ", k, "unavailable", JSON.stringify(r.alternatives));
  }
}
