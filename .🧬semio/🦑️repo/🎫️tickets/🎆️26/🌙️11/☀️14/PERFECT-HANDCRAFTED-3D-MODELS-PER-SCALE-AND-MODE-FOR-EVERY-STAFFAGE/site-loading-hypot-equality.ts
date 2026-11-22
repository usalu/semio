/** 📏️ Proves that the allocation-free `portable.hypot` returns the same bits as the implementation it replaces, and times both.
 * Usage: `bun site-loading-hypot-equality.ts <previous portable module> [million calls]` — the previous module is `git show HEAD:<portable path>` written to a generated file.
 * Inputs: two-, three- and four-argument calls over 600 binades, the lengths a drawing meets (1e-9 … 1e3), exact ties, zeros, signed zeros, subnormals, the largest doubles, infinities and NaN in every position. */
import { resolve } from "node:path";
import * as current from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

const previous = (await import(resolve(process.argv[2]!))) as typeof current;
const millions = Number(process.argv[3] ?? 20);
let state = 0x2545f491;
const next = (): number => {
  state ^= state << 13;
  state ^= state >>> 17;
  state ^= state << 5;
  return (state >>> 0) / 4294967296;
};
const special = [0, -0, 1, -1, 3, 4, 5, 1e-320, -4.9e-324, 2.2250738585072014e-308, 1.7976931348623157e308, -1.7976931348623157e308, 1e-160, 1e160, Infinity, -Infinity, NaN, 0.1, 1e-9, 3e-7, 1e-5];
const sample = (): number => {
  const pick = next();
  if (pick < 0.02) return special[Math.floor(next() * special.length)]!;
  if (pick < 0.5) return (next() * 2 - 1) * 10 ** (next() * 12 - 9);
  return (next() * 2 - 1) * 2 ** Math.floor(next() * 600 - 300);
};
let calls = 0, different = 0;
const check = (...values: number[]): void => {
  calls += 1;
  if (!Object.is(current.hypot(...values), previous.hypot(...values))) {
    different += 1;
    if (different <= 5) console.log("DIFFERENT", values, current.hypot(...values), previous.hypot(...values));
  }
};
check();
for (const a of special) {
  check(a);
  for (const b of special) {
    check(a, b);
    for (const c of special) check(a, b, c);
  }
}
for (let i = 0; i < millions * 1e6; i += 1) {
  const arity = i % 8;
  if (arity < 5) check(sample(), sample());
  else if (arity < 7) check(sample(), sample(), sample());
  else check(sample(), sample(), sample(), sample());
}
const time = (hypot: (...values: number[]) => number): number => {
  const start = performance.now();
  let sum = 0;
  for (let i = 0; i < 5e6; i += 1) sum += hypot(i * 1e-6 - 2.5, 1.25 - i * 3e-7);
  return sum > 0 ? performance.now() - start : Number.NaN;
};
time(previous.hypot);
time(current.hypot);
console.log(JSON.stringify({ calls, different, previousMsPer5MCalls: Math.round(time(previous.hypot)), currentMsPer5MCalls: Math.round(time(current.hypot)) }));
process.exit(different === 0 ? 0 : 1);
