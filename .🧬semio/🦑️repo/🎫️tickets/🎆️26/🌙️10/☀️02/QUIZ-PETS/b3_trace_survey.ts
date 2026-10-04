/** 🔍️ Ticket tool of work package B3: which second-round frame fields the committed stage-trace scripts exercise — per script the frames with particles, tools (by kind), a tilt, a pet held, ladders, lifted copies, a state other than the resting one, a lid lowered by a mood, and the rates — replayed tick by tick from the committed fixture exactly as the case's adapter replays them.
 *
 * Usage (from the repository root): `bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/b3_trace_survey.ts"`
 * Prints one line per script; writes nothing.
 */
import { readFileSync } from "node:fs";
import type { Frame, Menagerie, StageEvent } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { advance, frameOf, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";
import type { Script } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧪️tests/🎪️stage-trace/🟦️.ts";

const FIXTURE = "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🎪️stage-trace/🔣️.json";
const { menagerie, scripts } = JSON.parse(readFileSync(FIXTURE, "utf8")) as { menagerie: Menagerie; scripts: readonly Script[] };

/** 🧮️ Counts of one script. */
type Tally = Record<string, number>;

/** 🎞️ What one frame adds to the tally. */
function count(tally: Tally, frame: Frame): void {
  const add = (key: string): void => {
    tally[key] = (tally[key] ?? 0) + 1;
  };
  add(`rate${frame.rate}`);
  if (frame.particles.length > 0) add("particles");
  if (frame.particles.length >= 160) add("capped");
  if (frame.ladders.length > 0) add("ladders");
  if (frame.lifts.length > 0) add("lifts");
  if (frame.held !== null) add("held");
  if (frame.puffs.length > 0) add("puffs");
  for (const actor of frame.actors) {
    const kind = menagerie.species.find((entry) => entry.id === actor.species)!;
    if (actor.tilt !== 0) add("tilted");
    if (actor.state !== kind.states[0]!.id) add("state");
    if (actor.footing !== "perch") add(`footing-${actor.footing}`);
    for (const tool of actor.tools) add(`tool-${tool.kind}`);
    if (actor.eyes.length > 0 && actor.eyes[0]!.lid > 0 && actor.eyes[0]!.lid < 1) add("lidded");
  }
}

for (const script of scripts) {
  const byTick = new Map<number, StageEvent[]>();
  for (const step of script.steps) byTick.set(step.at, [...(byTick.get(step.at) ?? []), ...step.events]);
  let stage = openStage(script.seed);
  const tally: Tally = {};
  for (let tick = 1; tick <= script.ticks; tick++) {
    stage = advance(menagerie, advance(menagerie, stage, byTick.get(tick - 1) ?? []), [{ kind: "ticked", ticks: 1 }]);
    count(tally, frameOf(menagerie, stage));
  }
  process.stdout.write(`${script.id} (${script.ticks} frames, digest ${script.expected?.digest}): ${Object.entries(tally).sort(([one], [two]) => (one < two ? -1 : 1)).map(([key, value]) => `${key} ${value}`).join(", ")}\n`);
}
