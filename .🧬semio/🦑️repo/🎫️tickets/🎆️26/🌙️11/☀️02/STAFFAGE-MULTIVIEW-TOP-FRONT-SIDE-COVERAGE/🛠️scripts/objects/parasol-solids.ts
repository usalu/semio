import { fileOf, patchView } from "./patch.ts";

const f = fileOf("🪑️objects", "market-parasol");
const ribs = new Set(["rib-b", "rib-c", "rib-d", "rib-f", "rib-g", "rib-h"]);
const ext = (z: number) => ({ kind: "extruded", z0: -z, z1: z });
const suffix = (id: string): string => id.replace(/^rib-[bcdfgh]-?/, "");

for (const band of [50, 100, 200]) {
  patchView(f, band, "elevation", (shapes) =>
    shapes.map((s) => {
      if (s.solid !== undefined || !ribs.has(s.bone)) return s;
      const part = suffix(s.id);
      if (part === "rod") return { ...s, solid: ext(0.011) };
      if (part === "tip") return { ...s, solid: ext(0.013) };
      if (part.startsWith("scallop")) return { ...s, solid: ext(0.004) };
      return s;
    }),
  );
  patchView(f, band, "plan", (shapes) =>
    shapes.map((s) => {
      if (s.solid !== undefined || !ribs.has(s.bone) || s.kind !== "capsule") return s;
      const { followTip, ...rest } = s;
      return { ...rest, x0: 0, y0: 0, x1: 1.2374, y1: 0, solid: ext(0.011) };
    }),
  );
}
