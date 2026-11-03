import { readFileSync, writeFileSync } from "node:fs";
import { fileOf, patchView } from "./patch.ts";
const f = fileOf("🪑️objects", "charcoal-grill");
for (const band of [50, 100, 200, 500]) for (const view of ["front", "plan"]) {
  patchView(f, band, view, (shapes) => shapes.map((s) => {
    if (s.solid?.kind !== "revolved" || s.solid.axisBone !== "lid-axis" || s.bone === "lid-axis") return s;
    return { ...s, bone: "lid-axis", points: s.points.map(([px, py]: number[]) => [+(py - 0.2).toFixed(5), +(0.2925 - px).toFixed(5)]) };
  }));
}
const text = readFileSync(f, "utf8");
const next = text.replace(/("id": "lid-vent-flue"[\s\S]{0,400}?"directionDeg": )90/, "$10");
if (next === text) throw new Error("flue not patched");
writeFileSync(f, next);
