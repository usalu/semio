import { fileOf, patchBones, patchView } from "./patch.ts";

const grill = fileOf("🪑️objects", "charcoal-grill");
for (const band of [50, 100, 200, 500])
  for (const view of ["front", "plan"])
    patchView(grill, band, view, (shapes) =>
      shapes.map((s) => {
        if (s.solid?.kind !== "revolved" || s.solid.axisBone !== "lid-axis") return s;
        return {
          ...s,
          bone: "lid",
          points: s.points.map(([a, b]: number[]) => [+(0.2925 - b).toFixed(5), +(0.2 + a).toFixed(5)]),
          solid: { ...s.solid, axisBone: "lid", axis: "v", axisX: 0.2925 },
        };
      }),
    );
patchBones(grill, (bones) => bones.filter((b) => b.id !== "lid-axis"));

const stool = fileOf("🛋️furniture", "bar-stool");
for (const band of [50, 100])
  for (const view of ["elevation", "front", "plan"])
    patchView(stool, band, view, (shapes) =>
      shapes.map((s) => (s.solid?.kind === "revolved" && s.solid.axisBone === "pad-axis" ? { ...s, solid: { ...s.solid, axisBone: "pad", axis: "v", axisX: 0 } } : s)),
    );
patchBones(stool, (bones) => bones.filter((b) => b.id !== "pad-axis"));
