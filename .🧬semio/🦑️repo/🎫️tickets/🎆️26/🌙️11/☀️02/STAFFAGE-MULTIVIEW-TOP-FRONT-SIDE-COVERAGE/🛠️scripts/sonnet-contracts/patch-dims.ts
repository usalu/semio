import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const sub = (from: string, to: string): void => {
  if (!t.includes(from)) throw new Error(`missing: ${from.slice(0, 50)}`);
  t = t.replace(from, to);
};
sub('"axisZ": {\n              "type": "number"', '"axisZ": {\n              "description": "Offset of the axis from the rest origin of axisBone, laterally (left is +) in the bone-local frame; default 0; the axis is carried with the bone frame.",\n              "type": "number"');
sub('"heightM": {\n          "type": "number",', '"heightM": {\n          "description": "Extent along world y (up): the vertical extent of the side elevation and of the front elevation.",\n          "type": "number",');
sub("Horizontal extent of the default side elevation drawing (the rig plane's x axis, ahead) and of the plan (top view); the catalog check compares it with the drawn extent. For a side elevation this is the real depth of the object, not its left-right width.", "Extent along world x (ahead): the horizontal extent of the side elevation and of the plan (top view, paper x); the catalog check compares it with the drawn extent. For a side elevation this is the real depth of the object, not its left-right width.");
sub("Extent perpendicular to the side elevation plane (the y axis of the plan (top view), the horizontal axis of the front elevation); the left-right width of an object drawn in side elevation.", "Extent along world z (lateral): the horizontal extent of the front elevation (paper x) and the vertical extent of the plan (top view, paper up); the left-right width of an object drawn in side elevation.");
writeFileSync(file, t);
