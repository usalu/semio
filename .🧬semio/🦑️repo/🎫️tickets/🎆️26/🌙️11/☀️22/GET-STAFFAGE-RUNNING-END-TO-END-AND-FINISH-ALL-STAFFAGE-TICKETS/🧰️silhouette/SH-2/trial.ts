/** 🧪️ A trial of candidate ops on the catalog file of a person, in memory only: the guard of the given scales (bounding box, IoU and largest boundary distance per view), the drawn extents `validateAsset` reads and the new `validateAsset` issues, before and after the ops.
 *  bun trial.ts <slug> <scales, e.g. 50,100> <ops.json> */
import { viewExtent } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { measureAsset, VIEWPOINTS } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { applyOps } from "./ops.ts";
import { catalogFile } from "./people.ts";

const [slug, scales, opsFile] = process.argv.slice(2) as [string, string, string];
const read = async (): Promise<any> => (await Bun.file(catalogFile(slug)).json()).asset;
const report = (name: string, asset: any): string[] => {
  for (const scale of scales.split(",").map(Number)) {
    const m = measureAsset(asset, scale, undefined, undefined, featureOf(scale));
    const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
    const extents = VIEWPOINTS.map((v) => { const e = viewExtent(state, v); return e ? `${e.horizontal.toFixed(3)}x${e.vertical.toFixed(3)}` : "-"; }).join(" ");
    console.log(`${name} 1:${scale} bbox ${m.bbox.map((b) => b.toFixed(4)).join(" ")}  ${VIEWPOINTS.map((v) => `${v} ${m.views[v].iou.toFixed(4)}/${(m.views[v].hausdorffM * 1000).toFixed(1)}mm${m.views[v].iou < 0.92 || m.views[v].hausdorffM > 2 * featureOf(scale) ? "!" : ""}`).join("  ")}  drawn ${extents}`);
  }
  return (validateAsset(asset as never) as any[]).map((i) => `${i.code} ${i.path ?? ""}: ${i.message.en}`.slice(0, 260));
};
const before = report("before", await read());
const asset = await read();
applyOps(asset, JSON.parse(await Bun.file(opsFile).text()));
const after = report("after ", asset);
for (const issue of after.filter((i) => !before.includes(i))) console.log(`   new issue: ${issue}`);
console.log(`issues ${before.length} -> ${after.length}`);
