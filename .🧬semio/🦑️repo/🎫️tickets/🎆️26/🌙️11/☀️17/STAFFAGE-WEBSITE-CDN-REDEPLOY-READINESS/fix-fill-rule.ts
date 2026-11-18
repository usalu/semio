/** Unions the overlapping subpaths of the filled 1:500 elevation shapes of jogger, leaning-person and person-with-umbrella (shapely, through the staffage oracle environment) and writes the result back into the asset documents. */
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { flattenPath } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { python } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🐍️.ts";

const ROOT = `${process.cwd()}/🎓️teaching/🏛️architecture/🧍️staffage/`;
const WORK = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const targets: Record<string, string[]> = { jogger: ["figure"], "leaning-person": ["figure", "forearm-l", "forearm-r"], "person-with-umbrella": ["forearm-r"] };
const f = (v: number) => (Math.round(v * 1e4) / 1e4).toString();
const shapes: any[] = [];
for (const [slug, ids] of Object.entries(targets)) {
  const file = `${ROOT}🗂️catalog/👤️humans/${slug}/🔣️.json`;
  const asset = JSON.parse(readFileSync(file, "utf8")).asset;
  for (const s of asset.representations["500"].geometry.elevation) if (ids.includes(s.id)) shapes.push({ slug, id: s.id, d: s.d, rings: flattenPath(s.d).rings.map((r: any[]) => r.map((p) => [p.x, p.y])) });
}
writeFileSync(`${WORK}rings-in.json`, JSON.stringify(shapes));
writeFileSync(`${WORK}union.py`, `import json,sys
from shapely.geometry import Polygon
from shapely.ops import unary_union
out=[]
for s in json.load(open(sys.argv[1],encoding="utf-8")):
    u=unary_union([Polygon(r).buffer(0) for r in s["rings"] if len(r)>=3]).simplify(0.0003,preserve_topology=True)
    polys=list(u.geoms) if u.geom_type=="MultiPolygon" else [u]
    rings=[]
    for p in polys:
        rings.append(list(p.exterior.coords)[:-1])
        rings+= [list(i.coords)[:-1] for i in p.interiors]
    out.append({"slug":s["slug"],"id":s["id"],"d":s["d"],"rings":rings})
json.dump(out,open(sys.argv[2],"w"))
`);
python(`${WORK}union.py`, "geometry", [`${WORK}rings-in.json`, `${WORK}rings-out.json`]);
const result: any[] = JSON.parse(readFileSync(`${WORK}rings-out.json`, "utf8"));
for (const r of result) {
  const file = `${ROOT}🗂️catalog/👤️humans/${r.slug}/🔣️.json`;
  const text = readFileSync(file, "utf8");
  const old = JSON.stringify(r.d);
  const start = text.indexOf('"500": {', text.indexOf('"representations"'));
  const at = text.indexOf(old, start);
  if (start < 0 || at < 0) throw new Error(`${r.slug}/${r.id}: old path not found`);
  const d = r.rings.map((ring: number[][]) => `M ${ring.map(([x, y]) => `${f(x)} ${f(y)}`).join(" L ")} Z`).join(" ");
  writeFileSync(file, text.slice(0, at) + JSON.stringify(d) + text.slice(at + old.length));
  console.log(r.slug, r.id, r.rings.length, "rings");
}
