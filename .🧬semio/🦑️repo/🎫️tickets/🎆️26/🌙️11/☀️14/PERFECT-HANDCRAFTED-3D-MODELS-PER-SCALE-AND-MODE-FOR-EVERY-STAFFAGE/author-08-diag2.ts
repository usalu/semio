import { measureAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { entryOf, featureM, loadSource, merged, poseOf } from "./volumes-lib.ts";
const [spec, scaleArg] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry));
const pm = measureAsset(m as never, Number(scaleArg), poseOf(m), undefined, featureM(Number(scaleArg), m));
const v = pm.views.elevation;
console.log("strokes", v.openStrokes, "outlines", v.outlines.length, "filled", v.filled.length);
for (const g of v.outlines) { const r = g[0]!; console.log(r.map((p) => `(${p.x.toFixed(3)},${p.y.toFixed(3)})`).join(" ")); }
