import { catalog, authoredScales, extentOf } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , ...ids] = process.argv;
let n = 0;
for (const a of catalog().filter((x) => ids.includes(x.id))) for (const d of authoredScales(a)) {
  const st: any = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } } as any);
  const e = extentOf(st.shapes.elevation)!, f = extentOf(st.shapes.front)!, p = extentOf(st.shapes.plan)!;
  const D = a.dimensions;
  const chk = (code: string, m: number, nom: number) => { if (Math.abs(m - nom) > Math.max(0.02 * nom, 0.015)) { n++; console.log(`${a.id} 1:${d} ${code} ${m.toFixed(3)} vs ${nom}`); } };
  chk("elev-w", e.maxX - e.minX, D.widthM); chk("elev-h", e.maxY - e.minY, D.heightM);
  chk("front-h", f.maxY - f.minY, D.heightM); chk("front-d", f.maxX - f.minX, D.depthM);
  chk("plan-w", p.maxX - p.minX, D.widthM); chk("plan-d", p.maxY - p.minY, D.depthM);
}
console.log("total", n);
