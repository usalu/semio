/** 🏞️ Probe of work package Q: how many hops the terrain suite's grid grants and refuses at each coarseness, and how many perches the first `n` generated layouts hold. `bun wp_q_probe_terrain.ts`. */
import { hopOf, perchesOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🏞️terrain/🟦️.ts";

for (const coarse of [8, 4, 2, 1, 0.5, 0.25]) {
  let granted = 0;
  let refused = 0;
  for (let dx = -168; dx <= 168; dx += 6.5 * coarse) {
    for (let dy = -80; dy <= 230; dy += 5.25 * coarse) {
      if (hopOf({ x: 217.3, y: 301.7 }, { x: 217.3 + dx, y: 301.7 + dy }) === null) refused++;
      else granted++;
    }
  }
  process.stdout.write(`coarse ${coarse}: granted ${granted} (> ${1500 / (coarse * coarse)}) refused ${refused} (> ${500 / (coarse * coarse)})\n`);
}

let state = 20261002 >>> 0;
const next = (bound: number): number => {
  state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
  return Math.floor((state / 4294967296) * bound);
};
let perches = 0;
for (let layout = 0; layout < 3000; layout++) {
  const width = 320 + next(8) * 40;
  const height = 240 + next(6) * 40;
  const clearance = [0, 40.5, 48, 56, 61.7][next(5)]!;
  const minimum = [0, 24, 60.5, 72][next(4)]!;
  const grain = [1, 4, 10][layout % 3]!;
  const surfaces = Array.from({ length: 1 + next(5) }, (_, index) => {
    const x0 = next(2 * width * grain) / grain - width / 2;
    return { id: `s${index}`, x0, x1: x0 + 8 + next(width * grain) / grain, y: next((height + 40) * grain) / grain - 10 };
  });
  const keepouts = Array.from({ length: next(9) }, () => ({ x: next((width + 80) * grain) / grain - 40, y: next((height + 80) * grain) / grain - 40, width: next(6) === 0 ? 0 : next(160 * grain) / grain, height: next(7) === 0 ? 0 : next(120 * grain) / grain }));
  perches += perchesOf(surfaces, keepouts, width, height, clearance, minimum).length;
  if ([12, 24, 30, 40, 60, 300, 1000, 1500, 3000].includes(layout + 1)) process.stdout.write(`layouts ${layout + 1}: perches ${perches}\n`);
}
