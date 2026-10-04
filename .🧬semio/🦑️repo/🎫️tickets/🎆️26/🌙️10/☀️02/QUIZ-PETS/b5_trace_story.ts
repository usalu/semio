/** 📖️ Ticket tool of work package B5: replays scripts of the stage-trace case (from the inputs `generate_behavior_vectors.py` last wrote) on the REAL stage and tells the story of their mischief — the prank picked (pet, row, the tick its push begins), every change of footing and activity of the pets, the copy as drawn, reclaims, the end of the prank and the moods — to time the events of a script and to show in the report what a trace holds.
 *
 * Usage (from the repository root, after a run of the generator):
 *   bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/b5_trace_story.ts" <script id> [<script id> …]
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { Menagerie, Stage, StageEvent } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { settled } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/💗️feeling/🟦️.ts";
import { advance, frameOf, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";

type Script = { readonly id: string; readonly seed: number; readonly ticks: number; readonly steps: readonly { readonly at: number; readonly events: readonly StageEvent[] }[] };

const inputs = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "wp-e", "stage-trace-inputs.json"), "utf8")) as { menagerie: Menagerie; scripts: readonly Script[] };
const menagerie = inputs.menagerie;
for (const id of process.argv.slice(2)) {
  const script = inputs.scripts.find((entry) => entry.id === id);
  if (script === undefined) {
    process.stdout.write(`no script ${id}\n`);
    continue;
  }
  const lines: string[] = [`== ${id} (seed ${script.seed}, ${script.ticks / 64} s)`];
  let stage: Stage = openStage(script.seed);
  const seen = new Map<string, string>();
  let drawn = false;
  for (let tick = 1; tick <= script.ticks; tick++) {
    const events = script.steps.filter((step) => step.at === tick - 1).flatMap((step) => step.events);
    const before = stage;
    stage = advance(menagerie, advance(menagerie, stage, events), [{ kind: "ticked", ticks: 1 }]);
    const time = `${(stage.tick / 64).toFixed(2).padStart(7)}s`;
    for (const event of events) if (event.kind === "reclaimed" || event.kind === "summoned" || (event.kind === "surveyed" && tick > 1)) lines.push(`${time} · ${event.kind}${event.kind === "reclaimed" ? ` ${event.fixture}` : event.kind === "summoned" ? ` ${event.species.join(",")}` : ` ${event.fixtures.length} fixtures`}`);
    if (before.lift === null && stage.lift !== null) lines.push(`${time} · prank: ${stage.lift.pusher} → ${stage.lift.fixture}, push begins at ${(stage.lift.since / 64).toFixed(2)}s (tick ${stage.lift.since}), side ${stage.lift.side}`);
    if (before.lift !== null && stage.lift !== null && before.lift.since !== stage.lift.since) lines.push(`${time} · lift moved to since ${stage.lift.since}`);
    if (before.lift !== null && stage.lift === null) lines.push(`${time} · prank over, rested ${stage.rested}`);
    if (before.lift === null && stage.lift === null && before.rested !== stage.rested) lines.push(`${time} · nobody could go, rested ${stage.rested}`);
    const copy = frameOf(menagerie, stage).lifts[0];
    if ((copy !== undefined) !== drawn || (copy !== undefined && stage.tick % 32 === 0)) lines.push(`${time}   copy ${copy === undefined ? "gone" : `dx ${copy.dx.toFixed(2)} dy ${copy.dy.toFixed(2)} opacity ${copy.opacity.toFixed(2)}`}`);
    drawn = copy !== undefined;
    for (const actor of stage.actors) {
      const now = `${actor.footing}/${actor.activity}`;
      if (seen.get(actor.species) === now) continue;
      seen.set(actor.species, now);
      const kind = menagerie.species.find((entry) => entry.id === actor.species)!;
      const feeling = settled(actor.feeling, kind.mood, stage.tick);
      lines.push(`${time} ${actor.species.padEnd(8)} ${now.padEnd(14)} at ${actor.x.toFixed(1)},${actor.y.toFixed(1)}${actor.pitch === null ? "" : ` on ${actor.pitch.wall}`} feels ${feeling.mood} ${feeling.intensity.toFixed(2)}`);
    }
  }
  process.stdout.write(`${lines.join("\n")}\n`);
}
