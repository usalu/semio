import { readFileSync, writeFileSync } from "node:fs";
const f = process.argv[2];
let s = readFileSync(f, "utf8");
const rep = (a, b) => {
  if (!s.includes(a)) throw new Error("missing: " + a.slice(0, 60));
  s = s.replace(a, b);
};
rep(`s.push(rect("plan-grille", u, "detail", 2,`, `s.push(rect("plan-grille", u, sc === 200 ? "body" : "detail", 2,`);
rep(`  s.push(path("vane", "vane", "body", 2, true, 0.6, VANE, { region: "discharge-vane", material: "abs-grey" }));
  s.push(rect("intake-grille", u, "detail", 2, true, 0.3, 0.03, 0.268, 0.11, 0.022,`, `  s.push(path("vane", "vane", "body", 2, true, 0.6, sc === 100 ? VANE_100 : VANE, { region: "discharge-vane", material: "abs-grey" }));
  s.push(rect("intake-grille", u, "detail", 2, true, 0.3, sc === 100 ? 0.025 : 0.03, sc === 100 ? 0.264 : 0.268, sc === 100 ? 0.12 : 0.11, sc === 100 ? 0.026 : 0.022,`);
rep(`const elevation = (sc) => {`, `const VANE_100 = "M 0,-0.017 L 0.11,-0.017 C 0.12,-0.017 0.127,-0.005 0.127,0.017 L 0.12,0.017 L 0,0.017 Z";

const elevation = (sc) => {`);
writeFileSync(f, s);
