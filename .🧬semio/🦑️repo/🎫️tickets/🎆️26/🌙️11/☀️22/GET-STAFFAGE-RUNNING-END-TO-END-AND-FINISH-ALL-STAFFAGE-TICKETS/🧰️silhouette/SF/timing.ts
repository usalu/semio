/** ⏱️ Where the time of the silhouette guard of one asset and scale goes: seconds of `evaluate`, of `worldSolids` and of `measureView` per view (each view printed as it ends, so a timeout still shows the views before it), with the counts of solids, triangles and drawn shapes.
 *  bun timing.ts <slug|file.json> <scale> [view,view] */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { VIEWPOINTS, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./lib.ts";

const [name, scaleArg, viewsArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg), feature = featureOf(scale);
let t = performance.now();
const lap = (): string => { const s = ((performance.now() - t) / 1000).toFixed(2); t = performance.now(); return s; };
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
console.log(`${asset.id} 1:${scale} evaluate ${lap()} s`);
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
console.log(`  worldSolids ${lap()} s: ${solids.length} solids, ${solids.reduce((n, w) => n + w.indices.length / 3, 0)} triangles`);
for (const view of (viewsArg?.split(",") ?? VIEWPOINTS) as (typeof VIEWPOINTS)[number][]) {
  const m = measureView(asset, state, solids, view, scale, undefined, feature);
  console.log(`  ${view} ${lap()} s: iou ${m.iou.toFixed(4)} H ${(m.hausdorffM * 1000).toFixed(1)} mm, ${m.shapes} shapes, ${m.openStrokes} strokes, cell ${(m.cell * 1000).toFixed(2)} mm`);
}
