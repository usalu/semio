/** 📏️ Prints `portable.hypot` of deterministic two- and three-argument samples as JSON lines `[arguments, result]`, for the third-party cross-check in `site-loading-hypot-decimal.py`.
 * Usage: `bun site-loading-hypot-samples.ts <count> > samples.jsonl` */
import { hypot } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

let state = 0x9e3779b9;
const next = (): number => {
  state ^= state << 13;
  state ^= state >>> 17;
  state ^= state << 5;
  return (state >>> 0) / 4294967296;
};
const sample = (): number => (next() < 0.5 ? (next() * 2 - 1) * 10 ** (next() * 12 - 9) : (next() * 2 - 1) * 2 ** Math.floor(next() * 400 - 200));
const lines: string[] = [];
for (let i = 0; i < Number(process.argv[2] ?? 100000); i += 1) {
  const values = i % 3 === 2 ? [sample(), sample(), sample()] : [sample(), sample()];
  lines.push(JSON.stringify([values, hypot(...values)]));
}
console.log(lines.join("\n"));
