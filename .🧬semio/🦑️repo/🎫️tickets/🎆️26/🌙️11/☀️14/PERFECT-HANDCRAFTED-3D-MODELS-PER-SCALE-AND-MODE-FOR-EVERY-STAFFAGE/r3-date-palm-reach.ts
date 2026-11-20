/** 📏️ Prints the reach of every frond of the round-3 date palm in the calm pose: largest |x|, |z| and height of rachis, vane edges and leaflet tips, to fit the crown to `dimensions`. */
import { join } from "node:path";
import { STAFFAGE, entryOf, type Frame, type Json } from "./volumes-lib.ts";
import { add, mul, unit } from "./r3-date-palm-kit.ts";
import { fixRig } from "./r3-date-palm-rig-fix.ts";
import { FrondGeom } from "./r3-date-palm-parts.ts";
const { solveChain } = (await import(join(STAFFAGE, "🔨️modules/🦴️rig/🟦️.ts"))) as { solveChain: (rig: Json, o: Json, r: Json) => Record<string, { frame: Frame }> };
const { asset } = entryOf("vegetation/date-palm");
const { fronds } = fixRig(asset);
const s = solveChain(asset.rig, asset.poses.find((p: Json) => p.id === "calm").bones, { x: 0, y: 0, angleDeg: 0 });
let [X0, X1, Z0, Z1, Y1] = [0, 0, 0, 0, 0];
for (const f of fronds) {
  const g = new FrondGeom(f, s[`${f.id}a`]!.frame, s[`${f.id}b`]!.frame);
  let [x0, x1, z0, z1, y1, R] = [0, 0, 0, 0, 0, 0];
  const ax = s["crown"]!.frame.o;
  for (let S = 0; S <= g.L; S += 0.02) {
    const q = g.at(S);
    const pts = [q.p];
    if (S >= 0.2 * g.L) for (const side of [1, -1]) {
      const e = g.e(S, side);
      pts.push(add(q.p, mul(e, g.vaneReach(S, 0.84))));
    }
    for (const p of pts) { R = Math.max(R, Math.hypot(p[0] - ax[0], p[2] - ax[2])); x0 = Math.min(x0, p[0]); x1 = Math.max(x1, p[0]); z0 = Math.min(z0, p[2]); z1 = Math.max(z1, p[2]); y1 = Math.max(y1, p[1]); }
  }
  [X0, X1, Z0, Z1, Y1] = [Math.min(X0, x0), Math.max(X1, x1), Math.min(Z0, z0), Math.max(Z1, z1), Math.max(Y1, y1)];
  console.log(f.k, f.id, f.tier, "az", f.az, "x", x0.toFixed(2), x1.toFixed(2), "z", z0.toFixed(2), z1.toFixed(2), "ytop", y1.toFixed(2), "L", (f.La + f.Lb).toFixed(2), "R", R.toFixed(3), "fit", (f.tier === "A" || f.tier === "B" ? (1 / Math.hypot(Math.cos((f.az * Math.PI) / 180) / 3.09, Math.sin((f.az * Math.PI) / 180) / 3.475) - f.baseR) / (R - f.baseR) : 1).toFixed(4));
}
console.log("crown x", X0.toFixed(3), X1.toFixed(3), "z", Z0.toFixed(3), Z1.toFixed(3), "top", Y1.toFixed(3));
