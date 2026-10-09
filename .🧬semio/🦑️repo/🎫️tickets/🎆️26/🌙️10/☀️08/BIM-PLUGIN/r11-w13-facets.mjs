import { readFileSync, writeFileSync } from "node:fs";
const [graphql, json, ts] = process.argv.slice(2);
const edit = (file, swaps, marker) => {
  let t = readFileSync(file, "utf8");
  if (t.includes(marker)) return;
  const crlf = t.includes("\r\n");
  t = t.replaceAll("\r\n", "\n");
  for (const [from, to] of swaps) { if (!t.includes(from)) throw new Error(file.slice(-30) + " missing " + from.slice(0, 60)); t = t.replace(from, to); }
  writeFileSync(file, crlf ? t.replaceAll("\n", "\r\n") : t);
};
edit(graphql, [["  Space\n  Material\n}", "  Space\n  Finish\n  Material\n}"], ["  Usage\n  Swing", "  Usage\n  Surface\n  Swing"], ["  LayerMass\n}", "  LayerMass\n  FinishArea\n}"]], "FinishArea");
edit(json, [['        "Space",\n        "Material"\n', '        "Space",\n        "Finish",\n        "Material"\n'], ['        "Usage",\n        "Swing"', '        "Usage",\n        "Surface",\n        "Swing"'], ['        "LayerMass"\n      ]', '        "LayerMass",\n        "FinishArea"\n      ]']], "FinishArea");
edit(ts, [['"Railing" | "Space" | "Material";', '"Railing" | "Space" | "Finish" | "Material";'], ['"Number" | "Usage" | "Swing"', '"Number" | "Usage" | "Surface" | "Swing"'], ['"LayerVolume" | "LayerMass";', '"LayerVolume" | "LayerMass" | "FinishArea";']], "FinishArea");
