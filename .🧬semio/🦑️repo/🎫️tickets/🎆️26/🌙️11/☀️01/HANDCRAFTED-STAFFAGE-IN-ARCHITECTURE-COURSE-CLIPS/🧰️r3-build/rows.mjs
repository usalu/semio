import { readFileSync, writeFileSync } from "node:fs";
const cat = process.argv[2];
const ids = process.argv.slice(3);
const rows = ids.map((id) => {
  const d = JSON.parse(readFileSync(`${cat}/🧱️components/${id}/🔣️.json`, "utf8")).asset;
  return {
    id, category: d.category, title: d.title, heightM: d.dimensions.heightM, widthM: d.dimensions.widthM,
    poses: d.poses.map((p) => p.id), clips: d.clips.map((c) => c.id),
    representations: Object.fromEntries(Object.entries(d.representations).map(([k, v]) => [k, v.status])),
    symbols: d.symbols.map((s) => s.id), analytics: ["thermal", "structural"], exports: d.exports,
    distinction: d.summary, completion: d.completion,
  };
});
writeFileSync(process.env.OUT, JSON.stringify(rows, null, 2));
