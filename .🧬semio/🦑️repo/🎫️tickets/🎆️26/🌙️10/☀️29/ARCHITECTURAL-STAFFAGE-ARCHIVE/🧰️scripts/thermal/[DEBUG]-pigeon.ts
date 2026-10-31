import { catalogAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🌡️thermal/🟦️.ts";
import { solveThermalAt } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const a = catalogAsset("pigeon");
for (const c of a.clips) for (const f of [0.1, 0.5, 0.9]) {
  const t = c.durationS * f * 3.1;
  const s = evaluate(a, { kind: "clip", clip: c.id, timeS: t });
  const r = solveThermalAt(a, { kind: "clip", clip: c.id, timeS: t });
  console.log("[DEBUG]", c.id, t.toFixed(2), "met", s.thermal.activityMet, "com", s.kinematics.comVelocity.x.toFixed(2), "core", r.body?.coreTempC.toFixed(2), "hc body", r.regions["plumage-body"]!.convectionCoefficientWM2K.toFixed(1), "surf", r.regions["plumage-body"]!.surfaceTempC.toFixed(1));
}
const p = a.poses.find((x) => x.id === "airborne-wings-up")!;
console.log("[DEBUG] airborne met", p.thermal.activityMet, p.thermal.breathing);
