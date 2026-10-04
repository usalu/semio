/** 🔬️ Ticket tool of work package B4: why an actor of the architecture menagerie does or does not set out for the shelf — the route its gear offers (`routeOf`), the outings made of it (`outingsTo`) and whether each would be taken (`embark` on a copy).
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/b4_debug.ts" <species> [x]
 */
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { Menagerie } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { advance, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";
import { draftOf, indexOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📝️draft/🟦️.ts";
import { embark, outingsTo } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚶️locomotion/🟦️.ts";
import { GRIP_BUDGET, routeOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🟦️.ts";
import { cardSurvey } from "./b4_explore.ts";

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const species = process.argv[2] ?? "waly";
const x = Number(process.argv[3] ?? "476");
const menagerie = ((await import(pathToFileURL(join(ROOT, "🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts")).href)) as { ARCHITECTURE_MENAGERIE: Menagerie }).ARCHITECTURE_MENAGERIE;
let stage = advance(menagerie, openStage(1), [{ kind: "tuned", mode: "lively" }, cardSurvey([{ id: "shelf", x0: 500, x1: 820, y0: 600, y1: 660 }, { id: "ground", x0: 0, x1: 1280, y0: 690, y1: 760 }], 1280, 720, false), { kind: "summoned", species: [species] }]);
stage = { ...stage, actors: stage.actors.map((actor) => ({ ...actor, perch: "ground", x, y: 690, goal: x })) };
const draft = draftOf(menagerie, stage);
const index = indexOf(draft.actors, species);
const kind = draft.kinds[index]!;
const from = draft.perches.find((perch) => perch.surface === "ground" && perch.x0 <= x && x <= perch.x1)!;
const to = draft.perches.find((perch) => perch.surface === "shelf")!;
const lines: string[] = [`perches ${JSON.stringify(draft.perches)}`, `pitches ${JSON.stringify(draft.pitches)}`];
lines.push(`route ${JSON.stringify(routeOf(x, from, to, kind.gear, kind.size, GRIP_BUDGET, draft.pitches, [], draft.keepouts))}`);
const outings = outingsTo(draft, index, { kind: "perch", perch: to }, [0.5, 0.5, 0.9], draft.tick);
lines.push(`outings ${outings.length}: ${outings.map((outing) => `${outing.trip.steps.length} steps ending ${outing.trip.ending} ladder ${outing.trip.ladder}`).join("; ")}`);
for (const outing of outings) lines.push(`embark ${embark(draftOf(menagerie, stage), index, outing, draft.tick)}`);
process.stdout.write(`${lines.join("\n")}\n`);
