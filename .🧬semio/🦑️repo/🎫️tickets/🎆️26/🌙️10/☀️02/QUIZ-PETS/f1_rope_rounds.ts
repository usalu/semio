/** 🎰️ Ticket tool of work package F1: how often one round of the gear spec of `🐕️pet-walk` ends with a rope, played on the stage — the real home overview (`f1_survey.mjs`, 1440 × 900) with its cast, lively; every grappler stands idle on the footer, the hand holds whoever sits on the tab `--tab` up in the air, and the stage runs for `--round` seconds of the pets' time. It prints per seed whether a grappler was on a rope in that round and what each grappler did, and the share of rounds with a rope.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/f1_rope_rounds.ts" [--seeds 24] [--round 120] [--tab s20] [--settle 30]
 */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { Menagerie, Stage, Surveyed } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { advance, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";

/** 🧾️ The value of an option, or its default. */
function option(name: string, fallback: string): string {
  const at = process.argv.indexOf(name);
  return at >= 0 && process.argv[at + 1] !== undefined ? process.argv[at + 1]! : fallback;
}

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const menagerie = ((await import(pathToFileURL(join(ROOT, "🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts")).href)) as { ARCHITECTURE_MENAGERIE: Menagerie }).ARCHITECTURE_MENAGERIE;
const dump = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "f1", "survey", "home.json"), "utf8")) as { sample?: { surveyed: Surveyed; pets: { id: string }[] }; samples?: { surveyed: Surveyed; pets: { id: string }[] }[] };
const sample = dump.sample ?? dump.samples![0]!;
const cast = sample.pets.map((pet) => pet.id);
const grapplers = cast.filter((id) => menagerie.species.find((kind) => kind.id === id)!.gear.includes("grapple"));
const tab = option("--tab", "s20");
const round = Number(option("--round", "120")) * 64;
const settle = Number(option("--settle", "30")) * 64;
const seeds = Number(option("--seeds", "24"));
const lines: string[] = [];
let hits = 0;
for (let seed = 1; seed <= seeds; seed++) {
  let stage: Stage = advance(menagerie, openStage(seed), [{ kind: "tuned", mode: "lively" }, sample.surveyed, { kind: "summoned", species: cast }, { kind: "permitted", play: true, mischief: false }]);
  stage = advance(menagerie, stage, [{ kind: "ticked", ticks: settle }]);
  const floor = stage.perches.reduce((lowest, perch) => (perch.y > lowest.y ? perch : lowest));
  const spots = option("--spots", "585,855").split(",").map(Number);
  stage = { ...stage, actors: stage.actors.map((actor, index) => (grapplers.includes(actor.species) && (actor.footing !== "perch" || actor.y !== floor.y) ? { ...actor, footing: "perch" as const, perch: floor.surface, pitch: null, x: spots[index % spots.length]!, y: floor.y, goal: spots[index % spots.length]!, vx: 0, vy: 0, activity: "idle" as const, since: stage.tick, until: stage.tick + 64, rope: null } : actor)), trips: stage.trips.filter((trip) => !grapplers.includes(trip.owner)), claims: stage.claims.filter((claim) => !grapplers.includes(claim.owner)) };
  const sitter = stage.actors.find((actor) => actor.footing === "perch" && actor.perch === tab && !grapplers.includes(actor.species));
  if (sitter !== undefined) stage = advance(menagerie, stage, [{ kind: "pressed", x: sitter.x, y: sitter.y - 20, pointer: "mouse" }, { kind: "dragged", x: sitter.x, y: sitter.y - 40 }, { kind: "ticked", ticks: 2 }, { kind: "dragged", x: 50, y: 437 }, { kind: "ticked", ticks: 2 }]);
  const doing: Record<string, string[]> = {};
  let roped = "";
  for (let tick = 0; tick < round && roped === ""; tick++) {
    stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
    for (const actor of stage.actors) {
      if (!grapplers.includes(actor.species)) continue;
      const line = `${actor.activity}/${actor.footing}`;
      const seen = (doing[actor.species] ??= []);
      if (seen.at(-1) !== line) seen.push(line);
      if (actor.footing === "rope") roped = actor.species;
    }
  }
  if (roped !== "") hits++;
  const onTab = stage.actors.filter((actor) => actor.perch === tab).map((actor) => actor.species).join("+") || "nobody";
  lines.push(`seed ${seed}: held ${sitter?.species ?? "nobody"}; rope ${roped || "none"}; on ${tab} at the end ${onTab}; ${Object.entries(doing).map(([id, seen]) => `${id}: ${seen.join(" ")}`).join(" | ")}`);
}
lines.push(`rounds with a rope: ${hits} of ${seeds}`);
process.stdout.write(`${lines.join("\n")}\n`);
