/** 🎞️ Ticket tool of work package F1: the first frame in which a stage-trace script plays differently on the core as it stood before F1 (the scratch copy `🗑️generated/f1/before/🐾️pets`) and on the core as it stands — every script named (all by default) is replayed tick by tick on both, as the stage-trace adapter replays it, and the fields of the first frame that differs are printed per actor (place, facing, activity, footing, opacity, tilt, pivot, body, tools) and for the frame as a whole.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/f1_frame_diff.ts" [script id …]
 */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { Frame, Menagerie, Stage, StageEvent } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";

type Script = { readonly id: string; readonly seed: number; readonly ticks: number; readonly steps: readonly { readonly at: number; readonly events: readonly StageEvent[] }[] };
type Facade = { readonly advance: (menagerie: Menagerie, stage: Stage, events: readonly StageEvent[]) => Stage; readonly frameOf: (menagerie: Menagerie, stage: Stage) => Frame; readonly openStage: (seed: number) => Stage };

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const now = (await import(pathToFileURL(join(ROOT, "🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts")).href)) as Facade;
const then = (await import(pathToFileURL(join(import.meta.dir, "🗑️generated/f1/before/🐾️pets/🔨️modules/🎪️stage/🟦️.ts")).href)) as Facade;
const vectors = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🎪️stage-trace/🔣️.json"), "utf8").replace(/^﻿/u, "")) as { menagerie: Menagerie; scripts: Script[] };
const wanted = process.argv.slice(2);
const lines: string[] = [];
for (const script of vectors.scripts) {
  if (wanted.length > 0 && !wanted.includes(script.id)) continue;
  const byTick = new Map<number, StageEvent[]>();
  for (const step of script.steps) byTick.set(step.at, [...(byTick.get(step.at) ?? []), ...step.events]);
  let [before, after] = [then.openStage(script.seed), now.openStage(script.seed)];
  let found = "plays alike";
  for (let tick = 1; tick <= script.ticks; tick++) {
    const events = byTick.get(tick - 1) ?? [];
    before = then.advance(vectors.menagerie, then.advance(vectors.menagerie, before, events), [{ kind: "ticked", ticks: 1 }]);
    after = now.advance(vectors.menagerie, now.advance(vectors.menagerie, after, events), [{ kind: "ticked", ticks: 1 }]);
    const [was, is] = [then.frameOf(vectors.menagerie, before), now.frameOf(vectors.menagerie, after)];
    if (JSON.stringify(was) === JSON.stringify(is)) continue;
    const changes: string[] = [];
    for (const actor of is.actors) {
      const old = was.actors.find((entry) => entry.species === actor.species);
      if (old === undefined) {
        changes.push(`${actor.species} only now`);
        continue;
      }
      for (const field of ["x", "y", "facing", "activity", "footing", "opacity", "tilt", "pivot", "body", "tools", "state", "mood"] as const) if (JSON.stringify(old[field]) !== JSON.stringify(actor[field])) changes.push(`${actor.species} ${field} ${JSON.stringify(old[field]).slice(0, 50)}→${JSON.stringify(actor[field]).slice(0, 50)}`);
    }
    for (const field of ["rate", "wake", "ladders", "puffs", "lifts", "held"] as const) if (JSON.stringify(was[field]) !== JSON.stringify(is[field])) changes.push(`frame ${field} ${JSON.stringify(was[field]).slice(0, 60)}→${JSON.stringify(is[field]).slice(0, 60)}`);
    found = `tick ${tick}: ${changes.slice(0, 5).join("; ") || "bones, eyes or particles"}`;
    break;
  }
  lines.push(`${script.id}: ${found}`);
}
process.stdout.write(`${lines.join("\n")}\n`);
