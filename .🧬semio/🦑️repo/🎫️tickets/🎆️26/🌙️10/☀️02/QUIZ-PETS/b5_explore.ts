/** 🪄️ Ticket tool of work package B5: plays mischief on the REAL stage over a page like an opened quiz page — three cards in a 896 px column, the middle one holding three task rows marked as fixtures — and writes a timeline of the prank: who is picked for which row, the way to its post, the lift (the copy's offset and opacity), the end, a reclaim and the throw-off, the moods; the invariant (no two bodies overlap) and the stage box are checked after every tick.
 *
 * Usage (from the repository root):
 *   bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/b5_explore.ts" [--menagerie architecture|troupe] [--seed N] [--seconds S] [--mode calm|lively] [--species a,b,c] [--keys k1,k2,k3] [--reclaim SECOND] [--still SECOND] [--move SECOND] [--quiet] [--name NAME]
 * Writes `🗑️generated/b5/explore-<name>.txt` and prints a summary line.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { Fixture, Menagerie, Rect, Stage, StageEvent, Surface, Wall } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { overlaps } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🟦️.ts";
import { bodiesOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📏️spacing/🟦️.ts";
import { advance, frameOf, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";
import { prospectsOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🗓️schedule/🟦️.ts";

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const OUT = join(import.meta.dir, "🗑️generated", "b5");
const WIDTH = 1440;
const HEIGHT = 860;

/** 🃏️ A card of the page: its body from `y0` to `y1` across the column. */
type Card = { readonly id: string; readonly y0: number; readonly y1: number };

/** 🧾️ The value of an option, or its default. */
function option(name: string, fallback: string): string {
  const at = process.argv.indexOf(name);
  return at >= 0 && process.argv[at + 1] !== undefined ? process.argv[at + 1]! : fallback;
}

/** 🗺️ The survey of the page: the tops of three cards in the column 272…1168 and the footer, the boxes of the cards as keep-outs, both sides of every card as walls, and the three rows of the middle card as fixtures with `keys`, moved down by `shift` pixels. */
export function quizPage(keys: readonly string[], shift = 0): StageEvent {
  const cards: Card[] = [
    { id: "quiz", y0: 110 + shift, y1: 220 + shift },
    { id: "quiz-tasks", y0: 290 + shift, y1: 420 + shift },
    { id: "quiz-crowd", y0: 490 + shift, y1: 640 + shift },
  ];
  const surfaces: Surface[] = [...cards.map((card) => ({ id: card.id, x0: 272, x1: 1168, y: card.y0 })), { id: "footer", x0: 6, x1: WIDTH - 6, y: HEIGHT - 26 }];
  const keepouts: Rect[] = [{ x: 0, y: 0, width: WIDTH, height: 40 }, ...cards.map((card) => ({ x: 272, y: card.y0, width: 896, height: card.y1 - card.y0 }))];
  const walls: Wall[] = cards.flatMap((card) => [
    { id: `${card.id}-left`, surface: card.id, side: -1 as const, x: 272, y0: card.y0, y1: card.y1 },
    { id: `${card.id}-right`, surface: card.id, side: 1 as const, x: 1168, y0: card.y0, y1: card.y1 },
  ]);
  const fixtures: Fixture[] = keys.map((key, index) => ({ id: `row-${index}`, key, x: 280, y: 296 + 32 * index + shift, width: 880, height: 28 }));
  return { kind: "surveyed", width: WIDTH, height: HEIGHT, surfaces, keepouts, walls, fixtures };
}

/** 🎪️ The menagerie: the architecture's, or the trace troupe with the grounds of `keys` given to its climbers. */
async function menagerieOf(name: string, keys: readonly string[]): Promise<Menagerie> {
  if (name === "architecture") return ((await import(pathToFileURL(join(ROOT, "🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts")).href)) as { ARCHITECTURE_MENAGERIE: Menagerie }).ARCHITECTURE_MENAGERIE;
  const troupe = (JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🎪️stage-trace/🔣️.json"), "utf8")) as { menagerie: Menagerie }).menagerie;
  return { ...troupe, species: troupe.species.map((kind) => ({ ...kind, grounds: kind.gear.includes("climb") || kind.locomotion.gait === "float" ? [...keys] : [] })) };
}

const name = option("--name", "run");
const seed = Number(option("--seed", "1"));
const seconds = Number(option("--seconds", "120"));
const mode = option("--mode", "lively") as "calm" | "lively";
const keys = option("--keys", "heating/u-values,heating/heating-load-and-demand,heating/heat-pumps").split(",");
const menagerie = await menagerieOf(option("--menagerie", "architecture"), keys);
const species = option("--species", menagerie.species.slice(0, 5).map((kind) => kind.id).join(",")).split(",");
const reclaimAt = Number(option("--reclaim", "-1"));
const stillAt = Number(option("--still", "-1"));
const moveAt = Number(option("--move", "-1"));
const lines: string[] = [];
let stage: Stage = advance(menagerie, openStage(seed), [{ kind: "tuned", mode }, { kind: "hushed", quiet: process.argv.includes("--quiet") }, quizPage(keys), { kind: "summoned", species }, { kind: "permitted", play: true, mischief: true }]);
const kinds = new Map(menagerie.species.map((kind) => [kind.id, kind]));
lines.push(`perches: ${stage.perches.map((perch) => `${perch.surface} ${perch.x0.toFixed(1)}…${perch.x1.toFixed(1)} @${perch.y}`).join("; ")}`);
lines.push(`pitches: ${stage.pitches.map((pitch) => `${pitch.wall} x${pitch.x} ${pitch.y0.toFixed(1)}…${pitch.y1.toFixed(1)}`).join("; ")}`);
let pranks = 0;
let lifts = 0;
let throws = 0;
let worst = 0;
const seen = new Map<string, string>();
for (let tick = 1; tick <= seconds * 64; tick++) {
  const events: StageEvent[] = [];
  if (reclaimAt >= 0 && tick === Math.round(reclaimAt * 64) && stage.lift !== null) events.push({ kind: "reclaimed", fixture: stage.lift.fixture });
  if (stillAt >= 0 && tick === Math.round(stillAt * 64)) events.push({ kind: "tuned", mode: "still" });
  if (stillAt >= 0 && tick === Math.round(stillAt * 64) + 64) events.push({ kind: "tuned", mode });
  if (moveAt >= 0 && tick === Math.round(moveAt * 64)) events.push(quizPage(keys, 24));
  const before = stage;
  stage = advance(menagerie, stage, [...events, { kind: "ticked", ticks: 1 }]);
  const time = `${(stage.tick / 64).toFixed(2).padStart(7)}s`;
  for (const event of events) lines.push(`${time} · ${event.kind}${"fixture" in event ? ` ${event.fixture}` : ""}${event.kind === "tuned" ? ` ${event.mode}` : ""}`);
  const bodies = bodiesOf(stage.actors, stage.actors.map((actor) => kinds.get(actor.species)!));
  worst = Math.max(worst, overlaps(bodies).length);
  if (before.lift === null && stage.lift !== null) {
    pranks++;
    lines.push(`${time} · prank: ${stage.lift.pusher} → ${stage.lift.fixture} (${stage.fixtures.find((entry) => entry.id === stage.lift!.fixture)?.key}), begins at ${(stage.lift.since / 64).toFixed(2)}s, side ${stage.lift.side}, room ${stage.lift.room.toFixed(1)}, unit ${stage.lift.unit.toFixed(3)}`);
  } else if (before.lift !== null && stage.lift === null) lines.push(`${time} · prank over (rested ${stage.rested})`);
  else if (before.lift !== null && stage.lift !== null && before.lift.since !== stage.lift.since) lines.push(`${time} · lift moved: since ${before.lift.since} → ${stage.lift.since}`);
  if (before.rested !== stage.rested && stage.lift === null && before.lift === null) lines.push(`${time} · nobody could go: cooldown from ${stage.rested}; prospects ${prospectsOf(menagerie, stage, stage.tick).map((prospect) => `${prospect.actor}→${prospect.fixture.id} ${prospect.post.kind === "wall" ? `${prospect.post.pitch.wall}@${prospect.post.y.toFixed(1)}` : `${prospect.post.perch.surface}@${prospect.post.x.toFixed(1)}`}`).join(", ")}`);
  const frame = frameOf(menagerie, stage);
  const copy = frame.lifts[0];
  if (copy !== undefined && (stage.tick % 16 === 0 || frame.lifts.length !== frameOf(menagerie, before).lifts.length)) {
    lifts++;
    lines.push(`${time}   copy dx ${copy.dx.toFixed(2)} dy ${copy.dy.toFixed(2)} tilt ${copy.tilt.toFixed(5)} opacity ${copy.opacity.toFixed(2)}`);
  }
  for (const actor of stage.actors) {
    const now = `${actor.footing}/${actor.activity}`;
    if (seen.get(actor.species) !== now) {
      const feeling = actor.feeling;
      lines.push(`${time} ${actor.species.padEnd(10)} ${now.padEnd(14)} at ${actor.x.toFixed(1)},${actor.y.toFixed(1)}${actor.pitch !== null ? ` on ${actor.pitch.wall}` : ""} mood ${feeling.mood} ${feeling.intensity.toFixed(2)}`);
      if (actor.activity === "tumble" && seen.get(actor.species)?.endsWith("/push")) throws++;
      seen.set(actor.species, now);
    }
  }
}
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, `explore-${name}.txt`), `${lines.join("\n")}\n`);
process.stdout.write(`${name}: pranks ${pranks}, copy samples ${lifts}, throws ${throws}, worst overlaps ${worst}\n`);
