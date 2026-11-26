/** 🧬️ What the two asset-dependent cases of `🧪️tests/🧊️chain-state` measure when the lateral offset is given to the bone that hangs in the sagittal frame of the trunk (the clavicle and the ilium of the anatomical rigs) instead of the limb root below it, whose frame is abducted and twisted: the elevation depth of the upper arms of `person-with-umbrella`, and the moved extents of `person-on-crutches` per view. Usage: chain-variant.ts */
import { readFileSync } from "node:fs";
import { evaluate, restShapeDepth } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

const load = (id: string): any => JSON.parse(readFileSync(`${process.env.S}/🗂️catalog/👤️humans/${id}/🔣️.json`, "utf8")).asset;
const select = (denominator: number) => ({ kind: "scale", scale: { denominator } }) as const;

for (const [label, right, left] of [["limb root (the case as written)", "upper-arm-r", "upper-arm-l"], ["clavicle", "clavicle-r", "clavicle-l"]] as const) {
  const person = load("person-with-umbrella");
  for (const bone of person.rig.bones) if (bone.id === right) bone.z = -0.23; else if (bone.id === left) bone.z = 0.23;
  const shapes = person.representations["50"].geometry.elevation;
  const r = restShapeDepth(person, shapes.find((s: any) => s.bone === "upper-arm-r"), "elevation"), l = restShapeDepth(person, shapes.find((s: any) => s.bone === "upper-arm-l"), "elevation");
  console.log(`umbrella, z = -0.23 / +0.23 on the ${label}: depth right ${r.toFixed(4)}, left ${l.toFixed(4)}, difference ${(r - l).toFixed(4)} (wanted > 0.4)`);
}
{
  const person = load("person-with-umbrella");
  const shapes = person.representations["50"].geometry.elevation;
  console.log(`umbrella, unchanged: depth right ${restShapeDepth(person, shapes.find((s: any) => s.bone === "upper-arm-r"), "elevation").toFixed(4)}, left ${restShapeDepth(person, shapes.find((s: any) => s.bone === "upper-arm-l"), "elevation").toFixed(4)}`);
}
for (const [label, pattern] of [["limb roots arm, thigh (the case as written)", /^(arm|upper-arm|thigh)-[lr]$/], ["clavicle, ilium", /^(clavicle|ilium)-[lr]$/]] as const) {
  const original = load("person-on-crutches");
  const moved = JSON.parse(JSON.stringify(original));
  for (const bone of moved.rig.bones) if (pattern.test(bone.id)) bone.z = (bone.z ?? 0) + (bone.id.endsWith("-l") ? 1 : -1) * 0.2;
  const worst: Record<string, number> = {};
  for (const pose of original.poses) for (const view of ["elevation", "front", "plan"] as const) {
    const a = extentOf(evaluate(original, { kind: "pose", pose: pose.id }, select(50)).shapes[view], true)!, b = extentOf(evaluate(moved, { kind: "pose", pose: pose.id }, select(50)).shapes[view], true)!;
    for (const edge of ["minX", "maxX", "minY", "maxY"] as const) worst[view] = Math.max(worst[view] ?? 0, Math.abs(a[edge] - b[edge]));
  }
  console.log(`crutches, z + (±0.2) on ${label}, no shape compensated: largest moved edge per view ${JSON.stringify(Object.fromEntries(Object.entries(worst).map(([k, v]) => [k, Number(v.toFixed(4))])))}`);
}
