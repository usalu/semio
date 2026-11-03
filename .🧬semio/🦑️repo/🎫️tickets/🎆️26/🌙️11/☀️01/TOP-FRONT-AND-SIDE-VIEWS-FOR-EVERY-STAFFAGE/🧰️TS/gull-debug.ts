import { catalog, extentOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const gull = catalog().find((a) => a.id === "herring-gull")!;
const s = evaluate(gull, { kind: "clip", clip: "landing", timeS: 2.56 }, { kind: "scale", scale: { denominator: 100 } });
const f = (n: number) => n.toFixed(3);
for (const view of ["elevation", "front"] as const)
  for (const e of s.shapes[view]) {
    const x = extentOf([e], true);
    if (x && x.minY < 0.05) console.log(`${view} ${e.shape.id} (${e.shape.bone}) y[${f(x.minY)},${f(x.maxY)}] x[${f(x.minX)},${f(x.maxX)}]`);
  }
