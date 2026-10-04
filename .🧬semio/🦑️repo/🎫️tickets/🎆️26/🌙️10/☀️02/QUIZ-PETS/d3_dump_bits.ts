/** 🧮️ Ticket tool of work package D3, TypeScript half: evaluates every exported function of the feeling, effects and mischief modules of `@semio-tech/pets` on generated inputs (thousands per module, among them signed zeros, NaN, infinities, subnormals, values on thresholds and ties, whole stories whose inputs are the previous outputs, synthetic species with lasting, circling and broken states, and the committed chemistry menagerie with drawn crowds) and writes every input and every result as an IEEE-754 bit pattern to `🗑️generated/d3/d3-bits.json`. `d3_check_bits.rs` beside this file recomputes every case with the Rust twins and compares bit for bit.
 *
 * Numbers travel as sixteen hexadecimal digits (`nan` for every NaN, whose payload no language promises; `null` for
 * an answer the twins give as `null`/`None`), so no decimal parser or printer stands between the two cores. Moods,
 * activities, cues, occasions, motions, modes and places travel as their index in the contract's order; species,
 * states, tricks, reactions and sightings as indices into the synthetic species and the menageries written at the top
 * of the file (decimal JSON, which both sides read with correctly rounded parsers). A case whose TypeScript evaluation
 * throws is recorded as `throws`; the Rust twin must panic on it.
 *
 * The inputs stay inside the domain both twins share: ticks are whole numbers below 2⁴⁰, counts whole and not
 * negative, lifetimes and lasting seconds below 10⁶ (beyond 1.4·10¹⁷ s the twin's double and the Rust `i64` part, as
 * recorded for clips in design §13.9), sides and facings ±1.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d3_dump_bits.ts
 *   bash .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/rust_scratch.sh d3 test --offline --release --test d3_bits -- --nocapture
 *
 * @see ./d3_check_bits.rs — the Rust half
 * @see ./dump_motion_bits.ts — the pattern (work package L)
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import * as effects from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/✨️effects/🟦️.ts";
import * as feeling from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/💗️feeling/🟦️.ts";
import * as mischief from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🪄️mischief/🟦️.ts";
import { ACTIVITIES, CUES, DRIFTS, type Feeling, type Fixture, type Menagerie, MOODS, PET_MODES, type Perch, type Pitch, type Rapport, type Reaction, type Species, type Trick } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";

const PETS = "../../../../../../../🧰️framework/🛍️products/🐾️pets";
const PLACES = ["above", "below", "beside", "any"] as const;

//#region 🔖️Encoding
const view = new DataView(new ArrayBuffer(8));

/** 🔢️ The bit pattern of a number as sixteen hexadecimal digits; `nan` for every NaN. */
function bits(value: number): string {
  if (Number.isNaN(value)) return "nan";
  view.setFloat64(0, value);
  return view.getBigUint64(0).toString(16).padStart(16, "0");
}

/** 🕳️ A result that may be `null`, as the token `null` or its bits. */
function maybe(value: number | null): string {
  return value === null ? "null" : bits(value);
}

type Case = { readonly fn: string; readonly args: readonly string[]; readonly text?: readonly string[]; readonly out: readonly string[] | "throws" };
const cases: Case[] = [];

/** 📝️ Records one evaluation: the flat arguments, the texts, and what the function answers (numbers, or `null` tokens), or that it throws. */
function record(fn: string, args: readonly number[], run: () => readonly (number | null)[], text?: readonly string[]): void {
  let out: readonly string[] | "throws";
  try {
    out = run().map(maybe);
  } catch {
    out = "throws";
  }
  cases.push(text === undefined ? { fn, args: args.map(bits), out } : { fn, args: args.map(bits), text, out });
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

/** 📏️ A multiple of `1 ÷ grain` in [low, high]. */
function grid(low: number, high: number, grain: number): number {
  return low + below((high - low) * grain + 1) / grain;
}

/** 🎚️ One of the given items. */
function pick<T>(items: readonly T[]): T {
  return items[below(items.length)]!;
}

const SPECIALS = [0, -0, Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, 5e-324, -5e-324, 1e-300, -1e-300, 1e300, -1e300, 0.1, 1 / 3, 2 ** 53];

/** 🎭️ Mostly what `usual` yields, with the given share a special number. */
function odd(share: number, usual: () => number): number {
  return chance(share) ? pick(SPECIALS) : usual();
}

/** ⏱️ A whole tick: mostly small, sometimes negative or far. */
function tick(): number {
  return chance(0.1) ? pick([0, -1, -640, 2 ** 31, 2 ** 32 + 7, 2 ** 40 - 3]) : below(40000) - 2000;
}

/** 💗️ An intensity: on the thresholds, dyadic, arbitrary, or special. */
function intensity(): number {
  return chance(0.08) ? pick([...SPECIALS, 1.5, -0.5, 1]) : chance(0.3) ? pick([0, feeling.MOOD_FAINT, feeling.MOOD_REST, feeling.MOOD_REST + 2 ** -50, 0.05 - 2 ** -50, 0.5, 0.8, 1]) : chance(0.4) ? grid(0, 1, 64) : unit();
}

/** 💓️ A feeling and its flat form `mood, intensity, since`. */
function felt(): Feeling {
  return { mood: pick(MOODS), intensity: intensity(), since: chance(0.85) ? below(6000) : tick() };
}

/** 🧾️ The flat form of a feeling. */
function flatFeeling(value: Feeling): number[] {
  return [MOODS.indexOf(value.mood), value.intensity, value.since];
}

/** 🧬️ A character and its flat form `mood, energy, sociability, curiosity`. */
function character(): { value: feeling.Character; flat: number[] } {
  const temperament = { energy: odd(0.03, unit), sociability: odd(0.03, unit), curiosity: odd(0.03, unit) };
  const mood = pick(MOODS);
  return { value: { mood, temperament }, flat: [MOODS.indexOf(mood), temperament.energy, temperament.sociability, temperament.curiosity] };
}
//#endregion 🔖️Generators

//#region 🔖️Documents
const chemistryFixture = JSON.parse(readFileSync(join(import.meta.dir, PETS, "🧫️fixtures/⚗️chemistry-rules/🔣️.json"), "utf8")) as { menagerie: Menagerie };
const TEMPLATE = chemistryFixture.menagerie.species[0]!;
const text = (en: string): { en: string; de: string } => ({ en, de: en });

/** 🧪️ A synthetic species: the template's body with drawn states (lasting or not, giving way to one another, to themselves, to nothing known), drawn tricks (cues, rungs, targets, moods) and a resting mood. */
function synthetic(index: number): Species {
  const count = index % 9 === 0 ? 0 : 1 + below(6);
  const ids = Array.from({ length: count }, (_, place) => `st${place}`);
  const states = ids.map((id) => {
    const entry: Record<string, unknown> = { id, name: text(id) };
    if (chance(0.7)) entry.lasts = pick([grid(0, 4, 64), free(0.001, 30), 0.001, 0.0078125, 1 / 3, 0, 120, 999999]);
    if (chance(0.75)) entry.then = chance(0.85) && count > 0 ? pick(ids) : "ghost";
    return entry;
  });
  const tricks = Array.from({ length: below(7) }, (_, place) => {
    const entry: Record<string, unknown> = { id: `tr${place}`, name: text(`trick ${place}`), clip: TEMPLATE.clips[0]?.id ?? "clip", cues: CUES.filter(() => chance(0.3)) };
    if (chance(0.4) && count > 0) entry.from = ids.filter(() => chance(0.5)).concat(chance(0.15) ? ["ghost"] : []);
    if (chance(0.3)) entry.to = chance(0.85) && count > 0 ? pick(ids) : "ghost";
    if (chance(0.4)) entry.mood = pick(MOODS);
    return entry;
  });
  return { ...structuredClone(TEMPLATE), id: `syn${index}`, states, tricks, mood: pick(MOODS) } as unknown as Species;
}

const SPECIES: Species[] = Array.from({ length: 80 }, (_, index) => synthetic(index));
const anyone = (side: Reaction["when"]): Reaction["when"] => {
  const { species: _species, ...rest } = side;
  return rest;
};
const MENAGERIES: Menagerie[] = [
  chemistryFixture.menagerie,
  { ...structuredClone(chemistryFixture.menagerie), id: "anyone", chemistry: chemistryFixture.menagerie.chemistry.map((reaction) => ({ ...structuredClone(reaction), when: anyone(reaction.when), near: anyone(reaction.near), within: reaction.within * 2 })) },
  { ...structuredClone(chemistryFixture.menagerie), id: "chancy", chemistry: chemistryFixture.menagerie.chemistry.map((reaction, place) => ({ ...structuredClone(reaction), chance: place % 2 === 0 ? 0.5 : reaction.chance, every: place % 3 === 0 ? 0.001 : reaction.every })) },
];
//#endregion 🔖️Documents

//#region 🔖️Moods
for (let index = 0; index < 400; index++) {
  const mood = index % 9;
  const at = tick();
  record("at_rest", [mood, at], () => flatFeeling(feeling.atRest(MOODS[mood]!, at)));
}
for (let index = 0; index < 600; index++) {
  const [mood, level] = [below(9), intensity()];
  record("shown_mood", [mood, level], () => [MOODS.indexOf(feeling.shownMood(MOODS[mood]!, level))]);
}
for (let index = 0; index < 6000; index++) {
  const present = felt();
  const mood = below(9);
  const amount = chance(0.1) ? pick([...SPECIALS, -0.5, feeling.MOOD_FAINT, feeling.MOOD_FAINT - 2 ** -55, 1, 1.5]) : chance(0.5) ? grid(0, 1, 32) : unit();
  const at = present.since + (chance(0.5) ? below(400) : pick([feeling.MOOD_HOLD - 1, feeling.MOOD_HOLD, feeling.MOOD_HOLD + 1, 0, -1]));
  record("impulse", [...flatFeeling(present), mood, amount, at], () => flatFeeling(feeling.impulse(present, MOODS[mood]!, amount, at)));
}
for (let index = 0; index < 8000; index++) {
  const present = felt();
  const resting = index % 4 === 0 ? MOODS.indexOf(present.mood) : below(9);
  const at = present.since + (chance(0.6) ? below(20000) - 200 : pick([0, 127, 128, 129, 64 * 512, 2 ** 33]));
  record("settled", [...flatFeeling(present), resting, at], () => flatFeeling(feeling.settled(present, MOODS[resting]!, at)));
}
for (let index = 0; index < 4000; index++) {
  const present = felt();
  const resting = index % 3 === 0 ? MOODS.indexOf(present.mood) : below(9);
  record("settles_at", [...flatFeeling(present), resting], () => [feeling.settlesAt(present, MOODS[resting]!)]);
  record("calms_at", [...flatFeeling(present), resting], () => [feeling.calmsAt(present, MOODS[resting]!)]);
}
for (let story = 0; story < 40; story++) {
  const resting = below(9);
  let present: Feeling = feeling.atRest(MOODS[resting]!, 0);
  let at = 0;
  for (let step = 0; step < 60; step++) {
    at += below(700);
    const before = feeling.settled(present, MOODS[resting]!, at);
    record("settled", [...flatFeeling(present), resting, at], () => flatFeeling(before));
    const mood = below(9);
    const amount = grid(0, 1, 16);
    present = feeling.impulse(before, MOODS[mood]!, amount, at);
    const after = present;
    record("impulse", [...flatFeeling(before), mood, amount, at], () => flatFeeling(after));
  }
}
//#endregion 🔖️Moods

//#region 🔖️Face
for (let index = 0; index < 9; index++) record("valence_of", [index], () => [feeling.valenceOf(MOODS[index]!)]);
for (let index = 0; index < 2000; index++) {
  const present = felt();
  record("spirits_of", flatFeeling(present), () => [feeling.spiritsOf(present)]);
  record("face_of", flatFeeling(present), () => {
    const face = feeling.faceOf(present);
    return [face.bend, face.lid, face.slant, face.drop];
  });
}
//#endregion 🔖️Face

//#region 🔖️Appraisal
for (let index = 0; index < 1500; index++) {
  const who = character();
  const mood = below(9);
  record("proneness_of", [...who.flat, mood], () => [feeling.pronenessOf(who.value, MOODS[mood]!)]);
}
for (let index = 0; index < 3000; index++) {
  const present = felt();
  const who = character();
  const mood = below(9);
  const amount = chance(0.05) ? pick(SPECIALS) : unit();
  const at = present.since + below(600);
  record("stirred", [...flatFeeling(present), mood, amount, ...who.flat, at], () => flatFeeling(feeling.stirred(present, MOODS[mood]!, amount, who.value, at)));
}
for (let index = 0; index < 4000; index++) {
  const present = felt();
  const who = character();
  const occasion = below(feeling.OCCASIONS.length);
  const at = present.since + below(600);
  record("appraised", [...flatFeeling(present), occasion, ...who.flat, at], () => flatFeeling(feeling.appraised(present, feeling.OCCASIONS[occasion]!, who.value, at)));
}
for (let index = 0; index < 2000; index++) {
  const present = felt();
  const who = character();
  const mood = below(10) - 1;
  const at = present.since + below(600);
  record("performed", [...flatFeeling(present), mood, ...who.flat, at], () => flatFeeling(feeling.performed(present, (mood < 0 ? {} : { mood: MOODS[mood]! }) as Trick, who.value, at)));
}
for (let index = 0; index < 2000; index++) {
  const present = felt();
  const energy = chance(0.05) ? pick(SPECIALS) : chance(0.3) ? grid(0, 1, 20) : free(-0.2, 1.2);
  const at = present.since + below(600);
  record("drowsed", [...flatFeeling(present), energy, at], () => flatFeeling(feeling.drowsed(present, energy, at)));
}
//#endregion 🔖️Appraisal

//#region 🔖️Wishes
for (let index = 0; index < 900; index++) {
  const [mood, level] = [below(9), intensity()];
  record("mood_weights", [mood, level], () => feeling.moodWeights(MOODS[mood]!, level));
}
for (let index = 0; index < 3000; index++) {
  const [first, second] = [felt(), felt()];
  if (index % 5 === 0) first.mood = "proud";
  if (index % 7 === 0) second.mood = "proud";
  record("encounter_bias", [...flatFeeling(first), ...flatFeeling(second)], () => {
    const leaning = feeling.encounterBias(first, second);
    return [leaning.affinity, ...leaning.shares, leaning.show];
  });
}
for (let index = 0; index < 1500; index++) {
  const shares = [odd(0.03, unit), odd(0.03, unit), odd(0.03, unit)];
  const leaning: feeling.Leaning = { affinity: unit(), shares: [odd(0.03, () => free(0, 3)), odd(0.03, () => free(0, 3)), odd(0.03, () => free(0, 3))], show: below(3) - 1 };
  record("swayed_shares", [...shares, leaning.affinity, ...leaning.shares, leaning.show], () => feeling.swayedShares(shares, leaning));
}
//#endregion 🔖️Wishes

//#region 🔖️Contagion
for (let index = 0; index < 5000; index++) {
  const [mine, theirs] = [felt(), felt()];
  if (index % 3 === 0) theirs.mood = mine.mood;
  const affinity = chance(0.05) ? pick(SPECIALS) : free(-1, 1);
  const sociability = chance(0.05) ? pick(SPECIALS) : unit();
  const at = below(9000);
  record("caught", [...flatFeeling(mine), ...flatFeeling(theirs), affinity, sociability, at], () => flatFeeling(feeling.caught(mine, theirs, affinity, sociability, at)));
}
//#endregion 🔖️Contagion

//#region 🔖️States
/** 🔦️ The index of a state id in a species, −1 for an id it does not have. */
function stateIndex(species: Species, id: string): number {
  return species.states.findIndex((entry) => entry.id === id);
}

/** 🎯️ A state of a species by index, or an id it does not know for −1. */
function stateName(species: Species, index: number): string {
  return index < 0 ? "nowhere" : species.states[index]!.id;
}

/** 🃏️ A state index of a species, sometimes −1 (unknown). */
function someState(species: Species): number {
  return species.states.length === 0 || chance(0.08) ? -1 : below(species.states.length);
}

for (let index = 0; index < 600; index++) {
  const has = chance(0.8);
  const lasts = chance(0.06) ? pick([0, -0, Number.NaN, -1, 5e-324, 1e-300]) : chance(0.4) ? grid(0, 8, 128) : free(0, 1000);
  record("lasting_ticks", [has ? 1 : 0, lasts], () => [feeling.lastingTicks((has ? { id: "s", name: text("s"), lasts } : { id: "s", name: text("s") }) as Species["states"][number])]);
  record("held_ticks", [lasts], () => [feeling.heldTicks(lasts)]);
  record("every_ticks", [lasts], () => [feeling.everyTicks({ every: lasts } as Reaction)]);
}
for (let index = 0; index < 8000; index++) {
  const which = below(SPECIES.length);
  const species = SPECIES[which]!;
  const entered = someState(species);
  const since = below(5000);
  const at = since + (chance(0.15) ? pick([0, -1, -500, 2 ** 36, 2 ** 40 - since]) : below(20000));
  record("state_at", [which, entered, since, at], () => {
    const standing = feeling.stateAt(species, stateName(species, entered), since, at);
    return [stateIndex(species, standing.state), standing.since];
  });
  record("state_ends", [which, entered, since], () => [feeling.stateEnds(species, stateName(species, entered), since)]);
}
for (let which = 0; which < SPECIES.length; which++) {
  const species = SPECIES[which]!;
  record("ladder_of", [which], () => feeling.ladderOf(species).map((id) => stateIndex(species, id)));
  for (let trick = 0; trick < species.tricks.length; trick++) {
    record("rungs_of", [which, trick], () => feeling.rungsOf(species, species.tricks[trick]!).map((id) => stateIndex(species, id)));
    for (let entered = -1; entered < species.states.length; entered++) {
      record("state_after_trick", [which, entered, trick], () => [stateIndex(species, feeling.stateAfterTrick(species, stateName(species, entered), species.tricks[trick]!))]);
      for (const direction of [-2, -1, 0, 1, 2]) record("step_rung", [which, trick, entered, direction], () => [stateIndex(species, feeling.stepRung(feeling.rungsOf(species, species.tricks[trick]!), stateName(species, entered), direction))]);
    }
  }
  for (let entered = -1; entered < species.states.length; entered++) for (const direction of [-3, -1, 0, 1, 4]) record("step_state", [which, entered, direction], () => [stateIndex(species, feeling.stepState(species, stateName(species, entered), direction))]);
}
//#endregion 🔖️States

//#region 🔖️Tricks
/** 🎪️ The index of a trick in its species, −1 for none. */
function trickIndex(species: Species, trick: Trick | null): number {
  return trick === null ? -1 : species.tricks.indexOf(trick);
}

for (let index = 0; index < 6000; index++) {
  const which = below(SPECIES.length);
  const species = SPECIES[which]!;
  const entered = someState(species);
  const present = felt();
  const cue = below(CUES.length);
  record("tricks_for", [which, cue, entered, ...flatFeeling(present)], () => feeling.tricksFor(species, CUES[cue]!, stateName(species, entered), present).map((trick) => trickIndex(species, trick)));
  const clicked = chance(0.2) ? pick([-1, -7, 2 ** 31, -(2 ** 33), 0]) : below(12) - 3;
  record("click_trick", [which, entered, ...flatFeeling(present), clicked], () => [trickIndex(species, feeling.clickTrick(species, stateName(species, entered), present, clicked))]);
  const drawn = chance(0.08) ? pick([...SPECIALS, -0.5, 1, 1.5, 1 - 2 ** -53]) : unit();
  record("whim_trick", [which, entered, ...flatFeeling(present), drawn], () => [trickIndex(species, feeling.whimTrick(species, stateName(species, entered), present, drawn))]);
  record("show_trick", [which, entered, ...flatFeeling(present), drawn], () => [trickIndex(species, feeling.showTrick(species, stateName(species, entered), present, drawn))]);
}
//#endregion 🔖️Tricks

//#region 🔖️Chemistry
/** 📦️ A drawn body `x, y, width, height`: on lattices that make boxes touch, overlap and line up, or anywhere, now and then special. */
function body(): number[] {
  return [odd(0.03, () => (chance(0.6) ? grid(60, 260, 2) : free(0, 400))), odd(0.03, () => (chance(0.6) ? pick([300, 250, 200, 340, 300.5]) : free(100, 400))), odd(0.03, () => (chance(0.7) ? pick([40, 46, 58, 28, 30]) : free(0, 80))), odd(0.03, () => (chance(0.7) ? pick([30, 50, 42, 38]) : free(0, 80)))];
}

/** 👀️ A sighting from its geometry and nothing else of note. */
function geometric(flat: readonly number[]): feeling.Sighting {
  return { species: "geo", state: "geo", held: 0, mood: "content", intensity: 0.25, activity: "idle", trick: null, x: flat[0]!, y: flat[1]!, width: flat[2]!, height: flat[3]! };
}

for (let index = 0; index < 6000; index++) {
  const [first, second] = [body(), body()];
  const reach = chance(0.05) ? pick(SPECIALS) : chance(0.4) ? pick([0, 0.5, 10, 40, 100]) : free(0, 150);
  const place = below(4);
  record("nearby", [...first, ...second, reach], () => [feeling.nearby(geometric(first), geometric(second), reach) ? 1 : 0]);
  record("seen", [...first, ...second, place], () => [feeling.seen(geometric(first), geometric(second), PLACES[place]!) ? 1 : 0]);
}
for (let index = 0; index < 6000; index++) {
  const first = [free(0, 400), free(100, 400), free(1, 80), free(1, 80)];
  const second = [free(0, 400), free(100, 400), free(1, 80), free(1, 80)];
  const across = Math.abs(first[0]! - second[0]!) - (first[2]! + second[2]!) / 2;
  const under = first[1]! - first[3]! - second[1]!;
  const over = second[1]! - second[3]! - first[1]!;
  const wide = across > 0 ? across : 0;
  const tall = under > 0 ? under : over > 0 ? over : 0;
  const gap = Math.sqrt(wide * wide + tall * tall);
  view.setFloat64(0, gap);
  view.setBigUint64(0, view.getBigUint64(0) + BigInt(below(5) - 2));
  const reach = view.getFloat64(0);
  record("nearby", [...first, ...second, reach], () => [feeling.nearby(geometric(first), geometric(second), reach) ? 1 : 0]);
}

/** 🕵️ A drawn sighting of a menagerie and its flat form `species, state, held, mood, intensity, activity, trick, x, y, width, height` (species −1 a stranger, state −1 an unknown id, trick −1 none and −2 an unknown id). */
function sighted(menagerie: Menagerie): { value: feeling.Sighting; flat: number[] } {
  const which = chance(0.05) ? -1 : below(menagerie.species.length);
  const species = which < 0 ? null : menagerie.species[which]!;
  const entered = species === null || chance(0.06) ? -1 : below(species.states.length);
  const held = chance(0.1) ? pick([0, 1, 63, 64, 65, 640]) : below(3000);
  const mood = below(9);
  const level = chance(0.5) ? pick([0.25, 0.25 + 2 ** -52, 0.5, 0.9, 0.2]) : intensity();
  const activity = chance(0.6) ? pick([0, 20, 1, 2, 6, 7, 8, 9]) : below(ACTIVITIES.length);
  const trick = species === null || chance(0.5) ? -1 : chance(0.1) ? -2 : species.tricks.length === 0 ? -1 : below(species.tricks.length);
  const box = body();
  const value: feeling.Sighting = { species: species === null ? "stranger" : species.id, state: species === null || entered < 0 ? "nowhere" : species.states[entered]!.id, held, mood: MOODS[mood]!, intensity: level, activity: ACTIVITIES[activity]!, trick: trick === -1 ? null : trick === -2 ? "ghost" : species!.tricks[trick]!.id, x: box[0]!, y: box[1]!, width: species !== null && chance(0.7) ? species.size.width : box[2]!, height: species !== null && chance(0.7) ? species.size.height : box[3]! };
  return { value, flat: [which, entered, held, mood, level, activity, trick, value.x, value.y, value.width, value.height] };
}

for (let index = 0; index < 6000; index++) {
  const which = below(MENAGERIES.length);
  const menagerie = MENAGERIES[which]!;
  const reaction = below(menagerie.chemistry.length);
  const side = below(3);
  const trait = side === 0 ? menagerie.chemistry[reaction]!.when : side === 1 ? menagerie.chemistry[reaction]!.near : (menagerie.chemistry[reaction]!.unless ?? menagerie.chemistry[reaction]!.when);
  const seen = sighted(menagerie);
  record("matches", [which, reaction, side, ...seen.flat], () => [feeling.matches(trait, seen.value) ? 1 : 0]);
}

/** 🎬️ A drawn beat of a menagerie and its flat form: `menagerie, tick, n, sightings…, k, (reaction, when, near, until)…, r, (a, b, drift)…, u, units…`. */
function beat(which: number): { flat: number[]; sightings: feeling.Sighting[]; coolings: { reaction: string; when: string; near: string; until: number }[]; rapports: Rapport[]; units: number[]; at: number } {
  const menagerie = MENAGERIES[which]!;
  const at = below(6000);
  const crowd = Array.from({ length: 2 + below(5) }, () => sighted(menagerie));
  const names = menagerie.species.map((species) => species.id);
  const coolings = Array.from({ length: below(5) }, () => ({ reaction: below(menagerie.chemistry.length), when: below(names.length), near: below(names.length), until: at + below(200) - 100 }));
  const rapports = Array.from({ length: below(4) }, () => ({ a: below(names.length), b: below(names.length), drift: chance(0.1) ? pick([0, -0.6, 0.6]) : free(-0.6, 0.6) }));
  const units = Array.from({ length: below(12) }, () => (chance(0.05) ? pick([Number.NaN, 1, 0, -0.1]) : unit()));
  return {
    flat: [which, at, crowd.length, ...crowd.flatMap((seen) => seen.flat), coolings.length, ...coolings.flatMap((entry) => [entry.reaction, entry.when, entry.near, entry.until]), rapports.length, ...rapports.flatMap((entry) => [entry.a, entry.b, entry.drift]), units.length, ...units],
    sightings: crowd.map((seen) => seen.value),
    coolings: coolings.map((entry) => ({ reaction: menagerie.chemistry[entry.reaction]!.id, when: names[entry.when]!, near: names[entry.near]!, until: entry.until })),
    rapports: rapports.map((entry) => ({ between: [names[entry.a]!, names[entry.b]!], drift: entry.drift })),
    units,
    at,
  };
}

/** 🗂️ An index of an id in a list, −1 for none. */
function indexIn(list: readonly string[], id: string | null): number {
  return id === null ? -1 : list.indexOf(id);
}

for (let index = 0; index < 5000; index++) {
  const which = below(MENAGERIES.length);
  const menagerie = MENAGERIES[which]!;
  const drawn = beat(which);
  const names = menagerie.species.map((species) => species.id);
  const reactions = menagerie.chemistry.map((reaction) => reaction.id);
  record("trials_of", drawn.flat, () => {
    const trials = feeling.trialsOf(menagerie, drawn.sightings, drawn.coolings, drawn.at, drawn.rapports);
    return [trials.length, ...trials.flatMap((trial) => [trial.reaction, trial.when, trial.near, trial.chancy ? 1 : 0]), feeling.drawsOf(trials)];
  });
  record("reactions_of", drawn.flat, () => {
    const outcome = feeling.reactionsOf(menagerie, drawn.sightings, drawn.at, drawn.coolings, drawn.units, drawn.rapports);
    return [
      outcome.consequences.length,
      ...outcome.consequences.flatMap((consequence) => {
        const species = menagerie.species.find((entry) => entry.id === consequence.on)!;
        return [indexIn(reactions, consequence.reaction), indexIn(names, consequence.on), indexIn(names, consequence.other), consequence.state === null ? -1 : stateIndex(species, consequence.state), consequence.mood === null ? -1 : MOODS.indexOf(consequence.mood), consequence.amount, consequence.rapport, consequence.encounter === null ? -1 : ACTIVITIES.indexOf(consequence.encounter), consequence.trick === null ? -1 : species.tricks.findIndex((trick) => trick.id === consequence.trick), consequence.activity === null ? -1 : ACTIVITIES.indexOf(consequence.activity)];
      }),
      outcome.coolings.length,
      ...outcome.coolings.flatMap((cooling) => [indexIn(reactions, cooling.reaction), indexIn(names, cooling.when), indexIn(names, cooling.near), cooling.until]),
      outcome.drawn,
    ];
  });
}
//#endregion 🔖️Chemistry

//#region 🔖️Effects
/** 🧩️ A drawn 32-bit word. */
function anyWord(): number {
  return chance(0.1) ? pick([0, 1, 0x9e3779b9, 0xffffffff, 0x80000000, 0x7fffffff]) : word();
}

for (let index = 0; index < 3000; index++) {
  const [a, b] = [anyWord(), anyWord()];
  record("lowbias32", [a], () => [effects.lowbias32(a)]);
  record("mix", [a, b], () => [effects.mix(a, b)]);
  record("unit", [a], () => [effects.unit(a)]);
  const particle = chance(0.2) ? pick([-1, -7, 2 ** 32, 2 ** 32 + 5, 2 ** 40 + 3, 2 ** 31]) : below(5000);
  const lane = below(6);
  record("scattered", [a, particle, lane], () => [effects.scattered(a, particle, lane)]);
  const [species, emitter, since] = [below(40), below(6), tick()];
  record("emitter_key", [a, species, emitter, since], () => [effects.emitterKey(a, species, emitter, since)]);
}
for (let chain = 0; chain < 200; chain++) {
  let folded = anyWord();
  for (let link = 0; link < 20; link++) {
    const [before, next] = [folded, anyWord()];
    folded = effects.mix(before, next);
    const after = folded;
    record("mix", [before, next], () => [after]);
  }
}

/** 🚿️ A drawn emission and its flat form `motion, count, life, speed, spread`. */
function emission(): { value: effects.Emission; flat: number[] } {
  const motion = below(DRIFTS.length);
  const count = chance(0.1) ? pick([0, 1, 32, 33, 40, 1000]) : 1 + below(32);
  const life = chance(0.1) ? pick([0, -1, 0.001, 0.0078125, 1e-9, 900000]) : chance(0.5) ? grid(0, 3, 64) : free(0.01, 4);
  const speed = odd(0.03, () => (chance(0.5) ? grid(0, 400, 1) : free(0, 400)));
  const spread = odd(0.03, () => (chance(0.5) ? grid(0, 1, 8) : unit()));
  return { value: { motion: DRIFTS[motion]!, count, life, speed, spread }, flat: [motion, count, life, speed, spread] };
}

for (let index = 0; index < 1500; index++) {
  const drawn = emission();
  record("life_ticks", drawn.flat, () => [effects.lifeTicks(drawn.value)]);
  record("swarm_of", drawn.flat, () => [effects.swarmOf(drawn.value)]);
  record("period_of", drawn.flat, () => [effects.periodOf(drawn.value)]);
  const [since, key, particle] = [tick(), anyWord(), below(400)];
  record("born_at", [...drawn.flat, since, key, particle], () => [effects.bornAt(drawn.value, since, key, particle)]);
  const stopped = chance(0.3) ? null : since + below(600) - 50;
  record("emitter_ends", [...drawn.flat, since, stopped === null ? 0 : 1, stopped ?? 0], () => [effects.emitterEnds(drawn.value, since, stopped)]);
}
for (let index = 0; index < 4000; index++) {
  const drawn = emission();
  if (drawn.value.life > 100000 || drawn.value.count > 64) continue;
  const origin = [odd(0.02, () => free(0, 800)), odd(0.02, () => free(0, 600))];
  const facing = chance(0.5) ? 1 : -1;
  const since = below(3000);
  const stopped = chance(0.4) ? null : since + below(400) - 20;
  const at = since + (chance(0.1) ? pick([-1, 0, 1, 2 ** 33]) : below(500) - 10);
  const key = anyWord();
  record("particles_of", [...drawn.flat, ...origin, facing, since, stopped === null ? 0 : 1, stopped ?? 0, at, key], () => {
    const particles = effects.particlesOf(drawn.value, { x: origin[0]!, y: origin[1]! }, facing, since, stopped, at, key);
    return [particles.length, ...particles.flatMap((particle) => [particle.x, particle.y, particle.scale, particle.rotation, particle.opacity, particle.age])];
  });
}
for (let index = 0; index < 1500; index++) {
  const size = below(200);
  const cap = chance(0.2) ? pick([0, 1, 160, 200]) : below(220);
  const ages = Array.from({ length: size }, () => (chance(0.5) ? below(8) : below(400)));
  record("capped", [cap, size, ...ages], () => effects.capped(ages.map((age, position) => ({ x: position, y: 0, scale: 1, rotation: 0, opacity: 1, age })), cap).map((particle) => particle.x));
}
//#endregion 🔖️Effects

//#region 🔖️Mischief
const VOCABULARY = ["physics", "heating", "heating/u-values", "heating/u-values/wall", "heating/u-values/wall-geg", "heating/heating-load", "heating/heating-load-and-demand", "", "/", "heating/", "Heizlast/Ü-Wert", "Heizlast", "🐾️pets/🧪️x", "🐾️pets", "a/b/c", "a", "a/"];
for (const ground of VOCABULARY) for (const key of VOCABULARY) record("fits", [], () => [mischief.fits(ground, key) ? 1 : 0], [ground, key]);
for (let index = 0; index < 1500; index++) {
  const grounds = Array.from({ length: below(5) }, () => pick(VOCABULARY));
  const keys = Array.from({ length: below(12) }, () => pick(VOCABULARY));
  record("fixture_for", [grounds.length, keys.length], () => mischief.fixtureFor(grounds, keys.map((key, position) => ({ key, position }))).map((item) => item.position), [...grounds, ...keys]);
}
for (let index = 0; index < 3000; index++) {
  const count = chance(0.1) ? 0 : below(20);
  const drawn = chance(0.1) ? pick([...SPECIALS, -0.5, 1, 1.5, 1 - 2 ** -53, 0.999999999]) : unit();
  record("chosen_fixture", [count, drawn], () => [mischief.chosenFixture(Array.from({ length: count }, (_, position) => position), drawn) ?? -1]);
}
record("patience_of", [0], () => [mischief.patienceOf(false)]);
record("patience_of", [1], () => [mischief.patienceOf(true)]);
for (let mode = 0; mode < PET_MODES.length; mode++) record("cooldown_of", [mode], () => [mischief.cooldownOf(PET_MODES[mode]!)]);
for (let index = 0; index < 4000; index++) {
  const flags = [chance(0.85) ? 1 : 0, chance(0.85) ? 1 : 0];
  const width = chance(0.1) ? pick([...SPECIALS, 1024, 1023.999, 1024.5]) : free(700, 1800);
  const mode = below(3);
  const quiet = chance(0.3) ? 1 : 0;
  const lifting = chance(0.15) ? 1 : 0;
  const [at, stirred, rested] = [below(40000), below(40000), below(40000)];
  const circumstances: mischief.Circumstances = { permitted: flags[0] === 1, fine: flags[1] === 1, width, mode: PET_MODES[mode]!, quiet: quiet === 1, lifting: lifting === 1, tick: at, stirred, rested };
  const flat = [flags[0]!, flags[1]!, width, mode, quiet, lifting, at, stirred, rested];
  record("allowed_from", flat, () => [mischief.allowedFrom(circumstances)]);
  record("allowed", flat, () => [mischief.allowed(circumstances) ? 1 : 0]);
}
for (let index = 0; index < 5000; index++) {
  const box = [odd(0.02, () => grid(100, 500, 4)), odd(0.02, () => grid(100, 400, 4)), odd(0.02, () => grid(200, 1000, 2)), odd(0.02, () => grid(10, 80, 2))];
  const fixture: Fixture = { id: "fixture", key: "heating", x: box[0]!, y: box[1]!, width: box[2]!, height: box[3]! };
  const pitches: Pitch[] = Array.from({ length: below(4) }, (_, place) => {
    const side = chance(0.5) ? -1 : 1;
    const x = odd(0.03, () => (side < 0 ? fixture.x - grid(-4, 30, 2) : fixture.x + fixture.width + grid(-4, 30, 2)));
    return { wall: `w${place}`, surface: `s${place}`, side, x, y0: odd(0.03, () => grid(0, 400, 2)), y1: odd(0.03, () => grid(200, 700, 2)) };
  });
  const perches: Perch[] = Array.from({ length: below(4) }, (_, place) => ({ surface: `p${place}`, x0: odd(0.03, () => grid(0, 600, 2)), x1: odd(0.03, () => grid(300, 1400, 2)), y: odd(0.03, () => fixture.y + grid(-10, 120, 2)) }));
  const width = chance(0.05) ? pick(SPECIALS) : grid(900, 1800, 1);
  record("station_for", [...box, pitches.length, ...pitches.flatMap((pitch) => [pitch.side, pitch.x, pitch.y0, pitch.y1]), perches.length, ...perches.flatMap((perch) => [perch.x0, perch.x1, perch.y]), width], () => {
    const station = mischief.stationFor(fixture, pitches, perches, width);
    if (station === null) return [0];
    const place = station.footing === "wall" ? pitches.findIndex((pitch) => pitch.wall === station.wall) : perches.findIndex((perch) => perch.surface === station.surface);
    return [1, station.footing === "wall" ? 1 : 0, place, station.x, station.y, station.side, station.room];
  });
}
for (let index = 0; index < 8000; index++) {
  const since = below(5000);
  const age = chance(0.1) ? pick([-1, -100, 0, 7, 8, 23, 24, 63, 64, 111, 112, 623, 624, 679, 680, 687, 688, 5000]) : below(700) - 5;
  const side = chance(0.5) ? 1 : -1;
  const room = odd(0.04, () => (chance(0.3) ? pick([0, 12, 40, -9]) : free(0, 80)));
  const span = odd(0.04, () => (chance(0.2) ? pick([0, -5, 883.2, 1e-9]) : free(20, 1400)));
  const drawn = odd(0.04, unit);
  record("lift_at", [since, since + age, side, room, span, drawn], () => {
    const lift = mischief.liftAt(since, since + age, side, room, span, drawn);
    return [lift.dx, lift.dy, lift.tilt, lift.opacity];
  });
  record("lift_wake", [since, since + age], () => [mischief.liftWake(since, since + age)]);
  record("lift_ends", [since], () => [mischief.liftEnds(since)]);
}
for (let index = 0; index < 2000; index++) {
  const pusher = [odd(0.04, () => free(0, 1440)), odd(0.04, () => free(0, 600))];
  const box = [odd(0.04, () => free(0, 600)), free(0, 400), odd(0.04, () => free(0, 1000)), free(10, 80)];
  const drawn = odd(0.04, unit);
  record("thrown_off", [...pusher, ...box, drawn], () => {
    const toss = mischief.thrownOff({ x: pusher[0]!, y: pusher[1]! }, { id: "f", key: "k", x: box[0]!, y: box[1]!, width: box[2]!, height: box[3]! }, drawn);
    return [toss.vx, toss.vy];
  });
}
//#endregion 🔖️Mischief

//#region 🔖️Constants
record("constants", [], () => [
  ...feeling.MOOD_PRIORITIES,
  feeling.MOOD_HOLD,
  feeling.MOOD_FAINT,
  feeling.MOOD_REST,
  feeling.MOOD_RISE,
  feeling.MOOD_OVERRIDE,
  ...feeling.MOOD_DECAYS,
  feeling.PRONE_DECAY,
  ...feeling.MOOD_VALENCES,
  ...feeling.MOOD_LIDS,
  ...feeling.MOOD_SLANTS,
  ...feeling.MOOD_DROPS,
  ...feeling.APPRAISALS.flatMap((stir) => [feeling.OCCASIONS.indexOf(stir.occasion), MOODS.indexOf(stir.mood), stir.amount]),
  ...feeling.PRONENESS.flat(),
  feeling.PRONE_GAIN,
  feeling.TRICK_AMOUNT,
  feeling.DROWSY_ENERGY,
  ...feeling.MOOD_WEIGHTS.flat(),
  ...feeling.MOOD_AFFINITIES,
  ...feeling.MOOD_SHARES.flat(),
  ...feeling.MOOD_SPREADS,
  feeling.CONTAGION_BEAT,
  feeling.CONTAGION_REACH,
  ...feeling.WILLING_MOODS.map((willing) => (willing ? 1 : 0)),
  feeling.WHIM_FAVOR,
  feeling.CHEMISTRY_BEAT,
  feeling.EFFECT_AMOUNT,
  feeling.LEAN_TICKS,
  effects.EMITTER_CAP,
  effects.STAGE_CAP,
  effects.TURN_RADIANS,
  effects.AHEAD,
  effects.DOWN,
  effects.UP,
  effects.LANE_BIRTH,
  effects.LANE_HEADING,
  effects.LANE_PACE,
  effects.LANE_PHASE,
  effects.LANE_LOOK,
  effects.FALL_SWAY,
  effects.FALL_SWAY_SPEED,
  effects.FALL_SWAY_RATE,
  effects.FALL_FADE_IN,
  effects.RISE_WANDER,
  effects.RISE_WANDER_RATE,
  effects.RISE_ROCK,
  effects.BURST_DRAG,
  effects.BURST_GRAVITY,
  effects.ORBIT_SQUASH,
  effects.ORBIT_DEPTH,
  effects.ORBIT_FADE,
  effects.DRIFT_MEANDER,
  effects.DRIFT_RATE,
  mischief.MISCHIEF_WIDTH,
  mischief.MISCHIEF_PATIENCE,
  mischief.MISCHIEF_PATIENCE_QUIET,
  mischief.MISCHIEF_COOLDOWN_CALM,
  mischief.MISCHIEF_COOLDOWN_LIVELY,
  mischief.STATION_GAP,
  mischief.STATION_SLACK,
  mischief.STATION_STEP,
  mischief.LIFT_ROOM,
  mischief.LIFT_BRACE,
  mischief.LIFT_SHOVE,
  mischief.LIFT_WOBBLE,
  mischief.LIFT_HOLD,
  mischief.LIFT_RETURN,
  mischief.LIFT_FADE,
  mischief.LIFT_RETURNS,
  mischief.LIFT_TICKS,
  mischief.LIFT_LIMIT,
  mischief.LIFT_LEAST,
  mischief.LIFT_GIVE,
  mischief.LIFT_RISE,
  mischief.LIFT_TILT,
  mischief.LIFT_WOBBLES,
  mischief.HALF_TURN_RADIANS,
  mischief.THROW_SPEED,
  mischief.THROW_SPREAD,
  mischief.THROW_LIFT,
  ...[mischief.NO_LIFT.dx, mischief.NO_LIFT.dy, mischief.NO_LIFT.tilt, mischief.NO_LIFT.opacity],
]);
//#endregion 🔖️Constants

//#region 🔖️Output
const directory = join(import.meta.dir, "🗑️generated", "d3");
mkdirSync(directory, { recursive: true });
const counts: Record<string, number> = {};
for (const entry of cases) counts[entry.fn] = (counts[entry.fn] ?? 0) + 1;
const numbers = cases.reduce((sum, entry) => sum + entry.args.length + (entry.out === "throws" ? 0 : entry.out.length), 0);
writeFileSync(join(directory, "d3-bits.json"), `{"species":${JSON.stringify(SPECIES)},\n"menageries":${JSON.stringify(MENAGERIES)},\n"cases":[\n${cases.map((entry) => JSON.stringify(entry)).join(",\n")}\n]}\n`);
console.log(`[DEBUG] d3 bits: ${cases.length} cases, ${numbers} numbers, ${cases.filter((entry) => entry.out === "throws").length} throwing; per function ${JSON.stringify(counts)}`);
//#endregion 🔖️Output
