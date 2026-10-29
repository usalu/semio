/** 🧪 Scratch: prints evaluated bounds and bone origins of one asset in the objects file. */
import { readFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [file, id, pose] = process.argv.slice(2);
const doc = JSON.parse(readFileSync(file!, "utf8"));
const asset = doc.assets.find((a: { id: string }) => a.id === id);
const state = evaluate(asset, { kind: "pose", pose: pose ?? asset.presentation.defaultPose });
console.log(JSON.stringify(state.bounds));
for (const [k, v] of Object.entries(state.bones)) console.log(k, JSON.stringify((v as { origin?: unknown }).origin ?? v).slice(0, 160));
