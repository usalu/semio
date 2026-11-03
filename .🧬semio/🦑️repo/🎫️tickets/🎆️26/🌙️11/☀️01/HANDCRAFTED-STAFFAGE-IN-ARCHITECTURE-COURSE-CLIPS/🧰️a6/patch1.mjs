import { readFileSync, writeFileSync } from "node:fs";
const f = process.argv[2];
let s = readFileSync(f, "utf8");
const rep = (a, b) => {
  if (!s.includes(a)) throw new Error("missing: " + a.slice(0, 60));
  s = s.replace(a, b);
};
rep(`"M 0.58,1.64 L -1.52,1.64 L -1.52,1.7 L 0.58,1.7 Z"`, `"M 0.62,1.63 L 0.62,1.7 L -1.5,1.7 C -1.6,1.7 -1.66,1.66 -1.7,1.58 L -1.52,1.63 Z"`);
rep(`"M 0.0,1.21 L 0.02,1.48 C 0.06,1.52 0.14,1.5 0.15,1.44 L 0.17,1.21 Z"`, `"M -0.02,1.21 L 0.0,1.44 L 0.04,1.5 L 0.12,1.5 L 0.16,1.44 L 0.18,1.21 Z"`);
rep(`"M -0.95,1.21 L -0.93,1.46 C -0.89,1.5 -0.81,1.48 -0.8,1.42 L -0.78,1.21 Z"`, `"M -0.97,1.21 L -0.95,1.44 L -0.91,1.5 L -0.83,1.5 L -0.79,1.44 L -0.77,1.21 Z"`);
rep(`s.push(path("steering-wheel", c, "detail", 6.5, false, 0.3, "M 0.36,1.3 L 0.3,1.5", { penMm: 0.25 }));`, `s.push(capsule("steering-column", c, "detail", 6.5, true, 0.3, 0.42, 1.24, 0.33, 1.36, 0.01, 0.01, { material: "exhaust-steel" }));
    s.push(ellipse("steering-wheel", c, "detail", 6.6, false, 0.3, 0.31, 1.39, 0.016, 0.1, { material: "exhaust-steel", rotationDeg: 0 }));`);
rep(`rect("radiator-shell", c, "body", 2.6, true, 0.78, 1.78, 0.56, 0.1, 0.74,`, `rect("radiator-shell", c, "body", 2.6, true, 0.78, 1.78, 0.56, 0.1, 0.7,`);
rep(`1.83, 1.3, 1.83, 1.34, 0.018`, `1.83, 1.26, 1.83, 1.3, 0.018`);
rep(`1.83, 1.375, 0.05, 0.032`, `1.83, 1.335, 0.05, 0.032`);
writeFileSync(f, s);
