import { fileOf, patchView } from "./patch.ts";

const f = fileOf("🪑️objects", "charcoal-grill");
for (const band of [50, 100, 200, 500])
  patchView(f, band, "plan", (shapes) => {
    if (shapes.some((s) => s.id === "charcoal-bed")) return shapes;
    const bed = { id: "charcoal-bed", bone: "charcoal", role: "body", z: 3.05, filled: true, tone: 0.25, kind: "ellipse", cx: 0, cy: 0, rx: 0.24, ry: 0.24, region: "charcoal-bed" };
    const ring = { id: "cooking-grate", bone: "grate", role: "body", z: 3.1, filled: false, tone: 0.5, kind: "ellipse", cx: 0, cy: 0, rx: 0.27, ry: 0.27, region: "cooking-grate" };
    const bars =
      band === 500
        ? []
        : [-0.18, -0.09, 0, 0.09, 0.18].map((y, i) => ({ id: `cooking-grate-bar-${i}`, bone: "grate", role: "detail", z: 3.1, filled: false, tone: 0.5, kind: "poly", points: [[-Math.sqrt(0.27 ** 2 - y ** 2), y], [Math.sqrt(0.27 ** 2 - y ** 2), y]], closed: false, region: "cooking-grate" }));
    const at = shapes.findIndex((s) => s.id === "bowl");
    return [...shapes.slice(0, at + 1), bed, ring, ...bars.map((b) => ({ ...b, points: b.points.map(([x, y]: number[]) => [+x!.toFixed(4), y]) })), ...shapes.slice(at + 1)];
  });
