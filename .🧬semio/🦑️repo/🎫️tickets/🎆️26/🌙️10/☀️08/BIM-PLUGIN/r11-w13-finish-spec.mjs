import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2];
let text = readFileSync(file, "utf8");
const swap = (from, to) => { if (!text.includes(from)) throw new Error("missing " + from.slice(0, 60)); text = text.replace(from, to); };
if (!text.includes('"Space", "Finish", "Material"')) {
  swap('(material) one row per layer or material run of every element.", variants: ["Wall", "CurtainWall", "Slab", "Roof", "Column", "Beam", "Window", "Door", "Void", "Stair", "Railing", "Space", "Material"]', '(finish) one row per finished surface of every room, or (material) one row per layer or material run of every element.", variants: ["Wall", "CurtainWall", "Slab", "Roof", "Column", "Beam", "Window", "Door", "Void", "Stair", "Railing", "Space", "Finish", "Material"]');
  swap('"Number", "Usage", "Swing"', '"Number", "Usage", "Surface", "Swing"');
  swap('"LayerArea", "LayerVolume", "LayerMass"] }', '"LayerArea", "LayerVolume", "LayerMass", "FinishArea"] }');
  swap("the measures of its quantity take-off and, for material rows, the measures of one layer.", "the measures of its quantity take-off, for finish rows the surface and its area and for material rows the measures of one layer.");
}
writeFileSync(file, text);
