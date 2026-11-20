/** 📐 Round 3, vegetation/agave: extents of the stated rosette (margins, keel, wing tops, spines) against `dimensions`, and tip heights per leaf. Usage: bun r3-agave-extent.ts */
import { LEAVES, blade, halfWidth, thickness, gammaDeg, SPINE, SPEAR } from "./r3-agave-plant.ts";
import { add, mul, D2R } from "./r3-agave-kit.ts";
const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9];
const put = (p: number[]) => p.forEach((x, i) => { mn[i] = Math.min(mn[i]!, x); mx[i] = Math.max(mx[i]!, x); });
for (const l of LEAVES) {
  const b = blade(l);
  for (let i = 0; i <= 100; i++) {
    const s = i / 100, c = b.at(s), n = b.normal(s), hw = halfWidth(l, s), t = thickness(l, s), g = Math.tan(gammaDeg(s) * D2R);
    for (const sg of [-1, 1]) put(add(add(c, mul(b.side, sg * hw)), mul(n, hw * g + 0.3 * t)));
    put(add(c, mul(n, -0.6 * t)));
  }
  const tipS = add(b.tip, mul(b.tangent(1), SPINE.length));
  put(tipS);
  console.log(`[DEBUG] leaf ${l.k} az ${l.az} tip ${tipS.map((x) => x.toFixed(3)).join(",")} reach ${Math.hypot(tipS[0], tipS[2]).toFixed(3)}`);
}
put([0, SPEAR.base + SPEAR.length, 0]);
console.log(`[DEBUG] x ${mn[0]!.toFixed(3)}..${mx[0]!.toFixed(3)} = ${(mx[0]! - mn[0]!).toFixed(3)} (2.5)  y ${mn[1]!.toFixed(3)}..${mx[1]!.toFixed(3)} (1.445)  z ${mn[2]!.toFixed(3)}..${mx[2]!.toFixed(3)} = ${(mx[2]! - mn[2]!).toFixed(3)} (2.373)`);
