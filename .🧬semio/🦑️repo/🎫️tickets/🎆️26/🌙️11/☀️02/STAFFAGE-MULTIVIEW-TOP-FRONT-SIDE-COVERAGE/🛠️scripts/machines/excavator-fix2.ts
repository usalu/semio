import { load, save, rep } from "./lib.ts";
const d = load("excavator");
for (const k of [50, 100, 200, 500]) {
  const G = rep(d, k).geometry;
  for (const s of G.front) if (s.id === "cab-pillar") s.solid = { kind: "extruded", z0: 0.665, z1: 0.685 };
  const srcId = k === 500 ? "house-counterweight" : "counterweight-body";
  const e = G.elevation.find((s: any) => s.id === srcId);
  const p = G.plan.find((s: any) => s.id === srcId);
  if (!G.front.some((s: any) => s.id === srcId)) {
    const copy = structuredClone(e);
    copy.z = (p?.z ?? e.z) - 0.5;
    G.front.unshift(copy);
  }
}
save("excavator", d);
