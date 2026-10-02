/** 🔎️ Ticket probe of work package D: flies every committed flight with the TypeScript terrain module and prints where it leaves the committed arc by more than 1e-9.
 *
 * Run from the repository root: `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/probe_terrain.ts`.
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { hopOf, hopStep } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🏞️terrain/🟦️.ts";

type Flight = { readonly id: string; readonly from: { x: number; y: number }; readonly to: { x: number; y: number }; readonly expected: { ticks: number; path: [number, number][]; apex: number } };

const document = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🦘️hop-ballistics/🔣️.json"), "utf8")) as { flights: Flight[] };
for (const flight of document.flights) {
  const hop = hopOf(flight.from, flight.to)!;
  let state = { x: flight.from.x, y: flight.from.y, vx: hop.vx, vy: hop.vy };
  let worst = 0;
  let where = "";
  for (let left = hop.ticks; left >= 1; left--) {
    state = hopStep(state.x, state.y, state.vx, state.vy, flight.to, left);
    const expected = flight.expected.path[hop.ticks - left]!;
    const off = Math.max(Math.abs(state.x - expected[0]), Math.abs(state.y - expected[1]));
    if (off > worst) {
      worst = off;
      where = `tick ${hop.ticks - left + 1} subject ${state.x},${state.y} oracle ${expected[0]},${expected[1]}`;
    }
  }
  console.log(`[DEBUG] ${flight.id}: ticks ${hop.ticks}/${flight.expected.ticks} worst ${worst} ${where}`);
}
