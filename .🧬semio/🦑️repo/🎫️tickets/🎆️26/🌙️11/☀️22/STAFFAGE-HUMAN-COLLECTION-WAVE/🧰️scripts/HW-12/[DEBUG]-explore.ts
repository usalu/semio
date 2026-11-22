import { loadDoc, evalOffsets, frameOf, at, rd } from "./lib.ts";
const doc = await loadDoc("standing-adult");
const a = doc.asset;
const relaxed = a.poses[0].bones;
const head = { neck: { angleDeg: -6, twistDeg: 14, abductionDeg: 6 }, head: { angleDeg: -10, twistDeg: 22, abductionDeg: 8 } };
const s = evalOffsets(a, { ...relaxed, ...head });
const H = frameOf(s, "head");
const p = (n: string, v: number[]) => console.log(n.padEnd(14), v.map((x) => rd(x, 3)).join(" "));
p("head o", H.o); p("head u", H.u); p("head v", H.v); p("head w", H.w);
p("chin", at(H, -0.075, -0.085, 0)); p("jaw-l", at(H, -0.07, -0.03, 0.05)); p("jaw-l-low", at(H, -0.09, -0.045, 0.035));
for (const b of ["clavicle-l", "torso", "neck"]) { const f = frameOf(s, b); p(b + " o", f.o); }
p("clav-l tip", [s.bones["clavicle-l"].tip.x, s.bones["clavicle-l"].tip.y, s.bones["clavicle-l"].tip.z]);
p("arm-l o", frameOf(s, "arm-l").o); p("arm-r o", frameOf(s, "arm-r").o);
