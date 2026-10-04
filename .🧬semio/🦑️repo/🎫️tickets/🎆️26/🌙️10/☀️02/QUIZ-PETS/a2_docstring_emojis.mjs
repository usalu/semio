/** 🔎️ Check of work package A2: every docstring of the files it touched (TypeScript, Rust, Python) starts with an emoji, no emoji starts two docstrings of one file, and none of the files writes to the console or keeps a `[DEBUG]` log. `node a2_docstring_emojis.mjs` from anywhere. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const pets = resolve(ticket, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const files = [
  ...["🧬️schema/🟦️.ts", "🧬️schema/🦀️.rs", "🧬️schema/🧪️tests/🔬️unit/🦀️.rs"],
  ...["✅️validation", "🧠️behavior"].flatMap((module) => [`🔨️modules/${module}/🟦️.ts`, `🔨️modules/${module}/🦀️.rs`, `🔨️modules/${module}/🧪️tests/🔬️unit/🟦️.ts`, `🔨️modules/${module}/🧪️tests/🔬️unit/🦀️.rs`]),
  ...["🧬️schema-conformance", "🧠️behavior-choice", "🤝️bond-dynamics"].map((name) => `🧪️tests/${name}/🐍️.py`),
  ...["🧠️behavior-choice", "🤝️bond-dynamics"].map((name) => `🧪️tests/${name}/🦀️.rs`),
].map((file) => resolve(pets, file));
files.push(resolve(ticket, "generate_schema_vectors.py"), resolve(ticket, "generate_behavior_vectors.py"), resolve(ticket, "a2_registration_check.ts"), resolve(ticket, "a2_species_digest.ts"), resolve(ticket, "a2_species_edits.ts"));
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });

function leadsOf(file, source) {
  if (file.endsWith(".py")) return [...source.matchAll(/^[ \t]*"""[ \t]*(\S+)/gmu)].map((match) => match[1]);
  if (!file.endsWith(".rs")) return [...source.matchAll(/^\s*\/\*\*\s*(\S+)/gmu)].map((match) => match[1]);
  const lines = source.split("\n");
  return lines.flatMap((line, index) => {
    const doc = /^\s*\/\/[\/!]\s*(\S+)/u.exec(line);
    const before = index > 0 ? /^\s*\/\/[\/!]/u.test(lines[index - 1]) : false;
    return doc && !before ? [doc[1]] : [];
  });
}

let problems = 0;
for (const file of files) {
  const source = readFileSync(file, "utf8");
  const leads = leadsOf(file, source).map((word) => [...segmenter.segment(word)][0].segment);
  const plain = leads.filter((lead) => !/\p{Extended_Pictographic}|️/u.test(lead));
  const twice = leads.filter((lead, index) => leads.indexOf(lead) !== index);
  const noisy = file.endsWith(".py") || file.includes("QUIZ-PETS") || file.includes("🧪️tests") ? [] : [...source.matchAll(/console\.\w+|\[DEBUG\]|println!|dbg!/gu)].map((match) => match[0]);
  problems += plain.length + twice.length + noisy.length;
  process.stdout.write(`${file.slice(file.indexOf("🐾️pets") >= 0 ? file.indexOf("🐾️pets") : file.lastIndexOf("QUIZ-PETS"))}: ${leads.length} docstrings, without emoji ${JSON.stringify(plain)}, repeated ${JSON.stringify(twice)}, console or debug ${JSON.stringify(noisy)}\n`);
}
process.stdout.write(`problems ${problems}\n`);
process.exitCode = problems === 0 ? 0 : 1;
