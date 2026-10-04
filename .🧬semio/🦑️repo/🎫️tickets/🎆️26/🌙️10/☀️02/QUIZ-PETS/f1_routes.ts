/** 🗺️ Ticket tool of work package F1: which ways with gear the real pages offer — it plays the `surveyed` payloads `f1_survey.mjs` measured on the running site (the home overview and a quiz's page at 1440 × 900) on the real stage with the cast that stood there, prints the perches and pitches the stage cuts from them and, for every species of the cast standing on every perch, how many perches it can reach with its gear (`routeOf`, by means), and then lets the stage run lively for a while and counts the trips, the footings and the gear seen.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/f1_routes.ts" [--dir <survey dir>] [--page home|heating] [--seconds 600] [--seeds 3]
 */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { Menagerie, Stage, Surveyed } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { advance, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";
import { GRIP_BUDGET, ladderTo, routeOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🟦️.ts";
import { overlaps } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🟦️.ts";
import { bodiesOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📏️spacing/🟦️.ts";

/** 🧾️ The value of an option, or its default. */
function option(name: string, fallback: string): string {
  const at = process.argv.indexOf(name);
  return at >= 0 && process.argv[at + 1] !== undefined ? process.argv[at + 1]! : fallback;
}

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const dir = option("--dir", join(import.meta.dir, "🗑️generated", "f1", "survey"));
const page = option("--page", "heating");
const seconds = Number(option("--seconds", "600"));
const seeds = Number(option("--seeds", "3"));
const watch = option("--watch", "");
const menagerie = ((await import(pathToFileURL(join(ROOT, "🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts")).href)) as { ARCHITECTURE_MENAGERIE: Menagerie }).ARCHITECTURE_MENAGERIE;
const dump = JSON.parse(readFileSync(join(dir, `${page}.json`), "utf8")) as { sample?: { surveyed: Surveyed; pets: { id: string }[] }; samples?: { surveyed: Surveyed; pets: { id: string }[] }[] };
const sample = dump.sample ?? dump.samples![0]!;
const cast = sample.pets.map((pet) => pet.id);
const lines: string[] = [];
const opened = advance(menagerie, openStage(1), [{ kind: "tuned", mode: "lively" }, sample.surveyed, { kind: "summoned", species: cast }, { kind: "permitted", play: true, mischief: true }]);
lines.push(`${page}: cast ${cast.join(", ")}`);
for (const perch of opened.perches) lines.push(`  perch ${perch.surface} ${perch.x0.toFixed(1)}…${perch.x1.toFixed(1)} @${perch.y.toFixed(1)}`);
for (const pitch of opened.pitches) lines.push(`  pitch ${pitch.wall} x ${pitch.x.toFixed(1)} ${pitch.y0.toFixed(1)}…${pitch.y1.toFixed(1)}`);
for (const id of cast) {
  const kind = menagerie.species.find((entry) => entry.id === id)!;
  const counts: Record<string, number> = {};
  for (const from of opened.perches) {
    for (const to of opened.perches) {
      if (to === from) continue;
      const legs = routeOf((from.x0 + from.x1) / 2, from, to, kind.gear, kind.size, GRIP_BUDGET, opened.pitches, [], opened.keepouts) ?? [];
      for (const leg of legs) counts[leg.means] = (counts[leg.means] ?? 0) + 1;
    }
  }
  const leans: string[] = [];
  if (kind.gear.includes("ladder")) for (const low of opened.perches) for (const pitch of opened.pitches) if (ladderTo(low, pitch, opened.keepouts, kind.size) !== null) leans.push(`${low.surface}→${pitch.wall}`);
  lines.push(`  ${id} ${kind.size.width}×${kind.size.height} gear ${kind.gear.join("+")}: legs ${JSON.stringify(counts)}, leans ${leans.length} ${leans.slice(0, 6).join(" ")}`);
}
for (let seed = 1; seed <= seeds; seed++) {
  let stage: Stage = advance(menagerie, openStage(seed), [{ kind: "tuned", mode: "lively" }, sample.surveyed, { kind: "summoned", species: cast }, { kind: "permitted", play: true, mischief: true }]);
  const footings: Record<string, number> = {};
  const trail: string[] = [];
  let trips = 0;
  let worst = 0;
  const kinds = new Map(menagerie.species.map((kind) => [kind.id, kind]));
  for (let tick = 0; tick < seconds * 64; tick++) {
    const before = stage;
    stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
    for (const trip of stage.trips) if (!before.trips.some((entry) => entry.owner === trip.owner && entry.from === trip.from)) trips++;
    for (const actor of stage.actors) {
      if (actor.species === watch) {
        const line = `${actor.activity}/${actor.footing}/${actor.perch ?? "-"}`;
        if (trail.at(-1)?.split(":")[1]?.split("@")[0] !== line) trail.push(`${stage.tick}:${line}@${actor.x.toFixed(0)}`);
      }
      footings[actor.footing] = (footings[actor.footing] ?? 0) + 1;
      if (actor.footing === "ladder" || actor.footing === "rope") footings[`${actor.species}:${actor.footing}`] = (footings[`${actor.species}:${actor.footing}`] ?? 0) + 1;
    }
    worst = Math.max(worst, overlaps(bodiesOf(stage.actors, stage.actors.map((actor) => kinds.get(actor.species)!))).length);
  }
  lines.push(`seed ${seed}: ${seconds} s lively: trips ${trips}, actor-ticks by footing ${JSON.stringify(footings)}, poofs ${stage.poofs}, worst overlaps ${worst}`);
  if (watch !== "") lines.push(`  ${watch}: ${trail.join(" ")}`);
}
process.stdout.write(`${lines.join("\n")}\n`);
