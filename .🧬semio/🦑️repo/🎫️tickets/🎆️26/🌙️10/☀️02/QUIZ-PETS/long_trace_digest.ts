/** 🧪️ Ticket tool of work package M, the TypeScript half of the long-run proof that the Rust twin of the pets stage yields the same bits: it composes scripted sessions, plays them through `@semio-tech/pets` and records two digests per simulated second.
 *
 * A session is 20 minutes (76 800 ticks) on one menagerie with one seed: a survey and a summons at tick 0, then every
 * 2 to 20 seconds something happens — the pointer wanders in bursts or leaves, someone is poked (aimed at an actor
 * that is on stage right now), glance points come and go, the terrain scrolls, loses a card, gains a shelf, is
 * measured again unchanged or is laid out anew at another size, the company changes, the mode is tuned (every session visits all
 * three modes and returns to its own) and a time of concentration begins or ends. The events are drawn from a small
 * linear congruential generator seeded by the session, so a rerun composes the same sessions; coordinates are
 * arbitrary doubles, the way a DOM survey delivers them.
 *
 * Sessions: both menageries (the trace troupe of the stage-trace fixture and the architecture menagerie), four
 * seeds, the three modes, quiet on and off — 48 sessions. Per session the tool writes every event with the tick it
 * happens at, and per second two FNV-1a digests over IEEE-754 bit patterns (little-endian doubles): the running
 * digest of every frame so far (the digest of the stage-trace case) and the digest of every number of the stage at
 * that tick (needs, gaze velocities, counters, rapports … — what no frame shows yet). `long_trace_digest.rs` replays
 * the same file through the Rust twin and compares checkpoint by checkpoint.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/long_trace_digest.ts [--minutes 20]
 *   bash .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/rust_scratch.sh wp-m run --release --offline --features sut --example long_trace_digest
 *
 * Output: `🗑️generated/wp-m/long-trace/{sessions.json, typescript-digests.json}`.
 *
 * @see ./long_trace_digest.rs — the Rust half
 * @see ./stage_scratch.ts — creates the scratch crate the Rust half runs in
 * @see ../../../../../../../🧰️framework/🛍️products/🐾️pets/🧪️tests/🎪️stage-trace/🟦️.ts — the frame digest
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { ACTIVITIES, PET_MODES, TICKS_PER_SECOND, type Frame, type Menagerie, type PetMode, type Point, type Rect, type Slug, type Stage, type StageEvent, type Surface } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { castOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🧠️behavior/🟦️.ts";
import { advance, frameOf, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const OUT = join(import.meta.dir, "🗑️generated", "wp-m", "long-trace");
const TROUPE = "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🎪️stage-trace/🔣️.json";
const ARCHITECTURE = "🎓️teaching/🏛️architecture/🐾️pets/🟦️.ts";
const FNV_OFFSET_BASIS = 2166136261;
const FNV_PRIME = 16777619;
const SEEDS = [1, 7, 20261002, 4294967295];
const SIZES: readonly (readonly [number, number])[] = [[1280, 720], [1024, 768], [390, 800], [1920, 1080]];
const BYTES = new DataView(new ArrayBuffer(8));

/** 🪜️ The events that happen at one tick of a session, before that tick passes. */
type Step = { readonly at: number; readonly events: StageEvent[] };

/** 📜️ A scripted session: its menagerie, the seed of the stage, how many ticks pass and what happens when. */
type Session = { readonly id: string; readonly menagerie: string; readonly seed: number; readonly ticks: number; readonly steps: Step[] };

/** 🗺️ What the composer remembers of the terrain it last surveyed. */
type Terrain = { width: number; height: number; surfaces: Surface[]; keepouts: Rect[] };

/** 🔢️ FNV-1a over the eight bytes of a double, little-endian. */
function fold(hash: number, value: number): number {
  BYTES.setFloat64(0, value, true);
  let folded = hash;
  for (let index = 0; index < 8; index++) folded = Math.imul(folded ^ BYTES.getUint8(index), FNV_PRIME) >>> 0;
  return folded;
}

/** 🧮️ The running digest after one more frame: the digest of the stage-trace case. */
function foldFrame(hash: number, menagerie: Menagerie, frame: Frame): number {
  let folded = fold(fold(fold(fold(hash, frame.tick), frame.rate), frame.wake === null ? -1 : frame.wake), frame.actors.length);
  for (const actor of frame.actors) {
    folded = fold(folded, menagerie.species.findIndex((species) => species.id === actor.species));
    folded = fold(fold(fold(fold(fold(folded, actor.x), actor.y), actor.facing), ACTIVITIES.indexOf(actor.activity)), actor.opacity);
    for (const number of actor.bones) folded = fold(folded, number);
    for (const eye of actor.eyes) folded = fold(fold(fold(folded, eye.x), eye.y), eye.lid);
    folded = fold(folded, actor.mood);
  }
  return folded;
}

/** 🧾️ The digest of every number of a stage; names become indices (species in the menagerie, surfaces in the survey, clips in the species, −1 for none). */
function foldStage(menagerie: Menagerie, stage: Stage): number {
  const kind = (id: Slug): number => menagerie.species.findIndex((species) => species.id === id);
  const surface = (id: string): number => stage.surfaces.findIndex((candidate) => candidate.id === id);
  const numbers: number[] = [stage.seed, stage.tick, PET_MODES.indexOf(stage.mode), stage.quiet ? 1 : 0, stage.width, stage.height, stage.pointer === null ? 0 : 1, stage.pointer === null ? 0 : stage.pointer.x, stage.pointer === null ? 0 : stage.pointer.y, stage.pointed, stage.glances.length];
  for (const glance of stage.glances) numbers.push(glance.x, glance.y);
  numbers.push(stage.surfaces.length, stage.keepouts.length, stage.perches.length);
  for (const perch of stage.perches) numbers.push(surface(perch.surface), perch.x0, perch.x1, perch.y);
  numbers.push(stage.wanted.length);
  for (const wanted of stage.wanted) numbers.push(kind(wanted));
  numbers.push(stage.actors.length);
  for (const actor of stage.actors) {
    const species = menagerie.species[kind(actor.species)];
    numbers.push(kind(actor.species), actor.perch === null ? -1 : surface(actor.perch), actor.x, actor.y, actor.vx, actor.vy, actor.facing, actor.faced, ACTIVITIES.indexOf(actor.activity), actor.since, actor.until, actor.goal);
    numbers.push(actor.partner === null ? -1 : kind(actor.partner), actor.clip === null || species === undefined ? -1 : species.clips.findIndex((clip) => clip.id === actor.clip));
    numbers.push(actor.gaze.x, actor.gaze.y, actor.gaze.vx, actor.gaze.vy, actor.blink, actor.mood, actor.needs.energy, actor.needs.sociability, actor.needs.curiosity, actor.opacity, actor.leaving ? 1 : 0, actor.draws);
  }
  numbers.push(stage.rapports.length);
  for (const rapport of stage.rapports) numbers.push(kind(rapport.between[0]), kind(rapport.between[1]), rapport.drift);
  numbers.push(stage.met, stage.draws);
  let folded = FNV_OFFSET_BASIS;
  for (const number of numbers) folded = fold(folded, number);
  return folded;
}

/** 🎲️ A linear congruential generator: units in [0, 1), the same for the same seed. */
function generator(seed: number): () => number {
  let state = seed >>> 0;
  return () => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return state / 4294967296;
  };
}

/** 🏗️ A terrain laid out anew: a stage box, three to six cards at arbitrary places, the floor, and up to six keep-outs that sit on or near the cards. */
function layout(unit: () => number): Terrain {
  const [width, height] = SIZES[Math.floor(unit() * SIZES.length)]!;
  const surfaces: Surface[] = [];
  const cards = 3 + Math.floor(unit() * 4);
  for (let card = 0; card < cards; card++) {
    const span = 120 + unit() * (width * 0.6);
    const x0 = unit() * (width - span * 0.5) - span * 0.1;
    surfaces.push({ id: `card-${card}`, x0, x1: x0 + span, y: 90 + unit() * (height - 180) });
  }
  surfaces.push({ id: "floor", x0: 0, x1: width, y: height });
  const keepouts: Rect[] = [];
  const boxes = Math.floor(unit() * 7);
  for (let box = 0; box < boxes; box++) {
    const on = surfaces[Math.floor(unit() * surfaces.length)]!;
    const span = 30 + unit() * 170;
    keepouts.push({ x: on.x0 + unit() * (on.x1 - on.x0), y: on.y - 10 - unit() * 60, width: unit() < 0.1 ? 0 : span, height: 12 + unit() * 60 });
  }
  return { width, height, surfaces, keepouts };
}

/** 📡️ The survey event of a terrain. */
function surveyed(terrain: Terrain): StageEvent {
  return { kind: "surveyed", width: terrain.width, height: terrain.height, surfaces: terrain.surfaces, keepouts: terrain.keepouts };
}

/** 📍️ A point on stage: near an actor that is there right now three times out of four, anywhere otherwise. */
function spot(unit: () => number, stage: Stage, menagerie: Menagerie, terrain: Terrain, reach: number): Point {
  if (stage.actors.length > 0 && unit() < 0.75) {
    const actor = stage.actors[Math.floor(unit() * stage.actors.length)]!;
    const species = menagerie.species.find((candidate) => candidate.id === actor.species)!;
    return { x: actor.x + (unit() - 0.5) * species.size.width * reach, y: actor.y - unit() * species.size.height * reach };
  }
  return { x: unit() * terrain.width, y: unit() * terrain.height };
}

/** 🎟️ A company for the stage: a cast of the menagerie at a capacity and an epoch, any few species, or nobody at all. */
function company(unit: () => number, menagerie: Menagerie, seed: number): Slug[] {
  const choice = unit();
  if (choice < 0.1) return [];
  if (choice < 0.6 && menagerie.casts.length > 0) {
    const cast = menagerie.casts[Math.floor(unit() * menagerie.casts.length)]!;
    return castOf(cast, [2, 4, 6][Math.floor(unit() * 3)]!, Math.floor(unit() * 12), seed);
  }
  const wanted: Slug[] = [];
  const count = 1 + Math.floor(unit() * Math.min(6, menagerie.species.length));
  while (wanted.length < count) {
    const id = menagerie.species[Math.floor(unit() * menagerie.species.length)]!.id;
    if (!wanted.includes(id)) wanted.push(id);
  }
  return wanted;
}

/** 🎬️ One session composed and played: the script and its digests per second, `[frames so far, stage]` flat. */
function play(id: string, name: string, menagerie: Menagerie, seed: number, mode: PetMode, quiet: boolean, ticks: number): { session: Session; digests: number[] } {
  const unit = generator((seed ^ Math.imul(PET_MODES.indexOf(mode) + 1, 0x9e3779b1) ^ (quiet ? 0x5bd1e995 : 0) ^ Math.imul(name.length, 0x85ebca6b)) >>> 0);
  const steps: Step[] = [];
  const later = new Map<number, StageEvent[]>();
  const digests: number[] = [];
  const schedule = (at: number, event: StageEvent): void => {
    later.set(at, [...(later.get(at) ?? []), event]);
  };
  let terrain = layout(unit);
  let stage = openStage(seed);
  let hash = FNV_OFFSET_BASIS;
  let next = 128 + Math.floor(unit() * 512);
  for (let tick = 0; tick < ticks; tick++) {
    const events: StageEvent[] = later.get(tick) ?? [];
    later.delete(tick);
    if (tick === 0) {
      events.push(surveyed(terrain), { kind: "summoned", species: menagerie.casts.length > 0 ? castOf(menagerie.casts[0]!, 6, 0, seed) : menagerie.species.slice(0, 6).map((species) => species.id) });
      if (mode !== "calm") events.push({ kind: "tuned", mode });
      if (quiet) events.push({ kind: "hushed", quiet: true });
      for (const [share, other] of [[0.25, "still"], [0.5, "lively"], [0.75, "calm"]] as const) {
        const at = Math.floor(ticks * share) + Math.floor(unit() * 640);
        schedule(at, { kind: "tuned", mode: other });
        schedule(at + 640 + Math.floor(unit() * 1280), { kind: "tuned", mode });
      }
    }
    if (tick === next) {
      const choice = unit();
      if (choice < 0.3) {
        const moves = 1 + Math.floor(unit() * 16);
        let at = tick;
        for (let move = 0; move < moves; move++) {
          const point = spot(unit, stage, menagerie, terrain, 4);
          if (move === 0) events.push({ kind: "pointed", x: point.x, y: point.y });
          else schedule(at, { kind: "pointed", x: point.x, y: point.y });
          at += 2 + Math.floor(unit() * 30);
        }
        if (unit() < 0.3) schedule(at, { kind: "unpointed" });
      } else if (choice < 0.45) {
        const point = spot(unit, stage, menagerie, terrain, 1);
        events.push({ kind: "poked", x: point.x, y: point.y });
      } else if (choice < 0.55) {
        const points: Point[] = [];
        for (let glance = Math.floor(unit() * 4); glance > 0; glance--) points.push(spot(unit, stage, menagerie, terrain, 6));
        events.push({ kind: "glanced", points });
      } else if (choice < 0.75) {
        const change = unit();
        if (change < 0.3) {
          const dy = (unit() - 0.5) * 300;
          terrain = { ...terrain, surfaces: terrain.surfaces.map((surface) => (surface.id === "floor" ? surface : { ...surface, y: surface.y + dy })), keepouts: terrain.keepouts.map((keepout) => ({ ...keepout, y: keepout.y + dy })) };
        } else if (change < 0.5) {
          const dx = (unit() - 0.5) * 240;
          const moved = terrain.surfaces[Math.floor(unit() * terrain.surfaces.length)]!.id;
          terrain = { ...terrain, surfaces: terrain.surfaces.map((surface) => (surface.id === moved && surface.id !== "floor" ? { ...surface, x0: surface.x0 + dx, x1: surface.x1 + dx } : surface)) };
        } else if (change < 0.65 && terrain.surfaces.length > 1) {
          const gone = Math.floor(unit() * (terrain.surfaces.length - (unit() < 0.2 ? 0 : 1)));
          terrain = { ...terrain, surfaces: terrain.surfaces.filter((_, index) => index !== gone) };
        } else if (change < 0.8) terrain = layout(unit);
        else if (change < 0.92) {
          const span = 120 + unit() * (terrain.width * 0.5);
          const x0 = unit() * (terrain.width - span);
          terrain = { ...terrain, surfaces: [{ id: `shelf-${tick}`, x0, x1: x0 + span, y: 90 + unit() * (terrain.height - 180) }, ...terrain.surfaces] };
        }
        events.push(surveyed(terrain));
      } else if (choice < 0.88) events.push({ kind: "summoned", species: company(unit, menagerie, seed) });
      else if (choice < 0.94) {
        const other = PET_MODES[Math.floor(unit() * PET_MODES.length)]!;
        events.push({ kind: "tuned", mode: other });
        schedule(tick + 320 + Math.floor(unit() * 1600), { kind: "tuned", mode });
      } else {
        events.push({ kind: "hushed", quiet: !stage.quiet });
        schedule(tick + 320 + Math.floor(unit() * 3200), { kind: "hushed", quiet });
      }
      next = tick + 128 + Math.floor(unit() * 1152);
    }
    if (events.length > 0) {
      steps.push({ at: tick, events });
      stage = advance(menagerie, stage, events);
    }
    stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
    hash = foldFrame(hash, menagerie, frameOf(menagerie, stage));
    if ((tick + 1) % TICKS_PER_SECOND === 0) digests.push(hash, foldStage(menagerie, stage));
  }
  return { session: { id, menagerie: name, seed, ticks, steps }, digests };
}

/** 🎪️ The menageries the sessions play on: the trace troupe and the architecture menagerie. */
async function menageries(): Promise<Record<string, Menagerie>> {
  const troupe = (JSON.parse(readFileSync(join(ROOT, TROUPE), "utf8")) as { menagerie: Menagerie }).menagerie;
  const module = (await import(pathToFileURL(join(ROOT, ARCHITECTURE)).href)) as { default: Menagerie };
  return { troupe, architecture: module.default };
}

/** 🔍️ The first place two JSON values differ, numbers compared as the same double; `undefined` when they are the same. */
function difference(path: string, left: unknown, right: unknown): string | undefined {
  if (typeof left === "number" && typeof right === "number") return Object.is(left, right) || left === right ? undefined : `${path}: typescript ${left} rust ${right}`;
  if (Array.isArray(left) && Array.isArray(right)) {
    if (left.length !== right.length) return `${path}: ${left.length} entries in typescript, ${right.length} in rust`;
    for (let index = 0; index < left.length; index++) {
      const found = difference(`${path}[${index}]`, left[index], right[index]);
      if (found !== undefined) return found;
    }
    return undefined;
  }
  if (left !== null && right !== null && typeof left === "object" && typeof right === "object") {
    for (const key of new Set([...Object.keys(left), ...Object.keys(right)])) {
      const found = difference(`${path}.${key}`, (left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key]);
      if (found !== undefined) return found;
    }
    return undefined;
  }
  return left === right ? undefined : `${path}: typescript ${JSON.stringify(left)} rust ${JSON.stringify(right)}`;
}

/** 🩺️ The probe: replays one recorded session and writes its stage and frame after every tick of a window as JSON lines (`probe-typescript.jsonl`); when the Rust half has written the same window (`probe-rust.jsonl`, its `--probe`), it names the first tick and member in which the two differ. */
function probe(id: string, from: number, to: number): void {
  const recorded = JSON.parse(readFileSync(join(OUT, "sessions.json"), "utf8")) as { menageries: Record<string, Menagerie>; sessions: Session[] };
  const session = recorded.sessions.find((candidate) => candidate.id === id);
  if (session === undefined) throw new Error(`no recorded session ${id}`);
  const menagerie = recorded.menageries[session.menagerie]!;
  const byTick = new Map(session.steps.map((step) => [step.at, step.events]));
  const lines: string[] = [];
  let stage = openStage(session.seed);
  for (let tick = 0; tick < Math.min(to, session.ticks); tick++) {
    const events = byTick.get(tick);
    if (events !== undefined) stage = advance(menagerie, stage, events);
    stage = advance(menagerie, stage, [{ kind: "ticked", ticks: 1 }]);
    if (tick + 1 >= from) lines.push(JSON.stringify({ tick: tick + 1, stage, frame: frameOf(menagerie, stage) }));
  }
  writeFileSync(join(OUT, "probe-typescript.jsonl"), `${lines.join("\n")}\n`);
  let rust: string[] = [];
  try {
    rust = readFileSync(join(OUT, "probe-rust.jsonl"), "utf8").trim().split("\n");
  } catch {
    console.log(`[DEBUG] probe: ${lines.length} ticks of ${id} written; no probe-rust.jsonl to compare with`);
    return;
  }
  for (let line = 0; line < Math.min(lines.length, rust.length); line++) {
    const found = difference("$", JSON.parse(lines[line]!), JSON.parse(rust[line]!));
    if (found !== undefined) {
      console.log(`[DEBUG] probe: ${id} first differs at tick ${from + line}: ${found}`);
      return;
    }
  }
  console.log(`[DEBUG] probe: ${id} ticks ${from}…${from + Math.min(lines.length, rust.length) - 1}: no difference (${lines.length} typescript lines, ${rust.length} rust lines)`);
}

/** 🎼️ Composes and plays every session and writes the scripts and the digests. */
async function record(minutes: number): Promise<void> {
  const ticks = Math.floor(minutes * 60 * TICKS_PER_SECOND);
  const stages = await menageries();
  const sessions: Session[] = [];
  const recorded: Record<string, number[]> = {};
  const started = performance.now();
  const tally = { events: 0, checkpoints: 0 };
  for (const [name, menagerie] of Object.entries(stages)) {
    for (const seed of SEEDS) {
      for (const mode of PET_MODES) {
        for (const quiet of [false, true]) {
          const id = `${name}-seed-${seed}-${mode}${quiet ? "-quiet" : ""}`;
          const played = play(id, name, menagerie, seed, mode, quiet, ticks);
          sessions.push(played.session);
          recorded[id] = played.digests;
          tally.events += played.session.steps.reduce((sum, step) => sum + step.events.length, 0);
          tally.checkpoints += played.digests.length / 2;
          console.log(`[DEBUG] ${id}: ${played.session.steps.length} steps, ${played.digests.length / 2} checkpoints, last ${played.digests[played.digests.length - 2]} ${played.digests[played.digests.length - 1]}`);
        }
      }
    }
  }
  mkdirSync(OUT, { recursive: true });
  writeFileSync(join(OUT, "sessions.json"), JSON.stringify({ menageries: stages, sessions }));
  writeFileSync(join(OUT, "typescript-digests.json"), JSON.stringify(recorded));
  console.log(`[DEBUG] typescript: ${sessions.length} sessions of ${ticks} ticks, ${tally.events} events, ${tally.checkpoints} checkpoints, ${Math.round(performance.now() - started)} ms → ${OUT}`);
}

const probing = process.argv.indexOf("--probe");
const timed = process.argv.indexOf("--minutes");
if (probing >= 0) probe(process.argv[probing + 1]!, Number(process.argv[probing + 2]), Number(process.argv[probing + 3]));
else await record(timed >= 0 ? Number(process.argv[timed + 1]) : 20);
