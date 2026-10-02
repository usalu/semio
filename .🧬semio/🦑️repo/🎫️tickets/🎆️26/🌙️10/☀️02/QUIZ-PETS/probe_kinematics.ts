/** 🔎️ Work package B probe: dumps what the TypeScript kinematics modules compute for a dense grid of inputs to `🗑️generated/wp-b/probe.json`, for `probe_kinematics.py` to judge with numpy.
 *
 * Run from the repository root: `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/probe_kinematics.ts`
 */

import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { randomPick, randomUnit, randomWords } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎲️randomness/🟦️.ts";
import { cosTurns, sinTurns } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/📐️trigonometry/🟦️.ts";

const STEPS = 400000;
const turns: number[] = [];
for (let step = 0; step <= STEPS; step++) turns.push(-8 + (16 * step) / STEPS);
const view = new DataView(new ArrayBuffer(8));
const nudged = (value: number, ulps: number): number => {
  view.setFloat64(0, value);
  const bits = view.getBigUint64(0);
  view.setBigUint64(0, value > 0 === ulps > 0 ? bits + BigInt(Math.abs(ulps)) : bits - BigInt(Math.abs(ulps)));
  return view.getFloat64(0);
};
for (let eighth = -64; eighth <= 64; eighth++) {
  const boundary = eighth / 8;
  turns.push(boundary);
  if (boundary !== 0) for (const ulps of [-3, -2, -1, 1, 2, 3]) turns.push(nudged(boundary, ulps));
}
for (const tiny of [5e-324, 1e-300, 1e-20, 1e-9, 2 ** -27, 2 ** -26]) turns.push(tiny, -tiny);
for (const large of [1e6 + 0.3, 123456789.125, 2 ** 40 + 0.25, 2 ** 52 + 1, 2 ** 53, 1e300]) turns.push(large, -large);

let state = 20261002;
const word = (): number => {
  state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
  return state;
};
const keys: number[][] = [];
for (let length = 0; length <= 12; length++) {
  for (let sample = 0; sample < 40; sample++) {
    const key: number[] = [];
    for (let index = 0; index < length; index++) key.push(sample % 4 === 0 ? [0, 1, 0xffffffff, 0x80000000][word() % 4]! : sample % 4 === 1 ? word() % 16 : word());
    keys.push(key);
  }
}

const out = join(import.meta.dir, "🗑️generated", "wp-b");
mkdirSync(out, { recursive: true });
writeFileSync(
  join(out, "probe.json"),
  JSON.stringify({
    turns,
    sines: turns.map(sinTurns),
    cosines: turns.map(cosTurns),
    keys,
    words: keys.map((key) => randomWords(key, 9)),
    units: keys.map(randomUnit),
    picks: keys.map((key) => randomPick(key, [0, 2, 0.5, -1, 3, 0])),
  }),
);
