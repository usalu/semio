import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , id, scale, pose, bones] = process.argv;
const a = catalog().find((x) => x.id === id)!;
const q: any = pose.includes("@") ? { kind: "clip", clip: pose.split("@")[0], timeS: +pose.split("@")[1] } : { kind: "pose", pose };
for (const base of [{ kind: "pose", pose: a.presentation.defaultPose } as any, q]) {
  const st = evaluate(a, base, { kind: "scale", scale: { denominator: +scale } } as any);
  console.log("--", JSON.stringify(base));
  for (const v of ["elevation", "front", "plan"] as const)
    for (const s of st.shapes[v] as any[]) if (bones.split(",").includes(s.shape.bone)) {
      const pts = s.rings.flat(); const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
      console.log(v.padEnd(9), s.shape.id.padEnd(16), s.shape.bone.padEnd(12), `x[${Math.min(...xs).toFixed(3)},${Math.max(...xs).toFixed(3)}] y[${Math.min(...ys).toFixed(3)},${Math.max(...ys).toFixed(3)}]`);
    }
}
