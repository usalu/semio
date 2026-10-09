import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2];
let text = readFileSync(file, "utf8");
const swap = (from, to) => { if (!text.includes(from)) throw new Error("missing " + from.slice(0, 60)); text = text.replace(from, to); };
if (!text.includes("    Finish,\n    Material,")) {
  swap("/// 📋️ What the rows of a schedule are: the elements of one kind, or (material) one row per layer or material run of every element.", "/// 📋️ What the rows of a schedule are: the elements of one kind, or (finish) one row per finished surface of every room, or (material) one row per layer or material run of every element.");
  swap("    Space,\n    Material,\n}", "    Space,\n    Finish,\n    Material,\n}");
  swap("    Usage,\n    Swing,", "    Usage,\n    Surface,\n    Swing,");
  swap("    LayerMass,\n}", "    LayerMass,\n    FinishArea,\n}");
  swap("the measures of its quantity take-off and, for material rows, the measures of one layer.", "the measures of its quantity take-off, for finish rows the surface and its area and for material rows the measures of one layer.");
}
writeFileSync(file, text);
