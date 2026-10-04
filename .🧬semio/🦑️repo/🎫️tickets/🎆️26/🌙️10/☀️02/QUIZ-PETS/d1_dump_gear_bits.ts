/** 🧮️ Ticket tool of work package D1 (round 2), TypeScript half: evaluates every exported function of the swing and climbing modules, the terrain additions (walls, pitches, lines of sight) and the trigonometry additions (`atanTurns`, `fastNegExp`) of `@semio-tech/pets` on generated inputs — lattice values, arbitrary doubles, signed zeros, NaN, infinities, subnormals, generated stages that grant and refuse, and whole trajectories whose inputs are the previous outputs — and writes every input and every result as an IEEE-754 bit pattern to `🗑️generated/d1/gear-bits.json`. `d1_check_gear_bits.rs` beside this file recomputes every case with the Rust twins and compares bit for bit.
 *
 * Numbers travel as sixteen hexadecimal digits (`nan` for every NaN, whose payload no language promises), so no decimal
 * parser or printer stands between the two cores. Structured arguments are flattened in a fixed order: a point is
 * `x, y`; a box `x, y, width, height`; a size `width, height`; a perch `surface, x0, x1, y` (surface `s<n>`); a wall or
 * a pitch `wall, surface, side, x, y0, y1` (wall `w<n>`); a ladder `wall, surface, side, foot, top`; a shot `surface,
 * facing, muzzle, hook, length, reel` (reel 0 zip, 1 swing); a grip `x, y, vx, vy`; a hang `grip, bob, previous`; a
 * canopy `x, y, vx, vy, bob, previous`; a chute `terminal, reach, flare, length`; a haul `rope, hand, before, x, y`; an
 * effort 0 climb, 1 hang, 2 rest; gear by its index in `GEARS`; a list is its count followed by its entries. An absent
 * answer is `0`, a present one `1` followed by its fields; an answered entry of a list is its index, −1 for none.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d1_dump_gear_bits.ts
 *   bash .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/rust_scratch.sh d1 test --offline --test gear_bits -- --nocapture
 *
 * @see ./d1_check_gear_bits.rs — the Rust half
 * @see ./📓️report2-d1.md — the recorded result
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import * as climbing from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🧗️climbing/🟦️.ts";
import * as swing from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🪢️swing/🟦️.ts";
import { type Pitch, WALL_LIP, nearestWall, segmentClear, segmentHits, wallAt, wallsOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🏞️terrain/🟦️.ts";
import { atanTurns, fastNegExp } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📐️trigonometry/🟦️.ts";
import { GEARS, type Canopy, type Gear, type Grip, type Hang, type Perch, type Point, type Rect, type Size } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";

type Ladder = NonNullable<ReturnType<typeof climbing.ladderFor>>;
type Shot = NonNullable<ReturnType<typeof climbing.shotFor>>;
type Wall = Parameters<typeof wallsOf>[0][number];
type Chute = swing.Chute;
type Haul = climbing.Haul;
type Effort = climbing.Effort;
type Leg = climbing.Leg;

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
const legTally = new Map<string, number>();

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

const EFFORTS: readonly Effort[] = ["climb", "hang", "rest"];
const point = (x: number, y: number): Point => ({ x, y });
const named = (prefix: string, index: number): string => `${prefix}${index}`;
const numbered = (name: string): number => Number(name.slice(1));
const pointFlat = (at: Point): number[] => [at.x, at.y];
const rectFlat = (box: Rect): number[] => [box.x, box.y, box.width, box.height];
const sizeFlat = (size: Size): number[] => [size.width, size.height];
const perchFlat = (perch: Perch): number[] => [numbered(perch.surface), perch.x0, perch.x1, perch.y];
const pitchFlat = (pitch: Pitch): number[] => [numbered(pitch.wall), numbered(pitch.surface), pitch.side, pitch.x, pitch.y0, pitch.y1];
const wallFlat = (wall: Wall): number[] => [numbered(wall.id), numbered(wall.surface), wall.side, wall.x, wall.y0, wall.y1];
const ladderFlat = (ladder: Ladder): number[] => [numbered(ladder.wall), numbered(ladder.surface), ladder.side, ...pointFlat(ladder.foot), ...pointFlat(ladder.top)];
const shotFlat = (shot: Shot): number[] => [numbered(shot.surface), shot.facing, ...pointFlat(shot.muzzle), ...pointFlat(shot.hook), shot.length, shot.reel === "zip" ? 0 : 1];
const gripFlat = (grip: Grip): number[] => [grip.x, grip.y, grip.vx, grip.vy];
const hangFlat = (hang: Hang): number[] => [...gripFlat(hang.grip), ...pointFlat(hang.bob), ...pointFlat(hang.previous)];
const canopyFlat = (canopy: Canopy): number[] => [canopy.x, canopy.y, canopy.vx, canopy.vy, ...pointFlat(canopy.bob), ...pointFlat(canopy.previous)];
const chuteFlat = (chute: Chute): number[] => [chute.terminal, chute.reach, chute.flare, chute.length];
const haulFlat = (haul: Haul): number[] => [haul.rope, ...pointFlat(haul.hand), ...pointFlat(haul.before), haul.x, haul.y];
const listFlat = <T>(items: readonly T[], flat: (item: T) => number[]): number[] => [items.length, ...items.flatMap(flat)];
const optional = <T>(found: T | null, flat: (item: T) => number[]): number[] => (found === null ? [0] : [1, ...flat(found)]);
const indexOf = <T>(items: readonly T[], found: T | null): number => (found === null ? -1 : items.indexOf(found));
const holdFlat = (hold: climbing.WallHold): number[] => [hold.x, hold.y, hold.over ? 1 : 0];
const tossFlat = (toss: climbing.Toss): number[] => [toss.vx, toss.vy];

/** 🗺️ A list of legs as flat numbers: per leg its means (0 ladder, 1 wall, 2 raise, 3 grapple), `at`, then the standing ladder's index and `up`, the pitch's index, the hold and the goal, the raised ladder, or the shot. */
function legsFlat(legs: readonly Leg[] | null, pitches: readonly Pitch[], ladders: readonly Ladder[]): number[] {
  if (legs === null) return [0];
  for (const leg of legs) legTally.set(leg.means === "wall" && leg.exit !== leg.pitch ? "wall-across-a-line" : leg.means, (legTally.get(leg.means === "wall" && leg.exit !== leg.pitch ? "wall-across-a-line" : leg.means) ?? 0) + 1);
  return [
    1,
    legs.length,
    ...legs.flatMap((leg) =>
      leg.means === "ladder"
        ? [0, leg.at, ladders.indexOf(leg.ladder), leg.up ? 1 : 0]
        : leg.means === "wall"
          ? [1, leg.at, pitches.indexOf(leg.pitch), ...holdFlat(leg.hold), pitches.indexOf(leg.exit), leg.goal]
          : leg.means === "raise"
            ? [2, leg.at, ...ladderFlat(leg.ladder)]
            : [3, leg.at, ...shotFlat(leg.shot)],
    ),
  ];
}
//#endregion 🔖️Encoding

//#region 🔖️Generators
let state = 20261003;

/** 🎰️ The next word of a 32-bit linear congruential generator. */
function word(): number {
  state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
  return state;
}

/** 🎲️ A double in [0, 1) with 53 generated bits. */
function unit(): number {
  return (word() * 2097152 + (word() >>> 11)) / 9007199254740992;
}

/** 🔟️ A whole number in [0, bound). */
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

/** 📍️ A coordinate of the stage: on a lattice of quarters, or anywhere, rarely special. */
function coordinate(low: number, high: number, share = 0.02): number {
  return odd(share, () => (chance(0.5) ? grid(low, high, 4) : free(low, high)));
}

/** 🐾️ A size of a pet: the three of the research brief, a lattice one or an arbitrary one. */
function size(): Size {
  return chance(0.6) ? pick([{ width: 40, height: 48 }, { width: 34, height: 40 }, { width: 46, height: 56 }]) : { width: odd(0.02, () => grid(20, 80, 2)), height: odd(0.02, () => grid(24, 96, 2)) };
}

/** ⬛️ A keep-out box; some without area. */
function rect(): Rect {
  return { x: coordinate(-40, 680), y: coordinate(-40, 520), width: chance(0.1) ? 0 : chance(0.05) ? -grid(0, 40, 4) : coordinate(0, 200), height: chance(0.1) ? 0 : chance(0.05) ? -grid(0, 40, 4) : coordinate(0, 160) };
}

/** 🪺️ A perch on one of four surfaces. */
function perch(): Perch {
  const x0 = coordinate(-20, 600);
  return { surface: named("s", below(4)), x0, x1: chance(0.05) ? x0 : x0 + coordinate(0, 300), y: coordinate(0, 520) };
}

/** 🧱️ A pitch of one of three walls on one of four surfaces; with the given share a coordinate is special. */
function pitch(share = 0.02): Pitch {
  const y0 = coordinate(-40, 500, share);
  return { wall: named("w", below(3)), surface: named("s", below(4)), side: chance(0.5) ? -1 : 1, x: coordinate(-20, 660, share), y0, y1: chance(0.05) ? y0 : y0 + coordinate(0, 300, share) };
}

/** 🏯️ A surveyed wall: pitches before they are cut. */
function wall(): Wall {
  const cut = pitch();
  return { id: cut.wall, surface: cut.surface, side: cut.side, x: cut.x, y0: cut.y0, y1: cut.y1 };
}

/** 🧺️ `count` entries of a generator. */
function some<T>(count: number, make: () => T): T[] {
  return Array.from({ length: count }, make);
}

/** 📦️ The box the survey reports for a card: its own, grown by 4 px on both sides. */
function solid(x0: number, x1: number, y0: number, y1: number): Rect {
  return { x: x0 - 4, y: y0, width: x1 - x0 + 8, height: y1 - y0 };
}

/** 🪜️ A stage on which a ladder may stand: a high perch, the pitch it crowns, a low perch, its box and a few blockers — the generator of the unit suite, on the stream of this tool. */
function ladderStage(): { low: Perch; high: Perch; face: Pitch; keepouts: Rect[]; size: Size } {
  const pet = pick([{ width: 40, height: 48 }, { width: 34, height: 40 }, { width: 46, height: 56 }]);
  const side: 1 | -1 = chance(0.5) ? -1 : 1;
  const x = 200 + below(800) / 4;
  const rim = 150 + below(600) / 4;
  const high = { surface: "s1", x0: side < 0 ? x + 6 : x - 180, x1: side < 0 ? x + 180 : x - 6, y: rim };
  const face: Pitch = { wall: "w1", surface: "s1", side, x, y0: rim, y1: rim + 20 + below(800) / 4 };
  const low = { surface: "s0", x0: x - 150 + below(600) / 4, x1: x + below(600) / 4, y: rim + 20 + below(900) / 4 };
  const keepouts = [solid(Math.min(x, x - side * 186), Math.max(x, x - side * 186), rim, rim + 300), ...some(below(3), () => ({ x: x - 120 + below(960) / 4, y: rim - 40 + below(1200) / 4, width: below(160) / 4, height: below(160) / 4 }))];
  return { low, high, face, keepouts, size: pet };
}

/** 🎯️ A stage on which a rope may be shot: feet, one to three perches above, their boxes and a few blockers — the generator of the unit suite, on the stream of this tool. */
function shotStage(): { feet: Point; perches: Perch[]; keepouts: Rect[]; size: Size } {
  const pet = pick([{ width: 40, height: 48 }, { width: 34, height: 40 }, { width: 46, height: 56 }]);
  const feet = point(100 + below(1600) / 4, 300 + below(800) / 4);
  const perches = some(1 + below(3), () => {
    const x0 = below(2000) / 4;
    return { surface: named("s", below(4)), x0, x1: x0 + 20 + below(800) / 4, y: 100 + below(1400) / 4 };
  });
  const keepouts = [...perches.map((edge) => solid(edge.x0, edge.x1, edge.y, edge.y + 40)), ...some(below(3), () => ({ x: below(2400) / 4, y: 100 + below(1600) / 4, width: below(240) / 4, height: below(240) / 4 }))];
  return { feet, perches, keepouts, size: pet };
}

/** 🏗️ Ladders that stand: granted placements of generated stages, with the stage they stand on. */
function ladders(count: number): { ladder: Ladder; stage: ReturnType<typeof ladderStage> }[] {
  const found: { ladder: Ladder; stage: ReturnType<typeof ladderStage> }[] = [];
  for (let attempt = 0; found.length < count && attempt < count * 40; attempt++) {
    const stage = ladderStage();
    const ladder = climbing.ladderFor(stage.low, stage.high, stage.face, stage.keepouts, stage.size);
    if (ladder !== null) found.push({ ladder, stage });
  }
  return found;
}

/** 🏹️ Shots that bite: granted shots of generated stages, with their stage. */
function shots(count: number): { shot: Shot; stage: ReturnType<typeof shotStage> }[] {
  const found: { shot: Shot; stage: ReturnType<typeof shotStage> }[] = [];
  for (let attempt = 0; found.length < count && attempt < count * 40; attempt++) {
    const stage = shotStage();
    const shot = climbing.shotFor(stage.feet, stage.perches, stage.keepouts, stage.size);
    if (shot !== null) found.push({ shot, stage });
  }
  return found;
}
//#endregion 🔖️Generators

//#region 🔖️Terrain
record("constants", [], () => [
  WALL_LIP,
  swing.HANG_ROD, swing.HANG_GRAVITY, swing.HANG_DAMPING, swing.HANG_CONE, swing.FOLLOW_STIFFNESS, swing.FOLLOW_DAMPING, ...swing.RELEASE_WEIGHTS, swing.RELEASE_DIVISOR, swing.RELEASE_STALE, swing.THROW_SHARE, swing.THROW_LEAST, swing.THROW_MOST, swing.THROW_RISE, swing.HARD_LANDING, swing.CHUTE_OPENING, swing.CHUTE_HEADROOM, swing.CHUTE_REFLEX, swing.CHUTE_FACTOR, swing.CHUTE_DESCENT, swing.CHUTE_STEER_GAIN, swing.CHUTE_STEER_SPEED, swing.CHUTE_STEER_EASE, swing.CHUTE_ROD, swing.CHUTE_GRAVITY, swing.CHUTE_DAMPING, swing.CHUTE_FLARE, swing.CHUTE_WIND, swing.CHUTE_WIND_RATE, swing.REEL_SPEED, swing.REEL_RAMP, swing.REEL_LEAST, swing.REEL_CAP, swing.REEL_DAMPING,
  climbing.HAND_HEIGHT, climbing.CLIMB_RISE, climbing.CLIMB_DESCENT, climbing.CLIMB_RAMP, climbing.GRIP_SPACING, climbing.GRIP_BUDGET, climbing.GRIP_CLIMB, climbing.GRIP_HANG, climbing.GRIP_REST, climbing.GRIP_BITE, climbing.WALL_FOLLOW, climbing.SLIDE_START, climbing.SLIDE_GAIN, climbing.SLIDE_SPEED, climbing.SLIP_PUSH, climbing.SLIP_LIFT, climbing.WALL_GRAB_TICKS, climbing.WALL_HANG_TICKS, climbing.MANTLE_TICKS, climbing.MANTLE_INSET, climbing.HOIST_HUMP, climbing.HOIST_RISE,
  climbing.LADDER_LEAN, climbing.LADDER_STEEP, climbing.LADDER_FLAT, climbing.LADDER_SHORT, climbing.LADDER_TALL, climbing.LADDER_TUCK, climbing.LADDER_HORNS, climbing.RUNG_SPACING, climbing.LADDER_FOOTING, climbing.LADDER_GIRTH, climbing.LADDER_RISE, climbing.LADDER_DESCENT, climbing.LADDER_RAMP, climbing.LADDER_EXIT, climbing.LADDER_FOLLOW, climbing.LADDER_SHIFT, climbing.LADDER_MOUNT_TICKS, climbing.LADDER_DISMOUNT_TICKS, climbing.LADDER_RAISE_TICKS, climbing.LADDER_IDLE, climbing.LADDER_LIFE, climbing.TOPPLE_STIFFNESS, climbing.TOPPLE_DAMPING, climbing.TOPPLE_PUSH, climbing.TOPPLE_STEP,
  climbing.HOOK_SPEED, climbing.HOOK_RETURN, climbing.ROPE_SHORT, climbing.ROPE_LONG, climbing.ROPE_ELEVATION, climbing.ROPE_MARGIN, climbing.ROPE_DETOUR, climbing.ROPE_FOLLOW, climbing.ROPE_RISE, climbing.MUZZLE_FORWARD, climbing.MUZZLE_HEIGHT, climbing.HOOK_INSET, climbing.HOOK_LIFT, climbing.ZIP_SLANT, climbing.ZIP_SPEED, climbing.ZIP_RAMP, climbing.ROPE_AIM_TICKS, climbing.ROPE_RECOIL_TICKS, climbing.ROPE_TUG_TICKS, climbing.ROPE_HOIST_TICKS, climbing.ROPE_SHRUG_TICKS, climbing.ROPE_MISS_CHANCE, climbing.ROPE_MISS_OVERSHOOT, climbing.ROPE_REST, climbing.ROPE_SULK,
  climbing.CROSS_REACH, climbing.LUNGE_TICKS,
]);

for (let index = 0; index < 900; index++) {
  const width = chance(0.9) ? 320 + below(8) * 40 : coordinate(0, 800, 0.2);
  const height = chance(0.9) ? 240 + below(6) * 40 : coordinate(0, 600, 0.2);
  const clearance = chance(0.85) ? pick([0, 6, 5.5, 6.5, 28.5, 40, 48, 61.7]) : coordinate(-10, 80, 0.2);
  const minimum = chance(0.9) ? pick([0, 24, 60.5, 72]) : coordinate(-10, 100, 0.2);
  const walls = some(1 + below(5), wall);
  const keepouts = some(below(9), rect);
  record("walls_of", [width, height, clearance, minimum, ...listFlat(walls, wallFlat), ...listFlat(keepouts, rectFlat)], () => listFlat(wallsOf(walls, keepouts, width, height, clearance, minimum), pitchFlat));
}
for (let index = 0; index < 700; index++) {
  const pitches = some(below(6), pitch);
  const asked = chance(0.5) && pitches.length > 0 ? pick(pitches) : pitch();
  const y = chance(0.3) ? pick([asked.y0, asked.y1]) : coordinate(-40, 800, 0.03);
  const wallIndex = numbered(asked.wall);
  record("wall_at", [...listFlat(pitches, pitchFlat), wallIndex, y], () => [indexOf(pitches, wallAt(pitches, named("w", wallIndex), y))]);
}
for (let index = 0; index < 900; index++) {
  const pitches = some(below(6), pitch);
  if (pitches.length > 1 && chance(0.3)) pitches.push({ ...pitches[0]!, x: 2 * 320 - pitches[0]!.x });
  const x = chance(0.2) ? 320 : coordinate(-40, 700, 0.03);
  const y = coordinate(-40, 600, 0.03);
  record("nearest_wall", [...listFlat(pitches, pitchFlat), x, y], () => [indexOf(pitches, nearestWall(pitches, x, y))]);
}
for (let index = 0; index < 600; index++) {
  const a = free(-300, 300);
  const b = free(-300, 300);
  const pitches = [...some(below(3), () => pitch(0)), { wall: "w0", surface: "s0", side: 1 as const, x: a, y0: -Math.abs(b) - 50, y1: -Math.abs(b) }, { wall: "w1", surface: "s1", side: -1 as const, x: b, y0: -Math.abs(a) - 50, y1: -Math.abs(a) }, ...some(below(2), () => pitch(0))];
  record("nearest_wall", [...listFlat(pitches, pitchFlat), 0, 0], () => [indexOf(pitches, nearestWall(pitches, 0, 0))]);
}
for (let index = 0; index < 1500; index++) {
  const box = { x: free(-50, 400), y: free(-50, 400), width: free(0, 200), height: free(0, 200) };
  const margin = chance(0.4) ? 0 : free(-10, 10);
  const left = box.x - margin;
  const right = box.x + box.width + margin;
  const top = box.y - margin;
  const bottom = box.y + box.height + margin;
  const across = free(top - 30, bottom + 30);
  const along = free(left - 30, right + 30);
  const [from, to] = pick([
    [point(right, top - 20), point(right, bottom + 20)],
    [point(left, top - 20), point(left, bottom + 20)],
    [point(left - 20, top), point(right + 20, top)],
    [point(left - 20, bottom), point(right + 20, bottom)],
    [point(right, across), point(right + 40, across)],
    [point(along, bottom), point(along, bottom + 40)],
    [point(left - 20, top - 20 * ((bottom - top) / (right - left))), point(right, bottom)],
    [point(right, across), point(right, across)],
  ]);
  record("segment_hits", [...pointFlat(from), ...pointFlat(to), ...rectFlat(box), margin], () => [segmentHits(from, to, box, margin) ? 1 : 0]);
}
for (let index = 0; index < 5000; index++) {
  const from = point(coordinate(-20, 300, 0.01), coordinate(-20, 300, 0.01));
  const to = chance(0.08) ? point(from.x, coordinate(-20, 300, 0.01)) : chance(0.08) ? point(coordinate(-20, 300, 0.01), from.y) : chance(0.03) ? from : point(coordinate(-20, 300, 0.01), coordinate(-20, 300, 0.01));
  const box = chance(0.85) ? { x: grid(0, 200, 4), y: grid(0, 200, 4), width: chance(0.1) ? 0 : grid(0, 120, 4), height: chance(0.1) ? 0 : grid(0, 120, 4) } : rect();
  const margin = chance(0.9) ? pick([0, 0, 2, 6, -3, -0]) : coordinate(-20, 20, 0.1);
  record("segment_hits", [...pointFlat(from), ...pointFlat(to), ...rectFlat(box), margin], () => [segmentHits(from, to, box, margin) ? 1 : 0]);
}
for (let index = 0; index < 900; index++) {
  const from = point(coordinate(-20, 400), coordinate(-20, 400));
  const to = point(coordinate(-20, 400), coordinate(-20, 400));
  const boxes = some(below(5), rect);
  const margin = pick([0, 2, 6, -3, 50.5]);
  record("segment_clear", [...pointFlat(from), ...pointFlat(to), ...listFlat(boxes, rectFlat), margin], () => [segmentClear(from, to, boxes, margin) ? 1 : 0]);
}
//#endregion 🔖️Terrain

//#region 🔖️Trigonometry
for (let index = 0; index < 2500; index++) {
  const y = odd(0.05, () => (chance(0.3) ? grid(-8, 8, 8) : chance(0.5) ? free(-400, 400) : free(-1e-6, 1e-6)));
  const x = chance(0.1) ? (chance(0.5) ? y : -y) : odd(0.05, () => (chance(0.3) ? grid(-8, 8, 8) : chance(0.5) ? free(-400, 400) : free(-1e-6, 1e-6)));
  record("atan_turns", [y, x], () => [atanTurns(y, x)]);
}
for (let index = 0; index < 1200; index++) {
  const x = odd(0.05, () => (chance(0.3) ? grid(-2, 20, 16) : free(-1, 40)));
  record("fast_neg_exp", [x], () => [fastNegExp(x)]);
}
//#endregion 🔖️Trigonometry

//#region 🔖️Swing
for (let index = 0; index < 4000; index++) {
  const before = point(coordinate(0, 800, 0.01), coordinate(0, 600, 0.01));
  const now = chance(0.4) ? before : point(before.x + coordinate(-20, 20, 0.01), before.y + coordinate(-20, 20, 0.01));
  const length = odd(0.02, () => grid(0, 160, 4));
  const angle = free(-0.5, 0.5);
  const bob = chance(0.05) ? before : point(before.x + length * Math.sin(2 * Math.PI * angle) + free(-2, 2), before.y + length * Math.cos(2 * Math.PI * angle) + free(-2, 2));
  const previous = point(bob.x + coordinate(-8, 8, 0.01), bob.y + coordinate(-8, 8, 0.01));
  const gravity = chance(0.8) ? pick([0, 900, 1800, 7200]) : coordinate(-100, 10000, 0.03);
  const damping = chance(0.8) ? pick([1, 0.95, 0.97, 0.9965]) : odd(0.03, () => free(0, 1.2));
  const rope = chance(0.4);
  record("swing_step", [...pointFlat(before), ...pointFlat(now), ...pointFlat(bob), ...pointFlat(previous), length, gravity, damping, rope ? 1 : 0], () => pointFlat(swing.swingStep(before, now, bob, previous, length, gravity, damping, rope)));
}
for (let swingIndex = 0; swingIndex < 30; swingIndex++) {
  const anchor = point(grid(100, 500, 4), grid(50, 200, 4));
  const length = grid(20, 160, 4);
  const angle = free(-0.3, 0.3);
  let bob = point(anchor.x + length * Math.sin(2 * Math.PI * angle), anchor.y + length * Math.cos(2 * Math.PI * angle));
  let previous = bob;
  const gravity = pick([900, 1800, 7200]);
  const damping = pick([1, 0.95, 0.97, 0.9965]);
  const rope = chance(0.3);
  for (let tick = 0; tick < 100; tick++) {
    const [b, p] = [bob, previous];
    const next = swing.swingStep(anchor, anchor, b, p, length, gravity, damping, rope);
    record("swing_step", [...pointFlat(anchor), ...pointFlat(anchor), ...pointFlat(b), ...pointFlat(p), length, gravity, damping, rope ? 1 : 0], () => pointFlat(next));
    previous = bob;
    bob = next;
  }
}
for (let index = 0; index < 1600; index++) {
  const anchor = point(coordinate(0, 600, 0.01), coordinate(0, 400, 0.01));
  const length = odd(0.02, () => grid(0, 80, 4));
  const bob = chance(0.05) ? anchor : point(anchor.x + coordinate(-100, 100, 0.01), anchor.y + coordinate(-100, 100, 0.01));
  record("cone_clamp", [...pointFlat(anchor), ...pointFlat(bob), length], () => pointFlat(swing.coneClamp(anchor, bob, length)));
}
for (let index = 0; index < 1000; index++) {
  const grip = { x: coordinate(0, 600, 0.01), y: coordinate(0, 400, 0.01), vx: coordinate(-800, 800, 0.01), vy: coordinate(-800, 800, 0.01) };
  const target = point(coordinate(0, 600, 0.01), coordinate(0, 400, 0.01));
  record("follow_step", [...gripFlat(grip), ...pointFlat(target)], () => gripFlat(swing.followStep(grip, target)));
}
for (let index = 0; index < 400; index++) {
  const feet = point(coordinate(0, 600, 0.02), coordinate(0, 400, 0.02));
  const length = odd(0.02, () => grid(10, 60, 4));
  record("hang_of", [...pointFlat(feet), length], () => hangFlat(swing.hangOf(feet, length)));
}
for (let drag = 0; drag < 24; drag++) {
  const length = pick([38.4, 32, 44.8, 30.6]);
  let hang = swing.hangOf(point(grid(100, 500, 4), grid(200, 400, 4)), length);
  const from = point(hang.grip.x, hang.grip.y);
  const move = point(free(-300, 300), free(-200, 100));
  const ticks = 20 + below(60);
  for (let tick = 1; tick <= 120; tick++) {
    const share = Math.min(tick / ticks, 1);
    const target = chance(0.02) ? point(free(0, 600), free(0, 400)) : point(from.x + move.x * share, from.y + move.y * share);
    const before = hang;
    hang = swing.hangStep(before, target, length);
    const after = hang;
    record("hang_step", [...hangFlat(before), ...pointFlat(target), length], () => hangFlat(after));
    record("lean_of", [after.grip.x, after.grip.y, ...pointFlat(after.bob), length], () => [swing.leanOf(point(after.grip.x, after.grip.y), after.bob, length)]);
  }
}
for (let index = 0; index < 600; index++) {
  const hang = { grip: { x: coordinate(0, 600), y: coordinate(0, 400), vx: coordinate(-600, 600), vy: coordinate(-600, 600) }, bob: point(coordinate(0, 600), coordinate(0, 400)), previous: point(coordinate(0, 600), coordinate(0, 400)) };
  const target = point(coordinate(0, 600, 0.02), coordinate(0, 400, 0.02));
  const length = odd(0.02, () => grid(10, 60, 4));
  record("hang_step", [...hangFlat(hang), ...pointFlat(target), length], () => hangFlat(swing.hangStep(hang, target, length)));
}
for (let index = 0; index < 1500; index++) {
  const anchor = point(coordinate(0, 600, 0.02), coordinate(0, 400, 0.02));
  const bob = chance(0.05) ? anchor : point(anchor.x + coordinate(-60, 60, 0.02), anchor.y + coordinate(-60, 60, 0.02));
  const length = odd(0.03, () => grid(1, 60, 4));
  record("lean_of", [...pointFlat(anchor), ...pointFlat(bob), length], () => [swing.leanOf(anchor, bob, length)]);
}
/** 💍️ A ring of pointer samples: a resting, a steady, an accelerating or a wild pointer, sometimes stopped at the end. */
function ring(): Point[] {
  const count = below(10);
  const start = point(coordinate(0, 600), coordinate(0, 400));
  const velocity = point(coordinate(-30, 30), coordinate(-30, 30));
  const thrust = point(coordinate(-2, 2), coordinate(-2, 2));
  const kind = below(4);
  const samples = Array.from({ length: count }, (_, tick) => (kind === 0 ? start : kind === 3 ? point(coordinate(0, 600, 0.02), coordinate(0, 400, 0.02)) : point(start.x + velocity.x * tick + (kind === 2 ? thrust.x * tick * tick : 0), start.y + velocity.y * tick + (kind === 2 ? thrust.y * tick * tick : 0))));
  if (count > 3 && chance(0.25)) for (let back = 2; back <= 1 + below(3); back++) samples[count - back] = samples[count - 1]!;
  return samples;
}
for (let index = 0; index < 1200; index++) {
  const samples = ring();
  record("ring_velocity", listFlat(samples, pointFlat), () => pointFlat(swing.ringVelocity(samples)));
}
for (let index = 0; index < 1200; index++) {
  const velocity = point(odd(0.03, () => (chance(0.3) ? grid(-800, 800, 1) : free(-1500, 1500))), odd(0.03, () => (chance(0.3) ? grid(-800, 800, 1) : free(-1500, 1500))));
  record("throw_of", pointFlat(velocity), () => pointFlat(swing.throwOf(velocity)));
}
for (let index = 0; index < 1200; index++) {
  const samples = ring();
  record("release_velocity", listFlat(samples, pointFlat), () => pointFlat(swing.releaseVelocity(samples)));
}
for (let index = 0; index < 1000; index++) {
  const hang = { grip: { x: coordinate(0, 600), y: coordinate(0, 400), vx: coordinate(-600, 600), vy: coordinate(-600, 600) }, bob: point(coordinate(0, 600), coordinate(0, 400)), previous: point(0, 0) };
  const held = { ...hang, previous: point(hang.bob.x + coordinate(-10, 10), hang.bob.y + coordinate(-10, 10)) };
  const samples = ring();
  record("throw_velocity", [...hangFlat(held), ...listFlat(samples, pointFlat)], () => pointFlat(swing.throwVelocity(held, samples)));
}
for (let index = 0; index < 1200; index++) {
  const vy = odd(0.03, () => (chance(0.3) ? grid(-600, 1200, 1) : free(-900, 1200)));
  const height = odd(0.03, () => (chance(0.3) ? grid(-50, 400, 2) : free(-100, 600)));
  record("impact_speed", [vy, height], () => [swing.impactSpeed(vy, height)]);
  record("chute_opens", [vy, height], () => [swing.chuteOpens(vy, height) ? 1 : 0]);
}
for (let index = 0; index < 400; index++) {
  const height = odd(0.05, () => grid(0, 100, 4));
  record("chute_of", [height], () => chuteFlat(swing.chuteOf(height)));
}
for (let index = 0; index < 600; index++) {
  const feet = point(coordinate(0, 600, 0.02), coordinate(0, 400, 0.02));
  const vx = coordinate(-300, 300, 0.02);
  const vy = coordinate(-300, 900, 0.02);
  const chute = swing.chuteOf(odd(0.02, () => grid(30, 60, 2)));
  record("canopy_of", [...pointFlat(feet), vx, vy, ...chuteFlat(chute)], () => canopyFlat(swing.canopyOf(feet, vx, vy, chute)));
}
for (let index = 0; index < 900; index++) {
  const flare = odd(0.03, () => (chance(0.5) ? pick([10, 12, 14]) : grid(-5, 20, 4)));
  const remaining = odd(0.03, () => (chance(0.3) ? pick([0, -0, flare, -4]) : grid(-20, 40, 8)));
  record("flare_of", [remaining, flare], () => [swing.flareOf(remaining, flare)]);
}
for (let index = 0; index < 900; index++) {
  const ticks = chance(0.1) ? -below(5000) : chance(0.1) ? below(2 ** 40) : below(200000);
  const phase = odd(0.03, () => (chance(0.3) ? grid(0, 1, 16) : free(-2, 2)));
  record("chute_wind", [ticks, phase], () => [swing.chuteWind(ticks, phase)]);
}
for (let descent = 0; descent < 40; descent++) {
  const chute = swing.chuteOf(pick([40, 48, 56]));
  let canopy = swing.canopyOf(point(grid(100, 500, 4), grid(0, 200, 4)), free(-300, 300), free(240, 900), chute);
  const target = free(0, 640);
  const ground = canopy.bob.y + free(50, 500);
  const phase = chance(0.5) ? free(0, 1) : null;
  for (let tick = 1; tick <= 120; tick++) {
    const remaining = ground - canopy.bob.y;
    const wind = phase === null ? 0 : swing.chuteWind(tick, phase);
    const before = canopy;
    canopy = swing.chuteStep(before, chute, target, remaining, wind);
    const after = canopy;
    record("chute_step", [...canopyFlat(before), ...chuteFlat(chute), target, remaining, wind], () => canopyFlat(after));
  }
}
for (let index = 0; index < 1000; index++) {
  const chute = swing.chuteOf(odd(0.02, () => grid(30, 60, 2)));
  const canopy = { x: coordinate(0, 600), y: coordinate(0, 400), vx: coordinate(-300, 300, 0.02), vy: coordinate(-300, 900, 0.02), bob: point(coordinate(0, 600), coordinate(0, 400)), previous: point(coordinate(0, 600), coordinate(0, 400)) };
  const target = coordinate(-100, 700, 0.02);
  const remaining = coordinate(-20, 300, 0.02);
  const wind = coordinate(-10, 10, 0.02);
  record("chute_step", [...canopyFlat(canopy), ...chuteFlat(chute), target, remaining, wind], () => canopyFlat(swing.chuteStep(canopy, chute, target, remaining, wind)));
}
for (let reel = 0; reel < 40; reel++) {
  const anchor = point(grid(100, 500, 4), grid(50, 200, 4));
  const length = grid(40, 180, 4);
  const angle = free(-0.25, 0.25);
  let bob = point(anchor.x + length * Math.sin(2 * Math.PI * angle), anchor.y + length * Math.cos(2 * Math.PI * angle));
  let previous = chance(0.5) ? bob : point(bob.x + free(-5, 5), bob.y + free(-5, 5));
  let rope = length;
  const least = pick([36, 43.2, 50.4]);
  for (let age = 0; age < 120; age++) {
    const [b, p, l] = [bob, previous, rope];
    const next = swing.reelStep(anchor, b, p, l, least, age);
    record("reel_step", [...pointFlat(anchor), ...pointFlat(b), ...pointFlat(p), l, least, age], () => [...pointFlat(next.bob), next.length]);
    previous = bob;
    bob = next.bob;
    rope = next.length;
  }
}
for (let index = 0; index < 1000; index++) {
  const anchor = point(coordinate(0, 600), coordinate(0, 300));
  const bob = point(anchor.x + coordinate(-150, 150, 0.02), anchor.y + coordinate(-150, 150, 0.02));
  const previous = point(bob.x + coordinate(-15, 15, 0.02), bob.y + coordinate(-15, 15, 0.02));
  const length = odd(0.02, () => grid(0, 200, 4));
  const least = odd(0.02, () => grid(0, 60, 4));
  const age = chance(0.1) ? -below(5) : below(30);
  record("reel_step", [...pointFlat(anchor), ...pointFlat(bob), ...pointFlat(previous), length, least, age], () => {
    const next = swing.reelStep(anchor, bob, previous, length, least, age);
    return [...pointFlat(next.bob), next.length];
  });
}
//#endregion 🔖️Swing

//#region 🔖️Climbing: walls
for (let index = 0; index < 1500; index++) {
  const from = point(coordinate(0, 600), coordinate(0, 500));
  const to = point(coordinate(0, 600), coordinate(0, 500));
  const height = odd(0.02, () => grid(20, 80, 2));
  const phase = odd(0.03, () => (chance(0.3) ? grid(-0.5, 1.5, 32) : free(-0.2, 1.2)));
  record("hoist_path", [...pointFlat(from), ...pointFlat(to), height, phase], () => pointFlat(climbing.hoistPath(from, to, height, phase)));
}
for (let index = 0; index < 600; index++) {
  const cut = pitch();
  const body = size();
  record("cling_of", [...pitchFlat(cut), ...sizeFlat(body)], () => [climbing.clingOf(cut, body)]);
  record("ledge_of", [...pitchFlat(cut), ...sizeFlat(body)], () => [climbing.ledgeOf(cut, body)]);
  record("rim_of", [...pitchFlat(cut), ...sizeFlat(body)], () => [climbing.rimOf(cut, body)]);
  record("foot_of", [...pitchFlat(cut), ...sizeFlat(body)], () => [climbing.footOf(cut, body)]);
  const y = coordinate(-40, 600, 0.03);
  record("climb_phase", [...pitchFlat(cut), y, ...sizeFlat(body)], () => [climbing.climbPhase(cut, y, body)]);
  record("slip_of", pitchFlat(cut), () => tossFlat(climbing.slipOf(cut)));
}
for (let index = 0; index < 300; index++) {
  const cut = { ...pitch(0), y1: -free(...pick([[24, 32], [56, 64], [120, 128]] as const)) };
  const body = size();
  record("foot_of", [...pitchFlat(cut), ...sizeFlat(body)], () => [climbing.footOf(cut, body)]);
}
for (let index = 0; index < 1000; index++) {
  const cut = pitch();
  const body = size();
  const y = chance(0.4) ? pick([climbing.rimOf(cut, body), climbing.footOf(cut, body)]) + pick([0, 0.25, -0.25]) : coordinate(-40, 800, 0.03);
  record("clings", [...pitchFlat(cut), y, ...sizeFlat(body)], () => [climbing.clings(cut, y, body) ? 1 : 0]);
}
/** 👑️ A perch on or near the top of `cut`: often on its surface at its height with the ledge on it or just beside. */
function crown(cut: Pitch, body: Size): Perch {
  if (chance(0.3)) return perch();
  const ledge = climbing.ledgeOf(cut, body);
  return { surface: chance(0.85) ? cut.surface : named("s", below(4)), x0: ledge - pick([0, 0.5, 20, 80, -0.5]), x1: ledge + pick([0, 0.5, 30, 120, -0.5]), y: chance(0.85) ? cut.y0 : cut.y0 + pick([0.5, -0.5, 10]) };
}
for (let index = 0; index < 1000; index++) {
  const cut = pitch();
  const body = size();
  const top = crown(cut, body);
  record("crowns", [...perchFlat(top), ...pitchFlat(cut), ...sizeFlat(body)], () => [climbing.crowns(top, cut, body) ? 1 : 0]);
}
for (let index = 0; index < 700; index++) {
  const cut = pitch();
  const body = size();
  const perches = some(below(5), () => crown(cut, body));
  record("rim_for", [...pitchFlat(cut), ...listFlat(perches, perchFlat), ...sizeFlat(body)], () => [indexOf(perches, climbing.rimFor(cut, perches, body))]);
}
for (let index = 0; index < 1500; index++) {
  const cut = pitch();
  const body = size();
  const x = climbing.clingOf(cut, body);
  const stand = chance(0.4) ? crown(cut, body) : chance(0.5) ? { surface: named("s", below(4)), x0: x - pick([0, 0.5, 40, -0.5]), x1: x + pick([0, 0.5, 40, -0.5]), y: pick([cut.y1, cut.y1 + 10, cut.y0 + 30, climbing.footOf(cut, body), climbing.rimOf(cut, body)]) } : perch();
  record("grip_for", [...perchFlat(stand), ...pitchFlat(cut), ...sizeFlat(body)], () => optional(climbing.gripFor(stand, cut, body), holdFlat));
}
for (let index = 0; index < 1000; index++) {
  const cut = pitch();
  const body = size();
  const pitches = some(below(5), () => (chance(0.6) ? { ...cut, x: cut.x + pick([0, 3, -6, 6, 6.5, -20]), side: chance(0.9) ? cut.side : (-cut.side as 1 | -1), y0: cut.y0 + pick([0, 10, -10, 60]), y1: cut.y1 + pick([0, 10, -100]) } : pitch()));
  const y = chance(0.7) ? cut.y0 + free(0, 200) : coordinate(-40, 800, 0.03);
  record("wall_holds", [...pitchFlat(cut), ...listFlat(pitches, pitchFlat), y, ...sizeFlat(body)], () => [indexOf(pitches, climbing.wallHolds(cut, pitches, y, body))]);
}
for (let index = 0; index < 1200; index++) {
  const grip = odd(0.03, () => (chance(0.5) ? grid(-2, 390, 4) : free(0, 384)));
  const effort = below(3);
  record("grip_step", [grip, effort], () => [climbing.gripStep(grip, EFFORTS[effort]!)]);
}
for (let index = 0; index < 1500; index++) {
  const y = coordinate(0, 600, 0.02);
  const goal = chance(0.1) ? y : coordinate(0, 600, 0.02);
  const ticks = chance(0.1) ? -below(5) : below(40);
  record("climb_step", [y, goal, ticks], () => [climbing.climbStep(y, goal, ticks)]);
}
for (let climb = 0; climb < 40; climb++) {
  let y = grid(200, 600, 4);
  const goal = y + pick([-1, 1]) * grid(0, 200, 4) + free(-0.5, 0.5);
  for (let ticks = 0; y !== goal && ticks < 400; ticks++) {
    const [from, at] = [y, ticks];
    y = climbing.climbStep(from, goal, at);
    const reached = y;
    record("climb_step", [from, goal, at], () => [reached]);
  }
}
for (let index = 0; index < 600; index++) {
  const y = coordinate(0, 600, 0.03);
  const goal = chance(0.05) ? y : chance(0.05) ? special() : y + pick([-1, 1]) * coordinate(0, 220);
  record("climb_ticks", [y, goal], () => [climbing.climbTicks(y, goal)]);
}
for (let index = 0; index < 1200; index++) {
  const y = coordinate(0, 600, 0.02);
  const vy = odd(0.03, () => (chance(0.3) ? pick([0, 30, 100, 160, 5000]) : free(-100, 300)));
  const floor = chance(0.3) ? y + free(0, 3) : coordinate(0, 700, 0.02);
  record("slide_step", [y, vy, floor], () => {
    const slid = climbing.slideStep(y, vy, floor);
    return [slid.y, slid.vy];
  });
}
for (let slide = 0; slide < 30; slide++) {
  let fall = { y: grid(200, 400, 4), vy: free(0, 60) };
  const floor = fall.y + free(10, 200);
  for (let tick = 0; tick < 80; tick++) {
    const before = fall;
    fall = climbing.slideStep(before.y, before.vy, floor);
    const after = fall;
    record("slide_step", [before.y, before.vy, floor], () => [after.y, after.vy]);
  }
}
for (let index = 0; index < 900; index++) {
  const cut = pitch();
  const body = size();
  const phase = chance(0.5) ? below(29) / 28 : odd(0.03, () => free(-0.2, 1.2));
  record("mantle_path", [...pitchFlat(cut), ...sizeFlat(body), phase], () => pointFlat(climbing.mantlePath(cut, body, phase)));
}
//#endregion 🔖️Climbing: walls

//#region 🔖️Climbing: wall lines
const HANDWORK: readonly climbing.Handwork[] = ["grab", "hang", "climb", "lunge", "mantle"];
const ENTRIES = ["grab", "hang", "cling"] as const;

/** 🏙️ A column of cards: two to four pitches of one side stacked with gaps, now and then one a few pixels aside, out of order or of another side, with their card tops as perches and a floor under them. */
function columnStage(): { pitches: Pitch[]; perches: Perch[]; size: Size; side: 1 | -1; x: number } {
  const pet = pick([{ width: 40, height: 48 }, { width: 34, height: 40 }, { width: 46, height: 56 }]);
  const side: 1 | -1 = chance(0.5) ? -1 : 1;
  const x = chance(0.5) ? 200 + below(400) / 4 : free(200, 300);
  const pitches: Pitch[] = [];
  let y = 100 + below(200) / 4;
  for (let index = 0, count = 2 + below(3); index < count; index++) {
    const length = 8 + below(480) / 4;
    pitches.push({ wall: named("w", index % 3), surface: named("s", index), side: chance(0.08) ? (-side as 1 | -1) : side, x: x + (chance(0.25) ? pick([-6, 6, 6.5, 3, -2]) : 0), y0: y, y1: y + length });
    y += length + (chance(0.25) ? climbing.CROSS_REACH * pet.height - climbing.GRIP_BITE + pick([0, 0, 0.25, -0.25]) : below(480) / 4);
  }
  if (chance(0.2)) {
    const twin = pick(pitches);
    pitches.splice(below(pitches.length + 1), 0, { ...twin, wall: named("w", 3 + below(2)), x: twin.x + pick([3, -2, 0]) });
  }
  if (chance(0.3)) pitches.reverse();
  if (chance(0.2)) pitches.push(pitch(0));
  const perches = pitches.map((cut) => ({ surface: cut.surface, x0: side < 0 ? x + 6 : x - 180, x1: side < 0 ? x + 180 : x - 6, y: cut.y0 }));
  perches.push({ surface: "s9", x0: 0, x1: 900, y: Math.max(...pitches.map((cut) => cut.y1)) });
  return { pitches, perches, size: pet, side, x };
}

for (let index = 0; index < 1500; index++) {
  const stage = columnStage();
  const from = chance(0.1) ? pitch() : pick(stage.pitches);
  const to = chance(0.1) ? pitch() : pick(stage.pitches);
  const body = chance(0.9) ? stage.size : size();
  record("crossable", [...pitchFlat(from), ...pitchFlat(to), ...sizeFlat(body)], () => [climbing.crossable(from, to, body) ? 1 : 0]);
  const phase = chance(0.4) ? below(17) / 16 : odd(0.03, () => free(-0.2, 1.2));
  record("lunge_path", [...pitchFlat(from), ...pitchFlat(to), ...sizeFlat(body), phase], () => pointFlat(climbing.lungePath(from, to, body, phase)));
}
for (let index = 0; index < 900; index++) {
  const stage = columnStage();
  const at = below(stage.pitches.length);
  record("chain_of", [...listFlat(stage.pitches, pitchFlat), at, ...sizeFlat(stage.size)], () => listFlat(climbing.chainOf(stage.pitches[at]!, stage.pitches, stage.size), (member) => [stage.pitches.indexOf(member)]));
}
for (let index = 0; index < 400; index++) {
  const stage = columnStage();
  const at = below(stage.pitches.length);
  const held = stage.pitches[at]!;
  const chain = climbing.chainOf(held, stage.pitches, stage.size);
  const start = chain.indexOf(held);
  const end = below(chain.length);
  const entry = below(3);
  const target = chain[end]!;
  const rim = climbing.rimOf(target, stage.size);
  const foot = climbing.footOf(target, stage.size);
  const goal = pick([rim, foot, rim + (foot - rim) * unit(), grid(rim - 2, foot + 2, 4)]);
  const x = climbing.clingOf(held, stage.size) + pick([0, -12, 12, 7.5, free(-15, 15)]);
  const y = entry === 2 ? climbing.rimOf(held, stage.size) + (climbing.footOf(held, stage.size) - climbing.rimOf(held, stage.size)) * unit() : climbing.footOf(held, stage.size) - grid(0, 30, 4);
  const mantle = chance(0.5);
  const args = [...listFlat(stage.pitches, pitchFlat), at, entry, x, y, end, goal, mantle ? 1 : 0, ...sizeFlat(stage.size)];
  const path = climbing.wallPath(chain, start, ENTRIES[entry]!, x, y, end, goal, mantle, stage.size);
  record("wall_path", args, () => listFlat(path, (clamber) => [clamber.x, clamber.y, clamber.hold, HANDWORK.indexOf(clamber.work)]));
  record("wall_cost", args, () => [climbing.wallCost(path)]);
}
for (let index = 0; index < 1200; index++) {
  const stage = columnStage();
  const from = pick(stage.perches);
  const to = chance(0.05) ? from : pick(stage.perches);
  const gear: Gear[] = chance(0.6) ? ["climb"] : GEARS.filter(() => chance(0.6));
  const x = chance(0.7) ? free(from.x0, from.x1) : coordinate(-100, 900);
  const grip = chance(0.7) ? climbing.GRIP_BUDGET : grid(0, 600, 1);
  const keepouts = some(below(2), rect);
  record("route_of", [x, ...perchFlat(from), ...perchFlat(to), gear.length, ...gear.map((item) => GEARS.indexOf(item)), ...sizeFlat(stage.size), grip, ...listFlat(stage.pitches, pitchFlat), 0, ...listFlat(keepouts, rectFlat)], () => legsFlat(climbing.routeOf(x, from, to, gear, stage.size, grip, stage.pitches, [], keepouts), stage.pitches, []));
}
//#endregion 🔖️Climbing: wall lines

//#region 🔖️Climbing: ladders
for (let index = 0; index < 1600; index++) {
  const stage = ladderStage();
  const low = chance(0.1) ? perch() : stage.low;
  const high = chance(0.05) ? perch() : stage.high;
  const face = chance(0.05) ? pitch() : stage.face;
  record("ladder_for", [...perchFlat(low), ...perchFlat(high), ...pitchFlat(face), ...listFlat(stage.keepouts, rectFlat), ...sizeFlat(stage.size)], () => optional(climbing.ladderFor(low, high, face, stage.keepouts, stage.size), ladderFlat));
}
const standing = ladders(400);
for (const { ladder, stage } of standing) {
  for (let survey = 0; survey < 3; survey++) {
    const dx = pick([0, 0, 3, -4.5, 8, 8.5, -40]);
    const dy = pick([0, 0, -5, 6, 8, 8.5, 12, 12.5]);
    const perches = [chance(0.9) ? { ...stage.low, y: stage.low.y + pick([0, 0, 6, -6, 12, 12.5]), x1: stage.low.x1 + pick([0, 0, -30, -200]) } : perch(), { ...stage.high, x0: stage.high.x0 + dx, x1: stage.high.x1 + dx, y: stage.high.y + dy }, ...some(below(2), perch)];
    const pitches = [{ ...stage.face, x: stage.face.x + dx, y0: stage.face.y0 + dy, y1: stage.face.y1 + dy }, ...some(below(2), pitch)];
    const keepouts = [...stage.keepouts.map((box) => ({ ...box, x: box.x + dx, y: box.y + dy })), ...some(below(2), rect)];
    const owner = chance(0.8) ? stage.size : size();
    record("ladder_holds", [...ladderFlat(ladder), ...listFlat(perches, perchFlat), ...listFlat(pitches, pitchFlat), ...listFlat(keepouts, rectFlat), ...sizeFlat(owner)], () => optional(climbing.ladderHolds(ladder, perches, pitches, keepouts, owner), ladderFlat));
  }
}
for (const bound of [climbing.LADDER_STEEP, climbing.LADDER_FLAT]) {
  let found = 0;
  for (let attempt = 0; found < 80 && attempt < 40000; attempt++) {
    const pet = pick([{ width: 40, height: 48 }, { width: 34, height: 40 }, { width: 46, height: 56 }]);
    const side: 1 | -1 = chance(0.5) ? -1 : 1;
    const px = chance(0.5) ? 200 + below(1600) / 4 : free(200, 600);
    const rim = 150 + below(400) / 4;
    const top = rim + climbing.LADDER_TUCK;
    const lowY = top + (chance(0.5) ? grid(pet.height, 3.5 * pet.height, 4) : free(pet.height, 3.5 * pet.height));
    const footX = px + side * bound * (lowY - top);
    if ((footX - px) * side !== bound * (lowY - top)) continue;
    found++;
    const face: Pitch = { wall: "w1", surface: "s1", side, x: px, y0: rim, y1: rim + 400 };
    const perches = [{ surface: "s0", x0: footX - 100, x1: footX + 100, y: lowY }, { surface: "s1", x0: side < 0 ? px + 6 : px - 180, x1: side < 0 ? px + 180 : px - 6, y: rim }];
    const ladder: Ladder = { wall: "w1", surface: "s0", side, foot: point(footX, lowY), top: point(px, top) };
    record("ladder_holds", [...ladderFlat(ladder), ...listFlat(perches, perchFlat), ...listFlat([face], pitchFlat), 0, ...sizeFlat(pet)], () => optional(climbing.ladderHolds(ladder, perches, [face], [], pet), ladderFlat));
  }
}

/** 🎋️ A ladder: a standing one, or one of generated ends, rarely special. */
function anyLadder(): Ladder {
  if (standing.length > 0 && chance(0.6)) return pick(standing).ladder;
  return { wall: named("w", below(3)), surface: named("s", below(4)), side: chance(0.5) ? -1 : 1, foot: point(coordinate(0, 600, 0.02), coordinate(200, 500, 0.02)), top: point(coordinate(0, 600, 0.02), coordinate(0, 300, 0.02)) };
}
for (let index = 0; index < 700; index++) {
  const ladder = anyLadder();
  const body = size();
  record("ladder_length", ladderFlat(ladder), () => [climbing.ladderLength(ladder)]);
  record("ladder_lean", ladderFlat(ladder), () => [climbing.ladderLean(ladder)]);
  record("ladder_exit", [...ladderFlat(ladder), ...sizeFlat(body)], () => [climbing.ladderExit(ladder, body)]);
  record("ladder_landing", [...ladderFlat(ladder), ...sizeFlat(body)], () => pointFlat(climbing.ladderLanding(ladder, body)));
  if (climbing.ladderLength(ladder) < 1e9) record("ladder_rungs", ladderFlat(ladder), () => [climbing.ladderRungs(ladder)]);
}
for (let index = 0; index < 1200; index++) {
  const travel = coordinate(-10, 200, 0.02);
  const goal = chance(0.1) ? travel : coordinate(-10, 200, 0.02);
  const ticks = chance(0.1) ? -below(5) : below(40);
  record("ladder_step", [travel, goal, ticks], () => [climbing.ladderStep(travel, goal, ticks)]);
}
for (const { ladder, stage } of standing.slice(0, 12)) {
  const exit = climbing.ladderExit(ladder, stage.size);
  for (const [start, goal] of [[0, exit], [exit, 0]] as const) {
    let travel = start;
    for (let ticks = 0; travel !== goal && ticks < 400; ticks++) {
      const [from, at] = [travel, ticks];
      travel = climbing.ladderStep(from, goal, at);
      const reached = travel;
      record("ladder_step", [from, goal, at], () => [reached]);
      record("ladder_at", [...ladderFlat(ladder), reached], () => pointFlat(climbing.ladderAt(ladder, reached)));
      record("ladder_phase", [reached], () => [climbing.ladderPhase(reached)]);
    }
  }
}
for (let index = 0; index < 1000; index++) {
  const ladder = anyLadder();
  const length = climbing.ladderLength(ladder);
  const travel = chance(0.2) ? pick([0, -0, length, -3, length + 10]) : odd(0.03, () => free(0, length));
  record("ladder_at", [...ladderFlat(ladder), travel], () => pointFlat(climbing.ladderAt(ladder, travel)));
}
for (let index = 0; index < 900; index++) {
  const travel = odd(0.03, () => (chance(0.3) ? grid(-21, 200, 4) : free(-21, 200)));
  record("ladder_phase", [travel], () => [climbing.ladderPhase(travel)]);
}
for (let index = 0; index < 900; index++) {
  const ladder = anyLadder();
  const body = size();
  const y = chance(0.3) ? ladder.foot.y - climbing.TOPPLE_STEP * body.height + pick([0, 0.5, -0.5]) : coordinate(0, 600, 0.03);
  record("spill_of", [...ladderFlat(ladder), y, ...sizeFlat(body)], () => optional(climbing.spillOf(ladder, y, body), tossFlat));
}
//#endregion 🔖️Climbing: ladders

//#region 🔖️Climbing: rope
for (let index = 0; index < 1500; index++) {
  const from = point(coordinate(0, 400), coordinate(0, 400));
  const to = point(coordinate(0, 400), coordinate(0, 400));
  const first = pick([from, to, point(coordinate(0, 400), coordinate(0, 400))]);
  const second = pick([from, to, first, point(coordinate(0, 400), coordinate(0, 400))]);
  const keepouts = [...some(below(4), rect), ...(chance(0.5) ? [{ x: first.x - free(0, 40), y: first.y - free(0, 40), width: free(0, 80), height: free(0, 80) }] : []), ...(chance(0.3) ? [{ x: second.x - 20, y: second.y, width: 40, height: 30 }] : [])];
  const margin = pick([0, 2, 6, -3, free(-5, 10)]);
  record("sighted", [...pointFlat(from), ...pointFlat(to), ...listFlat(keepouts, rectFlat), margin, ...pointFlat(first), ...pointFlat(second)], () => [climbing.sighted(from, to, keepouts, margin, first, second) ? 1 : 0]);
}
for (let index = 0; index < 1800; index++) {
  const stage = shotStage();
  const feet = chance(0.05) ? point(coordinate(0, 600, 0.2), coordinate(0, 600, 0.2)) : stage.feet;
  record("shot_for", [...pointFlat(feet), ...listFlat(stage.perches, perchFlat), ...listFlat(stage.keepouts, rectFlat), ...sizeFlat(stage.size)], () => optional(climbing.shotFor(feet, stage.perches, stage.keepouts, stage.size), shotFlat));
}
const biting = shots(400);
for (const { shot, stage } of biting) {
  for (let survey = 0; survey < 3; survey++) {
    const dy = pick([0, 0, -3, 5, 8, 8.5, -8.5]);
    const target = stage.perches.find((edge) => edge.surface === shot.surface && edge.x0 <= shot.hook.x && shot.hook.x <= edge.x1 && edge.y - 1 === shot.hook.y) ?? stage.perches[0]!;
    const perches = [...stage.perches.filter((edge) => edge !== target), { ...target, y: target.y + dy, x0: target.x0 + pick([0, 0, 5, 200]) }, ...some(below(2), perch)];
    const keepouts = [...stage.keepouts.map((box) => ({ ...box, y: box.y + dy })), ...some(below(2), rect)];
    record("shot_holds", [...shotFlat(shot), ...listFlat(perches, perchFlat), ...listFlat(keepouts, rectFlat)], () => optional(climbing.shotHolds(shot, perches, keepouts), shotFlat));
  }
}
/** 🎇️ A shot: a biting one, or one of generated ends. */
function anyShot(): Shot {
  if (biting.length > 0 && chance(0.7)) return pick(biting).shot;
  const muzzle = point(coordinate(0, 600, 0.02), coordinate(200, 500, 0.02));
  const hook = point(coordinate(0, 600, 0.02), coordinate(0, 300, 0.02));
  return { surface: named("s", below(4)), facing: chance(0.5) ? -1 : 1, muzzle, hook, length: Math.sqrt((hook.x - muzzle.x) ** 2 + (hook.y - muzzle.y) ** 2), reel: chance(0.5) ? "zip" : "swing" };
}
for (let index = 0; index < 700; index++) {
  const shot = anyShot();
  const edge = chance(0.5) ? { surface: shot.surface, x0: shot.hook.x - free(0, 100), x1: shot.hook.x + free(0, 100), y: shot.hook.y + 1 } : perch();
  const body = size();
  record("miss_of", [...shotFlat(shot), ...perchFlat(edge)], () => pointFlat(climbing.missOf(shot, edge)));
  record("landing_for", [...shotFlat(shot), ...perchFlat(edge), ...sizeFlat(body)], () => pointFlat(climbing.landingFor(shot, edge, body)));
  const rope = chance(0.3) ? shot.length : odd(0.03, () => free(0, shot.length));
  record("haul_of", [...shotFlat(shot), rope, ...sizeFlat(body)], () => haulFlat(climbing.haulOf(shot, rope, body)));
}
for (let index = 0; index < 1000; index++) {
  const from = point(coordinate(0, 600, 0), coordinate(0, 500, 0));
  const to = chance(0.05) ? from : point(coordinate(0, 600, 0), coordinate(0, 500, 0));
  const speed = chance(0.8) ? pick([640, 1280]) : grid(1, 2000, 4);
  record("hook_ticks", [...pointFlat(from), ...pointFlat(to), speed], () => [climbing.hookTicks(from, to, speed)]);
  const ticks = chance(0.1) ? -below(4) : below(Math.max(climbing.hookTicks(from, to, speed) + 3, 1));
  record("hook_step", [...pointFlat(from), ...pointFlat(to), speed, ticks], () => pointFlat(climbing.hookStep(from, to, speed, ticks)));
}
for (const { shot, stage } of biting.slice(0, 120)) {
  const steps: readonly ["zip_step" | "sway_step" | "haul_step", (shot: Shot, haul: Haul, ticks: number, size: Size) => Haul][] = [["haul_step", climbing.haulStep], [shot.reel === "zip" ? "sway_step" : "zip_step", shot.reel === "zip" ? climbing.swayStep : climbing.zipStep]];
  for (const [name, step] of steps) {
    let haul = climbing.haulOf(shot, shot.length, stage.size);
    const least = swing.REEL_LEAST * stage.size.height;
    for (let ticks = 0; haul.rope > least && ticks < 200; ticks++) {
      const [before, at] = [haul, ticks];
      haul = step(shot, before, at, stage.size);
      const after = haul;
      record(name, [...shotFlat(shot), ...haulFlat(before), at, ...sizeFlat(stage.size)], () => haulFlat(after));
      if (name === "haul_step") record(shot.reel === "zip" ? "zip_step" : "sway_step", [...shotFlat(shot), ...haulFlat(before), at, ...sizeFlat(stage.size)], () => haulFlat(after));
    }
  }
  record("haul_ticks", [...shotFlat(shot), ...sizeFlat(stage.size)], () => [climbing.haulTicks(shot, stage.size)]);
}
for (let index = 0; index < 300; index++) {
  const shot = anyShot();
  const body = size();
  record("haul_ticks", [...shotFlat(shot), ...sizeFlat(body)], () => [climbing.haulTicks(shot, body)]);
}
//#endregion 🔖️Climbing: rope

//#region 🔖️Climbing: routes
for (let index = 0; index < 1800; index++) {
  const pet = pick([{ width: 40, height: 48 }, { width: 34, height: 40 }, { width: 46, height: 56 }]);
  const rim = 200 + below(400) / 4;
  const x0 = 200 + below(400) / 4;
  const x1 = x0 + 100 + below(600) / 4;
  const floorY = rim + pick([40, 60, 93, 100, 140, 180, 220]);
  const shelf: Perch = { surface: "s1", x0: x0 + 6, x1: x1 - 6, y: rim };
  const floor: Perch = { surface: "s0", x0: 0, x1: 640 + below(200), y: floorY };
  const box = solid(x0, x1, rim, floorY);
  const pitches: Pitch[] = [{ wall: "w0", surface: "s1", side: -1, x: x0, y0: rim, y1: floorY }, { wall: "w1", surface: "s1", side: 1, x: x1, y0: rim, y1: floorY }];
  if (chance(0.2)) pitches.push(pitch());
  const ladderList: Ladder[] = [];
  for (const face of pitches) if (chance(0.15)) {
    const raised = climbing.ladderFor(floor, shelf, face, [box], pick([pet, { width: 46, height: 56 }]));
    if (raised !== null) ladderList.push(raised);
  }
  if (chance(0.1)) ladderList.push(anyLadder());
  const keepouts = [box, ...some(below(2), rect)];
  const gear: Gear[] = GEARS.filter(() => chance(0.55));
  const up = chance(0.6);
  const [from, to] = chance(0.05) ? [floor, floor] : up ? [floor, shelf] : [shelf, floor];
  const x = chance(0.7) ? free(from.x0, from.x1) : coordinate(-100, 900);
  const grip = chance(0.7) ? climbing.GRIP_BUDGET : grid(0, 384, 1);
  record("route_of", [x, ...perchFlat(from), ...perchFlat(to), gear.length, ...gear.map((item) => GEARS.indexOf(item)), ...sizeFlat(pet), grip, ...listFlat(pitches, pitchFlat), ...listFlat(ladderList, ladderFlat), ...listFlat(keepouts, rectFlat)], () => legsFlat(climbing.routeOf(x, from, to, gear, pet, grip, pitches, ladderList, keepouts), pitches, ladderList));
}
//#endregion 🔖️Climbing: routes

const directory = join(import.meta.dir, "🗑️generated", "d1");
mkdirSync(directory, { recursive: true });
writeFileSync(join(directory, "gear-bits.json"), JSON.stringify(cases));
const tally = new Map<string, number>();
for (const recorded of cases) tally.set(recorded.fn, (tally.get(recorded.fn) ?? 0) + 1);
console.log(`[DEBUG] ${cases.length} cases, ${tally.size} functions: ${[...tally].map(([fn, count]) => `${fn} ${count}`).join(", ")}`);
console.log(`[DEBUG] legs answered by routeOf: ${[...legTally].map(([means, count]) => `${means} ${count}`).join(", ")}`);
