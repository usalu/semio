/** 🌲️ Model-against-reference measurements of a tree at one scale in its default pose (catalog asset with the volumes source merged): overall height, spread along x and z, clear stem (lowest foliage), stem diameter at breast height (1.3 m) and at the ground, crown widest height, crown-to-height ratio.
 * Usage: `bun r5-trees-measure.ts <category/slug> <scale> <wood material prefix> <leaf material prefix>`
 * @see ./r5-brief.md */
import { load, meshesIn, rigAt } from "./r3-lib.ts";

const [spec, scale, woodKey = "bark", leafKey = "leaf"] = process.argv.slice(2);
const { asset } = load(spec, "r5-trees-measure.ts <category/slug> <scale> [wood] [leaf]");
const s = Number(scale);
const frames = rigAt(asset, s, { kind: "pose", pose: asset.presentation.defaultPose }).frames;
const body = asset.volumes[scale!].body as any[];
const meshes = meshesIn(asset, frames, body, s);
const isLeaf = (id: string): boolean => (body.find((b) => b.id === id)!.material as string).includes(leafKey);
const isWood = (id: string): boolean => (body.find((b) => b.id === id)!.material as string).includes(woodKey);
const box = (pred: (id: string) => boolean, yLo = -Infinity, yHi = Infinity) => {
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (const m of meshes) if (pred(m.id)) for (let i = 0; i < m.world.length; i += 3) {
    const y = m.world[i + 1]!;
    if (y < yLo || y > yHi) continue;
    for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, m.world[i + k]!); hi[k] = Math.max(hi[k]!, m.world[i + k]!); }
  }
  return { lo, hi };
};
const all = box(() => true), leaf = box(isLeaf);
const f = (x: number): string => x.toFixed(3);
console.log(`height ${f(all.hi[1]!)} m, spread x ${f(all.hi[0]! - all.lo[0]!)} m, z ${f(all.hi[2]! - all.lo[2]!)} m`);
console.log(`lowest foliage ${f(leaf.lo[1]!)} m, foliage height ${f(leaf.hi[1]! - leaf.lo[1]!)} m`);
for (const y of [0.05, 1.3]) {
  const b = box((id) => isWood(id) && /stem|trunk|flare|collar/.test(id) && !/root|spur|buttress/.test(id), y - 0.05, y + 0.05);
  console.log(`stem at ${y} m: ${f(b.hi[0]! - b.lo[0]!)} x ${f(b.hi[2]! - b.lo[2]!)} m`);
}
let best = [0, 0];
for (let y = leaf.lo[1]!; y < leaf.hi[1]!; y += 0.1) {
  const b = box(isLeaf, y - 0.05, y + 0.05);
  const w = Math.max(b.hi[0]! - b.lo[0]!, b.hi[2]! - b.lo[2]!);
  if (w > best[1]!) best = [y, w];
}
console.log(`crown widest ${f(best[1]!)} m at ${f(best[0]!)} m (${(best[0]! / all.hi[1]!).toFixed(2)} H); height to spread ${(all.hi[1]! / Math.max(all.hi[0]! - all.lo[0]!, all.hi[2]! - all.lo[2]!)).toFixed(2)}`);
