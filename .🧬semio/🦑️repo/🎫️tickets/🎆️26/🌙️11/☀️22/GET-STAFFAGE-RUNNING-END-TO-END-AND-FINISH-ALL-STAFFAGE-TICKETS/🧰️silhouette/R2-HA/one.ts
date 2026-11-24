/** 🎯️ The checks of single views of an object as the real test measures them (`measureView` with the feature raster and tiles), optional ops applied first: IoU and largest boundary distance, `!` marks a failing one.
 *  bun one.ts <id|file.json> <scale> <view,…|all> [ops.json|-] */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { VIEWPOINTS, bodyDimensions, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOps } from "./ops.ts";
import { fileOf } from "./people.ts";
const [name, scaleArg, viewsArg, opsArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (opsArg && opsArg !== "-") applyOps(asset, JSON.parse(await Bun.file(opsArg).text()));
for (const scale of scaleArg!.split(",").map(Number)) {
  const feature = featureOf(scale);
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  const solids = worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true);
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  const bb = bodyDimensions(solids).map((b: number, k: number) => `${b.toFixed(4)}/${dims[k]}${Math.abs(b - dims[k]) > feature ? "!" : ""}`).join(" ");
  const out: string[] = [];
  for (const view of viewsArg === "all" || viewsArg === undefined ? VIEWPOINTS : (viewsArg.split(",") as never[])) {
    const m = measureView(asset, state, solids, view, scale, undefined, feature);
    out.push(`${view} ${m.iou.toFixed(3)}${m.iou < 0.92 ? "!" : ""}/${(m.hausdorffM * 1000).toFixed(1)}mm${m.hausdorffM > 2 * feature ? "!" : ""}`);
  }
  console.log(`${asset.id} 1:${scale} bbox ${bb}  ${out.join("  ")}`);
}
