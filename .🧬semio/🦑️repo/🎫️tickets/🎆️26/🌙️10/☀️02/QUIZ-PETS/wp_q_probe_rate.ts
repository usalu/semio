/** 📡️ Probe of work package Q: which clause of the stage suite's "reports the highest rate anyone needs" check stops holding when the watched window grows from 2 000 to 20 000 ticks, and when. `bun wp_q_probe_rate.ts`. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { TICKS_PER_SECOND, type Menagerie, type Stage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { advance, frameOf, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const FIXTURES = resolve(HERE, "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧫️fixtures");

for (const name of ["🧬️schema-conformance", "🎪️stage-trace"]) {
  const menagerie = (JSON.parse(readFileSync(resolve(FIXTURES, name, "🔣️.json"), "utf8")) as { menagerie: Menagerie }).menagerie;
  const everyone = menagerie.species.map((kind) => kind.id);
  let stage: Stage = advance(menagerie, openStage(3), [{ kind: "tuned", mode: "lively" }, { kind: "surveyed", width: 1280, height: 720, surfaces: [{ id: "card", x0: 300, x1: 800, y: 400 }, { id: "floor", x0: 0, x1: 1280, y: 720 }], keepouts: [] }, { kind: "summoned", species: everyone }]);
  for (let tick = 0; tick < 30 * TICKS_PER_SECOND; tick++) stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
  const first: Record<string, number> = {};
  const count: Record<string, number> = {};
  const note = (clause: string, tick: number): void => {
    first[clause] ??= tick;
    count[clause] = (count[clause] ?? 0) + 1;
  };
  for (let tick = 0; tick < 20000; tick++) {
    stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
    const frame = frameOf(menagerie, stage);
    if (frame.tick !== stage.tick) note("tick", tick);
    if (frame.wake !== null) note("wake", tick);
    if (frame.actors.length !== stage.actors.length) note("drawn", tick);
    if (stage.actors.length < everyone.length - 1) note(`absent (${everyone.length - stage.actors.length})`, tick);
    if (![16, 32, 64].includes(frame.rate)) note(`rate ${frame.rate}`, tick);
    if (stage.actors.some((actor) => actor.activity === "walk" || actor.activity === "hop" || actor.activity === "fall" || actor.activity === "land" || actor.opacity < 1) && frame.rate !== 64) note("slow", tick);
  }
  process.stdout.write(`${name}: first tick per broken clause ${JSON.stringify(first)}, ticks per broken clause ${JSON.stringify(count)}\n`);
}
