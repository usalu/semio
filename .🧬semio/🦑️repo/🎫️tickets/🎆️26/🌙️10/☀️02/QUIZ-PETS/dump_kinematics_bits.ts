#!/usr/bin/env bun
/** 🧱️ Ticket tool of work package K: dumps the outputs of the TypeScript kinematics (`📐️trigonometry`, `🎲️randomness`, `🦴️rig`) for a few hundred inputs per function as IEEE-754 bit patterns, so the Rust twins can be held to them bit for bit.
 *
 * Inputs and outputs are sixteen hexadecimal digits per double (no decimal text, so no JSON reader can round them
 * differently); integers stay integers. The inputs are drawn with the module's own counter-based randomness
 * (a pure function of the indices below) and joined by hand-picked edges. From the repository root:
 *
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/dump_kinematics_bits.ts
 *
 * writes `🗑️generated/wp-k/kinematics-bits.json` beside this file; `compare_kinematics_bits.rs` reads it in the
 * scratch crate (`bash rust_scratch.sh wp-k test --offline proof -- --nocapture`).
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Species } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { clamp, cosTurns, lerp, sinTurns, smoothstep } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📐️trigonometry/🟦️.ts";
import { randomBetween, randomPick, randomUnit, randomWords } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎲️randomness/🟦️.ts";
import { type Affine, type Pose, compose, invert, lookOffset, solveRig, transform } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🦴️rig/🟦️.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(HERE, "../../../../../../..");
const SEED = 20261003;
const VIEW = new DataView(new ArrayBuffer(8));

/** 🔬️ The 64-bit pattern of a double as sixteen hexadecimal digits. */
function bits(value: number): string {
  VIEW.setFloat64(0, value);
  return VIEW.getBigUint64(0).toString(16).padStart(16, "0");
}

/** 🎰️ A double between the bounds, a pure function of its stream and index. */
function drawn(stream: number, index: number, low: number, high: number): number {
  return randomBetween([SEED, stream, index], low, high);
}

/** 🧮️ `count` numbers produced by `make`. */
function series(count: number, make: (index: number) => number): number[] {
  return Array.from({ length: count }, (_, index) => make(index));
}

/** 🔳️ A matrix with entries from [−4, 4] and a translation from [−400, 400]. */
function matrix(stream: number, index: number): Affine {
  return [drawn(stream, index * 6, -4, 4), drawn(stream, index * 6 + 1, -4, 4), drawn(stream, index * 6 + 2, -4, 4), drawn(stream, index * 6 + 3, -4, 4), drawn(stream, index * 6 + 4, -400, 400), drawn(stream, index * 6 + 5, -400, 400)];
}

const angles = [
  ...[0, -0, 0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875, 1, -0.125, -0.25, -0.5, -0.75, -1, 1 / 3, 1 / 7, 20 / 360, -20 / 360, 5e-324, -5e-324, 1e-300, 2 ** -27, 2 ** -54, 0.124999999999999986, 0.12500000000000003, 0.375 - 2 ** -54, 0.625 + 2 ** -53, 123456.789, -98765.4321, 2 ** 52 + 0.5, 2 ** 53, 1e300, -1e300, Number.MAX_VALUE],
  ...series(200, (index) => drawn(1, index, -4, 4)),
  ...series(100, (index) => drawn(2, index, -720, 720) / 360),
  ...series(60, (index) => drawn(3, index, -1e6, 1e6)),
  ...series(40, (index) => (index - 20) / 16),
];
const clamps = [...series(300, (index) => [drawn(4, index, -3, 3), drawn(5, index, -1, 1), drawn(6, index, -1, 2)]), [0, 2, -2], [-5, 2, -2], [-0, 0, 0], [0.5, 0, 1]];
const lerps = series(300, (index) => 0).map((_, index) => [drawn(7, index, -500, 500), drawn(8, index, -500, 500), drawn(9, index, -0.5, 1.5)]);
const steps = [...series(300, (index) => drawn(10, index, -0.5, 1.5)), 0, 1, 0.5, -0, 1e-300];
const keys = [[], [0], [5], [5, 0, 0, 0, 0], [SEED, 0xffffffff, 0], [0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff], ...series(300, (index) => index).map((index) => [SEED, index % 7, index * 2654435761])].map((key) => key.map((word) => word >>> 0));
const ranges = series(300, (index) => index).map((index) => ({ key: [SEED, 11, index], low: drawn(12, index, -100, 100), high: drawn(13, index, -100, 100) }));
const picks = series(300, (index) => index).map((index) => ({ key: [SEED, 14, index], weights: series(1 + (index % 11), (slot) => (slot % 4 === 3 ? 0 : slot % 5 === 4 ? -1 : drawn(15, index * 16 + slot, 0, 3))) }));
const pairs = series(300, (index) => index).map((index) => ({ parent: matrix(16, index), local: matrix(17, index) }));
const singles = [...series(300, (index) => matrix(18, index)), [1, 0, 0, 1, 0, 0], [1, 2, 2, 4, 5, 6], [0, 0, 0, 0, 3, 4], [0, 1, -1, 0, 3, 4], [2, 0, 0, 4, 6, 8]] as Affine[];
const points = series(300, (index) => index).map((index) => ({ matrix: matrix(19, index), x: drawn(20, index, -2000, 2000), y: drawn(21, index, -2000, 2000) }));
const looks = [
  ...series(300, (index) => index).map((index) => ({ eye: [drawn(22, index, 0, 1920), drawn(23, index, 0, 1080)], target: [drawn(24, index, -200, 2120), drawn(25, index, -200, 1280)], reach: drawn(26, index, -20, 200) })),
  { eye: [12.5, -3], target: [12.5, -3], reach: 40 },
  { eye: [3, 4], target: [0, 0], reach: 0 },
  { eye: [0, 0], target: [1e-200, 1e-200], reach: 40 },
  { eye: [0, 0], target: [1e200, -1e200], reach: 40 },
];
const species = (JSON.parse(readFileSync(resolve(ROOT, "🧰️framework/🛍️products/🐾️pets/🧫️fixtures/🦴️rig-solving/🔣️.json"), "utf8")) as { readonly species: readonly Species[] }).species;
const poses = species.flatMap((kind, which) =>
  series(150, (index) => index).map((index) => {
    const covered = index % 10 === 9 ? Math.max(0, kind.bones.length - 2) : kind.bones.length;
    const pose: Pose = series(covered, (bone) => bone).map((bone) => {
      const slot = (which * 150 + index) * 16 + bone;
      const turned = index % 5 === 0 ? [0, 90, 180, 270, 360, -90, -360, 720][(index / 5 + bone) % 8]! : drawn(29, slot, -720, 720);
      return { x: drawn(27, slot, -8, 8), y: drawn(28, slot, -8, 8), rotation: turned, scaleX: drawn(30, slot, -1.5, 1.5), scaleY: drawn(31, slot, 0.5, 1.5) };
    });
    return { species: kind.id, pose };
  }),
);

const document = {
  $comment: "Generated by dump_kinematics_bits.ts of ticket QUIZ-PETS (work package K) from the TypeScript kinematics — a proof run, not a fixture.",
  sinTurns: angles.map((turns) => ({ in: [bits(turns)], out: [bits(sinTurns(turns))] })),
  cosTurns: angles.map((turns) => ({ in: [bits(turns)], out: [bits(cosTurns(turns))] })),
  clamp: clamps.map(([value, low, high]) => ({ in: [bits(value!), bits(low!), bits(high!)], out: [bits(clamp(value!, low!, high!))] })),
  lerp: lerps.map(([from, to, amount]) => ({ in: [bits(from!), bits(to!), bits(amount!)], out: [bits(lerp(from!, to!, amount!))] })),
  smoothstep: steps.map((amount) => ({ in: [bits(amount)], out: [bits(smoothstep(amount))] })),
  randomWords: keys.map((key, index) => ({ key, count: index % 13, out: randomWords(key, index % 13) })),
  randomUnit: keys.map((key) => ({ key, out: [bits(randomUnit(key))] })),
  randomBetween: ranges.map(({ key, low, high }) => ({ key, in: [bits(low), bits(high)], out: [bits(randomBetween(key, low, high))] })),
  randomPick: picks.map(({ key, weights }) => ({ key, in: weights.map(bits), out: randomPick(key, weights) })),
  compose: pairs.map(({ parent, local }) => ({ in: [...parent, ...local].map(bits), out: compose(parent, local).map(bits) })),
  invert: singles.map((entry) => ({ in: entry.map(bits), out: invert(entry).map(bits) })),
  transform: points.map(({ matrix: entry, x, y }) => {
    const carried = transform(entry, x, y);
    return { in: [...entry, x, y].map(bits), out: [bits(carried.x), bits(carried.y)] };
  }),
  lookOffset: looks.map(({ eye, target, reach }) => {
    const offset = lookOffset({ x: eye[0]!, y: eye[1]! }, { x: target[0]!, y: target[1]! }, reach);
    return { in: [eye[0]!, eye[1]!, target[0]!, target[1]!, reach].map(bits), out: [bits(offset.x), bits(offset.y)] };
  }),
  solveRig: poses.map(({ species: id, pose }) => ({ species: id, in: pose.flatMap((entry) => [entry.x, entry.y, entry.rotation, entry.scaleX, entry.scaleY]).map(bits), out: solveRig(species.find((kind) => kind.id === id)!, pose).map(bits) })),
};

const target = resolve(HERE, "🗑️generated/wp-k/kinematics-bits.json");
mkdirSync(dirname(target), { recursive: true });
writeFileSync(target, `${JSON.stringify(document)}\n`);
const outputs = Object.entries(document).filter(([name]) => name !== "$comment") as [string, { out: unknown }[]][];
for (const [name, vectors] of outputs) process.stdout.write(`${name}: ${vectors.length} inputs, ${vectors.reduce((sum, vector) => sum + (Array.isArray(vector.out) ? vector.out.length : 1), 0)} outputs\n`);
