/** 🔭 Prints the product's paint depth (`shapeDepth`) of chosen front shapes in the stored and the played pose, to check the depth order of cello and case. */
import { readFileSync } from "node:fs";
import { evaluate, shapeDepth, restShapeDepth } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const asset = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")).asset;
const view = (process.argv[2] ?? "front") as "front";
for (const pose of ["stored", "played"]) {
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 50 } } as never);
  for (const id of ["corpus", "fingerboard", "bridge", "bow-stick", "case-lid-shell", "case-back-shell", "stopper"]) {
    const shape = asset.representations["50"].geometry[view].find((s: { id: string }) => s.id === id);
    if (shape) console.log("[DEBUG]", pose, id, shapeDepth(state, shape, view).toFixed(3), restShapeDepth(asset, shape, view).toFixed(3));
  }
}
