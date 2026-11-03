import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , id, scale, pose] = process.argv;
const a = catalog().find((x) => x.id === id)!;
const st: any = evaluate(a, { kind: "pose", pose: pose ?? a.presentation.defaultPose }, { kind: "scale", scale: { denominator: +scale } } as any);
for (const v of ["elevation", "front", "plan"] as const) {
  console.log("--", v);
  for (const s of st.shapes[v]) { const p = s.rings.flat(); const xs = p.map((q: any) => q.x), ys = p.map((q: any) => q.y); console.log(`  ${s.shape.id.padEnd(22)} ${s.shape.bone.padEnd(14)} ${s.shape.role.padEnd(7)} ${s.shape.solid ? s.shape.solid.kind : s.shape.kind} x[${Math.min(...xs).toFixed(3)},${Math.max(...xs).toFixed(3)}] y[${Math.min(...ys).toFixed(3)},${Math.max(...ys).toFixed(3)}]`); }
}
