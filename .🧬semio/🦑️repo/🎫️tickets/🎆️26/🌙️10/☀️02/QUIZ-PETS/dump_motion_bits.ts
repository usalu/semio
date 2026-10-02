/** 🧮️ Ticket tool of work package L, TypeScript half: evaluates the animation and terrain modules of `@semio-tech/pets` on generated inputs (a few hundred and more per function, among them signed zeros, NaN, infinities, subnormals, values on ties and whole trajectories) and writes every input and every result as an IEEE-754 bit pattern to `🗑️generated/wp-l/motion-bits.json`. `check_motion_bits.rs` beside this file recomputes every case with the Rust twins and compares bit for bit.
 *
 * Numbers travel as sixteen hexadecimal digits (`nan` for every NaN, whose payload no language promises), so no decimal
 * parser or printer stands between the two cores. A case whose TypeScript evaluation throws is recorded as `throws`;
 * the Rust twin must panic on it.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/dump_motion_bits.ts
 *   bash .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/rust_scratch.sh wp-l test --offline --test motion_bits -- --nocapture
 *
 * @see ./check_motion_bits.rs — the Rust half
 * @see ./📓️report-wp-l.md — the recorded result
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { BLINK_TICKS, GAZE_DAMPING, GAZE_STIFFNESS, blendPose, clipTicks, easeBezier, lidAt, sampleClip, sampleTrack, springStep } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎞️animation/🟦️.ts";
import type { BonePose } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🦴️rig/🟦️.ts";
import { FALL_SPEED, GRAVITY, HOP_CLEARANCE, HOP_DISTANCE, HOP_HEIGHT, HOP_STEEPNESS, HOP_TICKS, fallStep, hopLanding, hopOf, hopStep, landingOf, nearestPerch, perchAt, perchesOf, strideTo } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🏞️terrain/🟦️.ts";
import { CHANNELS, type Clip, type Ease, type Key, type Perch, type Point, type Rect, type Species, type Surface, type Track } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";

//#region 🔖️Encoding
const view = new DataView(new ArrayBuffer(8));

/** 🔢️ The bit pattern of a number as sixteen hexadecimal digits; `nan` for every NaN. */
function bits(value: number): string {
  if (Number.isNaN(value)) return "nan";
  view.setFloat64(0, value);
  return view.getBigUint64(0).toString(16).padStart(16, "0");
}

type Case = { readonly fn: string; readonly args: readonly string[]; readonly out: readonly string[] | "throws" };
const cases: Case[] = [];

/** 📝️ Records one evaluation: the flat arguments and what the function answers, or that it throws. */
function record(fn: string, args: readonly number[], run: () => readonly number[]): void {
  let out: readonly string[] | "throws";
  try {
    out = run().map(bits);
  } catch {
    out = "throws";
  }
  cases.push({ fn, args: args.map(bits), out });
}
//#endregion 🔖️Encoding

//#region 🔖️Generators
let state = 20261002;

/** 🎰️ The next word of a 32-bit linear congruential generator. */
function word(): number {
  state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
  return state;
}

/** 🎲️ A double in [0, 1) with 53 generated bits. */
function unit(): number {
  return (word() * 2097152 + (word() >>> 11)) / 9007199254740992;
}

/** 🔢️ A whole number in [0, bound). */
function below(bound: number): number {
  return Math.floor(unit() * bound);
}

/** 🪙️ True with the given probability. */
function chance(share: number): boolean {
  return unit() < share;
}

/** 🌊️ A double anywhere in [low, high). */
function free(low: number, high: number): number {
  return low + (high - low) * unit();
}

/** 📏️ A multiple of `1 ÷ grain` in [low, high], exact in binary for a grain that is a power of two. */
function grid(low: number, high: number, grain: number): number {
  return low + below((high - low) * grain + 1) / grain;
}

const SPECIALS = [0, -0, Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, 5e-324, -5e-324, 1e-300, -1e-300, 1e300, -1e300, 2 ** 53, 0.1, 1 / 3];
const SMALL_SPECIALS = [0, -0, Number.NaN, 5e-324, -5e-324, 1e-300, -1, 0.1, 1 / 3];

/** ⚠️ A number few callers expect. */
function special(): number {
  return SPECIALS[below(SPECIALS.length)]!;
}

/** 🎭️ Mostly what `usual` yields, with the given share a special number. */
function odd(share: number, usual: () => number): number {
  return chance(share) ? special() : usual();
}

/** 🎚️ One of the given items. */
function pick<T>(items: readonly T[]): T {
  return items[below(items.length)]!;
}

const EASES: readonly Ease[] = [[0.25, 0.1, 0.25, 1], [0.42, 0, 1, 1], [0, 0, 0.58, 1], [0.42, 0, 0.58, 1], [0.34, 1.56, 0.64, 1], [0.36, 0, 0.66, -0.56], [0.68, -0.6, 0.32, 1.6], [0, 1, 1, 0], [1, 0, 0, 1], [1 / 3, 0, 2 / 3, 1]];

/** 🎢️ An easing: a committed one, or generated control points, rarely with a special number among them. */
function ease(): Ease {
  if (chance(0.3)) return pick(EASES);
  return [odd(0.03, () => free(0, 1)), odd(0.03, () => free(-1, 2)), odd(0.03, () => free(0, 1)), odd(0.03, () => free(-1, 2))];
}

/** 🔑️ Keys and their flat form `at, value, eased, x1, y1, x2, y2` each: tidy ones ascend from 0 to 1, untidy ones are in any order. */
function keys(count: number, tidy: boolean): { keys: Key[]; flat: number[] } {
  const ats = Array.from({ length: count }, () => (tidy ? grid(0, 1, 32) : odd(0.06, () => grid(0, 1, 16))));
  if (tidy) {
    ats.sort((left, right) => left - right);
    if (count > 0) ats[0] = 0;
    if (count > 1) ats[count - 1] = 1;
  }
  const made: Key[] = [];
  const flat: number[] = [];
  for (const at of ats) {
    const value = odd(0.03, () => (chance(0.3) ? grid(-20, 20, 4) : free(-50, 50)));
    const curve = chance(0.4) ? ease() : undefined;
    made.push(curve === undefined ? { at, value } : { at, value, ease: curve });
    flat.push(at, value, curve === undefined ? 0 : 1, ...(curve ?? [0, 0, 0, 0]));
  }
  return { keys: made, flat };
}

/** 🧍️ A pose of `count` bones and its flat form `x, y, rotation, scaleX, scaleY` each. */
function pose(count: number): { pose: BonePose[]; flat: number[] } {
  const made: BonePose[] = [];
  const flat: number[] = [];
  for (let bone = 0; bone < count; bone++) {
    const posed = { x: odd(0.03, () => free(-20, 20)), y: odd(0.03, () => free(-20, 20)), rotation: odd(0.03, () => free(-360, 360)), scaleX: odd(0.03, () => free(0.5, 1.5)), scaleY: odd(0.03, () => free(0.5, 1.5)) };
    made.push(posed);
    flat.push(posed.x, posed.y, posed.rotation, posed.scaleX, posed.scaleY);
  }
  return { pose: made, flat };
}

/** 🪺️ Perches on the surfaces `s0` … `s3` and their flat form `count, (surface, x0, x1, y) …`. */
function perches(count: number, wild: number): { perches: Perch[]; flat: number[] } {
  const made: Perch[] = [];
  const flat: number[] = [count];
  for (let index = 0; index < count; index++) {
    const surface = below(4);
    const x0 = odd(wild, () => grid(-40, 600, 2));
    const x1 = odd(wild, () => x0 + grid(0, 300, 2));
    const y = odd(wild, () => grid(40, 480, 4));
    made.push({ surface: `s${surface}`, x0, x1, y });
    flat.push(surface, x0, x1, y);
  }
  return { perches: made, flat };
}

/** 🔎️ The index of an answered perch in its list, −1 for none. */
function indexOf(list: readonly Perch[], perch: Perch | null): number {
  return perch === null ? -1 : list.indexOf(perch);
}
//#endregion 🔖️Generators

//#region 🔖️Animation
record("constants", [], () => [GRAVITY, FALL_SPEED, HOP_CLEARANCE, HOP_STEEPNESS, HOP_HEIGHT, HOP_DISTANCE, HOP_TICKS, GAZE_STIFFNESS, GAZE_DAMPING, BLINK_TICKS]);

for (let index = 0; index < 900; index++) {
  const curve = ease();
  const amount = index % 3 === 0 ? grid(0, 1, 64) : odd(0.05, () => free(-0.25, 1.25));
  record("ease_bezier", [...curve, amount], () => [easeBezier(curve, amount)]);
}

for (let index = 0; index < 900; index++) {
  const count = below(7);
  const made = keys(count, chance(0.7));
  const channel = below(CHANNELS.length);
  const track: Track = { bone: "body", channel: CHANNELS[channel]!, keys: made.keys };
  const phase = count > 0 && index % 4 === 0 ? pick(made.keys).at : index % 4 === 1 ? grid(0, 1, 64) : odd(0.06, () => free(-0.2, 1.2));
  record("sample_track", [channel, count, ...made.flat, phase], () => [sampleTrack(track, phase)]);
}

for (const [at, phase] of [[0, Number.NaN], [Number.NaN, 0.5], [Number.NaN, Number.NaN], [0.5, Number.NaN]] as const) {
  for (let channel = 0; channel < CHANNELS.length; channel++) record("sample_track", [channel, 1, at, 7, 0, 0, 0, 0, 0, phase], () => [sampleTrack({ bone: "body", channel: CHANNELS[channel]!, keys: [{ at, value: 7 }] }, phase)]);
}

for (let index = 0; index < 500; index++) {
  const seconds = index % 4 === 0 ? grid(0, 4, 128) : index % 4 === 1 ? pick(SMALL_SPECIALS) : index % 4 === 2 ? free(0, 0.05) : free(0, 90);
  record("clip_ticks", [seconds], () => [clipTicks({ id: "clip", seconds, loop: false, tracks: [] })]);
}

for (let index = 0; index < 500; index++) {
  const bones = 1 + below(6);
  const species = { bones: Array.from({ length: bones }, (_, bone) => ({ id: `b${bone}`, x: 0, y: 0 })) } as unknown as Species;
  const seconds = index % 5 === 0 ? pick(SMALL_SPECIALS) : index % 5 === 1 ? grid(0, 4, 128) : free(0.01, 3);
  const loop = chance(0.5);
  const tracks: Track[] = [];
  const flat: number[] = [];
  const count = below(7);
  for (let track = 0; track < count; track++) {
    const bone = below(bones + 1);
    const channel = below(CHANNELS.length);
    const length = 2 + below(4);
    const made = keys(length, chance(0.85));
    tracks.push({ bone: `b${bone}`, channel: CHANNELS[channel]!, keys: made.keys });
    flat.push(bone, channel, length, ...made.flat);
  }
  const clip: Clip = { id: "clip", seconds, loop, tracks };
  const ticks = index % 7 === 0 ? pick([-5, -1, 0, 100000, 2 ** 40]) : below(600);
  record("sample_clip", [bones, seconds, loop ? 1 : 0, count, ...flat, ticks], () => sampleClip(species, clip, ticks).flatMap((bone) => [bone.x, bone.y, bone.rotation, bone.scaleX, bone.scaleY]));
}

for (let index = 0; index < 400; index++) {
  const bones = below(7);
  const from = pose(bones);
  const to = pose(bones);
  const amount = index % 4 === 0 ? pick([0, -0, 1, 0.5, 0.25, -0.5, 1.5]) : odd(0.06, () => free(-0.25, 1.25));
  record("blend_pose", [bones, ...from.flat, ...to.flat, amount], () => blendPose(from.pose, to.pose, amount).flatMap((bone) => [bone.x, bone.y, bone.rotation, bone.scaleX, bone.scaleY]));
}

for (let index = 0; index < 900; index++) {
  const gaze = index % 3 === 0;
  const args = [odd(0.04, () => free(-2, 2)), odd(0.04, () => free(-40, 40)), odd(0.04, () => free(-2, 2)), gaze ? GAZE_STIFFNESS : odd(0.04, () => free(0, 16000)), gaze ? GAZE_DAMPING : odd(0.04, () => free(0, 130))] as const;
  record("spring_step", args, () => {
    const stepped = springStep(...args);
    return [stepped.position, stepped.velocity];
  });
}

for (const run of [{ position: 1, velocity: 0, target: 0, stiffness: GAZE_STIFFNESS, damping: GAZE_DAMPING, ticks: 2400 }, { position: -0.3, velocity: 7.5, target: 0.9, stiffness: GAZE_STIFFNESS, damping: GAZE_DAMPING, ticks: 600 }, { position: 0.1, velocity: 0, target: 0.7, stiffness: 15000, damping: 8, ticks: 600 }, { position: 4, velocity: -3, target: 1 / 3, stiffness: 100, damping: 0, ticks: 600 }, { position: 1, velocity: 1, target: 0, stiffness: 16000, damping: 16, ticks: 300 }]) {
  let spring = { position: run.position, velocity: run.velocity };
  for (let tick = 0; tick < run.ticks; tick++) {
    const before = spring;
    spring = springStep(before.position, before.velocity, run.target, run.stiffness, run.damping);
    const after = spring;
    record("spring_step", [before.position, before.velocity, run.target, run.stiffness, run.damping], () => [after.position, after.velocity]);
  }
}

for (const ticks of [-1000, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 64, 1000, 2 ** 40]) record("lid_at", [ticks], () => [lidAt(ticks)]);
//#endregion 🔖️Animation

//#region 🔖️Terrain
for (let index = 0; index < 700; index++) {
  const wild = index % 5 === 0 ? 0.05 : 0;
  const grain = pick([1, 4, 8]);
  const width = odd(wild, () => 320 + below(8) * 40);
  const height = odd(wild, () => 240 + below(6) * 40);
  const clearance = odd(wild, () => pick([0, 40.5, 48, 56, 61.7]));
  const minimum = odd(wild, () => pick([0, 24, 60.5, 72]));
  const surfaces: Surface[] = [];
  const keepouts: Rect[] = [];
  const flat: number[] = [];
  const surfaceCount = 1 + below(5);
  flat.push(surfaceCount);
  for (let surface = 0; surface < surfaceCount; surface++) {
    const x0 = odd(wild, () => (chance(0.1) ? pick([0, -0]) : grid(-320, 640, grain)));
    const x1 = odd(wild, () => x0 + 8 + grid(0, 640, grain));
    const y = odd(wild, () => grid(-10, 520, grain));
    surfaces.push({ id: `s${surface}`, x0, x1, y });
    flat.push(x0, x1, y);
  }
  const keepoutCount = below(9);
  flat.push(keepoutCount);
  for (let keepout = 0; keepout < keepoutCount; keepout++) {
    const box = { x: odd(wild, () => grid(-40, 680, grain)), y: odd(wild, () => grid(-40, 560, grain)), width: odd(wild, () => (chance(0.15) ? pick([0, -0, -8]) : grid(0, 160, grain))), height: odd(wild, () => (chance(0.15) ? pick([0, -0, -8]) : grid(0, 120, grain))) };
    keepouts.push(box);
    flat.push(box.x, box.y, box.width, box.height);
  }
  record("perches_of", [width, height, clearance, minimum, ...flat], () => {
    const found = perchesOf(surfaces, keepouts, width, height, clearance, minimum);
    return [found.length, ...found.flatMap((perch) => [Number(perch.surface.slice(1)), perch.x0, perch.x1, perch.y])];
  });
}

for (let index = 0; index < 500; index++) {
  const made = perches(below(7), index % 6 === 0 ? 0.08 : 0);
  const surface = below(5);
  const on = made.perches.length > 0 ? pick(made.perches) : null;
  const x = on !== null && index % 3 === 0 ? pick([on.x0, on.x1]) : odd(0.05, () => grid(-60, 700, 2));
  record("perch_at", [...made.flat, surface, x], () => [indexOf(made.perches, perchAt(made.perches, `s${surface}`, x))]);
}

for (let index = 0; index < 500; index++) {
  const made = perches(below(7), index % 6 === 0 ? 0.08 : 0);
  const x = odd(0.05, () => (chance(0.5) ? grid(-60, 700, 2) : free(-60, 700)));
  const y = odd(0.05, () => (chance(0.5) ? grid(0, 520, 4) : free(0, 520)));
  record("nearest_perch", [...made.flat, x, y], () => [indexOf(made.perches, nearestPerch(made.perches, x, y))]);
}

for (let index = 0; index < 400; index++) {
  const centre = free(100, 400);
  const across = free(1, 90);
  const down = free(1, 90);
  const mirrored: Perch[] = [{ surface: "s0", x0: centre + across, x1: centre + across + 40, y: centre + down }, { surface: "s1", x0: centre + down, x1: centre + down + 40, y: centre + across }];
  record("nearest_perch", [2, 0, mirrored[0]!.x0, mirrored[0]!.x1, mirrored[0]!.y, 1, mirrored[1]!.x0, mirrored[1]!.x1, mirrored[1]!.y, centre, centre], () => [indexOf(mirrored, nearestPerch(mirrored, centre, centre))]);
}

for (let index = 0; index < 900; index++) {
  const x = odd(0.04, () => (chance(0.5) ? grid(-200, 800, 64) : free(-200, 800)));
  const speed = odd(0.04, () => (chance(0.5) ? grid(-16, 160, 1) : free(0, 160)));
  const goal = index % 3 === 0 ? x + pick([-1, 1]) * (speed / 64) * pick([0, 0.5, 1, 1.5, 2]) : odd(0.04, () => (chance(0.5) ? grid(-200, 800, 2) : free(-200, 800)));
  record("stride_to", [x, goal, speed], () => [strideTo(x, goal, speed)]);
}

for (let walk = 0; walk < 12; walk++) {
  const goal = grid(-100, 700, 2);
  const speed = pick([36, 40, 48, 55.5, 64, 90, 1 / 3 + 30]);
  let x = free(-100, 700);
  for (let tick = 0; tick < 400 && x !== goal; tick++) {
    const before = x;
    x = strideTo(before, goal, speed);
    const after = x;
    record("stride_to", [before, goal, speed], () => [after]);
  }
}

for (let index = 0; index < 700; index++) {
  const y = odd(0.04, () => (chance(0.5) ? grid(-50, 600, 64) : free(-50, 600)));
  const vy = index % 4 === 0 ? pick([FALL_SPEED, FALL_SPEED - 28.125, FALL_SPEED - 28, 871.875, 871.8750000000001, 5000, -0, 0]) : odd(0.04, () => (chance(0.5) ? grid(-600, 900, 8) : free(-600, 950)));
  record("fall_step", [y, vy], () => {
    const fallen = fallStep(y, vy);
    return [fallen.y, fallen.vy];
  });
}

for (let drop = 0; drop < 10; drop++) {
  let fall = { y: free(-20, 200), vy: free(-500, 100) };
  for (let tick = 0; tick < 90; tick++) {
    const before = fall;
    fall = fallStep(before.y, before.vy);
    const after = fall;
    record("fall_step", [before.y, before.vy], () => [after.y, after.vy]);
  }
}

for (let index = 0; index < 500; index++) {
  const made = perches(below(8), index % 6 === 0 ? 0.08 : 0);
  const on = made.perches.length > 0 ? pick(made.perches) : null;
  const x = on !== null && index % 3 === 0 ? pick([on.x0, on.x1, (on.x0 + on.x1) / 2]) : odd(0.05, () => grid(-60, 700, 2));
  const fromY = on !== null && index % 4 === 0 ? on.y - pick([0, 0.25, 14]) : odd(0.05, () => grid(0, 520, 4));
  const toY = on !== null && index % 4 === 1 ? on.y + pick([0, 0.25, 14]) : odd(0.05, () => fromY + grid(-20, 200, 4));
  record("landing_of", [...made.flat, x, fromY, toY], () => [indexOf(made.perches, landingOf(made.perches, x, fromY, toY))]);
}

/** 🚀️ The ends of a hop: lattice points, arbitrary doubles, the limits of reach, and now and then a special number. */
function ends(index: number): { from: Point; to: Point } {
  const from = { x: odd(0.02, () => (chance(0.5) ? grid(0, 640, 4) : free(0, 640))), y: odd(0.02, () => (chance(0.5) ? grid(60, 480, 4) : free(60, 480))) };
  if (index % 5 === 0) return { from, to: { x: from.x + pick([-1, 1]) * pick([0, HOP_DISTANCE, HOP_DISTANCE + 0.5, 96, 32]), y: from.y + pick([0, -(HOP_HEIGHT - HOP_CLEARANCE), -(HOP_HEIGHT - HOP_CLEARANCE) - 0.5, 155, 165, 210, 220, 24]) } };
  return { from, to: { x: odd(0.02, () => from.x + (chance(0.5) ? grid(-170, 170, 4) : free(-170, 170))), y: odd(0.02, () => from.y + (chance(0.5) ? grid(-90, 230, 4) : free(-90, 230))) } };
}

const launches: { from: Point; to: Point }[] = [];
for (let index = 0; index < 1200; index++) {
  const { from, to } = ends(index);
  record("hop_of", [from.x, from.y, to.x, to.y], () => {
    const hop = hopOf(from, to);
    if (hop !== null && launches.length < 120 && index % 3 === 1) launches.push({ from, to });
    return hop === null ? [0, 0, 0, 0] : [1, hop.vx, hop.vy, hop.ticks];
  });
}

for (let index = 0; index < 500; index++) {
  const args = [odd(0.03, () => free(0, 640)), odd(0.03, () => free(0, 480)), odd(0.03, () => free(-400, 400)), odd(0.03, () => free(-600, 950)), odd(0.03, () => grid(0, 640, 4)), odd(0.03, () => grid(0, 480, 4)), pick([-2, 0, 1, 1, 2, 2, 3, 7, 15, 48])] as const;
  record("hop_step", args, () => {
    const flown = hopStep(args[0], args[1], args[2], args[3], { x: args[4], y: args[5] }, args[6]);
    return [flown.x, flown.y, flown.vx, flown.vy];
  });
}

for (const { from, to } of launches) {
  const hop = hopOf(from, to)!;
  let flight = { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
  for (let left = hop.ticks; left >= 1; left--) {
    const before = flight;
    flight = hopStep(before.x, before.y, before.vx, before.vy, to, left);
    const after = flight;
    record("hop_step", [before.x, before.y, before.vx, before.vy, to.x, to.y, left], () => [after.x, after.y, after.vx, after.vy]);
  }
  const made = perches(below(6), 0);
  const added: Perch[] = [{ surface: "s0", x0: from.x - 40, x1: from.x + 20, y: from.y }, { surface: "s1", x0: to.x - 30, x1: to.x + 30, y: to.y }, { surface: "s2", x0: (from.x + to.x) / 2 - 12, x1: (from.x + to.x) / 2 + 12, y: Math.max(from.y, to.y) - grid(0, 60, 4) }].filter(() => chance(0.7));
  const list = [...added, ...made.perches];
  const flat = [list.length, ...list.flatMap((perch) => [Number(perch.surface.slice(1)), perch.x0, perch.x1, perch.y])];
  record("hop_landing", [...flat, from.x, from.y, to.x, to.y], () => [indexOf(list, hopLanding(list, from, to, hop))]);
}
//#endregion 🔖️Terrain

//#region 🔖️Output
const directory = join(import.meta.dir, "🗑️generated", "wp-l");
mkdirSync(directory, { recursive: true });
const counts: Record<string, number> = {};
for (const entry of cases) counts[entry.fn] = (counts[entry.fn] ?? 0) + 1;
const numbers = cases.reduce((sum, entry) => sum + entry.args.length + (entry.out === "throws" ? 0 : entry.out.length), 0);
writeFileSync(join(directory, "motion-bits.json"), `[\n${cases.map((entry) => JSON.stringify(entry)).join(",\n")}\n]\n`);
console.log(`[DEBUG] motion bits: ${cases.length} cases, ${numbers} numbers, ${cases.filter((entry) => entry.out === "throws").length} throwing; per function ${JSON.stringify(counts)}`);
//#endregion 🔖️Output
