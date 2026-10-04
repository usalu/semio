/** ⏱️ Ticket tool of work package B3: how long `frameOf` takes for a crowded stage — ten actors, eight of them running a plume of their own (eight emitters, 32 particles alive each, so the stage-wide cap of 160 bites) —, in microseconds per frame, beside the same stage without plumes.
 *
 * Usage (from the repository root): `bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/b3_measure.ts" [frames]`
 * Prints one line per measurement; writes nothing.
 */
import { readFileSync } from "node:fs";
import type { Actor, Emitter, Menagerie, Species, Stage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { advance, openStage } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🟦️.ts";
import { frameOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎥️projection/🟦️.ts";

const ROOT = "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json";
const FRAMES = Number(process.argv[2] ?? 4000);
const sample = (JSON.parse(readFileSync(ROOT, "utf8")) as { menagerie: Menagerie }).menagerie;
const blobby = sample.species[0]!;
const motions: readonly Emitter["motion"][] = ["fall", "rise", "drift", "orbit", "fall", "rise", "drift", "burst"];

/** 🧬️ Species `index` of the crowd: blobby under another id with one emitter of its own (32 alive, a second of life). */
function member(index: number): Species {
  const emitter: Emitter = { id: "spray", bone: "tuft", x: 0, y: -4, shape: { kind: "ellipse", cx: 0, cy: 0, rx: 1.5, ry: 1.5 }, fill: "accent", stroke: "none", motion: motions[index % motions.length]!, count: 32, life: 1, speed: 60, spread: 0.5 };
  return { ...blobby, id: `blob-${index}`, emitters: [emitter] };
}

const menagerie: Menagerie = { ...sample, species: Array.from({ length: 10 }, (_, index) => member(index)), bonds: [], casts: [], chemistry: [] };
const ids = menagerie.species.map((kind) => kind.id);
let stage = advance(menagerie, openStage(9), [
  { kind: "tuned", mode: "lively" },
  { kind: "surveyed", width: 1600, height: 720, surfaces: [{ id: "floor", x0: 0, x1: 1600, y: 720 }], keepouts: [], walls: [], fixtures: [] },
  { kind: "summoned", species: ids },
  { kind: "ticked", ticks: 4 * 64 },
]);

/** 💨️ The stage with the first `count` actors running their plume since 2 s ago. */
function plumed(base: Stage, count: number): Stage {
  return { ...base, actors: base.actors.map((actor: Actor, index) => ({ ...actor, emitters: index < count ? [{ emitter: "spray", since: base.tick - 128, until: null }] : [] })) };
}

/** 🧪️ Microseconds per frame over `FRAMES` frames of consecutive ticks, after a warm-up. */
function measure(base: Stage): { micros: number; particles: number; actors: number } {
  let particles = 0;
  for (let tick = 0; tick < 200; tick++) particles = frameOf(menagerie, { ...base, tick: base.tick + tick }).particles.length;
  const started = performance.now();
  for (let tick = 0; tick < FRAMES; tick++) particles = frameOf(menagerie, { ...base, tick: base.tick + tick }).particles.length;
  return { micros: ((performance.now() - started) * 1000) / FRAMES, particles, actors: base.actors.length };
}

stage = { ...stage, actors: stage.actors.map((actor) => ({ ...actor, opacity: 1 })) };
for (const [label, base] of [
  ["no plumes", plumed(stage, 0)],
  ["8 plumes (8 emitters, cap reached)", plumed(stage, 8)],
  ["no plumes", plumed(stage, 0)],
  ["8 plumes (8 emitters, cap reached)", plumed(stage, 8)],
] as const) {
  const result = measure(base);
  process.stdout.write(`${label}: ${result.actors} actors, ${result.particles} particles in the last frame, ${result.micros.toFixed(1)} µs per frame over ${FRAMES} frames\n`);
}
