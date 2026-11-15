import { entryOf, loadSource, merged, featureM, poseOf } from "./volumes-lib.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids, measureView, Mask } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const { entry, asset: raw } = entryOf("humans/cyclist");
const asset: any = merged(raw, loadSource(entry)!);
const state: any = evaluate(asset, { kind: "pose", pose: poseOf(asset) }, { kind: "scale", scale: { denominator: 50 } });
const solids = worldSolids(asset, state.bones as never, 50);
const m = measureView(asset, state, solids as never, "elevation", 50, undefined, 0.0075);
const { box, cell } = m;
console.log(box, cell, m.filled.length, m.outlines.length);
const drawn = new Mask(box, cell); for (const g of m.filled) drawn.fill(g); for (const g of m.outlines) drawn.fill(g);
for (const [x, y] of [[0.69, 1.172], [0.70, 1.175], [0.72, 1.178], [0.75, 1.17], [0.80, 1.17]]) console.log(x, y, drawn.has(x, y));
let n = 0; for (const g of m.filled) { const xs = g.flat().map((p: any) => p.x), ys = g.flat().map((p: any) => p.y); if (Math.min(...xs) > 0.6 && Math.max(...xs) < 0.9 && Math.min(...ys) > 1.0 && Math.max(...ys) < 1.25) console.log("filled", Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)); }
