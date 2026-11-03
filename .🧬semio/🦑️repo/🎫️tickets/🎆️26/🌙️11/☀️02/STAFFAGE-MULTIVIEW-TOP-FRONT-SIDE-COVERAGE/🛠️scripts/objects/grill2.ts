import { fileOf, patchView } from "./patch.ts";
const f = fileOf("🪑️objects", "charcoal-grill");
for (const band of [50, 100, 200]) {
  let dial: any;
  patchView(f, band, "elevation", (shapes) => { dial = shapes.find((s) => s.id === "vent-dial"); return shapes; });
  patchView(f, band, "front", (shapes) => shapes.filter((s) => !s.id.startsWith("vent-hole")).map((s) => (s.id === "vent-dial" ? { ...dial, solid: { kind: "extruded", z0: -0.04, z1: 0.04 } } : s)));
}
