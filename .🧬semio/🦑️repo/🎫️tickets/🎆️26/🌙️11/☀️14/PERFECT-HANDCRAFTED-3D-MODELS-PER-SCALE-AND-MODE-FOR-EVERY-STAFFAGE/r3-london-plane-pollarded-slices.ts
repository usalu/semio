/** 📏 Round 3 london plane: half extents of the 1:50 foliage per height slice (x, z, and the mean radius of the plan hull), the measured profile the 1:500 crown sections are entered from. */
import { meshesAt } from "./volumes-lib.ts";
import { REGS, asset, build } from "./r3-london-plane-pollarded-model.ts";
const lists = build();
const ms = meshesAt({ ...asset, volumes: { "50": { status: "authored", denominator: 50, ...lists["50"]! } } }, 50, lists["50"]!.body.filter((s) => REGS["50"]![s.id]?.group !== undefined));
for (let y = 4.4; y < 9; y += 0.25) {
  let x0 = 9, x1 = -9, z0 = 9, z1 = -9;
  for (const m of ms) for (let i = 0; i < m.world.length; i += 3) if (Math.abs(m.world[i + 1]! - y) < 0.125) { x0 = Math.min(x0, m.world[i]!); x1 = Math.max(x1, m.world[i]!); z0 = Math.min(z0, m.world[i + 2]!); z1 = Math.max(z1, m.world[i + 2]!); }
  console.log(y.toFixed(2), "x", x0.toFixed(2), x1.toFixed(2), "z", z0.toFixed(2), z1.toFixed(2));
}
