/** 📐️ Round 3, vegetation/hosta-clump: extents of the stated clump (margins, midrib underside, petioles) against `dimensions`, and junction and tip of every leaf. Usage: bun r3-hosta-clump-extent.ts */
import { ALL, OUTER, STATIONS, outerWavy, wingPoint } from "./r3-hosta-clump-plant.ts";
import { add, mul } from "./r3-agave-kit.ts";
const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9];
const put = (p: number[]) => p.forEach((x, i) => { mn[i] = Math.min(mn[i]!, x); mx[i] = Math.max(mx[i]!, x); });
for (const b of ALL) {
  let low = 1e9, top = 0;
  for (const s of STATIONS) for (const sg of [1, -1] as const) {
    const p = wingPoint(b, s, sg, outerWavy(b.leaf, s), 0.003);
    put(p);
    top = Math.max(top, p[1]);
    low = Math.min(low, p[1], add(b.mid.at(s), mul(b.mid.normal(s), -0.006))[1]!);
  }
  for (let i = 0; i <= 10; i++) put(b.pet.at(i / 10));
  console.log(`[DEBUG] leaf ${b.leaf.k} ${b.leaf.tier} az ${b.leaf.az} j r ${Math.hypot(b.j[0], b.j[2]).toFixed(3)} y ${b.j[1].toFixed(3)} tip r ${Math.hypot(b.tip[0], b.tip[2]).toFixed(3)} y ${b.tip[1].toFixed(3)} low ${low.toFixed(3)} top ${top.toFixed(3)} elA ${b.elA.toFixed(1)} elB ${b.elB.toFixed(1)}`);
}
console.log(`[DEBUG] x ${mn[0]!.toFixed(3)}..${mx[0]!.toFixed(3)} = ${(mx[0]! - mn[0]!).toFixed(3)} (0.876)  y ${mn[1]!.toFixed(3)}..${mx[1]!.toFixed(3)} (0.618)  z ${mn[2]!.toFixed(3)}..${mx[2]!.toFixed(3)} = ${(mx[2]! - mn[2]!).toFixed(3)} (0.901) ${OUTER.length}`);
