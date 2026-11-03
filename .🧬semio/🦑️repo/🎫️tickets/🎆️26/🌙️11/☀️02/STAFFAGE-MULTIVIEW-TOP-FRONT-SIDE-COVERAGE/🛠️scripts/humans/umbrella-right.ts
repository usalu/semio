import { load, save } from "./lib.ts";
import { shift } from "./lateral.ts";
const d = load("person-with-umbrella"); const a = d.asset;
const z = a.rig.bones.find((b: any) => b.id === "upper-arm-r").z;
let n = 0, bad: string[] = [];
for (const r of Object.values<any>(a.representations)) {
  if (r.status !== "authored") continue;
  for (const v of ["front", "plan"] as const) for (const s of r.geometry[v]) {
    if (!/^(umbrella|rib-front|rib-back)$/.test(s.bone)) continue;
    if (s.solid?.kind === "revolved") { s.solid.axisZ = +((s.solid.axisZ ?? 0) + z).toFixed(5); n++; }
    else if (shift(s, v, z)) n++; else bad.push(`${r.denominator}/${v}/${s.id}`);
  }
}
console.log("umbrella parts moved to the right hand:", n, "unmoved:", bad.join(","), "z", z); save("person-with-umbrella", d);
