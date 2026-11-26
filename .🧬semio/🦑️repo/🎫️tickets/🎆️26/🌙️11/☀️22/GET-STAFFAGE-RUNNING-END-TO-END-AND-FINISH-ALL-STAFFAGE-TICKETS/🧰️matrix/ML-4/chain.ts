/** 🧊️ The two asset-dependent cases of `🧪️tests/🧊️chain-state` measured: the elevation depth of the two upper arms of `person-with-umbrella` with the bones moved to z = ∓0.23, and every extent of `person-on-crutches` that moves when the limb roots gain z = ±0.2 and the revolved axes lose it. Usage: chain.ts [<umbrella.json> [<crutches.json>]] */
import { readFileSync } from "node:fs";
import { evaluate, restShapeDepth } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

const S = process.env.S!;
const load = (file: string): any => JSON.parse(readFileSync(file, "utf8")).asset;
const select = (denominator: number) => ({ kind: "scale", scale: { denominator } }) as const;

const person = load(process.argv[2] ?? `${S}/🗂️catalog/👤️humans/person-with-umbrella/🔣️.json`);
console.log("umbrella rig", JSON.stringify(person.rig.bones.filter((b: any) => /upper-arm/.test(b.id)).map((b: any) => ({ id: b.id, parent: b.parent, z: b.z }))));
for (const bone of person.rig.bones) if (bone.id === "upper-arm-r") bone.z = -0.23; else if (bone.id === "upper-arm-l") bone.z = 0.23;
for (const s of person.representations["50"].geometry.elevation.filter((s: any) => /upper-arm-[lr]/.test(s.bone))) console.log("umbrella elevation", s.id, s.bone, s.kind, "solid", JSON.stringify(s.solid ?? null).slice(0, 300), "depth", restShapeDepth(person, s, "elevation"));

const original = load(process.argv[3] ?? `${S}/🗂️catalog/👤️humans/person-on-crutches/🔣️.json`);
const moved = JSON.parse(JSON.stringify(original));
const worldZ = (asset: any, id: string): number => { const byId = new Map<string, any>(asset.rig.bones.map((b: any) => [b.id, b])); let z = 0; for (let b = byId.get(id); b !== undefined; b = b.parent === null ? undefined : byId.get(b.parent)) z += b.z ?? 0; return z; };
console.log("crutches roots", JSON.stringify(original.rig.bones.filter((b: any) => /^(arm|upper-arm|thigh)-[lr]$/.test(b.id)).map((b: any) => ({ id: b.id, parent: b.parent, z: b.z }))));
for (const bone of moved.rig.bones) if (/^(arm|upper-arm|thigh)-[lr]$/.test(bone.id)) bone.z = (bone.id.endsWith("-l") ? 1 : -1) * 0.2;
const kinds: Record<string, number> = {};
for (const [scale, representation] of Object.entries<any>(moved.representations)) if (representation.status === "authored") for (const view of ["elevation", "front", "plan"]) for (const shape of representation.geometry[view] ?? []) {
  const solid = shape.solid;
  if (scale === "50") { const k = `${view} ${solid?.kind ?? "flat"}`; kinds[k] = (kinds[k] ?? 0) + 1; }
  if (solid?.kind === "revolved") solid.axisZ = (solid.axisZ ?? 0) + worldZ(original, solid.axisBone) - worldZ(moved, solid.axisBone);
}
console.log("crutches 1:50 shape kinds", JSON.stringify(kinds));
let reds = 0;
for (const pose of original.poses) for (const view of ["elevation", "front", "plan"] as const) {
  const sa = evaluate(original, { kind: "pose", pose: pose.id }, select(50)), sb = evaluate(moved, { kind: "pose", pose: pose.id }, select(50));
  const a = extentOf(sa.shapes[view], true)!, b = extentOf(sb.shapes[view], true)!;
  for (const edge of ["minX", "maxX", "minY", "maxY"] as const) if (Math.abs(a[edge] - b[edge]) > 1e-6) {
    reds++; console.log("crutches", pose.id, view, edge, a[edge].toFixed(4), b[edge].toFixed(4));
    for (let i = 0; i < sa.shapes[view].length; i++) { const ea = extentOf([sa.shapes[view][i]!], true), eb = extentOf([sb.shapes[view][i]!], true); if (ea && eb && Math.abs(ea[edge] - eb[edge]) > 1e-6) { const sh: any = sa.shapes[view][i]!.shape; console.log("   shape", sh.id, sh.bone, sh.solid?.kind ?? "flat", sh.solid?.axisBone ?? "", ea[edge].toFixed(4), eb[edge].toFixed(4)); } }
  }
}
console.log("crutches moved edges:", reds);
