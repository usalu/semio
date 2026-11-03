import { readFileSync, writeFileSync } from "node:fs";
const root = process.argv[2]!;
const schema = `${root}/🧬️schema`;
const doc = JSON.parse(readFileSync(`${schema}/🔣️.json`, "utf8")) as { $defs: Record<string, any> };
const extruded = doc.$defs.Solid.oneOf.find((alternative: any) => alternative.properties.kind.const === "extruded");
extruded.properties.zTip = {
  description: "Optional linear taper: the lateral span [z0, z1] (left is +) at the tip of the bone, in the same order as z0..z1 (zTip[0] < zTip[1]); the span at the bone origin is z0..z1 and the span between is interpolated linearly along the bone (u). Absent means a constant span. A tapered limb, blade or branch is one extruded solid.",
  type: "array",
  items: { type: "number" },
  minItems: 2,
  maxItems: 2,
};
writeFileSync(`${schema}/🔣️.json`, `${JSON.stringify(doc, null, 2)}\n`);
let ts = readFileSync(`${schema}/🟦️.ts`, "utf8");
const from = '{ readonly kind: "extruded"; readonly z0: number; readonly z1: number }';
if (!ts.includes(from)) throw new Error("twin");
ts = ts.replace(from, '{ readonly kind: "extruded"; readonly z0: number; readonly z1: number; readonly zTip?: readonly [number, number] }');
const doc2 = "the plan the forward range of the profile by `z0`..`z1`. */";
if (!ts.includes(doc2)) throw new Error("doc");
ts = ts.replace(doc2, "the plan the forward range of the profile by `z0`..`z1`. The optional `zTip` is the span at the bone tip in the same order as `z0`..`z1` (`zTip[0] < zTip[1]`); the span interpolates linearly along the bone from `z0`..`z1` at its origin, so a tapered limb, blade or branch is one extruded solid. */");
writeFileSync(`${schema}/🟦️.ts`, ts);
const catalog = `${root}/🔨️modules/🗂️catalog/🟦️.ts`;
let c = readFileSync(catalog, "utf8");
const anchor = '        if (shape.solid.kind === "extruded" && !(shape.solid.z0 < shape.solid.z1))';
if (!c.includes(anchor)) throw new Error("validator");
c = c.replace(anchor, '        if (shape.solid.kind === "extruded" && shape.solid.zTip !== undefined && !(shape.solid.zTip[0] < shape.solid.zTip[1])) add(`${sp}.solid.zTip`, "solid-taper-order", `Shape "${shape.id}" needs zTip[0] < zTip[1], in the same order as z0 < z1.`, `Form "${shape.id}" braucht zTip[0] < zTip[1], in derselben Reihenfolge wie z0 < z1.`);\n' + anchor);
c = c.replace("The print-profile findings of the `true-extent` symbols of an asset", "The print-profile findings of the symbols of an asset");
writeFileSync(catalog, c);
