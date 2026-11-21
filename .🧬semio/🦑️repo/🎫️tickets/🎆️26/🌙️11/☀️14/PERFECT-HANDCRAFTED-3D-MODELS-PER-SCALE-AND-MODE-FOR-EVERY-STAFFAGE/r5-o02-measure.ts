/** 📐 Round 5 O02: world extents (x ahead, y up, z lateral; metres) of the whole body and of named body solids of one asset at every authored scale in its default pose, for the model-against-reference tables.
 * Usage: `bun r5-o02-measure.ts <category/slug> [solid ...]`
 * @see ./r5-brief.md */
import { load, meshesIn, rigAt } from "./r3-lib.ts";

const [spec, ...names] = process.argv.slice(2);
const { asset } = load(spec, "bun r5-o02-measure.ts <category/slug> [solid ...]");
const box = (solids: readonly any[], frames: any): string => {
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (const m of meshesIn(asset, frames, solids)) for (let i = 0; i < m.world.length; i++) { const k = i % 3; lo[k] = Math.min(lo[k]!, m.world[i]!); hi[k] = Math.max(hi[k]!, m.world[i]!); }
  return [0, 1, 2].map((k) => `${"xyz"[k]} ${lo[k]!.toFixed(4)}..${hi[k]!.toFixed(4)} (${(hi[k]! - lo[k]!).toFixed(4)})`).join("  ");
};
for (const scale of ["50", "100", "200", "500"]) {
  const v = asset.volumes[scale];
  if (v?.status !== "authored") continue;
  const frames = rigAt(asset, Number(scale), { kind: "pose", pose: asset.presentation.defaultPose }).frames;
  console.log(`1:${scale} body ${v.body.length} solids  ${box(v.body, frames)}`);
  for (const n of names) {
    const s = v.body.filter((q: any) => q.id === n || q.id.startsWith(n + "*") || (n.endsWith("*") && q.id.startsWith(n.slice(0, -1))));
    if (s.length) console.log(`   ${n.padEnd(18)} ${box(s, frames)}`);
  }
}
