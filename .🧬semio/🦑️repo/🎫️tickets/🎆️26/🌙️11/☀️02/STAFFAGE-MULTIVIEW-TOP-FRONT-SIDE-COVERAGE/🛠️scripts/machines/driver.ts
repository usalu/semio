import { load, save, rep } from "./lib.ts";
const d = load("city-car");
const G = rep(d, 50).geometry;
const meta = { bone: "chassis", role: "detail", filled: false, tone: 0.2, material: "interior-trim", region: "interior" };
G.front.push({ kind: "ellipse", cx: 0.29, cy: 1.08, rx: 0.085, ry: 0.08, id: "steering-wheel", z: 3.1, ...meta });
G.plan.push({ kind: "ellipse", cx: 0.56, cy: 0.29, rx: 0.03, ry: 0.085, rotationDeg: 0, id: "steering-wheel", z: 3.1, ...meta });
const hs = [["l", 0.2], ["r", -0.38]] as const;
for (const [s, y] of hs) G.plan.push({ kind: "rect", x: -0.01, y, w: 0.05, h: 0.18, r: 0.02, id: `seat-front-headrest-${s}`, z: 3, ...meta, filled: true, tone: 0.3 });
save("city-car", d);
