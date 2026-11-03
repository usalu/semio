import { readFileSync, writeFileSync } from "node:fs";
let t = readFileSync("fix-symbols.ts", "utf8");
const rep = (a: string, b: string) => { if (!t.includes(a)) throw new Error("missing " + a); t = t.replace(a, b); };
rep('poly("slide-arrow", "track", "detail", [[2.05, -1.1], [2.9, -1.1]])', 'poly("slide-arrow", "track", "detail", [[2.05, 1], [2.9, 1]])');
rep('poly("slide-arrow-head", "track", "detail", [[2.82, -1.05], [2.9, -1.1], [2.82, -1.15]])', 'poly("slide-arrow-head", "track", "detail", [[2.82, 1.05], [2.9, 1], [2.82, 0.95]])');
rep('const outline = [[0, -0.05], [0.3, -0.05], [0.3, 1.2], [0, 1.2]];', 'const outline = [[-0.15, 0], [0.15, 0], [0.15, 1.25], [-0.15, 1.25]];');
rep('poly("sash-line", "sash", "glazing", [[0, 0], [1.2, 0]], false, { region: "pane-glass" }),', 'poly("sash-line", "sill", "glazing", [[0, 0.05], [0, 1.25]], false, { region: "pane-glass" }),');
rep('[poly("tilt-arrow", "sill", "detail", [[0.15, 1.15], [0.03, 1.15]]), poly("tilt-arrow-head", "sill", "detail", [[0.07, 1.19], [0.03, 1.15], [0.07, 1.11]])]', '[poly("tilt-arrow", "sill", "detail", [[0, 1.15], [-0.12, 1.15]]), poly("tilt-arrow-head", "sill", "detail", [[-0.08, 1.19], [-0.12, 1.15], [-0.08, 1.11]])]');
writeFileSync("fix-symbols.ts", t);
