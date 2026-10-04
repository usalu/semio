/** 🔎️ Check of work package A4: every docstring of the files it wrote starts with an emoji, no emoji starts two docstrings of one file, none of them writes to the console or keeps a `[DEBUG]` log, and no line inside a definition is a comment. `node a4_docstring_emojis.mjs` from anywhere. */
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const pets = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const files = [
  "🔨️modules/🏞️terrain/🟦️.ts",
  "🔨️modules/🏞️terrain/🧪️tests/🔬️unit/🟦️.ts",
  "🔨️modules/🧗️climbing/🟦️.ts",
  "🔨️modules/🧗️climbing/🧪️tests/🔬️unit/🟦️.ts",
  ...["🧗️wall-climbing", "🪜️ladder-geometry", "🎣️grapple-reach"].flatMap((directory) => [`🧪️tests/${directory}/🟦️.ts`, `🧪️tests/${directory}/🐍️.py`]),
];
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
for (const file of files) {
  const path = resolve(pets, file);
  if (!existsSync(path)) {
    process.stdout.write(`${file}: missing\n`);
    continue;
  }
  const source = readFileSync(path, "utf8");
  const python = file.endsWith(".py");
  const opening = python ? /^[ \t]*(?:[ru]?"""|#:)[ \t]*(\S+)/gmu : /^[ \t]*\/\*\*[ \t]*(\S+)/gmu;
  const leads = [...source.matchAll(opening)].map((match) => [...segmenter.segment(match[1])][0].segment);
  const plain = leads.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead));
  const twice = leads.filter((lead, index) => leads.indexOf(lead) !== index);
  const noisy = [...source.matchAll(/console\.\w+|\[DEBUG\]|\bprint\(/gu)].map((match) => match[0]);
  const comments = python ? source.split("\n").filter((line) => /^\s+#(?! (?:region|endregion)\b)/u.test(line)) : source.split("\n").filter((line) => /^\s+\/\/(?!#(?:region|endregion)\b)/u.test(line));
  problems += plain.length + twice.length + noisy.length + comments.length;
  process.stdout.write(`${file}: ${leads.length} docstrings, without emoji ${JSON.stringify(plain)}, repeated ${JSON.stringify(twice)}, console or debug ${JSON.stringify(noisy)}, comments inside ${comments.length}\n`);
}
process.exitCode = problems === 0 ? 0 : 1;
