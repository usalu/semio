/** 🖼️ Where the time of the sharp reference of the guard goes for one asset, scale and view: seconds of the owned `measureView` and of `sharpIou`, the grid and the number of polygons and vertices each mask hands to librsvg.
 *  bun oracle-timing.ts <slug|file.json> <scale> <view> */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { sharpIou } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/🟦️.ts";
import { fileOf } from "./lib.ts";

const [name, scaleArg, view] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg), feature = featureOf(scale);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
let t = performance.now();
const m = measureView(asset, state, solids, view as never, scale, undefined, feature);
const owned = (performance.now() - t) / 1000;
const count = (groups: readonly (readonly (readonly unknown[])[])[]): string => `${groups.length} polygons ${groups.reduce((n, g) => n + g.reduce((k, r) => k + r.length, 0), 0)} vertices`;
console.log(`${asset.id} 1:${scale} ${view}: owned ${owned.toFixed(1)} s, grid ${Math.ceil((m.box.maxX - m.box.minX) / m.cell)} x ${Math.ceil((m.box.maxY - m.box.minY) / m.cell)} cells of ${(m.cell * 1000).toFixed(2)} mm; filled ${count(m.filled)}, outlines ${count(m.outlines)}, reference ${count(m.reference)}`);
t = performance.now();
const reference = await sharpIou(m);
console.log(`  sharp ${((performance.now() - t) / 1000).toFixed(1)} s: ${reference.toFixed(4)} against owned ${m.iou.toFixed(4)}`);
