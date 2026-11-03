import { fileOf, patchView } from "./patch.ts";

const f = fileOf("🪑️objects", "market-parasol");
for (const band of [100, 200])
  patchView(f, band, "elevation", (shapes) =>
    shapes.map((s) => {
      if (!/^rib-[bcdfgh]-rod$/.test(s.id) || s.kind !== "poly") return s;
      const { points, closed, ...rest } = s;
      const [[x0, y0], [x1, y1]] = points as [number, number][];
      return { ...rest, kind: "capsule", filled: false, x0, y0, x1, y1, r0: 0.011, r1: 0.007 };
    }),
  );
