/** 🔎️ PW-03 cross-scale probe: where the silhouettes of two scales' bodies differ in a view, with the solids responsible (owned raster of the volume tools).
 * Usage: `repo tools exec bun -- pw03-cross-probe.ts <species id>:<fine>:<coarse>:<view> ...` */
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { Mask, distanceField, gridFor, meshMask, meshesAt, projectWorld, resolveList, type Json, type View } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-lib.ts";

for (const spec of process.argv.slice(2)) {
const [id, fine, coarse, view] = spec.split(":") as [string, string, string, View];
console.log(`== ${spec}`);
const asset: Json = JSON.parse(readFileSync(join(import.meta.dir, "../../🗑️generated/PW-03", id, "candidate.json"), "utf8")).asset;
const lists = [fine, coarse].map((s) => meshesAt(asset, Number(s), resolveList(asset.volumes[s], "body")));
const pts = lists.flat().flatMap((m) => Array.from({ length: m.world.length / 3 }, (_, i) => projectWorld(view, [m.world[i * 3]!, m.world[i * 3 + 1]!, m.world[i * 3 + 2]!])));
const box0 = { minX: Math.min(...pts.map((p) => p.x)), minY: Math.min(...pts.map((p) => p.y)), maxX: Math.max(...pts.map((p) => p.x)), maxY: Math.max(...pts.map((p) => p.y)) };
const { box, cell } = gridFor(box0, Math.max(box0.maxX - box0.minX, box0.maxY - box0.minY), 640);
const [a, b] = lists.map((l) => meshMask(view, l, box, cell));
const edge = (m: Mask): Mask => {
  const e = new Mask(box, cell);
  for (let y = 0; y < m.height; y++) for (let x = 0; x < m.width; x++) {
    const i = y * m.width + x;
    if (m.cells[i] && (x === 0 || y === 0 || x === m.width - 1 || y === m.height - 1 || !m.cells[i - 1] || !m.cells[i + 1] || !m.cells[i - m.width] || !m.cells[i + m.width])) e.cells[i] = 1;
  }
  return e;
};
const [ea, eb] = [edge(a!), edge(b!)];
for (const [name, from, to, list] of [["fine→coarse", ea, eb, lists[0]!], ["coarse→fine", eb, ea, lists[1]!]] as const) {
  const field = distanceField(to);
  const far: { x: number; y: number; d: number }[] = [];
  let n = 0;
  for (let i = 0; i < from.cells.length; i++) if (from.cells[i]) {
    n++;
    if (field[i]! > 0.1) far.push({ x: box.minX + ((i % from.width) + 0.5) * cell, y: box.minY + (Math.floor(i / from.width) + 0.5) * cell, d: field[i]! });
  }
  const ds = [...from.cells.keys()].filter((i) => from.cells[i]).map((i) => field[i]!).sort((p, q) => p - q);
  console.log(`[DEBUG] ${name}: ${n} edge cells, ${far.length} beyond 0.1 m, p95 ${ds[Math.floor(0.95 * ds.length)]!.toFixed(3)}`);
  const buckets: Record<string, number> = {};
  for (const p of far) {
    let best = "", bd = Infinity;
    for (const m of list) for (let k = 0; k < m.world.length; k += 9) {
      const q = projectWorld(view, [m.world[k]!, m.world[k + 1]!, m.world[k + 2]!]);
      const dd = Math.hypot(q.x - p.x, q.y - p.y);
      if (dd < bd) [bd, best] = [dd, m.id];
    }
    buckets[best] = (buckets[best] ?? 0) + 1;
  }
  console.log(`  ${Object.entries(buckets).sort((p, q) => q[1] - p[1]).slice(0, 25).map(([k, v]) => `${k}:${v}`).join(" ")}`);
}
}
