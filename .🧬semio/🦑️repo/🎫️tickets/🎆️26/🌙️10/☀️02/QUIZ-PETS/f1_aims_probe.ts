/** 🎯️ Ticket tool of work package F1: why a grappler on the footer of the home overview does or does not get a rope onto a card's tab — it builds the overview's card at 1440 × 900 (tab 659.8…780.2 × 631.8…654.2 on a body down to 849.3, footer at 874), puts the sample hopper (48 × 52, gear grapple) at `--x`, and prints the perches, the legs `routeOf` offers to the tab and the outings `outingsTo` plans.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/f1_aims_probe.ts" [--x 600]
 */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Menagerie, Stage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { advance, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";
import { draftOf, indexOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📝️draft/🟦️.ts";
import { embark, outingsTo } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚶️locomotion/🟦️.ts";
import { GRIP_BUDGET, routeOf, shotFor } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🟦️.ts";

/** 🧾️ The value of an option, or its default. */
function option(name: string, fallback: string): string {
  const at = process.argv.indexOf(name);
  return at >= 0 && process.argv[at + 1] !== undefined ? process.argv[at + 1]! : fallback;
}

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const sample = (JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json"), "utf8")) as { menagerie: Menagerie }).menagerie;
const play: Menagerie = { ...sample, species: sample.species.map((entry) => (entry.id === "hoppy" ? { ...entry, size: { width: 48, height: 52 }, gear: ["grapple"] } : entry)) };
const parts = [
  { id: "tab", x0: 659.8, x1: 780.2, y0: 631.8, y1: 654.2 },
  { id: "card", x0: 659.8, x1: 780.2, y0: 654.2, y1: 849.3 },
];
const x = Number(option("--x", "600"));
let stage: Stage = advance(play, openStage(3), [{ kind: "tuned", mode: "lively" }, { kind: "surveyed", width: 1440, height: 900, surfaces: [...parts.map((part) => ({ id: part.id, x0: part.x0, x1: part.x1, y: part.y0 })), { id: "footer", x0: 0, x1: 1440, y: 874 }], keepouts: parts.map((part) => ({ x: part.x0, y: part.y0, width: part.x1 - part.x0, height: part.y1 - part.y0 })), walls: [], fixtures: [] }, { kind: "summoned", species: ["hoppy"] }]);
stage = { ...stage, actors: stage.actors.map((actor) => ({ ...actor, perch: "footer", x, y: 874, goal: x, activity: "idle" as const, until: stage.tick + 100000, opacity: 1 })) };
const draft = draftOf(play, stage);
const index = indexOf(draft.actors, "hoppy");
const lines: string[] = [];
for (const perch of draft.perches) lines.push(`perch ${perch.surface} ${perch.x0}…${perch.x1} @${perch.y}`);
const from = draft.perches.find((perch) => perch.surface === "footer" && perch.x0 <= x && x <= perch.x1)!;
const tab = draft.perches.find((perch) => perch.surface === "tab")!;
const kind = draft.kinds[index]!;
for (let stand = 300; stand <= 660; stand += 24) lines.push(`stand ${stand}: shot ${JSON.stringify(shotFor({ x: stand, y: from.y }, [tab], draft.keepouts, kind.size))}`);
lines.push(`legs ${JSON.stringify(routeOf(x, from, tab, kind.gear, kind.size, GRIP_BUDGET, draft.pitches, [], draft.keepouts))}`);
const outings = outingsTo(draft, index, { kind: "perch", perch: tab }, [0.5, 0.5, 0.9], draft.tick);
lines.push(`outings ${outings.length}: ${outings.map((outing) => outing.trip.steps.length).join(", ")}`);
for (const outing of outings) {
  const outside = outing.trip.steps.filter((step) => step.x - kind.size.width / 2 < 0 || step.x + kind.size.width / 2 > 1440 || step.y - kind.size.height < 0 || step.y > 900);
  lines.push(`  outside ${outside.length}, embark ${embark(draftOf(play, stage), index, outing, draft.tick)}, first ${JSON.stringify(outing.trip.steps[0])}, lowest ${Math.max(...outing.trip.steps.map((step) => step.y))}`);
}
process.stdout.write(`${lines.join("\n")}\n`);
