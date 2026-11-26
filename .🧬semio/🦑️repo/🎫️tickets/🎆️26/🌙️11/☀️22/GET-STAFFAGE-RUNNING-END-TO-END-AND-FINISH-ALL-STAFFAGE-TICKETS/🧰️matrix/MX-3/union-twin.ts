/** 🪞️ The extruded-solid case of `solid3` seen along its sweep: `unionPolygons([low, high, ...sweeps])` with `low` and `high` the same ring (the profile placed at both ends of a sweep along the viewing axis) and the degenerate quads of its edges, for every flat shape with an extruded solid of a document: union-twin.ts <json> <scale>: prints the shapes whose union is smaller than the profile that shapely measures (area of the ring, simple polygon) */
import { readFileSync } from "node:fs";
import { unionPolygons } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
type P = { x: number; y: number };
const area = (r: readonly P[]): number => r.reduce((s, p, k) => s + p.x * r[(k + 1) % r.length]!.y - r[(k + 1) % r.length]!.x * p.y, 0) / 2;
const [file, scale] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
let bad = 0, total = 0;
for (const view of ["elevation", "front", "plan"]) for (const s of asset.representations[scale!].geometry[view] as any[]) {
  if (s.kind !== "poly" || s.solid?.kind !== "extruded" || s.points.length < 20) continue;
  total++;
  const ring: P[] = s.points.map(([x, y]: number[]) => ({ x, y }));
  const sweeps = ring.map((p, i) => [p, ring[(i + 1) % ring.length]!, ring[(i + 1) % ring.length]!, p]);
  let got = 0, note = "";
  try { got = unionPolygons([ring, ring, ...sweeps]).reduce((t, r) => t + area(r), 0); } catch (e) { note = String(e).slice(0, 100); }
  const want = Math.abs(area(ring));
  if (note || Math.abs(got) < want * 0.99) { bad++; console.log(`${view} ${s.id} (${ring.length} points): union ${Math.abs(got).toFixed(5)} against ring ${want.toFixed(5)} ${note}`); }
}
console.log(`bad ${bad} of ${total}`);
