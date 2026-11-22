/** 🌓️ The product's silhouette numbers of one view and scale, optionally with 2D shapes and 3D solids left out by id prefix, to find what disagrees. */
import { readFileSync } from "node:fs";
import { evaluateDrawing } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { measureBodyView, worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
const [file, k, view, drop2, drop3] = process.argv.slice(2);
const a = JSON.parse(readFileSync(file!, "utf8")).asset;
const d = Number(k);
const no2 = (drop2 ?? "").split(",").filter(Boolean);
const no3 = (drop3 ?? "").split(",").filter(Boolean);
for (const v of ["elevation", "front", "plan"]) a.representations[k!].geometry[v] = a.representations[k!].geometry[v].filter((s: { id: string }) => !no2.some((p) => s.id.startsWith(p)));
a.volumes[k!].body = a.volumes[k!].body.filter((s: { id: string }) => !no3.some((p) => s.id.startsWith(p)));
const state = evaluateDrawing(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
const m = measureBodyView(a, state, worldSolids(a, state.bones, d), view as "elevation", d, featureOf(d), new Set());
console.log("[DEBUG]", view, k, JSON.stringify(m, (key, v) => (typeof v === "number" ? Number(v.toFixed(4)) : v)).slice(0, 600));
