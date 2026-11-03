import { sampleAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🧱️synthetic-figure/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts";
const a = sampleAsset();
for (const p of ["stand", "carry-bag"]) {
  const s = evaluate(a, { kind: "pose", pose: p }, { kind: "scale", scale: { denominator: 50 } });
  console.log(p, JSON.stringify(Object.fromEntries(Object.entries(s.anchors).map(([k, v]) => [k, [v.at, (v as any).z]]))));
  for (const v of ["elevation", "front", "plan"] as const) console.log(v, JSON.stringify(extentOf(s.shapes[v])), s.shapes[v].filter((x) => /bag|strap/.test(x.shape.id)).map((x) => x.shape.id));
  console.log("root", JSON.stringify((s as any).root), a.dimensions);
}
console.log(a.rig.bones.map((b: any) => b.id).join(","));
