/** 📏 Round 5 (E01 appliances): world extents (x, y, z in m) of the whole list and of chosen solids of one scale in the default pose, for the model-vs-reference tables of the reports.
 * Usage: `bun r5-e01-measure.ts <category/slug> <scale> [body|thermal|structure] [solid ids...]` */
import { load, meshesIn, rigAt } from "./r3-lib.ts";

const [spec, scale = "50", list = "body", ...ids] = process.argv.slice(2);
const { asset } = load(spec, "bun r5-e01-measure.ts <category/slug> <scale> [list] [ids...]");
const frames = rigAt(asset, Number(scale), { kind: "pose", pose: asset.presentation.defaultPose }).frames;
const solids = asset.volumes[scale][list] as { id: string }[];
const extent = (chosen: { id: string }[]) => {
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (const m of meshesIn(asset, frames, chosen, Number(scale))) for (let i = 0; i < m.world.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, m.world[i]!); hi[i % 3] = Math.max(hi[i % 3]!, m.world[i]!); }
  return lo.map((l, k) => `${"xyz"[k]} ${l.toFixed(4)}..${hi[k]!.toFixed(4)} (${(hi[k]! - l).toFixed(4)})`).join("  ");
};
console.log(`ALL ${list} 1:${scale}`.padEnd(28), extent(solids));
for (const id of ids) {
  const s = solids.filter((q) => q.id === id || q.id.startsWith(id + "-"));
  if (s.length) console.log(id.padEnd(28), extent(s));
  else console.log(id.padEnd(28), "missing");
}
