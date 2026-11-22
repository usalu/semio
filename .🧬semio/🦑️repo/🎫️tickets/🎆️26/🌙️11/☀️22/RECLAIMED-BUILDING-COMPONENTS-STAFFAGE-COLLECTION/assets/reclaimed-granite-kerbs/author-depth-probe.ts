/** 🕳️ Prints the bone origins and the elevation painting depth of the kerb shapes of one scale in the default pose. */
import { readFileSync } from "node:fs";
import { evaluate, shapeDepth } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const asset = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")).asset;
const state = evaluate(asset, { kind: "pose", pose: process.argv[2] ?? "stored" }, { kind: "scale", scale: { denominator: Number(process.argv[3] ?? 500) } } as never);
for (const s of state.shapes.elevation.filter((x: { shape: { id: string } }) => x.shape.id.startsWith("kerb-") && !x.shape.id.includes("-a") && !x.shape.id.includes("-t")))
  console.log(`[DEBUG] ${s.shape.id}`, JSON.stringify(state.bones[s.shape.bone].frame.o), shapeDepth(state, s.shape, "elevation").toFixed(3));
