/** 🔍️ Probe of the product silhouette measure of one view of the candidate: bounding boxes of the drawn groups and of the projected solid groups (diagnostic input of the author, not product code). */
import { readFileSync } from "node:fs";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const [file, scaleArg, view] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const d = Number(scaleArg);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
const body = worldSolids(asset, state.bones, d, "body");
const m = measureView(asset, state as never, body, view as never, d, undefined, d * 0.00015);
const bb = (g: any) => { let a = [1e9, 1e9, -1e9, -1e9]; for (const r of g) for (const p of r) { a = [Math.min(a[0], p.x), Math.min(a[1], p.y), Math.max(a[2], p.x), Math.max(a[3], p.y)]; } return a.map((v) => v.toFixed(3)).join(" "); };
console.log("[DEBUG] iou", m.iou.toFixed(3), "p95", m.p95M.toFixed(4), "cell", m.cell);
m.filled.forEach((g, i) => console.log("[DEBUG] drawn", i, bb(g)));
m.reference.forEach((g, i) => console.log("[DEBUG] solid", i, bb(g)));
