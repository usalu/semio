import { fileOf, patchView } from "./patch.ts";
const f = fileOf("🛋️furniture", "bar-stool");
for (const [band, top, pen] of [["200", 0.7, 0.18], ["500", 0.66, undefined]] as const)
  for (const view of ["front", "elevation"])
    patchView(f, band, view, (s) => {
      if (s.some((x) => x.id === "leg-m")) return s;
      const i = s.findIndex((x) => x.id === "leg-r");
      const m: any = { id: "leg-m", bone: "frame", role: "body", z: 2, filled: false, tone: 0.3, region: "legs", kind: "poly", points: [[0, top], [0, 0]], closed: false };
      if (pen) m.penMm = pen;
      return [...s.slice(0, i + 1), m, ...s.slice(i + 1)];
    });
