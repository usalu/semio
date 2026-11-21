/** 📐 Round 5 appliances (E03): world extents of the whole body and of chosen body solids at every authored scale in the default pose (catalog asset with the volumes source merged), for the model-vs-reference tables of the reports.
 * Usage: `bun r5-appliances-measure.ts <category/slug> [solid-id ...]`
 * @see ./r5-brief.md */
import { load, meshesIn, rigAt } from "./r3-lib.ts";

const [spec, ...ids] = process.argv.slice(2);
const { asset } = load(spec, "bun r5-appliances-measure.ts <category/slug> [solid-id ...]");
const box = (meshes: { world: ArrayLike<number> }[]): number[] => {
  const b = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) { b[k] = Math.min(b[k]!, m.world[i + k]!); b[k + 3] = Math.max(b[k + 3]!, m.world[i + k]!); }
  return b;
};
const fmt = (b: number[]): string => `x ${(b[3]! - b[0]!).toFixed(4)} [${b[0]!.toFixed(4)}, ${b[3]!.toFixed(4)}]  y ${(b[4]! - b[1]!).toFixed(4)} [${b[1]!.toFixed(4)}, ${b[4]!.toFixed(4)}]  z ${(b[5]! - b[2]!).toFixed(4)} [${b[2]!.toFixed(4)}, ${b[5]!.toFixed(4)}]`;
for (const scale of ["50", "100", "200", "500"]) {
  const volume = asset.volumes[scale];
  if (volume?.status !== "authored") continue;
  const frames = rigAt(asset, Number(scale), { kind: "pose", pose: asset.presentation.defaultPose }).frames;
  const body = volume.body.filter((s: { texture?: boolean }) => !s.texture);
  console.log(`1:${scale} body ${fmt(box(meshesIn(asset, frames, body, Number(scale))))}`);
  for (const id of ids) {
    const solid = body.find((s: { id: string }) => s.id === id);
    if (solid) console.log(`  ${id.padEnd(18)} ${fmt(box(meshesIn(asset, frames, [solid], Number(scale))))}`);
  }
}
