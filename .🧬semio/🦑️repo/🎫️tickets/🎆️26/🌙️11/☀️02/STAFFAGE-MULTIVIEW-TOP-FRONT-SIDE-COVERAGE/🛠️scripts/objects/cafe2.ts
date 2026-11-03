import { fileOf, patchView } from "./patch.ts";
const f = fileOf("🛋️furniture", "cafe-chair");
patchView(f, 100, "plan", (shapes) => shapes.map((s) => (s.id.startsWith("back-post") && s.kind === "poly" ? (() => { const { points, closed, ...rest } = s; const [[x0, y0], [x1, y1]] = points; return { ...rest, kind: "capsule", x0, y0, x1, y1, r0: 0.0125, r1: 0.0125, followTip: true }; })() : s)));
patchView(f, 500, "plan", (shapes) => shapes.map((s) => (s.id === "back-post" ? { id: "back-panel", bone: "back", role: "body", z: 4, filled: false, tone: s.tone ?? 0.4, kind: "capsule", x0: 0.215, y0: -0.006, x1: 0.385, y1: -0.006, r0: 0.016, r1: 0.016, solid: { kind: "extruded", z0: -0.19, z1: 0.19 } } : s)));
