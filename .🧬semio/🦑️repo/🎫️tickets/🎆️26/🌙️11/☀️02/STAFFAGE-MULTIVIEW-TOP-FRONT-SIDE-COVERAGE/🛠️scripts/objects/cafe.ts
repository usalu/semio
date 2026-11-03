import { fileOf, patchView } from "./patch.ts";
const f = fileOf("🛋️furniture", "cafe-chair");
const radius: Record<number, number> = { 100: 0.014, 200: 0.015, 500: 0.016 };
for (const band of [50, 100, 200, 500]) {
  patchView(f, band, "plan", (shapes) => shapes.map((s) => {
    const leg = s.id.startsWith("leg-front") || s.id.startsWith("leg-rear");
    if (leg && s.kind === "poly") {
      const [a, b] = s.points as [number, number][];
      const [foot, top] = Math.abs(a[1]) > Math.abs(b[1]) ? [a, b] : [b, a];
      const { points, closed, ...rest } = s;
      return { ...rest, kind: "capsule", x0: foot[0], y0: foot[1], x1: top[0], y1: top[1], r0: radius[band], r1: radius[band], followTip: true };
    }
    if (leg && s.kind === "capsule" && s.id.includes("tube") && Math.abs(s.y0) < Math.abs(s.y1)) {
      return { ...s, x0: s.x1, y0: s.y1, x1: s.x0, y1: s.y0, r0: s.r1, r1: s.r0 };
    }
    return s;
  }));
}
patchView(f, 500, "front", (shapes) => {
  if (shapes.some((s) => s.id === "leg-rear-0")) return shapes;
  const ref = shapes.find((s) => s.id === "leg-front-0");
  const mk = (id: string, sign: number) => ({ ...ref, id, bone: "leg-rear", z: 2.5, points: [[0.23 * sign, 0], [0.18 * sign, 0.4]] });
  const i = shapes.findIndex((s) => s.id === "leg-front-1");
  return [...shapes.slice(0, i + 1), mk("leg-rear-0", 1), mk("leg-rear-1", -1), ...shapes.slice(i + 1)];
});
