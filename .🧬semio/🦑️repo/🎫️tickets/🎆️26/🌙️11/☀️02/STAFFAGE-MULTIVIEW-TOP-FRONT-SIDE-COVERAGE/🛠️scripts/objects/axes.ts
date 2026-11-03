import { fileOf, patchBones, patchView } from "./patch.ts";
const fix = (file: string, boneShapes: string, from: string, to: string, bone: any, after: string, views: [number, string][]) => {
  patchBones(file, (bones) => (bones.some((b) => b.id === to) ? bones : bones.flatMap((b) => (b.id === after ? [b, bone] : [b]))));
  for (const [band, view] of views) patchView(file, band, view, (shapes) => shapes.map((s) => (s.bone === boneShapes && s.solid?.axisBone === from ? { ...s, solid: { ...s.solid, axisBone: to } } : s)));
};
const bands = [50, 100, 200, 500];
const views = (list: number[]): [number, string][] => list.flatMap((b) => ["elevation", "front", "plan"].map((v) => [b, v] as [number, string]));
fix(fileOf("🪑️objects", "charcoal-grill"), "lid", "lid-vent", "lid-axis", { id: "lid-axis", parent: "lid", title: { en: "Lid axis of revolution (fixed)", de: "Deckel-Rotationsachse (fest)" }, x: 0.2925, y: 0.2, angleDeg: 90, lengthM: 0.07, joint: { type: "fixed" } }, "lid-vent", views(bands));
fix(fileOf("🛋️furniture", "bar-stool"), "pad", "seat", "pad-axis", { id: "pad-axis", parent: "pad", title: { en: "Seat pad axis of revolution (fixed)", de: "Sitzpolster-Rotationsachse (fest)" }, x: 0, y: 0, angleDeg: 90, lengthM: 0.05, joint: { type: "fixed" } }, "pad", views(bands));
