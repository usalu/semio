/** 🔎️ Check of work package Q: every docstring of the files it touched starts with an emoji, and no emoji starts two docstrings of one file; also that none of them writes to the console or keeps a `[DEBUG]` log. `node wp_q_docstring_emojis.mjs`. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const pets = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const files = ["🧪️tests/🎚️config/🟦️.ts", ...["✅️validation", "🎞️animation", "🎪️stage", "🎲️randomness", "🏞️terrain", "📐️trigonometry", "🦴️rig", "🧠️behavior"].map((module) => `🔨️modules/${module}/🧪️tests/🔬️unit/🟦️.ts`)];
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
for (const file of files) {
  const source = readFileSync(resolve(pets, file), "utf8");
  const leads = [...source.matchAll(/^\s*\/\*\*\s*(\S+)/gmu)].map((match) => [...segmenter.segment(match[1])][0].segment);
  const plain = leads.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead));
  const twice = leads.filter((lead, index) => leads.indexOf(lead) !== index);
  const noisy = [...source.matchAll(/console\.\w+|\[DEBUG\]/gu)].map((match) => match[0]);
  problems += plain.length + twice.length + noisy.length;
  process.stdout.write(`${file}: ${leads.length} docstrings, without emoji ${JSON.stringify(plain)}, repeated ${JSON.stringify(twice)}, console or debug ${JSON.stringify(noisy)}\n`);
}
process.exitCode = problems === 0 ? 0 : 1;
