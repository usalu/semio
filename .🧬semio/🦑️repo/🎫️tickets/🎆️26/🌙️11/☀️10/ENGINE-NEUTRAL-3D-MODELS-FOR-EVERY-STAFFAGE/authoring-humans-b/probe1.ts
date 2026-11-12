import { readFileSync } from "node:fs";
import { tessellateSolid } from "../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
const v = JSON.parse(readFileSync("../volumes/humans/standing-adult.json", "utf8")).volume.solids;
for (const id of ["hoodie", "head"]) {
  const s = v.find((x: any) => x.id === id);
  const g = tessellateSolid({ ...s, node: s.bone } as never, { chordM: 0.002, minSegments: 12, maxSegments: 64 });
  const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9];
  for (let i = 0; i < g.positions.length; i += 3) for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, g.positions[i + k]!); mx[k] = Math.max(mx[k]!, g.positions[i + k]!); }
  console.log(id, g.positions.length / 3, g.indices.length / 3, mn, mx);
}
