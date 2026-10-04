/** 🔩️ Ticket tool of work package F1: which outings with its gear a pet of the real home overview gets to another perch — it plays the `surveyed` payload `f1_survey.mjs` measured, places the pet `--species` on the perch at `--x` (the footer by default), and prints for every perch it could venture to the outings `outingsTo` plans, quickest first: their ticks, the footings they pass through, and whether their claim is clear.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/f1_gear_probe.ts" [--page home] [--species housy] [--x 300] [--tick 600]
 */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { Menagerie, Stage, Surveyed } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { advance, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";
import { draftOf, indexOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📝️draft/🟦️.ts";
import { embark, outingsTo } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚶️locomotion/🟦️.ts";
import { GRIP_BUDGET, routeOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🟦️.ts";

/** 🧾️ The value of an option, or its default. */
function option(name: string, fallback: string): string {
  const at = process.argv.indexOf(name);
  return at >= 0 && process.argv[at + 1] !== undefined ? process.argv[at + 1]! : fallback;
}

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const page = option("--page", "home");
const species = option("--species", "housy");
const menagerie = ((await import(pathToFileURL(join(ROOT, "🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts")).href)) as { ARCHITECTURE_MENAGERIE: Menagerie }).ARCHITECTURE_MENAGERIE;
const dump = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "f1", "survey", `${page}.json`), "utf8")) as { sample?: { surveyed: Surveyed; pets: { id: string }[] }; samples?: { surveyed: Surveyed; pets: { id: string }[] }[] };
const sample = dump.sample ?? dump.samples![0]!;
let stage: Stage = advance(menagerie, openStage(1), [{ kind: "tuned", mode: "lively" }, sample.surveyed, { kind: "summoned", species: [species] }]);
const until = Number(option("--tick", "600"));
while (stage.tick < until) stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
const draft = draftOf(menagerie, stage);
const index = indexOf(draft.actors, species);
const body = draft.actors[index]!;
const floor = draft.perches.filter((perch) => perch.y === Math.max(...draft.perches.map((entry) => entry.y)));
const x = Number(option("--x", "300"));
const home = floor.find((perch) => perch.x0 <= x && x <= perch.x1) ?? floor[0]!;
body.x = x;
body.y = home.y;
body.footing = "perch";
body.perch = home.surface;
const lines: string[] = [`${species} at ${body.x},${body.y} on ${home.surface} ${home.x0.toFixed(1)}…${home.x1.toFixed(1)}, tick ${stage.tick}`];
for (const perch of draft.perches) {
  if (perch === home) continue;
  const outings = outingsTo(draft, index, { kind: "perch", perch }, [0.5, 0.5, 0.9], draft.tick);
  const legs = routeOf(body.x, home, perch, draft.kinds[index]!.gear, draft.kinds[index]!.size, GRIP_BUDGET, draft.pitches, [], draft.keepouts) ?? [];
  lines.push(`  to ${perch.surface} ${perch.x0.toFixed(1)}…${perch.x1.toFixed(1)} @${perch.y.toFixed(1)}: ${outings.length} outings; legs ${legs.map((leg) => `${leg.means}@${leg.at.toFixed(0)}${leg.means === "raise" ? ` ${leg.ladder.wall} foot ${leg.ladder.foot.x.toFixed(0)} top ${leg.ladder.top.y.toFixed(0)}` : ""}`).join(", ")}`);
  for (const outing of outings) {
    const footings: string[] = [];
    for (const step of outing.trip.steps) if (footings.at(-1) !== `${step.footing}:${step.activity}`) footings.push(`${step.footing}:${step.activity}`);
    const lowest = Math.max(...outing.trip.steps.map((step) => step.y));
    lines.push(`    ${outing.trip.steps.length} ticks, ladder ${outing.trip.ladder === null ? "-" : outing.trip.ladder}, rope ${outing.rope === null ? "-" : `${outing.rope.shot.reel} ${outing.rope.shot.length.toFixed(0)}`}, lowest feet ${lowest.toFixed(1)}, embarks ${embark(draftOf(menagerie, stage), index, outing, draft.tick)}: ${footings.join(" ")}`);
  }
}
process.stdout.write(`${lines.join("\n")}\n`);
