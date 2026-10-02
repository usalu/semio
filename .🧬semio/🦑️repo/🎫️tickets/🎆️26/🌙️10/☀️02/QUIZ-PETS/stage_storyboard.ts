/** 📜️ Ticket tool: runs a scripted session of the pets stage and prints a readable timeline of what every actor did, with the statistics the tuning of the behaviour constants is judged by.
 *
 * Usage (from the repository root):
 *   bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/stage_storyboard.ts" [options]
 *
 *   --menagerie <path>   a menagerie as JSON (the document itself or under `menagerie`) or a TypeScript module that
 *                        exports one, relative to the repository root; default: the trace troupe of
 *                        🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🎪️stage-trace/🔣️.json
 *   --scene <id>         the cast on stage (default: home)        --capacity <n>   most actors at once (default 6)
 *   --seed <n>           the seed of the stage (default 1)        --mode <mode>    still | calm | lively (default calm)
 *   --minutes <n>        how long the session lasts (default 10)  --width/--height the stage box (default 1280 × 720)
 *   --quiet-at <s>       a time of concentration begins           --loud-at <s>    … and ends
 *   --vanish-at <s>      the first card vanishes                  --pointer        the pointer wanders every 7 s
 *   --epoch <s>          the cast rotates every s seconds         --poke-at <s>    the first actor is poked
 *   --brief              statistics only, no timeline
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { ACTIVITIES, MENAGERIE_SCHEMA, TICKS_PER_SECOND, type Activity, type Actor, type Menagerie, type PetMode, type Rect, type Stage, type StageEvent, type Surface } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { MODE_LIMITS, castOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🧠️behavior/🟦️.ts";
import { advance, frameOf, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const DEFAULT_MENAGERIE = "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🎪️stage-trace/🔣️.json";

/** 🎛️ The value after a flag, or its default. */
function option(name: string, fallback: string): string {
  const at = process.argv.indexOf(`--${name}`);
  return at >= 0 && at + 1 < process.argv.length ? process.argv[at + 1]! : fallback;
}

/** 🚩️ Whether a flag is present. */
function flag(name: string): boolean {
  return process.argv.includes(`--${name}`);
}

/** 🎪️ The menagerie a path names. */
async function menagerieAt(path: string): Promise<Menagerie> {
  const absolute = resolve(ROOT, path);
  if (absolute.endsWith(".json")) {
    const document = JSON.parse(readFileSync(absolute, "utf8"));
    return (document.schema === MENAGERIE_SCHEMA ? document : document.menagerie) as Menagerie;
  }
  const module = (await import(pathToFileURL(absolute).href)) as Record<string, unknown>;
  for (const value of Object.values(module)) if (typeof value === "object" && value !== null && (value as { schema?: unknown }).schema === MENAGERIE_SCHEMA) return value as Menagerie;
  throw new Error(`${path} exports no menagerie`);
}

/** 🕰️ A tick as `mm:ss.t`. */
function clock(tick: number): string {
  const seconds = tick / TICKS_PER_SECOND;
  const minutes = Math.floor(seconds / 60);
  return `${String(minutes).padStart(2, "0")}:${(seconds - minutes * 60).toFixed(1).padStart(4, "0")}`;
}

/** 📊️ A share as a percentage. */
function percent(part: number, whole: number): string {
  return `${((100 * part) / Math.max(whole, 1)).toFixed(1).padStart(5)} %`;
}

const menagerie = await menagerieAt(option("menagerie", DEFAULT_MENAGERIE));
const scene = option("scene", "home");
const capacity = Number(option("capacity", "6"));
const seed = Number(option("seed", "1"));
const mode = option("mode", "calm") as PetMode;
const minutes = Number(option("minutes", "10"));
const width = Number(option("width", "1280"));
const height = Number(option("height", "720"));
const quietAt = Number(option("quiet-at", "-1"));
const loudAt = Number(option("loud-at", "-1"));
const vanishAt = Number(option("vanish-at", "-1"));
const pokeAt = Number(option("poke-at", "-1"));
const epochSeconds = Number(option("epoch", "-1"));
const brief = flag("brief");
const wander = flag("pointer");

const cast = menagerie.casts.find((entry) => entry.scene === scene) ?? menagerie.casts[0];
if (cast === undefined) throw new Error("the menagerie has no cast");
const cards: Surface[] = [
  { id: "card-a", x0: width * 0.06, x1: width * 0.4, y: height * 0.42 },
  { id: "card-b", x0: width * 0.46, x1: width * 0.86, y: height * 0.42 },
  { id: "card-c", x0: width * 0.2, x1: width * 0.7, y: height * 0.72 },
];
const floor: Surface = { id: "floor", x0: 0, x1: width, y: height };
const keepouts: Rect[] = [
  { x: width * 0.3, y: height * 0.42 - 30, width: 60, height: 28 },
  { x: width * 0.88, y: height - 40, width: width * 0.12, height: 40 },
];
const surveyed = (surfaces: Surface[]): StageEvent => ({ kind: "surveyed", width, height, surfaces, keepouts });

let stage: Stage = advance(menagerie, openStage(seed), [{ kind: "tuned", mode }, surveyed([...cards, floor]), { kind: "summoned", species: castOf(cast, capacity, 0, seed) }]);
const total = Math.floor(minutes * 60 * TICKS_PER_SECOND);
const spans = new Map<string, number[]>();
const overall = ACTIVITIES.map(() => 0);
const rates = new Map<number, number>();
const starts = new Map<string, number>();
const encounters = new Map<string, number>();
const lines: string[] = [];
let moversPeak = 0;
let moverTicks = 0;
let actorTicks = 0;
let fidgetersPeak = 0;
let falls = 0;
let epoch = 0;

/** 🗒️ One line of the timeline. */
function note(tick: number, who: string, what: string): void {
  if (!brief) lines.push(`${clock(tick)}  ${who.padEnd(12)} ${what}`);
}

/** 🔍️ What changed for one actor between two ticks. */
function compare(before: Actor | undefined, after: Actor | undefined, tick: number, species: string): void {
  if (after === undefined) {
    if (before !== undefined) note(tick, species, "is gone");
    return;
  }
  if (before === undefined) {
    note(tick, species, `arrives on ${after.perch} at x=${after.x.toFixed(0)}`);
    return;
  }
  if (!before.leaving && after.leaving) note(tick, species, "is summoned away");
  if (before.activity === after.activity && before.since === after.since) return;
  const activity = after.activity;
  starts.set(activity, (starts.get(activity) ?? 0) + 1);
  if (activity === "fall") falls++;
  if ((activity === "greet" || activity === "cuddle" || activity === "squabble") && after.partner !== null && species < after.partner) encounters.set(activity, (encounters.get(activity) ?? 0) + 1);
  const detail =
    activity === "walk"
      ? `walks ${after.goal >= after.x ? "→" : "←"} ${Math.abs(after.goal - after.x).toFixed(0)} px${after.partner === null ? "" : ` to ${after.partner}`}${after.leaving ? " (leaving)" : ""}`
      : activity === "hop"
        ? `hops to x=${after.goal.toFixed(0)}`
        : activity === "idle"
          ? after.partner === null
            ? before.activity === "idle"
              ? ""
              : `rests (${((after.until - tick) / TICKS_PER_SECOND).toFixed(0)} s)`
            : `waits for ${after.partner}`
          : activity === "land"
            ? `lands on ${after.perch}`
            : `${activity}${after.partner === null ? "" : ` with ${after.partner}`}${after.clip === null ? "" : ` [${after.clip}]`} (${((after.until - tick) / TICKS_PER_SECOND).toFixed(1)} s)`;
  if (detail !== "") note(tick, species, detail);
}

for (let tick = 1; tick <= total; tick++) {
  const events: StageEvent[] = [];
  const second = tick / TICKS_PER_SECOND;
  if (second === quietAt) events.push({ kind: "hushed", quiet: true });
  if (second === loudAt) events.push({ kind: "hushed", quiet: false });
  if (second === vanishAt) events.push(surveyed([cards[1]!, cards[2]!, floor]));
  if (second === pokeAt && stage.actors.length > 0) events.push({ kind: "poked", x: stage.actors[0]!.x, y: stage.actors[0]!.y - 10 });
  if (wander && tick % (7 * TICKS_PER_SECOND) === 0) events.push({ kind: "pointed", x: (tick * 37) % width, y: (tick * 17) % height });
  if (epochSeconds > 0 && tick % (epochSeconds * TICKS_PER_SECOND) === 0) {
    epoch++;
    events.push({ kind: "summoned", species: castOf(cast, capacity, epoch, seed) });
  }
  for (const event of events) note(tick, "· stage", event.kind === "summoned" ? `summoned ${event.species.join(", ")}` : event.kind);
  events.push({ kind: "ticked", ticks: 1 });
  const next = advance(menagerie, stage, events);
  for (const species of menagerie.species) compare(stage.actors.find((actor) => actor.species === species.id), next.actors.find((actor) => actor.species === species.id), tick, species.id);
  if (next.rapports !== stage.rapports && !brief) note(tick, "· rapport", next.rapports.map((rapport) => `${rapport.between.join("–")} ${rapport.drift >= 0 ? "+" : ""}${rapport.drift.toFixed(3)}`).join(", ") || "none");
  stage = next;
  let movers = 0;
  let fidgeters = 0;
  for (const actor of stage.actors) {
    const index = ACTIVITIES.indexOf(actor.activity);
    overall[index]!++;
    actorTicks++;
    if (!spans.has(actor.species)) spans.set(actor.species, ACTIVITIES.map(() => 0));
    spans.get(actor.species)![index]!++;
    if ((actor.activity === "walk" || actor.activity === "hop") && !actor.leaving) movers++;
    if (actor.activity === "fidget") fidgeters++;
  }
  moversPeak = Math.max(moversPeak, movers);
  fidgetersPeak = Math.max(fidgetersPeak, fidgeters);
  if (movers > 0) moverTicks++;
  const rate = frameOf(menagerie, stage).rate;
  rates.set(rate, (rates.get(rate) ?? 0) + 1);
}

for (const line of lines) console.log(line);
console.log("");
console.log(`== ${menagerie.id} · scene ${cast.scene} · mode ${mode} · seed ${seed} · ${minutes} min · capacity ${capacity} · limits ${JSON.stringify(MODE_LIMITS[mode])}`);
console.log(`share of time per activity (all actors): ${ACTIVITIES.map((activity, index) => `${activity} ${percent(overall[index]!, actorTicks).trim()}`).join(" · ")}`);
for (const [species, counts] of spans) {
  const sum = counts.reduce((left, right) => left + right, 0);
  console.log(`  ${species.padEnd(12)} ${ACTIVITIES.filter((_, index) => counts[index]! > 0).map((activity) => `${activity} ${percent(counts[ACTIVITIES.indexOf(activity as Activity)]!, sum).trim()}`).join(" · ")}`);
}
console.log(`starts per minute: ${[...starts.entries()].map(([activity, count]) => `${activity} ${(count / minutes).toFixed(2)}`).join(" · ")}`);
console.log(`encounters per minute: ${(([...encounters.values()].reduce((left, right) => left + right, 0)) / minutes).toFixed(2)} (${[...encounters.entries()].map(([kind, count]) => `${kind} ${count}`).join(", ") || "none"})`);
console.log(`movers at once: peak ${moversPeak}, somebody walks or hops ${percent(moverTicks, total).trim()} of the time · fidgeters at once: peak ${fidgetersPeak} · falls ${falls}`);
console.log(`frame rate share: ${[64, 32, 16, 0].map((rate) => `${rate} → ${percent(rates.get(rate) ?? 0, total).trim()}`).join(" · ")}`);
console.log(`final rapports: ${stage.rapports.map((rapport) => `${rapport.between.join("–")} ${rapport.drift.toFixed(3)}`).join(", ") || "none"} · stage draws ${stage.draws}`);
