import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const a: any = structuredClone(catalog().find((x) => x.id === "labrador-dog")!);
const r = a.representations["50"].geometry;
const base = r.front.find((s: any) => s.id === "head");
const mk = (id: string, f: number[], t: number[], d: number) => ({ id, bone: "head", role: "body", z: base.z, filled: true, tone: base.tone, region: "coat-head", material: base.material, kind: "member", from: { x: f[0], y: f[1], z: 0 }, to: { x: t[0], y: t[1], z: 0 }, section: { kind: "circle", d } });
for (const v of ["front", "plan"]) { r[v] = r[v].filter((s: any) => !(s.bone === "head" && (s.id === "head" || s.id === "nose"))); r[v].push(mk("cranium", [0, 0.018], [0.13, 0.018], 0.115), mk("muzzle", [0.13, 0.015], [0.25, 0.015], 0.06)); }
for (const pose of ["standing-alert", "sniffing-head-down"]) {
  const st: any = evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 50 } } as any);
  for (const v of ["elevation", "front", "plan"]) for (const s of st.shapes[v]) if (s.shape.bone === "head" && ["cranium", "muzzle", "head"].includes(s.shape.id)) { const p = s.rings.flat(); console.log(pose, v, s.shape.id, p.length, p.length ? `x[${Math.min(...p.map((q: any) => q.x)).toFixed(3)},${Math.max(...p.map((q: any) => q.x)).toFixed(3)}] y[${Math.min(...p.map((q: any) => q.y)).toFixed(3)},${Math.max(...p.map((q: any) => q.y)).toFixed(3)}]` : "EMPTY"); }
}
