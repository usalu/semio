import { fileOf, patchView } from "./patch.ts";
const f = fileOf("🪑️objects", "bicycle-stand");
patchView(f, 500, "front", (shapes) =>
  shapes.map((s) => (s.id === "hoop-end-on" ? { ...s, d: "M -0.02415 0 L -0.02415 0.77585 C -0.02415 0.78919 -0.01334 0.8 0 0.8 C 0.01334 0.8 0.02415 0.78919 0.02415 0.77585 L 0.02415 0 Z" } : s)),
);
patchView(f, 500, "plan", (shapes) =>
  shapes.map((s) => (s.id === "hoop-top" ? { ...s, x0: -0.3508, x1: 0.3508, y0: 0, y1: 0, r0: 0.02415, r1: 0.02415 } : s)),
);
