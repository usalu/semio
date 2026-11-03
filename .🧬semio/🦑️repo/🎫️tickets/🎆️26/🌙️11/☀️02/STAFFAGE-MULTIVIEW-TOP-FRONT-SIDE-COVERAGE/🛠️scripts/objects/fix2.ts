import { fileOf, patchView } from "./patch.ts";

patchView(fileOf("🪑️objects", "chalkboard"), 500, "elevation", (shapes) => shapes.map((s) => (s.id === "tray" ? { ...s, region: "tray" } : s)));

const table = fileOf("🛋️furniture", "operating-table");
for (const band of [50, 100, 200]) {
  patchView(table, band, "front", (shapes) =>
    shapes.map((s) => (s.id === "st-base" && s.kind === "poly" ? { ...s, points: s.points.map(([x, y]: number[]) => [Math.abs(x) > 0.25 ? Math.sign(x) * 0.25 : x, y]) } : s)),
  );
  patchView(table, band, "plan", (shapes) => shapes.map((s) => (s.id === "st-base" && s.kind === "rect" ? { ...s, y: -0.25, h: 0.5 } : s)));
}
