import { readFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermal } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const file = process.argv[2]!;
const doc = JSON.parse(readFileSync(file, "utf8"));
const asset = doc.asset;
const pose = process.argv[3] ?? asset.presentation.defaultPose;
const state = evaluate(asset, { kind: "pose", pose });
const t0 = performance.now();
const f = solveThermal(state);
console.log("[DEBUG] ms", performance.now() - t0);
console.log("[DEBUG] body", JSON.stringify(f.body));
for (const [id, r] of Object.entries(f.regions)) console.log("[DEBUG]", id.padEnd(14), r.surfaceTempC.toFixed(2), r.skinTempC?.toFixed(2), "rad", r.netRadiativeWM2.toFixed(1), "sens", r.sensibleWM2.toFixed(1), "lat", r.latentWM2.toFixed(1), "tau", r.timeConstantS.toFixed(0));
console.log("[DEBUG] totals", JSON.stringify(f.totals));
