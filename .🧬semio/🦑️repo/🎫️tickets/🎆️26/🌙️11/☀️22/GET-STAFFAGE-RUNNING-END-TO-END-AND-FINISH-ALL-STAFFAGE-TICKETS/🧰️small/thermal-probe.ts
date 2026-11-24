/** 🌡️ Prints the solved thermal field of a catalog asset in its default pose (or a named pose): bun thermal-probe.ts <file> [pose]. */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermal } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const pose = process.argv[3] ?? asset.presentation.defaultPose;
const field = solveThermal(evaluate(asset, { kind: "pose", pose }));
console.log(pose, JSON.stringify(field.environment));
console.log("body", JSON.stringify(field.body));
for (const region of asset.thermal.regions) console.log(region.id, region.model, JSON.stringify(field.regions[region.id]));
