/** 🎭 Reading aid: dumps the product's silhouette groups (3D reference and 2D filled shapes) of one scale and view of the candidate as JSON (`bun author-masks.ts <scale> <view> <out.json>`). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { evaluateDrawing } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { measureView, worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const a = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const d = Number(process.argv[2]);
const view = process.argv[3] as "elevation";
const state = evaluateDrawing(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: d as 50 } });
const m = measureView(a, state as never, worldSolids(a, state.bones, d), view, d, undefined, featureOf(d));
console.log(`[DEBUG] iou ${m.iou.toFixed(4)} p95 ${m.p95M.toFixed(4)} haus ${m.hausdorffM.toFixed(4)} recall ${m.recall.toFixed(4)} precision ${m.precision.toFixed(4)}`);
writeFileSync(process.argv[4]!, JSON.stringify({ reference: m.reference, filled: m.filled, outlines: m.outlines.length }));
