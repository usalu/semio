/** 🔬️ Author 28: product silhouette IoU with elevation (or other view) shapes removed one by one. bun author-28-prod.ts <asset> <scale> [view] */
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, sc, view = "elevation"] = process.argv.slice(2);
const mod: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
const { entry, asset } = entryOf(spec!);
const m: any = merged(asset, loadSource(entry)!);
const run = (a: any) => { const r = mod.measureVolumes(a, { scale: Number(sc), rules: ["volumes-silhouette"] }); return Object.fromEntries(r.filter((x: any) => x.name.startsWith("iou:")).map((x: any) => [x.name.slice(4), +x.value.toFixed(3)])); };
console.log("base", JSON.stringify(run(m)));
const geo = m.representations[sc!].geometry[view];
for (let i = 0; i < geo.length; i++) {
  const c = structuredClone(m);
  c.representations[sc!].geometry[view].splice(i, 1);
  const r = run(c);
  console.log(geo[i].id, JSON.stringify(r));
}
if (process.env.OPEN) {
  const c = structuredClone(m);
  for (const s of c.representations[sc!].geometry[view]) if (s.filled === false && typeof s.d === "string") s.d = s.d.replace(/\s*Z\s*$/i, "");
  console.log("open-Z", JSON.stringify(run(c)));
  const c2 = structuredClone(m);
  const g = c2.representations[sc!].geometry[view];
  for (const s of g) if (s.id === "tyre-front" || s.id === "tyre-rear") console.log(s.id, JSON.stringify(s).slice(0, 400));
}
