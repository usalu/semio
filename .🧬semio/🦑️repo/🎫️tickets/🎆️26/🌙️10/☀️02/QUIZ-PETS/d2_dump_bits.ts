/** 🧮️ Ticket tool of work package D2, TypeScript half: evaluates every export of the clearance and gesture modules of `@semio-tech/pets` on generated inputs and writes every input and every result as an IEEE-754 bit pattern; replays the committed gesture corpus in all its variants; runs the reference world of the clearance suite. `d2_check_bits.rs` and `d2_check_world.rs` beside this file recompute everything with the Rust twins and compare bit for bit.
 *
 * Output, all in `🗑️generated/d2/`:
 * - `bits.json` — one case per evaluation `{ fn, args, out }`: flat arguments and results as sixteen hexadecimal digits
 *   (`nan` for every NaN, whose payload no language promises), or `out: "throws"`. Lists, boxes, claims and states are
 *   flattened in the layouts named at their encoders below; owners are numbers `k` standing for the slug `o<k>`.
 *   Inputs: lattice values (quarter pixels, so touching and exact fits happen), arbitrary doubles, signed zeros, NaN,
 *   infinities and subnormals where a function takes plain numbers, and whole trajectories of the recognisers whose
 *   inputs are the previous outputs (drawn circles, pettings, shakes, random walks, presses, histories of attention).
 * - `corpus.json` — every committed press, history, circle, petting, held path and stretch of travel of
 *   `👆️gesture-recognition`, replayed like the subject adapters replay them: presses and histories as answered, every
 *   hover and held trace in each of the sixteen variants with its full cue sequence and a digest of the recogniser's
 *   state after every tick, and the travel in all sixteen variants past every committed body (cues; the state digest
 *   in the variants 0 and 9).
 * - `world.json` — runs of the reference world of the clearance unit suite (every rule, slices of four, each rule
 *   switched off): a digest of the mode, the feet and the box of every pet after every tick, and the final tally and
 *   counts.
 *
 * Digests fold 32-bit words with `h = (h ^ word) × 16777619` from 2166136261 (FNV-1a on words); a number contributes the
 * high and the low word of its bit pattern.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d2_dump_bits.ts
 *   bash .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/rust_scratch.sh d2 test --release --offline --test d2_bits -- --nocapture
 *   bash .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/rust_scratch.sh d2 test --release --offline --lib world_check -- --nocapture
 *
 * @see ./d2_check_bits.rs, ./d2_check_world.rs — the Rust halves
 * @see ./d2_scratch.ts — the scratch crate and the mutants
 * @see ./📓️report2-d2.md — the recorded result
 */
import { plugin } from "bun";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import * as C from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🚧️clearance/🟦️.ts";
import * as G from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/👆️gesture/🟦️.ts";
import { CUES, type Circling, type Hover, POINTERS, PRESS_PHASES, type Press, type Shaking, type Stroking, TIERS, type Warmth } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";

const idle = (): void => {};
plugin({
  name: "vitest-stand-in",
  setup(build) {
    build.module("vitest", () => ({ exports: { describe: Object.assign(idle, { each: () => idle }), it: Object.assign(idle, { each: () => idle }), expect: idle }, loader: "object" }));
  },
});

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const PETS = join(ROOT, "🧰️framework/🛍️products/🐾️pets");
const OUT = join(import.meta.dir, "🗑️generated", "d2");

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

/** 🧾️ Folds 32-bit words into a running FNV-1a digest on words. */
class Digest {
  value = 2166136261;
  /** ➕️ Folds the two words of a number. */
  number(number: number): void {
    view.setFloat64(0, number);
    this.word(view.getUint32(0));
    this.word(view.getUint32(4));
  }
  /** 🔩️ Folds one word. */
  word(word: number): void {
    this.value = Math.imul((this.value ^ word) >>> 0, 16777619) >>> 0;
  }
  /** 🔡️ The digest as eight hexadecimal digits. */
  hex(): string {
    return this.value.toString(16).padStart(8, "0");
  }
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

/** 📐️ A coordinate: half of the time on the quarter-pixel lattice, else any double. */
function coordinate(low: number, high: number): number {
  return chance(0.5) ? grid(low, high, 4) : free(low, high);
}
//#endregion 🔖️Generators

//#region 🔖️Clearance encoders
type Box = C.Extent;

/** 📦️ A box of a given size range near a place: `x0, y0, x1, y1`. */
function box(x: number, y: number, spread: number, size: number, special = 0): Box {
  const x0 = odd(special, () => coordinate(x - spread, x + spread));
  const y0 = odd(special, () => coordinate(y - spread, y + spread));
  return { x0, y0, x1: x0 + odd(special, () => coordinate(0, size)), y1: y0 + odd(special, () => coordinate(0, size)) };
}

/** 🧱️ A box flattened. */
function flatBox(box: Box): number[] {
  return [box.x0, box.y0, box.x1, box.y1];
}

/** 🔲️ A list of boxes flattened: count, then the boxes. */
function flatBoxes(boxes: readonly Box[]): number[] {
  return [boxes.length, ...boxes.flatMap(flatBox)];
}

/** 🐾️ A list of bodies flattened: count, then owner and box each. */
function flatBodies(bodies: readonly C.Body[]): number[] {
  return [bodies.length, ...bodies.flatMap((body) => [Number(body.owner.slice(1)), ...flatBox(body.extent)])];
}

/** 🎫️ A claim flattened: owner, slice count, `from, until, box` each, a rest flag and the rest (zeros without one). */
function flatClaim(claim: C.Claim): number[] {
  return [Number(claim.owner.slice(1)), claim.slices.length, ...claim.slices.flatMap((slice) => [slice.from, slice.until, ...flatBox(slice.extent)]), claim.rest === null ? 0 : 1, ...(claim.rest === null ? [0, 0, 0, 0] : flatBox(claim.rest))];
}

/** 🗂️ A list of claims flattened: count, then the claims. */
function flatClaims(claims: readonly C.Claim[]): number[] {
  return [claims.length, ...claims.flatMap(flatClaim)];
}

/** 👥️ Pairs flattened: count, then the owners of each pair. */
function flatPairs(pairs: readonly C.Pair[]): number[] {
  return [pairs.length, ...pairs.flatMap((pair) => [Number(pair.first.slice(1)), Number(pair.second.slice(1))])];
}

/** 🔤️ Owners flattened: count, then the owner numbers. */
function flatOwners(owners: readonly string[]): number[] {
  return [owners.length, ...owners.map((owner) => Number(owner.slice(1)))];
}

/** ❓️ An optional number flattened: a flag and the number (0 without one). */
function flatMaybe(value: number | null): number[] {
  return value === null ? [0, 0] : [1, value];
}

/** 🪵️ A perch flattened: surface number, `x0, x1, y`. */
function flatPerch(perch: { surface: string; x0: number; x1: number; y: number }): number[] {
  return [Number(perch.surface.slice(1)), perch.x0, perch.x1, perch.y];
}

/** 👪️ A crowd of bodies on a coarse lattice around a place, so that touching, overlapping and near bodies happen; owners `o0, o1, …` or drawn from `owners`. */
function crowd(count: number, x: number, y: number, spread: number, owners = 0): C.Body[] {
  const bodies: C.Body[] = [];
  for (let index = 0; index < count; index++) {
    const x0 = x + grid(-spread, spread, 2);
    const y0 = y + grid(-spread / 2, spread / 2, 2);
    bodies.push({ owner: `o${owners > 0 ? below(owners) : index}`, extent: { x0, y0, x1: x0 + grid(4, 60, 2), y1: y0 + grid(4, 60, 2) } });
  }
  return bodies;
}

/** 🛤️ A path of boxes of a body 40 × 50 that moves by a fixed step per tick, now and then with a quarter-pixel jitter. */
function path(x: number, y: number, dx: number, dy: number, ticks: number): Box[] {
  const boxes: Box[] = [];
  for (let tick = 0; tick < ticks; tick++) {
    const jitter = chance(0.2) ? grid(-1, 1, 4) : 0;
    boxes.push({ x0: x + dx * tick + jitter, y0: y + dy * tick, x1: x + dx * tick + 40 + jitter, y1: y + dy * tick + 50 });
  }
  return boxes;
}

/** 🎟️ A claim of a generated path, its owner drawn from `owners`. */
function claimFor(owners: number): C.Claim {
  const made = path(grid(0, 240, 1), grid(0, 160, 1), below(9) - 4, below(9) - 4, 1 + below(30));
  return C.claimOf(`o${below(owners)}`, below(40), made, pick([1, 1, 2, 3, 4]), chance(0.7) ? made[made.length - 1]! : null);
}
//#endregion 🔖️Clearance encoders

//#region 🔖️Clearance
record("clearance_constants", [], () => [C.MARGIN, C.SEAM, C.FOREVER, C.PUSHES, C.PATIENCE, C.DELAY, C.SLIDE_OFF_SPEED, C.SLIDE_OFF_GAIN, C.SLIDE_OFF_LIMIT, C.LEAN, C.LANE_LIFT, C.SCOOT_HASTE, ...C.STEERINGS, C.UPRIGHT.left, C.UPRIGHT.right, C.UPRIGHT.above, C.TALLY.ticks, C.TALLY.actors, C.TALLY.overlaps, C.TALLY.nears, C.TALLY.poofs, C.TALLY.waits]);

for (let index = 0; index < 600; index++) {
  const height = odd(0.05, () => coordinate(0, 80));
  const sine = odd(0.05, () => (chance(0.3) ? pick([-1, 1, 0, -0, 0.5, -0.5]) : free(-1, 1)));
  record("leaning", [height, sine], () => {
    const posture = C.leaning(height, sine);
    return [posture.left, posture.right, posture.above];
  });
}

for (let index = 0; index < 600; index++) {
  const args = [odd(0.04, () => coordinate(10, 80)), odd(0.04, () => coordinate(10, 80)), odd(0.04, () => coordinate(0, 130)), odd(0.04, () => coordinate(-10, 60))];
  if (index % 7 === 0) args[2] = args[0]!;
  record("canopied", args, () => {
    const posture = C.canopied({ width: args[0]!, height: args[1]! }, { width: args[2]!, height: args[3]! });
    return [posture.left, posture.right, posture.above];
  });
}

for (let index = 0; index < 800; index++) {
  const args = [odd(0.03, () => coordinate(0, 960)), odd(0.03, () => coordinate(0, 520)), odd(0.03, () => coordinate(20, 60)), odd(0.03, () => coordinate(30, 60)), odd(0.03, () => (chance(0.5) ? 0 : coordinate(0, 20))), odd(0.03, () => coordinate(0, 20)), odd(0.03, () => coordinate(0, 20)), odd(0.03, () => coordinate(0, 40)), odd(0.03, () => pick([4, 0, 2, 4.25]))];
  record("body_of", [below(20), ...args], () => flatBox(C.bodyOf("o1", { x: args[0]!, y: args[1]! }, { width: args[2]!, height: args[3]! }, args[4]!, { left: args[5]!, right: args[6]!, above: args[7]! }, args[8]!).extent));
}

for (let index = 0; index < 400; index++) {
  const one = box(400, 300, 300, 80, 0.03);
  const dx = odd(0.05, () => coordinate(-50, 50));
  const dy = odd(0.05, () => coordinate(-50, 50));
  record("shifted", [...flatBox(one), dx, dy], () => flatBox(C.shifted(one, dx, dy)));
  record("grown", [...flatBox(one), dx], () => flatBox(C.grown(one, dx)));
}

for (let index = 0; index < 800; index++) {
  const one = box(100, 100, 40, 60, 0.04);
  const other = index % 5 === 0 ? { x0: chance(0.5) ? -0 : 0, y0: one.y0, x1: one.x1, y1: chance(0.5) ? -0 : 0 } : box(100, 100, 40, 60, 0.04);
  if (index % 5 === 0) one.x0 = chance(0.5) ? 0 : -0;
  record("united", [...flatBox(one), ...flatBox(other)], () => flatBox(C.united(one, other)));
  record("meets", [...flatBox(one), ...flatBox(other)], () => [C.meets(one, other) ? 1 : 0]);
}

for (let index = 0; index < 500; index++) {
  const bodies = crowd(below(11), 300, 300, 80);
  const margin = pick([0, 1, 2, 4, 0.5, 8]);
  record("overlaps", flatBodies(bodies), () => flatPairs(C.overlaps(bodies)));
  record("near_misses", [...flatBodies(bodies), margin], () => flatPairs(C.nearMisses(bodies, margin)));
}

for (let index = 0; index < 600; index++) {
  const bodies = crowd(below(6), 200, 100, 120, 6);
  const claims = Array.from({ length: below(4) }, () => claimFor(6));
  const owner = below(7);
  const tick = below(60);
  const extent = box(150, 80, 120, 60);
  record("obstacles_of", [owner, ...flatBodies(bodies), ...flatClaims(claims), tick], () => flatBoxes(C.obstaclesOf(`o${owner}`, bodies, claims, tick)));
  record("free_at", [...flatBox(extent), owner, ...flatBodies(bodies), ...flatClaims(claims), tick], () => [C.freeAt(extent, `o${owner}`, bodies, claims, tick) ? 1 : 0]);
  const obstacles = Array.from({ length: below(7) }, () => box(150, 80, 120, 60, 0.02));
  record("free_among", [...flatBox(extent), ...flatBoxes(obstacles)], () => [C.freeAmong(extent, obstacles) ? 1 : 0]);
}

for (let index = 0; index < 1500; index++) {
  const x = odd(0.02, () => coordinate(60, 260));
  const half = coordinate(8, 30);
  const extent = { x0: x - half, y0: 250, x1: x + half, y1: 300 };
  const obstacles = Array.from({ length: below(7) }, () => {
    const x0 = coordinate(-20, 320);
    const y0 = coordinate(200, 330);
    return { x0, y0, x1: x0 + coordinate(2, 60), y1: y0 + coordinate(2, 60) };
  });
  const low = odd(0.04, () => coordinate(0, 120));
  const high = index % 9 === 0 ? low - pick([0, 0.25, 1]) : odd(0.04, () => low + coordinate(0, 240));
  record("slot_in", [x, ...flatBox(extent), low, high, ...flatBoxes(obstacles)], () => flatMaybe(C.slotIn(x, extent, low, high, obstacles)));
  const perch = { surface: "s0", x0: low, x1: high, y: 300 };
  const foot = pick([0, half, half - 2, 12.5]);
  record("spot_on", [...flatPerch(perch), x, ...flatBox(extent), foot, ...flatBoxes(obstacles)], () => flatMaybe(C.spotOn(perch, x, extent, foot, obstacles)));
  const drop = odd(0.04, () => coordinate(-40, 200));
  record("column_over", [...flatPerch(perch), x, ...flatBox(extent), drop, foot, ...flatBoxes(obstacles)], () => flatMaybe(C.columnOver(perch, x, extent, drop, foot, obstacles)));
}

for (let index = 0; index < 700; index++) {
  const made = path(coordinate(0, 200), coordinate(0, 200), grid(-8, 8, 4), grid(-8, 8, 4), below(25));
  const from = below(200) - 20;
  const span = pick([-2, 0, 1, 1, 2, 3, 4, 7]);
  const rest = chance(0.6) ? box(100, 100, 100, 50) : null;
  record("claim_of", [below(5), from, ...flatBoxes(made), span, rest === null ? 0 : 1, ...(rest === null ? [0, 0, 0, 0] : flatBox(rest))], () => flatClaim(C.claimOf("o1", from, made, span, rest)));
}

/** 🃏️ A claim with arbitrary slices (not necessarily cut by `claimOf`). */
function rawClaim(owners: number): C.Claim {
  const slices: C.Slice[] = [];
  let tick = below(30) - 5;
  for (let slice = below(5); slice > 0; slice--) {
    const until = tick + below(6);
    slices.push({ from: tick, until, extent: box(100, 100, 80, 60) });
    tick = until + 1;
  }
  return { owner: `o${below(owners)}`, slices, rest: chance(0.6) ? box(100, 100, 80, 60) : null };
}

for (let index = 0; index < 800; index++) {
  const claim = rawClaim(3);
  const tick = below(50) - 10;
  record("slice_at", [...flatClaim(claim), tick], () => {
    const found = C.sliceAt(claim, tick);
    return found === null ? [0, 0, 0, 0, 0] : [1, ...flatBox(found)];
  });
}

for (let index = 0; index < 1500; index++) {
  const owners = 1 + below(5);
  const bodies: C.Body[] = [];
  const claims: C.Claim[] = [];
  for (let other = 1; other <= owners; other++) {
    const x = below(60) * 4;
    const y = below(40) * 4;
    if (below(4) > 0) bodies.push({ owner: `o${other}`, extent: { x0: x, y0: y, x1: x + 40, y1: y + 50 } });
    if (below(2) > 0) {
      const made = path(x, y, below(9) - 4, below(9) - 4, 1 + below(30));
      claims.push(C.claimOf(`o${other}`, below(20), made, 1 + below(4), below(3) > 0 ? made[made.length - 1]! : null));
    }
    if (chance(0.1)) claims.push(rawClaim(owners + 1));
  }
  if (chance(0.2)) bodies.push({ owner: "o0", extent: box(100, 100, 100, 50) });
  const made = path(below(60) * 4, below(40) * 4, below(9) - 4, below(9) - 4, 1 + below(30));
  const plan = chance(0.9) ? C.claimOf("o0", below(20), made, 1 + below(4), below(3) > 0 ? made[made.length - 1]! : null) : rawClaim(1);
  record("claim_clear", [...flatClaim(plan), ...flatBodies(bodies), ...flatClaims(claims)], () => [C.claimClear(plan, bodies, claims) ? 1 : 0]);
  if (index % 3 === 0) {
    const owner = below(owners + 1);
    const tick = below(40);
    record("released", [...flatClaims(claims), owner], () => flatClaims(C.released(claims, `o${owner}`)));
    record("pruned", [...flatClaims(claims), tick], () => flatClaims(C.pruned(claims, tick)));
  }
}

for (let index = 0; index < 1500; index++) {
  const x = coordinate(150, 200);
  const extent = { x0: x - 20, y0: 250, x1: x + 20, y1: 300 };
  const stride = odd(0.04, () => (chance(0.1) ? pick([0, -0]) : coordinate(-20, 20)));
  const obstacles = Array.from({ length: below(7) }, () => {
    const x0 = coordinate(100, 260);
    const y0 = coordinate(220, 310);
    return { x0, y0, x1: x0 + coordinate(1, 40), y1: y0 + coordinate(1, 60) };
  });
  record("guarded_stride", [...flatBox(extent), stride, ...flatBoxes(obstacles)], () => [C.guardedStride(extent, stride, obstacles)]);
}

for (let index = 0; index < 600; index++) {
  const hopper = box(100, 250, 20, 60);
  const hurdle = box(140, 250, 20, 60);
  const rise = odd(0.04, () => coordinate(0, 100));
  record("vaults", [...flatBox(hopper), rise, ...flatBox(hurdle)], () => [C.vaults(hopper, rise, hurdle) ? 1 : 0]);
}

for (let index = 0; index < 500; index++) {
  const bodies = crowd(below(9), 200, 0, 40, 9);
  record("order_of", flatBodies(bodies), () => flatOwners(C.orderOf(bodies)));
  const before = Array.from({ length: below(7) }, () => `o${below(6)}`);
  const after = Array.from({ length: below(7) }, () => `o${below(6)}`);
  record("order_kept", [...flatOwners(before), ...flatOwners(after)], () => [C.orderKept(before, after) ? 1 : 0]);
}

for (let index = 0; index < 1500; index++) {
  const count = below(7);
  const bodies: C.Body[] = [];
  for (let member = 0; member < count; member++) {
    const x0 = index % 2 === 0 ? 80 + below(1000) / 4 : free(60, 380);
    bodies.push({ owner: `o${member}`, extent: { x0, y0: 248, x1: x0 + (index % 2 === 0 ? 30 + below(30) : free(20, 70)), y1: 304 } });
  }
  const x0 = odd(0.02, () => coordinate(60, 200));
  const perch = { surface: "s1", x0, x1: x0 + odd(0.02, () => coordinate(20, 360)), y: 300 };
  const margin = pick([4, 0, 2, 4.5]);
  record("seat_of", [...flatBodies(bodies), ...flatPerch(perch), margin], () => {
    const seating = C.seatOf(bodies, perch, margin);
    return [seating.seats.length, ...seating.seats.flatMap((seat) => [Number(seat.owner.slice(1)), seat.shift]), ...flatOwners(seating.leavers)];
  });
}

for (let index = 0; index < 800; index++) {
  const shifts = Array.from({ length: below(6) }, () => odd(0.04, () => (chance(0.2) ? pick([0, -0]) : coordinate(-30, 30))));
  const stride = odd(0.04, () => coordinate(-2, 8));
  record("scoot_fraction", [shifts.length, ...shifts, stride], () => [C.scootFraction(shifts, stride)]);
  const args = [odd(0.04, () => coordinate(0, 400)), odd(0.04, () => coordinate(0, 400)), odd(0.04, () => (chance(0.2) ? pick([0, 1, 1.5, -0]) : free(0, 1)))];
  record("scooted", args, () => [C.scooted(args[0]!, args[1]!, args[2]!)]);
}

for (let index = 0; index < 1500; index++) {
  const bodies = crowd(below(6), 120, 250, 60, 4);
  const x = coordinate(60, 180);
  const top = coordinate(140, 260);
  const before = { x0: x, y0: top - 50, x1: x + coordinate(20, 60), y1: top };
  const fall = odd(0.03, () => (chance(0.2) ? pick([0, 3, 4, 4.25, -2]) : coordinate(-10, 40)));
  const after = { x0: before.x0 + coordinate(-3, 3), y0: before.y0 + fall, x1: before.x1 + coordinate(-3, 3), y1: before.y1 + fall };
  const owner = below(5);
  record("head_under", [...flatBox(before), ...flatBox(after), owner, ...flatBodies(bodies)], () => {
    const host = C.headUnder(before, after, `o${owner}`, bodies);
    return [host === null ? -1 : bodies.indexOf(host)];
  });
  const host = box(120, 250, 40, 60);
  const rider = { x0: coordinate(60, 180), y0: 0, x1: 0, y1: host.y0 - pick([0, C.SEAM, 2 * C.SEAM, 3 * C.SEAM, -C.SEAM, 1, 0.5]) };
  rider.x1 = rider.x0 + coordinate(10, 60);
  rider.y0 = rider.y1 - 40;
  record("lift_onto", [...flatBox(after), ...flatBox(host)], () => [C.liftOnto(after, host)]);
  record("rests_on", [...flatBox(rider), ...flatBox(host)], () => [C.restsOn(rider, host) ? 1 : 0]);
  record("slide_side", [...flatBox(rider), ...flatBox(host)], () => [C.slideSide(rider, host)]);
  const side = pick([1, -1]);
  const ticks = pick([0, 1, 2, 10, 47, 48, 49, 100, 5000, below(200)]);
  const low = coordinate(-10, 200);
  const high = low + coordinate(0, 400);
  const obstacles = Array.from({ length: below(5) }, () => box(rider.x0, rider.y0, 40, 50));
  record("slide_of", [...flatBox(rider), side, ticks, low, high, ...flatBoxes(obstacles)], () => {
    const slide = C.slideOf(rider, side, ticks, low, high, obstacles);
    return [slide.stride, slide.side];
  });
}

for (let index = 0; index < 400; index++) {
  const ticks = pick([0, 1, 10, 47, 48, 49, 1000, -1, -16, -100, 2 ** 40, below(300)]);
  record("slide_stride", [ticks], () => [C.slideStride(ticks)]);
}

for (let index = 0; index < 1500; index++) {
  const extent = { x0: 100, y0: 250, x1: 140, y1: 300 };
  const own = index % 3 === 0 ? box(120, 270, 30, 60) : extent;
  const obstacles = Array.from({ length: below(6) }, () => {
    const x0 = coordinate(-50, 250);
    const y0 = coordinate(150, 350);
    return { x0, y0, x1: x0 + coordinate(4, 64), y1: y0 + coordinate(4, 64) };
  });
  const iterations = pick([0, 1, 2, 4, 4, 6, 40]);
  record("pushed_out", [...flatBox(own), ...flatBoxes(obstacles), iterations], () => {
    const push = C.pushedOut(own, obstacles, iterations);
    return push === null ? [0, 0, 0] : [1, push.x, push.y];
  });
}

for (let index = 0; index < 800; index++) {
  const bodies = crowd(1 + below(5), 100, 50, 80, 5);
  const claims = Array.from({ length: below(4) }, () => claimFor(5));
  const plans = Array.from({ length: below(4) }, () => {
    const made = path(coordinate(0, 200), coordinate(0, 100), grid(-8, 8, 2), grid(-8, 8, 2), 1 + below(20));
    return C.claimOf("o0", below(10), made, 1 + below(3), chance(0.7) ? made[made.length - 1]! : null);
  });
  const stay = chance(0.7) ? box(80, 50, 80, 50) : null;
  const tick = below(40);
  record("must_poof", [0, stay === null ? 0 : 1, ...(stay === null ? [0, 0, 0, 0] : flatBox(stay)), ...flatClaims(plans), ...flatBodies(bodies), ...flatClaims(claims), tick], () => [C.mustPoof("o0", stay, plans, bodies, claims, tick) ? 1 : 0]);
  const crowded = crowd(below(8), 150, 50, 100);
  const corridors = Array.from({ length: below(4) }, () => claimFor(8));
  record("evicted", [...flatBodies(crowded), ...flatClaims(corridors), tick], () => flatOwners(C.evicted(crowded, corridors, tick)));
}

for (let index = 0; index < 300; index++) {
  const height = odd(0.05, () => coordinate(0, 80));
  const margin = odd(0.05, () => pick([0, 4, 2, 4.25]));
  record("lane_lift", [height, margin], () => [C.laneLift(height, margin)]);
  const count = below(1000);
  const actors = pick([0, 1, 7, below(10000000), 2 ** 33]);
  record("per_million", [count, actors], () => [C.perMillion(count, actors)]);
}

for (let index = 0; index < 400; index++) {
  const tally = { ticks: below(100000), actors: below(1000000), overlaps: below(10), nears: below(100000), poofs: below(100), waits: below(100000) };
  const bodies = crowd(below(9), 200, 100, 60);
  const margin = pick([2, 0, 1, 4]);
  const poofs = below(3);
  const waits = below(9);
  record("tallied", [tally.ticks, tally.actors, tally.overlaps, tally.nears, tally.poofs, tally.waits, ...flatBodies(bodies), margin, poofs, waits], () => {
    const next = C.tallied(tally, bodies, margin, poofs, waits);
    return [next.ticks, next.actors, next.overlaps, next.nears, next.poofs, next.waits];
  });
}
//#endregion 🔖️Clearance

//#region 🔖️Gesture encoders
/** 🤏️ A press flattened: phase index, `x, y, since, slop`. */
function flatPress(press: Press): number[] {
  return [PRESS_PHASES.indexOf(press.phase), press.x, press.y, press.since, press.slop];
}

/** 🛡️ Guards flattened as four flags. */
function flatGuards(guards: G.Guards): number[] {
  return [guards.control ? 1 : 0, guards.scrolled ? 1 : 0, guards.quiet ? 1 : 0, guards.still ? 1 : 0];
}

/** 🔥️ A warmth flattened: `heat, since, until`, tier index, `run, tricks`. */
function flatWarmth(warmth: Warmth): number[] {
  return [warmth.heat, warmth.since, warmth.until, TIERS.indexOf(warmth.tier), warmth.run, warmth.tricks];
}

/** 🌀️ A circling state flattened in the order of its fields. */
function flatCircling(circling: Circling): number[] {
  return [circling.live ? 1 : 0, circling.inside, circling.sx, circling.sy, circling.px, circling.py, circling.turn, circling.quarters, circling.steps, circling.against, circling.first, circling.last, circling.open, circling.near, circling.far, circling.rest];
}

/** 🖐️ A stroking state flattened in the order of its fields. */
function flatStroking(stroking: Stroking): number[] {
  return [stroking.live ? 1 : 0, stroking.way, stroking.from, stroking.began, stroking.peak, stroking.reached, stroking.top, stroking.bottom, stroking.topSince, stroking.bottomSince, stroking.count, stroking.first, stroking.second, stroking.rest];
}

/** 🫨️ A shaking state flattened in the order of its fields. */
function flatShaking(shaking: Shaking): number[] {
  return [shaking.live ? 1 : 0, shaking.ax, shaking.ay, shaking.at, shaking.fx, shaking.fy, shaking.reached, shaking.count, shaking.mark1, shaking.mark2, shaking.mark3, shaking.rest];
}

/** 🪶️ A hover state flattened: circling, then stroking. */
function flatHover(hover: Hover): number[] {
  return [...flatCircling(hover.circling), ...flatStroking(hover.stroking)];
}

/** 🔔️ A cue as its index in `CUES`, −1 for none. */
function cueIndex(cue: string | null): number {
  return cue === null ? -1 : CUES.indexOf(cue as (typeof CUES)[number]);
}

const SIGNALS = ["click", "hold", "unhold", "lift", "drop", "abort"] as const;

/** 📣️ A press signal as its index, −1 for none. */
function signalIndex(signal: string | null): number {
  return signal === null ? -1 : SIGNALS.indexOf(signal as (typeof SIGNALS)[number]);
}

/** 🚧️ Guards now and then with one or more up. */
function someGuards(share: number): G.Guards {
  if (!chance(share)) return G.UNGUARDED;
  return { control: chance(0.3), scrolled: chance(0.3), quiet: chance(0.3), still: chance(0.2) };
}

/** ⬛️ A body rect of a pet. */
function petBody(): { x: number; y: number; width: number; height: number } {
  return { x: coordinate(300, 500), y: coordinate(200, 300), width: coordinate(20, 80), height: coordinate(20, 80) };
}

/** 🌪️ A fuzzed circling state with plausible counters. */
function fuzzCircling(): Circling {
  return { live: chance(0.7), inside: below(400), sx: pick([-1, 0, 1]), sy: pick([-1, 0, 1]), px: coordinate(-200, 200), py: coordinate(-200, 200), turn: pick([-1, 0, 1]), quarters: below(6) - 1, steps: below(8), against: below(3), first: below(400), last: below(400), open: coordinate(0, 40000), near: coordinate(0, 40000), far: coordinate(0, 80000), rest: pick([0, 0, 0, below(500)]) };
}

/** 🧹️ A fuzzed stroking state with plausible counters. */
function fuzzStroking(): Stroking {
  return { live: chance(0.7), way: pick([-1, 0, 1]), from: coordinate(-60, 60), began: below(400), peak: coordinate(-60, 60), reached: below(400), top: coordinate(-40, 40), bottom: coordinate(-40, 40), topSince: coordinate(-40, 40), bottomSince: coordinate(-40, 40), count: below(5) - 1, first: below(400), second: below(400), rest: pick([0, 0, 0, below(500)]) };
}

/** 🥁️ A fuzzed shaking state with plausible counters. */
function fuzzShaking(): Shaking {
  return { live: chance(0.7), ax: coordinate(200, 600), ay: coordinate(200, 600), at: below(400), fx: coordinate(200, 600), fy: coordinate(200, 600), reached: below(400), count: below(6), mark1: below(400), mark2: below(400), mark3: below(400), rest: pick([0, 0, 0, below(500)]) };
}

/** 〰️ A pointer path past a body: a drawn circle (any direction, radius, speed, wobble), a petting, a shake, a random walk or a straight pass, sampled and held like a device of 30 to 144 Hz. */
function pointerPath(body: { x: number; y: number; width: number; height: number }, ticks: number): { x: number; y: number }[] {
  const cx = body.x + body.width / 2 + coordinate(-10, 10);
  const cy = body.y + body.height / 2 + coordinate(-10, 10);
  const kind = below(5);
  const reach = Math.max(body.width, body.height);
  const radius = free(0.4, 2.8) * reach;
  const turns = free(0.3, 4) * pick([1, -1]);
  const start = unit();
  const swing = free(0.2, 0.8) * body.width;
  const hertz = free(0.5, 5);
  const every = pick([1, 1, 2, 3, 4]);
  const whole = chance(0.5);
  const points: { x: number; y: number }[] = [];
  let walkX = cx;
  let walkY = cy;
  for (let tick = 0; tick < ticks; tick++) {
    let x: number;
    let y: number;
    if (kind === 0) {
      const angle = 2 * Math.PI * (start + (turns * tick) / 64);
      const wobble = 1 + 0.1 * Math.sin(tick / 7);
      x = cx + radius * wobble * Math.cos(angle);
      y = cy + radius * wobble * Math.sin(angle);
    } else if (kind === 1 || kind === 2) {
      const wave = Math.sin(2 * Math.PI * (start + (hertz * tick) / 64));
      x = cx + (kind === 1 ? swing : swing * 2) * wave;
      y = cy + (kind === 1 ? 2 : swing) * wave * free(0.9, 1.1);
    } else if (kind === 3) {
      walkX += free(-6, 6);
      walkY += free(-6, 6);
      x = walkX;
      y = walkY;
    } else {
      x = cx - 300 + (600 * tick) / ticks;
      y = cy + radius / 3;
    }
    if (whole) {
      x = Math.round(x);
      y = Math.round(y);
    } else {
      x = Math.round(x * 4) / 4;
      y = Math.round(y * 4) / 4;
    }
    points.push(tick % every === 0 || points.length === 0 ? { x, y } : points[points.length - 1]!);
  }
  return points;
}
//#endregion 🔖️Gesture encoders

//#region 🔖️Gesture
record("gesture_constants", [], () => [
  G.SLOP_FINE, G.SLOP_COARSE, G.HOLD_TICKS, G.HEAT_CLICK, G.HEAT_HOLD, G.HEAT_LEAK, G.HEAT_HELLO, G.HEAT_TRICK, G.HEAT_ENOUGH, G.HEAT_FORGIVEN, G.ENOUGH_TICKS, G.SCROLL_TICKS,
  G.CIRCLE_MARGIN, G.CIRCLE_REACH, G.CIRCLE_HYSTERESIS, G.CIRCLE_QUARTERS, G.CIRCLE_FAST, G.CIRCLE_SLOW, G.CIRCLE_ROUND, G.CIRCLE_CLOSE, G.CIRCLE_AGAINST, G.CIRCLE_OUT_TICKS, G.CIRCLE_REST,
  G.STROKE_MARGIN, G.STROKE_HYSTERESIS, G.STROKE_LENGTH, G.STROKE_SLOW, G.STROKE_FAST, G.STROKE_SLANT, G.STROKE_SEGMENTS, G.STROKE_WINDOW, G.STROKE_PAUSE,
  G.SHAKE_AMPLITUDE, G.SHAKE_HYSTERESIS, G.SHAKE_SPEED, G.SHAKE_REVERSALS, G.SHAKE_WINDOW, G.SHAKE_PAUSE, G.SHAKE_REST,
  ...flatPress(G.IDLE), ...flatWarmth(G.COLD), ...flatGuards(G.UNGUARDED),
]);

for (const pointer of POINTERS) record("slop_of", [POINTERS.indexOf(pointer)], () => [G.slopOf(pointer)]);

for (let history = 0; history < 400; history++) {
  let press: Press = chance(0.8) ? G.IDLE : { phase: pick(PRESS_PHASES), x: coordinate(0, 400), y: coordinate(0, 400), since: below(100), slop: pick([6, 10]) };
  let tick = below(100);
  let x = coordinate(100, 300);
  let y = coordinate(100, 300);
  for (let step = 0; step < 12; step++) {
    tick += pick([0, 0, 1, 1, 2, 5, 27, 28, 29, 40]);
    const kind = pick([0, 1, 1, 1, 2, 3, 4, 4, 4]);
    x += pick([0, 0, 0.25, 1, 3, 5.75, 6, 9.75, 10, 20]) * pick([1, -1]);
    y += pick([0, 0, 0.25, 1, 3, 6, 10]) * pick([1, -1]);
    const pointer = pick(POINTERS);
    const guards = someGuards(0.15);
    const input: G.PressInput = kind === 0 ? { kind: "pressed", x, y, pointer } : kind === 1 ? { kind: "dragged", x, y } : kind === 2 ? { kind: "released", x, y } : kind === 3 ? { kind: "cancelled" } : { kind: "ticked" };
    const before = press;
    const stepTick = tick;
    record("press_step", [...flatPress(before), kind, x, y, POINTERS.indexOf(pointer), stepTick, ...flatGuards(guards)], () => {
      const result = G.pressStep(before, input, stepTick, guards);
      press = result.state;
      return [...flatPress(result.state), signalIndex(result.signal)];
    });
  }
}

for (let index = 0; index < 500; index++) {
  const press: Press = { phase: pick(PRESS_PHASES), x: coordinate(0, 400), y: coordinate(0, 400), since: below(100000) - 50, slop: pick([6, 10]) };
  record("press_due", flatPress(press), () => {
    const due = G.pressDue(press);
    return due === null ? [0, 0] : [1, due];
  });
}

for (let index = 0; index < 1500; index++) {
  const heat = odd(0.04, () => (chance(0.3) ? grid(0, 12, 32) : free(0, 12)));
  const since = below(5000) - 100;
  const tick = chance(0.2) ? since - below(50) : since + below(2000);
  const caress = chance(0.4) ? "hold" : "click";
  record("heat_at", [heat, since, tick], () => [G.heatAt(heat, since, tick)]);
  record("heat_after", [heat, since, tick, caress === "hold" ? 1 : 0], () => [G.heatAfter(heat, since, tick, caress)]);
  const level = odd(0.04, () => (chance(0.4) ? pick([0, 1, 3, 7, 1.0078125, 3.0078125, 6.9921875, -0, 7.000000000000001, 0.9999999999999999]) : free(-1, 12)));
  record("tier_of", [level], () => [TIERS.indexOf(G.tierOf(level))]);
}

for (let history = 0; history < 120; history++) {
  let warmth: Warmth = chance(0.8) ? G.COLD : { heat: free(0, 8), since: below(500), until: pick([0, below(1000)]), tier: pick(TIERS), run: below(5), tricks: below(5) };
  let tick = below(100);
  for (let caress = 0; caress < 40; caress++) {
    tick += pick([0, 1, 1, 1, 8, 16, 32, 64, 200, 511, 512, 513]);
    const kind = chance(0.35) ? "hold" : "click";
    const before = warmth;
    const at = tick;
    record("warmth_after", [...flatWarmth(before), at, kind === "hold" ? 1 : 0], () => {
      warmth = G.warmthAfter(before, at, kind);
      return flatWarmth(warmth);
    });
  }
}

for (let index = 0; index < 50; index++) {
  const rest = pick([0, 1, 128, below(100000), -16]);
  record("no_circling", [rest], () => flatCircling(G.noCircling(rest)));
  record("no_stroking", [rest], () => flatStroking(G.noStroking(rest)));
  record("no_shaking", [rest], () => flatShaking(G.noShaking(rest)));
  record("no_hover", [rest], () => flatHover(G.noHover(rest)));
}

for (let trace = 0; trace < 150; trace++) {
  const body = petBody();
  const points = pointerPath(body, 40 + below(120));
  let circling = chance(0.9) ? G.noCircling(0) : fuzzCircling();
  let stroking = chance(0.9) ? G.noStroking(0) : fuzzStroking();
  let hover = G.noHover(0);
  let tick = below(100);
  points.forEach((point) => {
    tick += chance(0.05) ? pick([2, 9, 41, 49]) : 1;
    const guards = someGuards(0.04);
    const at = tick;
    const circled = circling;
    record("circle_step", [...flatCircling(circled), point.x, point.y, body.x, body.y, body.width, body.height, at, ...flatGuards(guards)], () => {
      const step = G.circleStep(circled, point, body, at, guards);
      circling = step.state;
      return [...flatCircling(step.state), cueIndex(step.cue)];
    });
    const stroked = stroking;
    record("stroke_step", [...flatStroking(stroked), point.x, point.y, body.x, body.y, body.width, body.height, at, ...flatGuards(guards)], () => {
      const step = G.strokeStep(stroked, point, body, at, guards);
      stroking = step.state;
      return [...flatStroking(step.state), cueIndex(step.cue)];
    });
    if (trace % 3 === 0) {
      const hovered = hover;
      record("hover_step", [...flatHover(hovered), point.x, point.y, body.x, body.y, body.width, body.height, at, ...flatGuards(guards)], () => {
        const step = G.hoverStep(hovered, point, body, at, guards);
        hover = step.state;
        return [...flatHover(step.state), cueIndex(step.cue)];
      });
      record("hover_busy", flatHover(hovered), () => [G.hoverBusy(hovered) ? 1 : 0]);
    }
  });
}

for (let index = 0; index < 1000; index++) {
  const body = petBody();
  const point = { x: odd(0.02, () => coordinate(200, 700)), y: odd(0.02, () => coordinate(100, 500)) };
  const tick = below(500);
  const guards = someGuards(0.1);
  const circling = fuzzCircling();
  record("circle_step", [...flatCircling(circling), point.x, point.y, body.x, body.y, body.width, body.height, tick, ...flatGuards(guards)], () => {
    const step = G.circleStep(circling, point, body, tick, guards);
    return [...flatCircling(step.state), cueIndex(step.cue)];
  });
  const stroking = fuzzStroking();
  record("stroke_step", [...flatStroking(stroking), point.x, point.y, body.x, body.y, body.width, body.height, tick, ...flatGuards(guards)], () => {
    const step = G.strokeStep(stroking, point, body, tick, guards);
    return [...flatStroking(step.state), cueIndex(step.cue)];
  });
  const shaking = fuzzShaking();
  const height = coordinate(20, 80);
  record("shake_step", [...flatShaking(shaking), point.x, point.y, height, tick, ...flatGuards(guards)], () => {
    const step = G.shakeStep(shaking, point, height, tick, guards);
    return [...flatShaking(step.state), cueIndex(step.cue)];
  });
}

for (let trace = 0; trace < 120; trace++) {
  const body = petBody();
  const height = body.height;
  const points = pointerPath(body, 40 + below(100));
  let shaking = chance(0.9) ? G.noShaking(0) : fuzzShaking();
  let tick = below(100);
  points.forEach((point) => {
    tick += chance(0.05) ? pick([2, 49, 65]) : 1;
    const guards = someGuards(0.03);
    const at = tick;
    const before = shaking;
    record("shake_step", [...flatShaking(before), point.x, point.y, height, at, ...flatGuards(guards)], () => {
      const step = G.shakeStep(before, point, height, at, guards);
      shaking = step.state;
      return [...flatShaking(step.state), cueIndex(step.cue)];
    });
  });
}
//#endregion 🔖️Gesture

//#region 🔖️Edges
/** ⏭️ The neighbouring double above (`step` 1) or below (`step` −1) a positive number. */
function neighbour(value: number, step: number): number {
  view.setFloat64(0, value);
  view.setBigUint64(0, view.getBigUint64(0) + BigInt(step));
  return view.getFloat64(0);
}

/** 🎯️ A body centred on the origin, so that every offset from its centre is the pointer itself, exactly. */
function centred(width: number, height: number): { x: number; y: number; width: number; height: number } {
  return { x: 0 - width / 2, y: 0 - height / 2, width, height };
}

for (const x of [0, -0]) {
  for (const low of [0, -0, -1]) {
    for (const high of [0, -0, 5]) {
      const extent = { x0: x - 10, y0: 0, x1: x + 10, y1: 10 };
      record("slot_in", [x, ...flatBox(extent), low, high, 0], () => flatMaybe(C.slotIn(x, extent, low, high, [])));
    }
  }
}

for (let index = 0; index < 300; index++) {
  const extent = { x0: 100, y0: 100, x1: 140, y1: 150 };
  const a = grid(1, 19, 4);
  const b = pick([a + 5, a - 5, grid(1, 24, 4)]);
  const obstacles = [{ x0: 120 - a, y0: 125 - b, x1: 120 + a, y1: 125 + b }];
  if (chance(0.3)) obstacles.push({ x0: 120 - b, y0: 125 - a, x1: 120 + b, y1: 125 + a });
  const iterations = pick([1, 4]);
  record("pushed_out", [...flatBox(extent), ...flatBoxes(obstacles), iterations], () => {
    const push = C.pushedOut(extent, obstacles, iterations);
    return push === null ? [0, 0, 0] : [1, push.x, push.y];
  });
}

for (let index = 0; index < 600; index++) {
  const width = free(20, 80);
  const body = centred(width, free(10, width));
  const reach = Math.max(body.width, body.height);
  const edge = index % 2 === 0 ? reach / 2 + G.CIRCLE_MARGIN : reach * G.CIRCLE_REACH;
  const x = pick([edge, neighbour(edge, 1), neighbour(edge, -1)]);
  const circling = chance(0.5) ? G.noCircling(0) : { ...fuzzCircling(), live: false };
  const tick = 1000;
  record("circle_step", [...flatCircling(circling), x, 0, body.x, body.y, body.width, body.height, tick, ...flatGuards(G.UNGUARDED)], () => {
    const step = G.circleStep(circling, { x, y: 0 }, body, tick, G.UNGUARDED);
    return [...flatCircling(step.state), cueIndex(step.cue)];
  });
}

for (let index = 0; index < 600; index++) {
  const body = centred(48, 48);
  const px = free(20, 150) * pick([1, -1]);
  const py = free(20, 150) * pick([1, -1]);
  const factor = free(0.5, 1.2);
  const point = { x: 0 - px * factor, y: 0 - py * factor };
  const tick = 500;
  const circling: Circling = { live: true, inside: tick - 1, sx: px > 0 ? 1 : -1, sy: py > 0 ? 1 : -1, px, py, turn: pick([1, -1, 0]), quarters: below(4), steps: 1 + below(4), against: below(2), first: tick - 60, last: tick - 10, open: px * px + py * py, near: 1000, far: 30000, rest: 0 };
  record("circle_step", [...flatCircling(circling), point.x, point.y, body.x, body.y, body.width, body.height, tick, ...flatGuards(G.UNGUARDED)], () => {
    const step = G.circleStep(circling, point, body, tick, G.UNGUARDED);
    return [...flatCircling(step.state), cueIndex(step.cue)];
  });
}

for (let index = 0; index < 800; index++) {
  const body = centred(48, 48);
  const rx = free(25, 100);
  const ry = free(25, 100);
  const radius = rx * rx + ry * ry;
  const near = radius * free(0.2, 0.99);
  const far = pick([G.CIRCLE_ROUND * G.CIRCLE_ROUND * near, G.CIRCLE_ROUND * (G.CIRCLE_ROUND * near), neighbour(G.CIRCLE_ROUND * G.CIRCLE_ROUND * near, 1), neighbour(G.CIRCLE_ROUND * G.CIRCLE_ROUND * near, -1)]);
  const open = pick([radius, radius / (G.CIRCLE_CLOSE * G.CIRCLE_CLOSE), (G.CIRCLE_CLOSE * G.CIRCLE_CLOSE) * radius, radius * free(0.6, 1.6)]);
  const tick = 800;
  const circling: Circling = { live: true, inside: tick - 1, sx: 1, sy: -1, px: rx, py: -ry, turn: 1, quarters: 4, steps: 4, against: 0, first: tick - pick([12, 16, 40]), last: tick - 5, open, near, far: Math.max(far, radius), rest: 0 };
  record("circle_step", [...flatCircling(circling), rx, ry, body.x, body.y, body.width, body.height, tick, ...flatGuards(G.UNGUARDED)], () => {
    const step = G.circleStep(circling, { x: rx, y: ry }, body, tick, G.UNGUARDED);
    return [...flatCircling(step.state), cueIndex(step.cue)];
  });
}

for (let index = 0; index < 1600; index++) {
  const fast = index % 2 === 1;
  const ticks = 3 + below(60);
  const speed = fast ? G.STROKE_FAST : G.STROKE_SLOW;
  const exact = (speed * ticks) / 64;
  const length = pick([exact, neighbour(exact, 1), neighbour(exact, -1)]);
  const body = centred(2 * length, 40);
  const reached = 300;
  const stroking: Stroking = { live: true, way: 1, from: 0 - length / 2, began: reached - ticks, peak: length / 2, reached, top: 0, bottom: 0, topSince: 0, bottomSince: 0, count: pick([-1, 0, 1, 2]), first: reached - 40, second: reached - 20, rest: 0 };
  const rx = length / 2 - (0.31 * length + 1);
  record("stroke_step", [...flatStroking(stroking), rx, 0, body.x, body.y, body.width, body.height, reached + 1, ...flatGuards(G.UNGUARDED)], () => {
    const step = G.strokeStep(stroking, { x: rx, y: 0 }, body, reached + 1, G.UNGUARDED);
    return [...flatStroking(step.state), cueIndex(step.cue)];
  });
}

for (let index = 0; index < 600; index++) {
  const body = centred(free(10, 80), 40);
  const hysteresis = G.STROKE_HYSTERESIS * body.width;
  const turning = index % 2 === 1;
  const peak = free(-5, 5);
  const rx = turning ? pick([peak - hysteresis, neighbour(Math.abs(peak - hysteresis), 1) * Math.sign(peak - hysteresis)]) : pick([hysteresis, neighbour(hysteresis, 1), neighbour(hysteresis, -1)]) * pick([1, -1]);
  const tick = 200;
  const stroking: Stroking = turning
    ? { live: true, way: 1, from: peak - 20, began: tick - 30, peak, reached: tick - 3, top: 0, bottom: 0, topSince: 0, bottomSince: 0, count: 0, first: 0, second: 0, rest: 0 }
    : { live: true, way: 0, from: 0, began: tick - 3, peak: 0, reached: tick - 3, top: 0, bottom: 0, topSince: 0, bottomSince: 0, count: -1, first: 0, second: 0, rest: 0 };
  record("stroke_step", [...flatStroking(stroking), rx, 0, body.x, body.y, body.width, body.height, tick, ...flatGuards(G.UNGUARDED)], () => {
    const step = G.strokeStep(stroking, { x: rx, y: 0 }, body, tick, G.UNGUARDED);
    return [...flatStroking(step.state), cueIndex(step.cue)];
  });
}

for (let index = 0; index < 200; index++) {
  const body = centred(40, 40);
  const y = pick([0, -0]);
  const tick = 100;
  const stroking: Stroking = { live: true, way: pick([0, 1, -1]), from: 0, began: tick - 5, peak: pick([0, 5, -5]), reached: tick - 2, top: pick([0, -0]), bottom: pick([0, -0]), topSince: pick([0, -0]), bottomSince: pick([0, -0]), count: 0, first: 0, second: 0, rest: 0 };
  const x = pick([0, -0, 0.5, -0.5]);
  record("stroke_step", [...flatStroking(stroking), x, y, body.x, body.y, body.width, body.height, tick, ...flatGuards(G.UNGUARDED)], () => {
    const step = G.strokeStep(stroking, { x, y }, body, tick, G.UNGUARDED);
    return [...flatStroking(step.state), cueIndex(step.cue)];
  });
}

for (let index = 0; index < 600; index++) {
  const slop = pick([G.SLOP_FINE, G.SLOP_COARSE]);
  const angle = 2 * Math.PI * unit();
  const x = slop * Math.cos(angle);
  const y = slop * Math.sin(angle);
  const press: Press = { phase: pick(["armed", "holding"] as const), x: 0, y: 0, since: 0, slop };
  record("press_step", [...flatPress(press), 1, x, y, 0, 3, ...flatGuards(G.UNGUARDED)], () => {
    const step = G.pressStep(press, { kind: "dragged", x, y }, 3, G.UNGUARDED);
    return [...flatPress(step.state), signalIndex(step.signal)];
  });
}
//#endregion 🔖️Edges

//#region 🔖️Corpus
type Trail = { readonly xs: readonly number[]; readonly ys: readonly number[] };
type Document = { quantum: number; hold: number; page: [number, number]; variants: number; presses: { id: string; ticks: number; events: { at: number; kind: string; x?: number; y?: number; pointer?: string; guards?: string[] }[] }[]; warmths: { id: string; caresses: [number, string][] }[]; circles: Trace[]; strokes: Trace[]; shakes: Trace[]; bodies: [number, number, number, number][]; travels: { id: string; path: number[] }[] };
type Trace = { id: string; body?: [number, number, number, number]; height?: number; path: number[]; guard?: keyof G.Guards };

/** 🧵️ The points of a path, one per tick, in quanta (the subject adapters' decoding). */
function trailOf(path: readonly number[], hold: number): Trail {
  let x = path[0] ?? 0;
  let y = path[1] ?? 0;
  const xs = [x];
  const ys = [y];
  for (let index = 2; index < path.length; ) {
    const token = path[index]!;
    if (token >= hold) {
      for (let repeat = 0; repeat < token - hold; repeat++) {
        xs.push(x);
        ys.push(y);
      }
      index += 1;
    } else {
      x += token;
      y += path[index + 1]!;
      xs.push(x);
      ys.push(y);
      index += 2;
    }
  }
  return { xs, ys };
}

/** 🪞️ A trail in one of its sixteen variants (the subject adapters' variants). */
function variantOf(trail: Trail, page: readonly [number, number], variant: number): Trail {
  const mirrored = (variant & 1) === 0 ? trail.xs : trail.xs.map((x) => page[0] - x);
  const flipped = (variant & 2) === 0 ? trail.ys : trail.ys.map((y) => page[1] - y);
  const xs = (variant & 4) === 0 ? mirrored : flipped;
  const ys = (variant & 4) === 0 ? flipped : mirrored;
  return (variant & 8) === 0 ? { xs, ys } : { xs: [...xs].reverse(), ys: [...ys].reverse() };
}

/** 🖼️ A body in the same variant. */
function boxVariant(body: readonly number[], page: readonly [number, number], variant: number): number[] {
  const x = (variant & 1) === 0 ? body[0]! : page[0] - body[0]! - body[2]!;
  const y = (variant & 2) === 0 ? body[1]! : page[1] - body[1]! - body[3]!;
  return (variant & 4) === 0 ? [x, y, body[2]!, body[3]!] : [y, x, body[3]!, body[2]!];
}

/** 🎛️ The guards with one of them up, or none. */
function guardOf(guard: keyof G.Guards | undefined): G.Guards {
  return guard === undefined ? G.UNGUARDED : { ...G.UNGUARDED, [guard]: true };
}

/** 🎠️ The cues and the state digest of the hover gestures along a trail past one body. */
function hoverRun(trail: Trail, quantum: number, body: readonly number[], guards: G.Guards, digested: boolean): { cues: number[][]; digest: string } {
  const rect = { x: body[0]! / quantum, y: body[1]! / quantum, width: body[2]! / quantum, height: body[3]! / quantum };
  const cues: number[][] = [];
  const digest = new Digest();
  let hover = G.noHover(0);
  for (let tick = 0; tick < trail.xs.length; tick++) {
    const step = G.hoverStep(hover, { x: trail.xs[tick]! / quantum, y: trail.ys[tick]! / quantum }, rect, tick, guards);
    hover = step.state;
    if (step.cue !== null) cues.push([tick, cueIndex(step.cue)]);
    if (digested) for (const number of flatHover(hover)) digest.number(number);
  }
  return { cues, digest: digested ? digest.hex() : "" };
}

/** 🥤️ The cues and the state digest of the shake along a trail. */
function heldRun(trail: Trail, quantum: number, height: number, guards: G.Guards): { cues: number[][]; digest: string } {
  const cues: number[][] = [];
  const digest = new Digest();
  let shaking = G.noShaking(0);
  for (let tick = 0; tick < trail.xs.length; tick++) {
    const step = G.shakeStep(shaking, { x: trail.xs[tick]! / quantum, y: trail.ys[tick]! / quantum }, height / quantum, tick, guards);
    shaking = step.state;
    if (step.cue !== null) cues.push([tick, cueIndex(step.cue)]);
    for (const number of flatShaking(shaking)) digest.number(number);
  }
  return { cues, digest: digest.hex() };
}

const corpus: { kind: string; id: string; variant: number; body: number; cues: number[][]; digest: string }[] = [];
const document = JSON.parse(readFileSync(join(PETS, "🧫️fixtures/👆️gesture-recognition/🔣️.json"), "utf8")) as Document;
for (const vector of document.presses) {
  const signals: number[][] = [];
  const digest = new Digest();
  let press = G.IDLE;
  for (let tick = 1; tick <= vector.ticks; tick++) {
    const passed = G.pressStep(press, { kind: "ticked" }, tick, G.UNGUARDED);
    press = passed.state;
    if (passed.signal !== null) signals.push([tick, signalIndex(passed.signal)]);
    for (const event of vector.events) {
      if (event.at !== tick) continue;
      const x = (event.x ?? 0) / document.quantum;
      const y = (event.y ?? 0) / document.quantum;
      const input: G.PressInput = event.kind === "pressed" ? { kind: "pressed", x, y, pointer: (event.pointer ?? "mouse") as (typeof POINTERS)[number] } : event.kind === "cancelled" ? { kind: "cancelled" } : { kind: event.kind as "dragged" | "released", x, y };
      const guards = (event.guards ?? []).reduce<G.Guards>((up, guard) => ({ ...up, [guard]: true }), G.UNGUARDED);
      const step = G.pressStep(press, input, tick, guards);
      press = step.state;
      if (step.signal !== null) signals.push([tick, signalIndex(step.signal)]);
    }
    for (const number of flatPress(press)) digest.number(number);
  }
  corpus.push({ kind: "press", id: vector.id, variant: 0, body: 0, cues: signals, digest: digest.hex() });
}
for (const vector of document.warmths) {
  const digest = new Digest();
  let warmth = G.COLD;
  for (const [tick, caress] of vector.caresses) {
    warmth = G.warmthAfter(warmth, tick, caress as G.Caress);
    for (const number of flatWarmth(warmth)) digest.number(number);
  }
  corpus.push({ kind: "warmth", id: vector.id, variant: 0, body: 0, cues: [], digest: digest.hex() });
}
for (const [kind, traces] of [["circle", document.circles], ["stroke", document.strokes]] as const) {
  for (const vector of traces) {
    const trail = trailOf(vector.path, document.hold);
    for (let variant = 0; variant < 16; variant++) {
      const run = hoverRun(variantOf(trail, document.page, variant), document.quantum, boxVariant(vector.body!, document.page, variant), guardOf(vector.guard), true);
      corpus.push({ kind, id: vector.id, variant, body: 0, ...run });
    }
  }
}
for (const vector of document.shakes) {
  const trail = trailOf(vector.path, document.hold);
  for (let variant = 0; variant < 16; variant++) corpus.push({ kind: "shake", id: vector.id, variant, body: 0, ...heldRun(variantOf(trail, document.page, variant), document.quantum, vector.height!, guardOf(vector.guard)) });
}
for (const travel of document.travels) {
  const trail = trailOf(travel.path, document.hold);
  for (let variant = 0; variant < 16; variant++) {
    const turned = variantOf(trail, document.page, variant);
    document.bodies.forEach((body, index) => {
      corpus.push({ kind: "travel", id: travel.id, variant, body: index, ...hoverRun(turned, document.quantum, boxVariant(body, document.page, variant), G.UNGUARDED, variant === 0 || variant === 9) });
    });
  }
}
//#endregion 🔖️Corpus

//#region 🔖️World
type Rules = Record<string, boolean>;
type WorldPet = { mode: string; x: number; y: number };
type World = { pets: WorldPet[]; tally: Record<string, number>; counts: Record<string, number> };
const suite = (await import(join(PETS, "🔨️modules/🚧️clearance/🧪️tests/🔬️unit/🟦️.ts"))) as { RULES: Rules; openWorld: (seed: number, rules: Rules, span: number) => World; tickWorld: (world: World) => void; bodiesOf: (world: World) => C.Body[] };
const MODES = ["stand", "walk", "plan", "drop", "held", "ride", "scoot", "gone"];
const RULE_NAMES = ["guard", "corridors", "vetting", "rests", "projection", "poof", "eviction", "heads", "steering", "seating"];
const COUNT_NAMES = ["surveys", "grabs", "hops", "glides", "lanes", "falls", "heads", "steered", "chutes", "refusals", "delays", "stalls", "timeouts", "evictions", "crowded", "seated", "spilled", "disorders", "brushes"];
const TALLY_NAMES = ["ticks", "actors", "overlaps", "nears", "poofs", "waits"];
const runs: { name: string; seed: number; off: string[]; span: number; ticks: number; digests: string[]; tally: number[]; counts: number[] }[] = [];

/** 🌍️ Runs one world and records a digest of every pet after every tick and the final tally and counts. */
function world(name: string, seed: number, off: readonly string[], span: number, ticks: number): void {
  const rules = { ...suite.RULES };
  for (const rule of off) rules[rule] = false;
  const run = suite.openWorld(seed, rules, span);
  const digests: string[] = [];
  for (let tick = 0; tick < ticks; tick++) {
    suite.tickWorld(run);
    const digest = new Digest();
    for (const pet of run.pets) {
      digest.number(MODES.indexOf(pet.mode));
      digest.number(pet.x);
      digest.number(pet.y);
    }
    for (const body of suite.bodiesOf(run)) for (const number of [Number(body.owner.slice(4)), ...flatBox(body.extent)]) digest.number(number);
    digests.push(digest.hex());
  }
  runs.push({ name, seed, off: [...off], span, ticks, digests, tally: TALLY_NAMES.map((key) => run.tally[key]!), counts: COUNT_NAMES.map((key) => run.counts[key]!) });
}

for (let seed = 1; seed <= 12; seed++) world("rules", seed, [], 1, 5000);
for (let seed = 1; seed <= 4; seed++) world("slices-of-four", seed, [], 4, 5000);
for (const rule of RULE_NAMES) for (let seed = 1; seed <= 3; seed++) world(`without-${rule}`, seed, [rule], 1, 3000);
for (let seed = 1; seed <= 3; seed++) world("without-heads-steering", seed, ["heads", "steering"], 1, 3000);
for (let seed = 1; seed <= 3; seed++) world("without-heads-steering-poof", seed, ["heads", "steering", "poof"], 1, 3000);
//#endregion 🔖️World

//#region 🔖️Output
mkdirSync(OUT, { recursive: true });
const counted: Record<string, number> = {};
for (const entry of cases) counted[entry.fn] = (counted[entry.fn] ?? 0) + 1;
const numbers = cases.reduce((sum, entry) => sum + entry.args.length + (entry.out === "throws" ? 0 : entry.out.length), 0);
writeFileSync(join(OUT, "bits.json"), `[\n${cases.map((entry) => JSON.stringify(entry)).join(",\n")}\n]\n`);
writeFileSync(join(OUT, "corpus.json"), `[\n${corpus.map((entry) => JSON.stringify(entry)).join(",\n")}\n]\n`);
writeFileSync(join(OUT, "world.json"), `[\n${runs.map((entry) => JSON.stringify(entry)).join(",\n")}\n]\n`);
const cued = corpus.reduce((sum, entry) => sum + entry.cues.length, 0);
console.log(`[DEBUG] bits: ${cases.length} cases, ${numbers} numbers, ${cases.filter((entry) => entry.out === "throws").length} throwing; per function ${JSON.stringify(counted)}`);
console.log(`[DEBUG] corpus: ${corpus.length} replays, ${cued} cues and signals, ${corpus.filter((entry) => entry.digest !== "").length} state digests`);
console.log(`[DEBUG] world: ${runs.length} runs, ${runs.reduce((sum, run) => sum + run.ticks, 0)} ticks, overlaps ${runs.filter((run) => run.off.length === 0).reduce((sum, run) => sum + run.tally[2]!, 0)} with every rule`);
//#endregion 🔖️Output
