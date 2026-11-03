import { readFileSync, writeFileSync } from "node:fs";
const root = process.argv[2]!;
const schema = `${root}/🧬️schema`;
const doc = JSON.parse(readFileSync(`${schema}/🔣️.json`, "utf8")) as { $defs: Record<string, any> };
const revolved = doc.$defs.Solid.oneOf.find((alternative: any) => alternative.properties.kind.const === "revolved");
revolved.properties.lateralScale = {
  description: "Optional elliptic cross-section: the scale of the lateral radius relative to the radius in the profile, perpendicular to the axis and to the sagittal normal; 1 is a physical circle (the default, not a fallback), 0.5 an ellipse half as wide as it is deep.",
  type: "number",
  exclusiveMinimum: 0,
  default: 1,
};
writeFileSync(`${schema}/🔣️.json`, `${JSON.stringify(doc, null, 2)}\n`);
let ts = readFileSync(`${schema}/🟦️.ts`, "utf8");
const from = 'readonly axisZ?: number; readonly mirror?: boolean }';
if (!ts.includes(from)) throw new Error("twin");
ts = ts.replace(from, 'readonly axisZ?: number; readonly lateralScale?: number; readonly mirror?: boolean }');
const doc2 = "so the solid moves rigidly with its bone;";
if (!ts.includes(doc2)) throw new Error("doc");
ts = ts.replace(doc2, "so the solid moves rigidly with its bone; the optional `lateralScale` (> 0, default 1, a physical circle) scales the lateral radius to an elliptic cross-section;");
writeFileSync(`${schema}/🟦️.ts`, ts);
const catalog = `${root}/🔨️modules/🗂️catalog/🟦️.ts`;
let c = readFileSync(catalog, "utf8");
const anchor = '          need(`${sp}.solid.axisBone`, shape.solid.axisBone, bones, "Bone");';
if (!c.includes(anchor)) throw new Error("validator");
c = c.replace(anchor, anchor + '\n          if (shape.solid.lateralScale !== undefined && !(shape.solid.lateralScale > 0)) add(`${sp}.solid.lateralScale`, "solid-lateral-scale", `Shape "${shape.id}" needs lateralScale > 0 in its revolved solid (1 is a circle).`, `Form "${shape.id}" braucht lateralScale > 0 in ihrem rotierten Körper (1 ist ein Kreis).`);');
writeFileSync(catalog, c);
