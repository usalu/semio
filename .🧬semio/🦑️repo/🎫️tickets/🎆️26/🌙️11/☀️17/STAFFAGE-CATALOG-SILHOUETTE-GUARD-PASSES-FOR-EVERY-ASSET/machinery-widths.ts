/** 🔬 Where the drawing and the volume of one asset disagree: worst mismatch clusters per view at one scale, in the world frame of the view (paper-view x, y in metres), with the drawn shape or volume solid nearest.
 *  bun silhouette-diag.ts <asset> <scale> [view] */
import { readFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GATE, strokesOf, drawnFor, Mask, VIEWPOINTS, boundaryOf, distanceField, fillSmallHoles, extentOf, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

void readFileSync;
const [name, scaleArg, viewArg] = process.argv.slice(2) as [string, string, any];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const e = extentOf(solids);
console.log("extent min", e.min.map((x) => +x.toFixed(3)), "max", e.max.map((x) => +x.toFixed(3)), "dims", asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM, "feature", feature);
const drawnE = drawnFor(asset, state, viewArg, scale);
console.log("drawn", drawnE.map((d) => `${d.bone}:${d.closed ? "c" : "o"}${d.filled ? "f" : ""}:pen${d.penM.toFixed(3)}`).slice(0, 0));
const q = process.argv[5] ?? "";
for (const s of solids) {
  if (!s.slender || !s.id.includes(q)) continue;
  const strokes = strokesOf(viewArg, s, drawnE);
  const w = strokes.map((r) => { const a = r[0]!, b = r[1]!, c = r[2]!; return Math.min(Math.hypot(b.x - a.x, b.y - a.y), Math.hypot(c.x - b.x, c.y - b.y)); });
  console.log(s.id.padEnd(26), "strokes", strokes.length, "min side", Math.min(...w).toFixed(3), "max", Math.max(...w).toFixed(3));
}
