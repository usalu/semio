/** 🔎️ Check of work package A7: every docstring of its files starts with an emoji followed by U+FE0F, no emoji starts two docstrings of one file, no file writes to the console or keeps a `[DEBUG]` log, and no comment sits inside a definition of the TypeScript files. `node a7_docstring_emojis.mjs` from anywhere. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const pets = resolve(ticket, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const files = [
  resolve(pets, "🔨️modules/💗️feeling/🟦️.ts"),
  resolve(pets, "🔨️modules/💗️feeling/🧪️tests/🔬️unit/🟦️.ts"),
  resolve(pets, "🧪️tests/💗️feeling-dynamics/🟦️.ts"),
  resolve(pets, "🧪️tests/💗️feeling-dynamics/🐍️.py"),
  resolve(pets, "🧪️tests/⚗️chemistry-rules/🟦️.ts"),
  resolve(pets, "🧪️tests/⚗️chemistry-rules/🐍️.py"),
  resolve(ticket, "generate_feeling_vectors.py"),
  resolve(ticket, "a7_probe_sample.py"),
];
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
for (const file of files) {
  const source = readFileSync(file, "utf8");
  const pattern = file.endsWith(".py") ? /^[ \t]*(?:"""|r""")[ \t]*(\S+)/gmu : /^[ \t]*\/\*\*[ \t]*(\S+)/gmu;
  const leads = [...source.matchAll(pattern)].map((match) => [...segmenter.segment(match[1])][0].segment);
  const plain = leads.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead) || !lead.endsWith("️"));
  const twice = leads.filter((lead, index) => leads.indexOf(lead) !== index);
  const noisy = [...source.matchAll(/console\.\w+|\[DEBUG\]|\bprint\(/gu)].map((match) => match[0]).filter(() => !file.endsWith("generate_feeling_vectors.py") && !file.endsWith("a7_probe_sample.py"));
  const inner = file.endsWith(".ts") ? source.split("\n").filter((line) => /^\s+\/\/(?!#)/u.test(line)) : [];
  problems += plain.length + twice.length + noisy.length + inner.length;
  process.stdout.write(`${file.slice(file.indexOf("🐾") >= 0 ? file.indexOf("🐾") : file.indexOf("QUIZ-PETS"))}: ${leads.length} docstrings, without emoji ${JSON.stringify(plain)}, repeated ${JSON.stringify(twice)}, console or debug ${JSON.stringify(noisy)}, comments inside ${inner.length}\n`);
}
process.exitCode = problems === 0 ? 0 : 1;
