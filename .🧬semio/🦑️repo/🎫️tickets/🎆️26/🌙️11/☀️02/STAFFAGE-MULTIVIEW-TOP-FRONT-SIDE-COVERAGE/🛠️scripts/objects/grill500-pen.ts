import { fileOf, patchView } from "./patch.ts";

const f = fileOf("🪑️objects", "charcoal-grill");
for (const view of ["front", "plan"])
  patchView(f, 500, view, (shapes) => shapes.map((s) => (s.id === "side-handle-l" || s.id === "side-handle-r" ? { ...s, penMm: 0.2 } : s)));
