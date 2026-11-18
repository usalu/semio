/** 🔬 Where the drawing and the volume of one asset disagree: worst mismatch clusters per view at one scale, in the world frame of the view (paper-view x, y in metres), with the drawn shape or volume solid nearest.
 *  bun silhouette-diag.ts <asset> <scale> [view] */
import { readFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GATE, Mask, VIEWPOINTS, boundaryOf, distanceField, fillSmallHoles, extentOf, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";


const [name, scaleArg, only] = process.argv.slice(2) as [string, string, string | undefined, string | undefined];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const f = (n: number) => +n.toFixed(4);
for (const s of solids) {
  if (only !== undefined && !s.id.includes(only)) continue;
  const mn = [Infinity, Infinity, Infinity], mx = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < s.positions.length; i += 3) for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, s.positions[i + k]!); mx[k] = Math.max(mx[k]!, s.positions[i + k]!); }
  console.log(s.id.padEnd(24), "bone", s.bone.padEnd(10), "x", f(mn[0]!), f(mx[0]!), " y", f(mn[1]!), f(mx[1]!), " z", f(mn[2]!), f(mx[2]!));
}
