import { readFileSync, writeFileSync } from "node:fs";
const root = process.argv[2]!;
const schema = `${root}/🧬️schema`;
const doc = JSON.parse(readFileSync(`${schema}/🔣️.json`, "utf8")) as { $defs: Record<string, any> };
const extruded = doc.$defs.Solid.oneOf.find((alternative: any) => alternative.properties.kind.const === "extruded");
extruded.required = ["kind", "sweep", "z0", "z1"];
extruded.properties = {
  kind: extruded.properties.kind,
  sweep: {
    description: "The bone-local axis the profile is swept along, required and never defaulted. w sweeps along the lateral axis (left is +): the profile lies in the bone's (u, v) plane, as every extruded solid has been authored. v sweeps along the bone normal: the profile lies in the bone's (u, w) plane and the thickness runs along v. z0, z1 and zTip are offsets along this axis from the bone origin at rest (they keep the name z of the default lateral sweep).",
    enum: ["v", "w"],
  },
  ...Object.fromEntries(Object.entries(extruded.properties).filter(([key]) => key !== "kind")),
};
writeFileSync(`${schema}/🔣️.json`, `${JSON.stringify(doc, null, 2)}\n`);
let ts = readFileSync(`${schema}/🟦️.ts`, "utf8");
const from = '{ readonly kind: "extruded"; readonly z0: number;';
if (!ts.includes(from)) throw new Error("twin");
ts = ts.replace(from, '{ readonly kind: "extruded"; readonly sweep: "v" | "w"; readonly z0: number;');
const doc2 = "`extruded`: a laterally prismatic part";
if (!ts.includes(doc2)) throw new Error("doc");
ts = ts.replace(doc2, "`extruded`: a prismatic part swept along the bone-local axis `sweep` (`w` lateral, with the profile in the (u, v) plane; `v` along the bone normal, with the profile in the (u, w) plane; required, never defaulted), here described for the lateral sweep: a part");
writeFileSync(`${schema}/🟦️.ts`, ts);
const catalog = `${root}/🔨️modules/🗂️catalog/🟦️.ts`;
let c = readFileSync(catalog, "utf8");
const anchor = '        if (shape.solid.kind === "extruded" && shape.solid.zTip !== undefined';
if (!c.includes(anchor)) throw new Error("validator");
c = c.replace(anchor, '        if (shape.solid.kind === "extruded" && (shape.solid as { sweep?: string }).sweep !== "v" && (shape.solid as { sweep?: string }).sweep !== "w") add(`${sp}.solid.sweep`, "solid-sweep", `Shape "${shape.id}" has an extruded solid without sweep; state "w" (lateral) or "v" (along the bone normal), there is no default.`, `Form "${shape.id}" hat einen extrudierten Körper ohne sweep; "w" (seitlich) oder "v" (entlang der Knochennormalen) angeben, es gibt keinen Standardwert.`);\n' + anchor);
writeFileSync(catalog, c);
