/** 🧗️ Ticket tool of work package B4: plays one of the gear scenarios on the REAL stage and writes a timeline of what every actor does with its gear — footing and activity changes, trips, standing ladders, ropes — with the invariant (no two bodies overlap) and the stage box (no body outside the stage but while it fades out) checked after every tick.
 *
 * The scenarios are the terrains of the B4 trace scripts: `gutter` (a page of two card columns with 30 px gaps, the
 * cards keep-outs and their sides walls, lively), `ladder` (a shelf too high to hop to over a wall that does not reach
 * down), `rope` (the same for a pet with a grappling gun), `topple` (the shelf jumps while a ladder stands), `cross` (a
 * column of cards climbed across its gaps) and `scroll` (the page scrolls under its climbers, then jumps).
 *
 * Usage (from the repository root):
 *   bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/b4_explore.ts" <scenario> [--seed N] [--seconds S] [--menagerie troupe|architecture]
 * Writes `🗑️generated/b4/explore-<scenario>-<seed>.txt`.
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import type { Menagerie, Rect, Species, Stage, StageEvent, Surface, Wall } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { overlaps } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🟦️.ts";
import { bodiesOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📏️spacing/🟦️.ts";
import { advance, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const OUT = join(import.meta.dir, "🗑️generated", "b4");
const WIDTH = 1280;
const HEIGHT = 720;

/** 🃏️ A card: its top a surface, its box a keep-out, its sides walls. */
export type Card = { readonly id: string; readonly x0: number; readonly x1: number; readonly y0: number; readonly y1: number };

/** 🗺️ The survey of cards and a floor: the tops of the cards, the floor, the boxes of the cards as keep-outs and both sides of every card as walls. */
export function cardSurvey(cards: readonly Card[], width = WIDTH, height = HEIGHT, floor = true): StageEvent {
  const surfaces: Surface[] = cards.map((card) => ({ id: card.id, x0: card.x0, x1: card.x1, y: card.y0 }));
  if (floor) surfaces.push({ id: "floor", x0: 0, x1: width, y: height });
  const keepouts: Rect[] = cards.map((card) => ({ x: card.x0, y: card.y0, width: card.x1 - card.x0, height: card.y1 - card.y0 }));
  const walls: Wall[] = cards.flatMap((card) => [
    { id: `${card.id}-left`, surface: card.id, side: -1 as const, x: card.x0, y0: card.y0, y1: card.y1 },
    { id: `${card.id}-right`, surface: card.id, side: 1 as const, x: card.x1, y0: card.y0, y1: card.y1 },
  ]);
  return { kind: "surveyed", width, height, surfaces, keepouts, walls, fixtures: [] };
}

/** 📰️ A page of two columns of cards with 30 px gaps between the cards of a column and an 80 px gutter between the columns, scrolled by `scroll` pixels; the last cards run on below the stage. */
export function page(scroll = 0): Card[] {
  const rows: readonly [number, number][] = [
    [60, 250],
    [280, 470],
    [500, 900],
  ];
  const cards: Card[] = [];
  for (const [column, x0, x1] of [
    ["l", 40, 600],
    ["r", 680, 1240],
  ] as const)
    rows.forEach(([y0, y1], row) => cards.push({ id: `${column}${row}`, x0, x1, y0: y0 - scroll, y1: y1 - scroll }));
  return cards;
}

/** 🏛️ A page for big pets: two columns of cards with 30 px gaps, a 160 px gutter between the columns and 110 px of room above the first cards; the last cards run on below the stage. */
export function roomy(): Card[] {
  const cards: Card[] = [];
  for (const [column, x0, x1] of [
    ["l", 40, 560],
    ["r", 720, 1240],
  ] as const)
    for (const [row, y0, y1] of [
      [0, 110, 300],
      [1, 330, 520],
      [2, 550, 900],
    ] as const)
      cards.push({ id: `${column}${row}`, x0, x1, y0, y1 });
  return cards;
}

/** 📝️ A quiz page: a header bar, a question column of two cards 30 px apart (no room on the lower one), and an answer column whose two cards leave room above each (160 and 140 px); the lower answer card runs on below the stage, so its wall reaches the floor, and the floor is free under the question column and in the 120 px gutter. */
export function quiz(): Card[] {
  return [
    { id: "header", x0: 40, x1: 1240, y0: 40, y1: 100 },
    { id: "q0", x0: 40, x1: 600, y0: 130, y1: 330 },
    { id: "q1", x0: 40, x1: 600, y0: 360, y1: 560 },
    { id: "s0", x0: 720, x1: 1240, y0: 260, y1: 420 },
    { id: "s1", x0: 720, x1: 1240, y0: 560, y1: 760 },
  ];
}

/** 🎬️ The events of a scenario by tick, its menagerie species and its mode; `script:<id>` plays a committed stage-trace script (its events at 0 then fold before the first tick, its species and mode are its own). */
function scenario(name: string): { species: string[]; mode: "calm" | "lively"; events: Map<number, StageEvent[]> } {
  const events = new Map<number, StageEvent[]>();
  const add = (tick: number, ...list: StageEvent[]) => events.set(tick, [...(events.get(tick) ?? []), ...list]);
  if (name.startsWith("script:")) {
    const document = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🎪️stage-trace/🔣️.json"), "utf8")) as { scripts: { id: string; steps: { at: number; events: StageEvent[] }[] }[] };
    const script = document.scripts.find((entry) => entry.id === name.slice(7))!;
    for (const step of script.steps) add(step.at, ...step.events);
    const first = events.get(0) ?? [];
    events.set(0, first.filter((event) => event.kind !== "tuned" && event.kind !== "summoned"));
    const summoned = first.find((event) => event.kind === "summoned");
    const tuned = first.find((event) => event.kind === "tuned");
    return { species: summoned?.kind === "summoned" ? [...summoned.species] : [], mode: tuned?.kind === "tuned" && tuned.mode === "lively" ? "lively" : "calm", events };
  }
  const shelf = (dx: number): Card[] => [
    { id: "shelf", x0: 500 + dx, x1: 820 + dx, y0: 600, y1: 660 },
    { id: "ground", x0: 0, x1: 1280, y0: 690, y1: 760 },
  ];
  if (name === "gutter") {
    add(0, cardSurvey(page()));
    return { species: ["mossy", "sparky", "thorny", "pebble", "misty"], mode: "lively", events };
  }
  if (name === "roomy") {
    add(0, cardSurvey(roomy()));
    return { species: ["mossy", "sparky", "thorny", "pebble", "misty"], mode: "lively", events };
  }
  if (name === "quiz") {
    add(0, cardSurvey(quiz()));
    return { species: ["mossy", "sparky", "thorny", "pebble", "misty"], mode: "lively", events };
  }
  if (name === "ladder" || name === "rope" || name === "topple") {
    add(0, cardSurvey(shelf(0).slice(1), WIDTH, HEIGHT, false));
    add(3 * 64, cardSurvey(shelf(0), WIDTH, HEIGHT, false));
    if (name === "topple") add(Number(option("--at", "67.5")) * 64, cardSurvey(shelf(160), WIDTH, HEIGHT, false));
    return { species: name === "rope" ? ["sparky"] : ["mossy"], mode: "lively", events };
  }
  if (name === "cross") {
    add(0, cardSurvey([]));
    add(3 * 64, cardSurvey([{ id: "c0", x0: 400, x1: 880, y0: 550, y1: 600 }, { id: "c1", x0: 400, x1: 880, y0: 630, y1: 760 }]));
    return { species: ["thorny"], mode: "lively", events };
  }
  if (name === "scroll") {
    add(0, cardSurvey(page()));
    for (let step = 0; step < 4; step++) add(8 * 64 + 8 * step, cardSurvey(page(step + 1)));
    add(Number(option("--at", "9")) * 64, cardSurvey(page(260)));
    add(Number(option("--gone", "12")) * 64, cardSurvey(page(260).filter((card) => card.id !== "r2")));
    return { species: ["mossy", "thorny", "sparky"], mode: "lively", events };
  }
  throw new Error(`unknown scenario ${name}`);
}

/** 🧾️ The value of an option, or its default. */
function option(name: string, fallback: string): string {
  const at = process.argv.indexOf(name);
  return at >= 0 && process.argv[at + 1] !== undefined ? process.argv[at + 1]! : fallback;
}

if (import.meta.main) {
  const name = process.argv[2] ?? "gutter";
  const seed = Number(option("--seed", "1"));
  const seconds = Number(option("--seconds", "90"));
  const which = option("--menagerie", "troupe");
  const menagerie: Menagerie =
    which === "troupe"
      ? (JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🎪️stage-trace/🔣️.json"), "utf8")) as { menagerie: Menagerie }).menagerie
      : which === "sample"
        ? (JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json"), "utf8")) as { menagerie: Menagerie }).menagerie
        : ((await import(pathToFileURL(join(ROOT, "🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts")).href)) as { ARCHITECTURE_MENAGERIE: Menagerie }).ARCHITECTURE_MENAGERIE;
  const plan = scenario(name);
  const chosen = option("--species", "");
  const species = chosen !== "" ? chosen.split(",") : which === "troupe" ? plan.species : menagerie.species.map((kind) => kind.id);
  let trips = 0;
  const kinds = new Map<string, Species>(menagerie.species.map((kind) => [kind.id, kind]));
  let stage: Stage = advance(menagerie, openStage(seed), [{ kind: "tuned", mode: plan.mode }, ...(plan.events.get(0) ?? []), { kind: "summoned", species }]);
  const lines: string[] = [];
  const seen = new Map<string, string>();
  let overlapTicks = 0;
  let outside = 0;
  for (let tick = 1; tick <= seconds * 64; tick++) {
    const events = tick > 1 ? (plan.events.get(tick - 1) ?? []) : [];
    const before = stage;
    stage = advance(menagerie, stage, [...events, { kind: "ticked", ticks: 1 }]);
    for (const trip of stage.trips) if (!before.trips.some((entry) => entry.owner === trip.owner && entry.from === trip.from)) trips++;
    if (events.length > 0) lines.push(`${(tick / 64).toFixed(2)}s survey`);
    const bodies = bodiesOf(stage.actors, stage.actors.map((actor) => kinds.get(actor.species)!));
    const clash = overlaps(bodies);
    if (clash.length > 0) {
      overlapTicks++;
      if (overlapTicks <= 5) lines.push(`${(tick / 64).toFixed(2)}s OVERLAP ${JSON.stringify(clash)}`);
    }
    for (const actor of stage.actors) {
      const kind = kinds.get(actor.species)!;
      const half = kind.size.width / 2;
      if (!actor.leaving && (actor.x - half < -1e-9 || actor.x + half > stage.width + 1e-9 || actor.y - kind.size.height < -1e-9 || actor.y > stage.height + 1e-9)) {
        outside++;
        if (outside <= 5) lines.push(`${(tick / 64).toFixed(2)}s OUTSIDE ${actor.species} ${actor.footing}/${actor.activity} at ${actor.x.toFixed(1)},${actor.y.toFixed(1)}`);
      }
      const trip = stage.trips.find((entry) => entry.owner === actor.species);
      const state = `${actor.footing}/${actor.activity}${trip === undefined ? "" : ` trip→${trip.ending}:${trip.landing}${trip.ladder === null ? "" : ` ladder ${trip.ladder}`}`}${actor.pitch === null ? "" : ` on ${actor.pitch.wall}`}${actor.rope === null ? "" : ` rope ${actor.rope.caught ? "caught" : "miss"}`}`;
      if (seen.get(actor.species) !== state) lines.push(`${(tick / 64).toFixed(2)}s ${actor.species.padEnd(10)} ${state} at ${actor.x.toFixed(1)},${actor.y.toFixed(1)} grip ${actor.grip.toFixed(0)}${actor.perch === null ? "" : ` perch ${actor.perch}`}`);
      seen.set(actor.species, state);
    }
    for (const species of [...seen.keys()]) if (!stage.actors.some((actor) => actor.species === species)) {
      lines.push(`${(tick / 64).toFixed(2)}s ${species.padEnd(10)} gone`);
      seen.delete(species);
    }
  }
  lines.push(`overlap ticks ${overlapTicks}, outside ${outside}, poofs ${stage.poofs}, ladders ${stage.ladders.length}, trips ${trips}`);
  mkdirSync(OUT, { recursive: true });
  writeFileSync(join(OUT, `explore-${name.replace(":", "-")}-${which}-${seed}.txt`), `${lines.join("\n")}\n`);
  process.stdout.write(`${lines.slice(-1)[0]}\n`);
}
