import { catalog, extentOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const crane = catalog().find((a) => a.id === "tower-crane")!;
const s = evaluate(crane, { kind: "pose", pose: "parked-weathervane" }, { kind: "scale", scale: { denominator: 100 } });
const f = (n: number) => n.toFixed(2);
for (const view of ["elevation", "plan"] as const)
  for (const e of s.shapes[view]) {
    const x = extentOf([e], true);
    if (x) console.log(`${view} ${e.shape.id} (${e.shape.bone}${(e.shape as { followTip?: boolean }).followTip ? ",follow" : ""}${e.shape.solid ? ",solid" : ""}) x[${f(x.minX)},${f(x.maxX)}] y[${f(x.minY)},${f(x.maxY)}]`);
  }
