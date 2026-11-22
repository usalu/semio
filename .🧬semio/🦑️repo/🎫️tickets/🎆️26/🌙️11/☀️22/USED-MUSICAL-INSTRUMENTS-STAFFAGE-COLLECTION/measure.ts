/** 📐️ Measures candidates of this ticket as the sheet tool places them: per non-key pose the bounding box of the 1:50 body solids, per authored scale the solid counts per list (expanded with `repeat`) and the bounding box in the default pose. Writes `🗑️generated/measure/<slug>.json` and prints one line per asset.
 *
 * `$BUN measure.ts <slug …>`
 *
 * @see ../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-lib.ts — `rigAt`, `listsIn`, the placement used here */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { expandedCount, loadAsset } from "../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-lib.ts";
import { listsIn, pointsOf, rigAt } from "../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-lib.ts";

const HERE = import.meta.dir;
const OUT = join(HERE, "🗑️generated/measure");
mkdirSync(OUT, { recursive: true });

/** 📦️ Axis-aligned box of the body meshes of one scale in one pose, rounded to millimetres. */
function box(asset: any, scale: number, pose: string): { min: number[]; max: number[]; size: number[]; counts: Record<string, number>; errors: number } {
  const rig = rigAt(asset, scale, { kind: "pose", pose });
  const lists = listsIn(asset, scale, rig.frames);
  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  for (const point of pointsOf(lists.body)) for (let axis = 0; axis < 3; axis++) {
    if (point[axis]! < min[axis]!) min[axis] = point[axis]!;
    if (point[axis]! > max[axis]!) max[axis] = point[axis]!;
  }
  const round = (value: number): number => Math.round(value * 1000) / 1000;
  return { min: min.map(round), max: max.map(round), size: max.map((value, axis) => round(value - min[axis]!)), counts: { body: expandedCount(lists.solids.body), thermal: expandedCount(lists.solids.thermal), structure: expandedCount(lists.solids.structure) }, errors: lists.errors.length };
}

for (const slug of process.argv.slice(2)) {
  const asset = loadAsset(join(HERE, "assets", slug, "🔣️.json")) as any;
  const poses = asset.poses.filter((pose: { key?: boolean }) => !pose.key).map((pose: { id: string }) => pose.id);
  const result = {
    slug,
    dimensions: asset.dimensions,
    poses: Object.fromEntries(poses.map((pose: string) => [pose, box(asset, 50, pose)])),
    scales: Object.fromEntries(["50", "100", "200", "500"].filter((scale) => asset.volumes[scale]?.status === "authored").map((scale) => [scale, box(asset, Number(scale), asset.presentation.defaultPose)])),
  };
  writeFileSync(join(OUT, `${slug}.json`), JSON.stringify(result, null, 1));
  console.log(`${slug}: ${poses.map((pose: string) => `${pose} ${result.poses[pose].size.join("×")}`).join("; ")}`);
}
