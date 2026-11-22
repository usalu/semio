/** 🧭️ Re-solves the humerus pose offsets of `animals/goat` after the scapulae went onto the lateral rest placement.
 *
 * The target of every pose is the fore limb as it was authored: the same pose offsets on a blade that swings in the paper plane at the breadth of the shoulder joint.
 * The humerus gets the `angleDeg`, `abductionDeg` and `twistDeg` that put elbow, carpus, fetlock and hoof tip back on that target; the scapula keeps its swing on the ribs.
 * Usage: `bun rework-goat-resolve.ts` prints the drift before and after; `--apply` writes the offsets into the catalog asset.
 *
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts — `solveFrames`
 */
import { readFileSync, writeFileSync } from "node:fs";
import { solveFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/goat/🔣️.json";
type Offset = { angleDeg?: number; abductionDeg?: number; twistDeg?: number };
const document = JSON.parse(readFileSync(CATALOG, "utf8"));
const asset = document.asset ?? document;
const root = { x: 0, y: 0, angleDeg: 0 };
const planar = structuredClone(asset.rig);
for (const bone of planar.bones) {
  if (!bone.id.startsWith("scapula-")) continue;
  const tip = asset.rig.bones.find((b: { id: string }) => b.id === bone.id);
  bone.z = tip.z < 0 ? -0.075 : 0.075;
  delete bone.abductionDeg;
  bone.lengthM = Math.hypot(0.728 - 0.636, -0.044 - 0.105);
  const humerus = planar.bones.find((b: { id: string }) => b.id === bone.id.replace("scapula", "humerus"));
  humerus.x = bone.lengthM;
  delete humerus.abductionDeg;
  delete humerus.twistDeg;
  humerus.angleDeg = Math.atan2(-0.9078, -0.4194) * 180 / Math.PI - Math.atan2(-0.149, 0.092) * 180 / Math.PI;
  bone.angleDeg = Math.atan2(-0.149, 0.092) * 180 / Math.PI - Math.atan2(-0.1727, 0.985) * 180 / Math.PI;
}
const chain = (side: string): string[] => [`forearm-f${side}`, `metacarpus-f${side}`, `hoof-f${side}`];
const points = (rig: unknown, offsets: Record<string, Offset>, side: string): number[][] => {
  const solved = solveFrames(rig as never, offsets as never, root)!;
  const out = chain(side).map((id) => [...solved[id]!.frame.o]);
  const hoof = solved[`hoof-f${side}`]!.frame;
  out.push([0, 1, 2].map((i) => hoof.o[i]! + hoof.u[i]! * 0.0998));
  return out;
};
const drift = (a: number[][], b: number[][]): number => Math.max(...a.map((p, i) => Math.hypot(p[0]! - b[i]![0]!, p[1]! - b[i]![1]!, p[2]! - b[i]![2]!)));
const restDrift = Math.max(drift(points(asset.rig, {}, "n"), points(planar, {}, "n")), drift(points(asset.rig, {}, "f"), points(planar, {}, "f")));
process.stdout.write(`[DEBUG] rest: target chain differs from the rig by ${(restDrift * 1000).toFixed(3)} mm\n`);
for (const pose of asset.poses) {
  for (const side of ["n", "f"]) {
    const id = `humerus-f${side}`;
    if (pose.bones[`scapula-f${side}`] === undefined && pose.bones[id] === undefined) continue;
    const target = points(planar, { ...pose.bones, [id]: { angleDeg: pose.bones[id]?.angleDeg ?? 0 } }, side);
    const cost = (p: number[]): number => {
      const now = points(asset.rig, { ...pose.bones, [id]: { angleDeg: p[0], abductionDeg: p[1], twistDeg: p[2] } }, side);
      return now.reduce((sum, q, i) => sum + (q[0]! - target[i]![0]!) ** 2 + (q[1]! - target[i]![1]!) ** 2 + (q[2]! - target[i]![2]!) ** 2, 0);
    };
    const before = drift(points(asset.rig, pose.bones, side), target);
    let best = [pose.bones[id]?.angleDeg ?? 0, 0, 0];
    let value = cost(best);
    for (let step = 8; step > 1e-5; step /= 2) {
      for (let moved = true; moved; ) {
        moved = false;
        for (let k = 0; k < 3; k += 1) for (const sign of [1, -1]) {
          const trial = best.map((v, i) => (i === k ? v + sign * step : v));
          const c = cost(trial);
          if (c < value - 1e-16) [best, value, moved] = [trial, c, true];
        }
      }
    }
    const rounded = best.map((v) => Number(v.toFixed(4)));
    const offset: Offset = { angleDeg: rounded[0] };
    if (Math.abs(rounded[1]!) >= 0.005) offset.abductionDeg = rounded[1];
    if (Math.abs(rounded[2]!) >= 0.005) offset.twistDeg = rounded[2];
    const after = drift(points(asset.rig, { ...pose.bones, [id]: offset }, side), target);
    process.stdout.write(`[DEBUG] ${pose.id} ${id}: ${(before * 1000).toFixed(2)} mm -> ${(after * 1000).toFixed(2)} mm ${JSON.stringify(offset)}\n`);
    if (process.argv.includes("--apply")) pose.bones[id] = offset;
  }
}
if (process.argv.includes("--apply")) {
  const fresh = JSON.parse(readFileSync(CATALOG, "utf8"));
  (fresh.asset ?? fresh).poses = asset.poses;
  writeFileSync(CATALOG, readFileSync(CATALOG, "utf8").endsWith("\n") ? `${JSON.stringify(fresh, null, 2)}\n` : JSON.stringify(fresh, null, 2));
}
