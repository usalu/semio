/** 🔍️ Prints the paper rings and order of one shape of one scale, pose and viewpoint as the print measure prepares them. */
import { readFileSync } from "node:fs";
import { prepare, sourceOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { evaluateDrawing } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const doc = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
const [scale, poseId, view, id] = process.argv.slice(2);
const state = evaluateDrawing(doc.asset, { kind: "pose", pose: poseId }, { kind: "scale", scale: { denominator: Number(scale) } });
const prepared = prepare(state, view as "front", sourceOf(state), 1000 / Number(scale));
for (const p of prepared) if (p.shape.id === id) console.log("[DEBUG]", p.order, p.weightMm, JSON.stringify(p.rings.map((r: { x: number; y: number }[]) => r.map((q) => [q.x.toFixed(3), q.y.toFixed(3)]))));
