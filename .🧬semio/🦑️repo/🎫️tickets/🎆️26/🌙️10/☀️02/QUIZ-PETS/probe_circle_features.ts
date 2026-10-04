/** 🔬️ Ticket tool: measures every lap a corpus contains, before any judgement of its quality, so that the gates of the circle detector can be chosen from the laps of deliberate circles and of ordinary travel.
 *
 * Usage (from the repository root, on a corpus the survey of `generate_gesture_vectors.py` left in the scratch folder):
 *   bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/probe_circle_features.ts" <corpus.json> <laps.json> [quarters]
 *
 * The lap logic is a copy of `circleStep` (band, Schmitt trigger, quadrants, stall, steps against the direction)
 * without the two quality gates and without the rest after a circle. Per lap that reaches `quarters` (5 when absent)
 * net quarter turns it records: the trace, the body, the tick, the direction, the ticks it took, its quarter turns,
 * the greatest over the least distance from the centre (`round`), the distance at its end over the distance at its
 * first quarter turn (`close`), its longest over its mean interval between quarter turns (`uneven`), and its path
 * length over the circumference of its mean distance (`winding`).
 */
import { readFileSync, writeFileSync } from "node:fs";
import { type Box, trailOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧪️tests/👆️gesture-recognition/🟦️.ts";
import { CIRCLE_AGAINST, CIRCLE_HYSTERESIS, CIRCLE_MARGIN, CIRCLE_OUT_TICKS, CIRCLE_REACH, CIRCLE_SLOW } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/👆️gesture/🟦️.ts";

type Corpus = { readonly quantum: number; readonly hold: number; readonly hovers: readonly { readonly id: string; readonly bodies: readonly Box[]; readonly path: readonly number[] }[] };
type Lap = { trace: string; body: number; tick: number; turn: number; ticks: number; steps: number; round: number; close: number; uneven: number; winding: number };

const [inputs, outputs, wanted] = process.argv.slice(2);
if (inputs === undefined || outputs === undefined) throw new Error("usage: probe_circle_features.ts <corpus.json> <laps.json> [quarters]");
const QUARTERS = wanted === undefined ? 5 : Number(wanted);
const corpus = JSON.parse(readFileSync(inputs, "utf8")) as Corpus;
const laps: Lap[] = [];

/** 🧭️ The quadrant of two signs in clockwise order as seen on screen. */
function quadrantOf(sx: number, sy: number): number {
  return sx > 0 ? (sy < 0 ? 0 : 1) : sy > 0 ? 2 : 3;
}

for (const trace of corpus.hovers) {
  const trail = trailOf(trace.path, corpus.hold);
  trace.bodies.forEach((box, body) => {
    const cx = (box[0] + box[2] / 2) / corpus.quantum;
    const cy = (box[1] + box[3] / 2) / corpus.quantum;
    const reach = Math.max(box[2], box[3]) / corpus.quantum;
    const inner = (reach / 2 + CIRCLE_MARGIN) ** 2;
    const outer = (reach * CIRCLE_REACH) ** 2;
    let live = false;
    let inside = 0;
    let sx = 0;
    let sy = 0;
    let px = 0;
    let py = 0;
    let lap = { turn: 0, quarters: 0, steps: 0, against: 0, first: 0, last: 0, near: 0, far: 0, open: 0, widest: 0, length: 0, sum: 0, samples: 0 };
    for (let tick = 0; tick < trail.xs.length; tick++) {
      const rx = trail.xs[tick]! / corpus.quantum - cx;
      const ry = trail.ys[tick]! / corpus.quantum - cy;
      const radius = rx * rx + ry * ry;
      const banded = radius >= inner && radius <= outer;
      if (!banded && (!live || tick - inside > CIRCLE_OUT_TICKS)) {
        live = false;
        sx = 0;
        sy = 0;
        lap = { ...lap, steps: 0 };
        continue;
      }
      if (banded) inside = tick;
      const nx = rx > CIRCLE_HYSTERESIS ? 1 : rx < -CIRCLE_HYSTERESIS ? -1 : sx;
      const ny = ry > CIRCLE_HYSTERESIS ? 1 : ry < -CIRCLE_HYSTERESIS ? -1 : sy;
      const known = live && sx !== 0 && sy !== 0 && nx !== 0 && ny !== 0;
      const quarter = known ? (quadrantOf(nx, ny) - quadrantOf(sx, sy) + 4) % 4 : 0;
      const cross = px * ry - py * rx;
      const step = quarter === 1 ? 1 : quarter === 3 ? -1 : quarter === 2 ? (cross > 0 ? 2 : cross < 0 ? -2 : 0) : 0;
      const size = Math.abs(step);
      const way = step > 0 ? 1 : -1;
      const lost = (quarter === 2 && step === 0) || (lap.steps > 0 && tick - lap.last > CIRCLE_SLOW);
      const fresh = lost || lap.steps === 0 || (step !== 0 && way !== lap.turn && lap.against + size > CIRCLE_AGAINST);
      const moved = live ? Math.sqrt((rx - px) ** 2 + (ry - py) ** 2) : 0;
      if (fresh) lap = step === 0 ? { ...lap, turn: 0, quarters: 0, steps: 0, against: 0 } : { turn: way, quarters: size, steps: size, against: 0, first: tick, last: tick, near: radius, far: radius, open: radius, widest: 0, length: 0, sum: Math.sqrt(radius), samples: 1 };
      else {
        const widest = step !== 0 ? Math.max(lap.widest, tick - lap.last) : lap.widest;
        lap = { ...lap, quarters: step === 0 ? lap.quarters : way === lap.turn ? lap.quarters + size : lap.quarters - size, steps: lap.steps + size, against: step !== 0 && way !== lap.turn ? lap.against + size : lap.against, last: step !== 0 ? tick : lap.last, near: Math.min(lap.near, radius), far: Math.max(lap.far, radius), widest, length: lap.length + moved, sum: lap.sum + Math.sqrt(radius), samples: lap.samples + 1 };
      }
      live = true;
      sx = nx;
      sy = ny;
      px = rx;
      py = ry;
      if (lap.steps > 0 && lap.quarters >= QUARTERS) {
        const ticks = tick - lap.first;
        laps.push({ trace: trace.id, body, tick, turn: lap.turn, ticks, steps: lap.steps, round: Math.sqrt(lap.far / lap.near), close: Math.sqrt(radius / lap.open), uneven: (lap.widest * (lap.steps - 1)) / Math.max(ticks, 1), winding: lap.length / (((2 * Math.PI * lap.sum) / lap.samples) * ((QUARTERS - 1) / 4)) });
        lap = { ...lap, turn: 0, quarters: 0, steps: 0, against: 0 };
      }
    }
  });
}
writeFileSync(outputs, `${JSON.stringify(laps)}\n`);
