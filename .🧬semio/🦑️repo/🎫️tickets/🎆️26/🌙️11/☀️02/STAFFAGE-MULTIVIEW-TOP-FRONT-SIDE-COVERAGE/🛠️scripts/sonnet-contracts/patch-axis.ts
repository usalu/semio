import { readFileSync, writeFileSync } from "node:fs";
const dir = process.argv[2]!;
const sub = (t: string, from: string, to: string): string => {
  if (!t.includes(from)) throw new Error(`missing: ${from.slice(0, 60)}`);
  return t.replace(from, to);
};
let json = readFileSync(`${dir}/🔣️.json`, "utf8");
json = sub(json, '"kind",\n            "axisBone",\n            "axisX"\n          ],', '"kind",\n            "axisBone",\n            "axis",\n            "axisX"\n          ],');
json = sub(json, '            "axisX": {\n              "description": "Offset of the axis', '            "axis": {\n              "description": "The direction of the axis in the bone-local frame of axisBone: u along the bone, v its normal in the sagittal plane, w the lateral axis (left is +). The axis is carried with the bone frame, so the solid revolves about the same line whatever the pose. Axisymmetric parts about a vertical bone use u; a part about a horizontal axle across the bone uses w.",\n              "enum": [\n                "u",\n                "v",\n                "w"\n              ]\n            },\n            "axisX": {\n              "description": "Offset of the axis');
writeFileSync(`${dir}/🔣️.json`, json);
let ts = readFileSync(`${dir}/🟦️.ts`, "utf8");
ts = sub(ts, 'readonly kind: "revolved"; readonly axisBone: Slug; readonly axisX: number;', 'readonly kind: "revolved"; readonly axisBone: Slug; readonly axis: "u" | "v" | "w"; readonly axisX: number;');
ts = sub(ts, "about an axis through the rest origin of `axisBone`, offset by", "about an axis through the rest origin of `axisBone` along its bone-local direction `axis` (`u` along the bone, `v` its normal in the sagittal plane, `w` the lateral axis), offset by");
writeFileSync(`${dir}/🟦️.ts`, ts);
