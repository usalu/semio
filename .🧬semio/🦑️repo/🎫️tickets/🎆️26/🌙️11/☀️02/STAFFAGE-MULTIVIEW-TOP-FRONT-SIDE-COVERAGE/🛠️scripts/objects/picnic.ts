import { fileOf, patchView } from "./patch.ts";
const f = fileOf("🛋️furniture", "picnic-table");
for (const view of ["front", "plan"]) patchView(f, 500, view, (shapes) => shapes.map((s) => s.id.startsWith("bench-") && s.kind === "poly" && s.closed === false ? { ...s, closed: true, points: [[-0.11, -0.02], [0.3, -0.02], [0.3, 0.02], [-0.11, 0.02]] } : s));
