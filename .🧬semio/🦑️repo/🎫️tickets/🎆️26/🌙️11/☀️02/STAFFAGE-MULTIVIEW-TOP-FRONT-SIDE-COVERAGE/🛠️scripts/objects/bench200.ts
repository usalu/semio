import { fileOf, patchView } from "./patch.ts";
patchView(fileOf("🛋️furniture", "park-bench"), 200, "front", (shapes) =>
  shapes.map((s) => (s.id === "l-rear-post" ? { ...s, points: [[0.174, 0.4], [0.206, 0.4], [0.3, 0.85], [0.268, 0.85]] } : s)),
);
