import { load, save } from "./lib.ts";
import { shift } from "./lateral.ts";
const OFFSET = 0.06;
const d = load("parent-carrying-toddler"); const a = d.asset;
a.rig.bones.find((b: any) => b.id === "toddler-torso").z = OFFSET;
const swap: Record<string, string> = { "-l": "-r", "-r": "-l" };
let moved = 0, renamed = 0;
for (const r of Object.values<any>(a.representations)) {
  if (r.status !== "authored") continue;
  for (const v of ["front", "plan"] as const) for (const s of r.geometry[v]) {
    if (!s.bone.startsWith("toddler-")) continue;
    if (shift(s, v, OFFSET)) moved++;
    const m = /^(toddler-(?:thigh|shank|boot))(-[lr])$/.exec(s.id);
    if (m && /-(n|f)$/.test(s.bone) && !s.id.includes("fixed")) {
      const nearRight = s.bone.endsWith("-n");
      const want = nearRight ? "-r" : "-l";
      if (m[2] !== want) { s.id = m[1] + want; renamed++; }
    }
  }
}
console.log("toddler shapes shifted", moved, "ids renamed to the correct hand", renamed); save("parent-carrying-toddler", d);
