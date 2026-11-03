import { load, save } from "./lib.ts";
const doc = load("person-with-white-cane");
const a = doc.asset;
for (const r of Object.values<any>(a.representations)) {
  if (r.status !== "authored") continue;
  for (const s of r.geometry.front) {
    if (s.solid?.kind !== "revolved" || !/^(arm|forearm|hand)-[lr]$/.test(s.bone)) continue;
    const ys = (s.points ?? []).flatMap((p: number[]) => [Math.abs(p[1])]);
    const rad = ys.length ? Math.max(...ys) : 0.045;
    const z = s.solid.axisZ;
    s.solid = { kind: "extruded", z0: +(z - rad).toFixed(4), z1: +(z + rad).toFixed(4) };
    console.log(r.denominator, s.id, s.kind, rad.toFixed(3), z);
  }
}
save("person-with-white-cane", doc);
