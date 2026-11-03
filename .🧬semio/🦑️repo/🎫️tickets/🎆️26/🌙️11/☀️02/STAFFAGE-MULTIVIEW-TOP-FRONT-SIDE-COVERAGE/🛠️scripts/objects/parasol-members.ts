import { fileOf, patchView } from "./patch.ts";

const f = fileOf("🪑️objects", "market-parasol");
const ribs = new Set(["rib-b", "rib-c", "rib-d", "rib-f", "rib-g", "rib-h"]);
const member = (s: any) => {
  const { x0, y0, x1, y1, r0, r1, followTip, solid, points, closed, d, ...rest } = s;
  return { ...rest, kind: "member", from: { x: 0, y: 0, z: 0 }, to: { x: 1.2374, y: 0, z: 0 }, section: { kind: "circle", d: 0.018 } };
};
for (const band of [50, 100, 200]) {
  patchView(f, band, "elevation", (shapes) => shapes.map((s) => (ribs.has(s.bone) && /^rib-[bcdfgh]-rod$/.test(s.id) ? member(s) : s)));
  patchView(f, band, "plan", (shapes) => shapes.map((s) => (ribs.has(s.bone) && /^rib-[bcdfgh]$/.test(s.id) ? member(s) : s)));
}
