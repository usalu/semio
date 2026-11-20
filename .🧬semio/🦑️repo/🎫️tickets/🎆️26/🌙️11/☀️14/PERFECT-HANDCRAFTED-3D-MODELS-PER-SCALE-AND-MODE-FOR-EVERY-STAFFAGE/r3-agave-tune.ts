/** 🎯 Round 3, vegetation/agave: solves, for the lowest eight leaves, the tip elevation e1 that puts the spine tip at a stated height above the bed and the length that puts it on the rosette ellipse of `dimensions`; prints the values that are then written into the table of `r3-agave-plant.ts` by hand. Usage: bun r3-agave-tune.ts */
import { LEAVES, blade, SPINE } from "./r3-agave-plant.ts";
import { add, mul, D2R } from "./r3-agave-kit.ts";
const E0 = [15, 17, 14, 18, 16, 19, 17, 20];
const TIPY = [0.03, 0.05, 0.04, 0.07, 0.06, 0.09, 0.08, 0.12];
const R = (az: number, fx: number, fz: number) => 1 / Math.hypot(Math.cos(az * D2R) / fx, Math.sin(az * D2R) / fz);
for (const l0 of LEAVES.slice(0, 8)) {
  let l = { ...l0, e0: E0[l0.k]! };
  for (let it = 0; it < 60; it++) {
    const b = blade(l);
    const tip = add(b.tip, mul(b.tangent(1), SPINE.length));
    const reach = Math.hypot(tip[0], tip[2]);
    const want = R(l.az, 1.296, 1.236);
    l = { ...l, e1: l.e1 + (TIPY[l.k]! - tip[1]) * 30, L: l.L * Math.pow(want / reach, 0.8) };
  }
  const b = blade(l);
  const tip = add(b.tip, mul(b.tangent(1), SPINE.length));
  console.log(`[DEBUG] k ${l.k} e0 ${l.e0} e1 ${l.e1.toFixed(1)} L ${l.L.toFixed(3)} tip ${tip.map((x) => x.toFixed(3)).join(",")}`);
}
