import { readFileSync } from "node:fs";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const a = JSON.parse(readFileSync("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/vacuum-cleaner/🔣️.json", "utf8")).asset;
for (const pose of ["running", "parked", "running-pulled"]) {
  const s: any = evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 50 } });
  for (const vp of ["elevation", "plan"]) for (const e of s.shapes[vp]) if (["wand-tube", "nozzle", "grip-sleeve", "hose-10-tube"].includes(e.shape.id)) {
    const xs = e.rings.flat().map((p: any) => p.x), ys = e.rings.flat().map((p: any) => p.y);
    console.log(pose, vp, e.shape.id, "x", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), "y", Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3));
  }
}
