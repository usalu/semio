/** 🔬️ Ticket tool of work package B5: replays the quiz-like page of `b5_explore.ts` up to a tick and tells why a pet that could play with a row does or does not set out for its post — every outing to the post, whether it lasts for the whole lift, whether its body stays inside the stage and whether its claim is clear of the others.
 *
 * Usage (from the repository root):
 *   bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/b5_debug.ts" --tick T [--seed N] [--mode calm|lively] [--species a,b,c] [--keys k1,k2,k3]
 */
import { pathToFileURL } from "node:url";
import { join, resolve } from "node:path";
import type { Fixture, Menagerie, Rect, Stage, StageEvent, Surface, Wall } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { advance, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";
import { prospectsOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🗓️schedule/🟦️.ts";
import { draftOf, indexOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📝️draft/🟦️.ts";
import { outingsTo } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚶️locomotion/🟦️.ts";
import { claimClear, claimOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🟦️.ts";
import { bodiesOf, extentAt } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📏️spacing/🟦️.ts";

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const WIDTH = 1440;
const HEIGHT = 860;

/** 🧾️ The value of an option, or its default. */
function option(name: string, fallback: string): string {
  const at = process.argv.indexOf(name);
  return at >= 0 && process.argv[at + 1] !== undefined ? process.argv[at + 1]! : fallback;
}

/** 🗺️ The survey of `b5_explore.ts`'s page. */
function quizPage(keys: readonly string[]): StageEvent {
  const cards = [
    { id: "quiz", y0: 110, y1: 220 },
    { id: "quiz-tasks", y0: 290, y1: 420 },
    { id: "quiz-crowd", y0: 490, y1: 640 },
  ];
  const surfaces: Surface[] = [...cards.map((card) => ({ id: card.id, x0: 272, x1: 1168, y: card.y0 })), { id: "footer", x0: 6, x1: WIDTH - 6, y: HEIGHT - 26 }];
  const keepouts: Rect[] = [{ x: 0, y: 0, width: WIDTH, height: 40 }, ...cards.map((card) => ({ x: 272, y: card.y0, width: 896, height: card.y1 - card.y0 }))];
  const walls: Wall[] = cards.flatMap((card) => [
    { id: `${card.id}-left`, surface: card.id, side: -1 as const, x: 272, y0: card.y0, y1: card.y1 },
    { id: `${card.id}-right`, surface: card.id, side: 1 as const, x: 1168, y0: card.y0, y1: card.y1 },
  ]);
  const fixtures: Fixture[] = keys.map((key, index) => ({ id: `row-${index}`, key, x: 280, y: 296 + 32 * index, width: 880, height: 28 }));
  return { kind: "surveyed", width: WIDTH, height: HEIGHT, surfaces, keepouts, walls, fixtures };
}

const menagerie = ((await import(pathToFileURL(join(ROOT, "🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts")).href)) as { ARCHITECTURE_MENAGERIE: Menagerie }).ARCHITECTURE_MENAGERIE;
const keys = option("--keys", "heating/u-values,heating/heating-load-and-demand,heating/heat-pumps").split(",");
const species = option("--species", "radiatory,waly,windowy,insuly,housy,thermy").split(",");
const until = Number(option("--tick", "2960"));
let stage: Stage = advance(menagerie, openStage(Number(option("--seed", "2"))), [{ kind: "tuned", mode: option("--mode", "lively") as "calm" | "lively" }, quizPage(keys), { kind: "summoned", species }, { kind: "permitted", play: true, mischief: true }]);
while (stage.tick < until - 1) stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
const lines: string[] = [`tick ${stage.tick}`];
for (const actor of stage.actors) lines.push(`  ${actor.species} ${actor.footing}/${actor.activity} at ${actor.x.toFixed(1)},${actor.y.toFixed(1)} until ${actor.until}`);
for (const prospect of prospectsOf(menagerie, stage, stage.tick)) {
  const draft = draftOf(menagerie, stage);
  const index = indexOf(draft.actors, prospect.actor);
  lines.push(`prospect ${prospect.actor} → ${prospect.fixture.id} ${JSON.stringify(prospect.post.kind === "wall" ? { wall: prospect.post.pitch.wall, y: prospect.post.y } : { perch: prospect.post.perch.surface, x: prospect.post.x })}`);
  if (prospect.post.kind !== "wall") continue;
  const outings = outingsTo(draft, index, { kind: "wall", pitch: prospect.post.pitch, y: prospect.post.y }, [0, 0, 1], stage.tick);
  lines.push(`  outings ${outings.length}`);
  for (const outing of outings) {
    const kind = draft.kinds[index]!;
    const claim = claimOf(outing.trip.owner, outing.trip.from, outing.trip.steps.map((step) => extentAt(outing.trip.owner, kind, step, 0, null)), 4, extentAt(outing.trip.owner, kind, outing.trip.steps[outing.trip.steps.length - 1]!, 0, null));
    lines.push(`  trip ${outing.trip.steps.length} steps, grip left ${outing.trip.grip.toFixed(1)}, clear ${claimClear(claim, bodiesOf(draft.actors, draft.kinds), draft.claims)}, first ${JSON.stringify(outing.trip.steps[0])}`);
  }
}
process.stdout.write(`${lines.join("\n")}\n`);
