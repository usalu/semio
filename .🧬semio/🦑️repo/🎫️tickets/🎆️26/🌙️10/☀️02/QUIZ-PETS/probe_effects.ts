/** 🔎️ Probe of work package A8: the golden words of the effects hash, how many particles each motion keeps alive over a long run, and how far a lift moves its copy. `bun TK/probe_effects.ts` from the repository root. */
import { EMITTER_CAP, emitterEnds, lifeTicks, lowbias32, mix, particlesOf, periodOf, unit } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/✨️effects/🟦️.ts";
import { LIFT_TICKS, liftAt, liftEnds } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🪄️mischief/🟦️.ts";

const hex = (word: number) => `0x${word.toString(16).padStart(8, "0")}`;
console.log("lowbias32(1)", hex(lowbias32(1)), "expected 0x688990c0");
console.log("lowbias32(0xdeadbeef)", hex(lowbias32(0xdeadbeef)), "expected 0xe628c683");
console.log("mix(0,0)", hex(mix(0, 0)), "expected 0xe577f3aa");
console.log("mix(1,2)", hex(mix(1, 2)), "expected 0xb111e030");
console.log("mix(mix(7,3),5)", hex(mix(mix(7, 3), 5)), "expected 0x5ab36a78");

let sum = 0;
let squares = 0;
const draws = 200000;
for (let index = 0; index < draws; index++) {
  const value = unit(mix(12345, index));
  sum += value;
  squares += value * value;
}
console.log("mean", (sum / draws).toFixed(4), "variance", (squares / draws - (sum / draws) ** 2).toFixed(4), "(uniform: 0.5, 0.0833)");

for (const motion of ["fall", "rise", "burst", "orbit", "drift"] as const) {
  for (const [count, life] of [[1, 0.4], [6, 1], [12, 1.6], [32, 0.25], [32, 1.6]] as const) {
    const emitter = { motion, count, life, speed: 60, spread: 0.25 };
    let most = 0;
    let total = 0;
    let reach = 0;
    const until = 2000;
    for (let tick = 0; tick < 2400; tick++) {
      const particles = particlesOf(emitter, { x: 0, y: 0 }, 1, 0, until, tick, 99);
      most = Math.max(most, particles.length);
      total += particles.length;
      for (const particle of particles) reach = Math.max(reach, Math.abs(particle.x), Math.abs(particle.y));
    }
    console.log(motion.padEnd(6), `count ${String(count).padStart(2)} life ${life} → ${lifeTicks(emitter)} ticks, period ${periodOf(emitter)}, most alive ${most}, mean alive ${(total / 2000).toFixed(2)}, reach ${reach.toFixed(1)} px, ends ${emitterEnds(emitter, 0, until)}, cap ${EMITTER_CAP}`);
  }
}

let far = 0;
let high = 0;
let tilted = 0;
for (let tick = -5; tick < LIFT_TICKS + 5; tick++) {
  const lift = liftAt(0, tick, 1, 40, 880, 0.5);
  far = Math.max(far, Math.abs(lift.dx));
  high = Math.max(high, Math.abs(lift.dy));
  tilted = Math.max(tilted, Math.abs(lift.tilt));
}
console.log("lift: ticks", LIFT_TICKS, "ends", liftEnds(0), "farthest", far.toFixed(3), "px, highest", high.toFixed(3), "px, most tilt", tilted.toExponential(3), "turns");
