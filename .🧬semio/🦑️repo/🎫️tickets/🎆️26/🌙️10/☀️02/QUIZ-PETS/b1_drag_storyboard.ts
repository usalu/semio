/** 🎬️ Ticket tool of work package B1: replays scripts of the committed stage traces tick by tick and prints a storyboard of the body — the press and who it touches, every change of footing and activity, and while a pet is off its perch its feet, velocity, tilt and parachute every quarter second — with the number of overlapping bodies after every tick.
 *
 * Usage (from the repository root): `bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/b1_drag_storyboard.ts" [script …]` (default: `drag-and-drop`).
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { TICKS_PER_SECOND, type Actor, type Menagerie, type Stage, type StageEvent } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { overlaps } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🟦️.ts";
import { bodiesOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📏️spacing/🟦️.ts";
import { advance, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";

const PETS = resolve(import.meta.dir, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const VECTORS = JSON.parse(readFileSync(resolve(PETS, "🧫️fixtures/🎪️stage-trace/🔣️.json"), "utf8")) as { menagerie: Menagerie; scripts: { id: string; seed: number; ticks: number; steps: { at: number; events: StageEvent[] }[] }[] };
const wanted = process.argv.length > 2 ? process.argv.slice(2) : ["drag-and-drop"];

/** 🕰️ A tick as seconds. */
function clock(tick: number): string {
  return `${(tick / TICKS_PER_SECOND).toFixed(2).padStart(6)} s`;
}

/** 📐️ The body of an actor in one line. */
function pose(actor: Actor): string {
  const chute = actor.chute === null ? "" : ` chute open ${actor.chute.open.toFixed(2)} sway-x ${(actor.chute.canopy.x - actor.x).toFixed(1)}`;
  return `x ${actor.x.toFixed(1)} y ${actor.y.toFixed(1)} v (${actor.vx.toFixed(0)}, ${actor.vy.toFixed(0)}) tilt ${actor.tilt.toFixed(3)}${chute}`;
}

for (const id of wanted) {
  const script = VECTORS.scripts.find((entry) => entry.id === id);
  if (script === undefined) throw new Error(`no script ${id}`);
  const kinds = VECTORS.menagerie.species.map((kind) => kind.id);
  let stage: Stage = openStage(script.seed);
  let clashes = 0;
  const lines: string[] = [`== ${id} · seed ${script.seed} · ${script.ticks} ticks`];
  for (let tick = 1; tick <= script.ticks; tick++) {
    const events = script.steps.filter((step) => step.at === tick - 1).flatMap((step) => step.events);
    for (const event of events) if (event.kind !== "dragged" && event.kind !== "pointed") lines.push(`${clock(tick - 1)}  · learner ${event.kind}${"x" in event ? ` at (${event.x}, ${event.y})` : ""}${"deed" in event ? ` ${event.deed} ${event.species}` : ""}${"play" in event ? ` play ${event.play}` : ""}`);
    const next = advance(VECTORS.menagerie, advance(VECTORS.menagerie, stage, events), [{ kind: "ticked", ticks: 1 }]);
    if (next.press.phase !== stage.press.phase || next.touched !== stage.touched) lines.push(`${clock(tick)}  · press ${next.press.phase}, touched ${next.touched ?? "nobody"}`);
    for (const actor of next.actors) {
      const before = stage.actors.find((entry) => entry.species === actor.species);
      if (before === undefined) lines.push(`${clock(tick)}  ${actor.species.padEnd(10)} arrives ${actor.footing} ${actor.perch ?? ""} ${actor.activity} ${pose(actor)}`);
      else if (before.footing !== actor.footing || before.activity !== actor.activity || before.perch !== actor.perch || before.host !== actor.host) lines.push(`${clock(tick)}  ${actor.species.padEnd(10)} ${before.footing}/${before.activity} → ${actor.footing}/${actor.activity}${actor.perch === null ? "" : ` on ${actor.perch}`}${actor.host === null ? "" : ` on ${actor.host}'s head`} ${pose(actor)}`);
      else if (actor.footing !== "perch" && tick % 16 === 0) lines.push(`${clock(tick)}  ${actor.species.padEnd(10)}   ${actor.footing}/${actor.activity} ${pose(actor)}`);
    }
    for (const actor of stage.actors) if (!next.actors.some((entry) => entry.species === actor.species)) lines.push(`${clock(tick)}  ${actor.species.padEnd(10)} is gone`);
    if (next.poofs > stage.poofs) lines.push(`${clock(tick)}  · poof (${next.poofs} so far)`);
    clashes += overlaps(bodiesOf(next.actors, next.actors.map((actor) => VECTORS.menagerie.species[kinds.indexOf(actor.species)]!))).length;
    stage = next;
  }
  lines.push(`== ${id}: overlapping body pairs summed over all ticks: ${clashes}`, "");
  process.stdout.write(`${lines.join("\n")}\n`);
}
