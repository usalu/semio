import { catalog, extentOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const gull = catalog().find((a) => a.id === "herring-gull")!;
const flat = { ...gull, poses: gull.poses.map((p) => ({ ...p, bones: Object.fromEntries(Object.entries(p.bones).map(([id, o]) => [id, { ...o, abductionDeg: 0 }])) })) };
const f = (n: number) => n.toFixed(3);
for (const scale of [50, 100, 200, 500])
  for (const pose of ["standing", "gliding", "landing", "touchdown"])
    for (const [label, asset] of [["spread", gull], ["flat", flat]] as const) {
      const s = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
      const [e, fr, p] = [s.shapes.elevation, s.shapes.front, s.shapes.plan].map((x) => extentOf(x)!);
      console.log(`1:${scale} ${pose} ${label} E x[${f(e!.minX)},${f(e!.maxX)}] y[${f(e!.minY)},${f(e!.maxY)}] | F x[${f(fr!.minX)},${f(fr!.maxX)}] y[${f(fr!.minY)},${f(fr!.maxY)}] | P x[${f(p!.minX)},${f(p!.maxX)}] y[${f(p!.minY)},${f(p!.maxY)}]`);
    }
