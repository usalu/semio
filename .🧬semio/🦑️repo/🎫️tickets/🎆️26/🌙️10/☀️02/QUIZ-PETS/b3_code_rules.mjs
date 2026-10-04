/** 🔎️ Check of work package B3: every docstring of its files starts with an emoji followed by U+FE0F, no emoji starts two docstrings of one file, no repository file writes to the console or keeps a `[DEBUG]` log, no comment sits inside a definition of the TypeScript files, and the projection uses none of the forbidden platform maths. `node b3_code_rules.mjs` from anywhere. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const pets = resolve(ticket, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const files = [
  resolve(pets, "🔨️modules/🎥️projection/🟦️.ts"),
  resolve(pets, "🔨️modules/🎥️projection/🧪️tests/🔬️unit/🟦️.ts"),
  resolve(pets, "🔨️modules/📝️draft/🟦️.ts"),
  resolve(pets, "🔨️modules/👥️population/🟦️.ts"),
  resolve(pets, "🔨️modules/👥️population/🦀️.rs"),
  resolve(pets, "🔨️modules/🎪️stage/🧪️tests/🔬️unit/🟦️.ts"),
  resolve(pets, "🧬️schema/🟦️.ts"),
  resolve(pets, "🧬️schema/🦀️.rs"),
  resolve(pets, "🧪️tests/🎪️stage-trace/🟦️.ts"),
  resolve(pets, "🧪️tests/🎪️stage-trace/🐍️.py"),
  resolve(ticket, "b3_measure.ts"),
  resolve(ticket, "b3_trace_survey.ts"),
  resolve(ticket, "b3_code_rules.mjs"),
];
const forbidden = /Math\.(sin|cos|tan|atan2?|exp|pow|hypot|log|random)\b|\bDate\b|performance\./gu;
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
for (const file of files) {
  const source = readFileSync(file, "utf8");
  const pattern = file.endsWith(".py") ? /^[ \t]*(?:"""|r""")[ \t]*(\S+)/gmu : file.endsWith(".rs") ? /^[ \t]*\/\/[/!][ \t]*(\S+)/gmu : /^[ \t]*\/\*\*[ \t]*(\S+)/gmu;
  const leads = [];
  if (file.endsWith(".rs")) {
    let previous = false;
    for (const line of source.split("\n")) {
      const match = /^[ \t]*\/\/[/!][ \t]*(\S+)/u.exec(line);
      if (match !== null && !previous) leads.push([...segmenter.segment(match[1])][0].segment);
      previous = /^[ \t]*\/\/[/!]/u.test(line) || (previous && /^[ \t]*#\[/u.test(line));
    }
  } else for (const match of source.matchAll(pattern)) leads.push([...segmenter.segment(match[1])][0].segment);
  const plain = leads.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead) || !lead.endsWith("️"));
  const twice = leads.filter((lead, index) => leads.indexOf(lead) !== index);
  const tool = file.includes("QUIZ-PETS");
  const noisy = tool ? [...source.matchAll(/\[DEBUG\]/gu)].map((match) => match[0]).filter(() => !file.endsWith("b3_code_rules.mjs")) : [...source.matchAll(/console\.\w+|\[DEBUG\]|\bprint\(|println!|eprintln!|dbg!/gu)].map((match) => match[0]);
  const inner = file.endsWith(".ts") ? source.split("\n").filter((line) => /^\s+\/\/(?!#)/u.test(line)) : [];
  const maths = file.endsWith("🎥️projection/🟦️.ts") ? [...source.matchAll(forbidden)].map((match) => match[0]) : [];
  const found = plain.length + twice.length + noisy.length + inner.length + maths.length;
  problems += found;
  const name = file.slice(file.indexOf("🐾") >= 0 ? file.indexOf("🐾") : file.indexOf("QUIZ-PETS"));
  process.stdout.write(found > 0 ? `${name}: ${leads.length} docstrings, without emoji ${JSON.stringify(plain)}, repeated ${JSON.stringify(twice)}, console or debug ${JSON.stringify(noisy)}, comments inside ${inner.length}, forbidden maths ${JSON.stringify(maths)}\n` : `${name}: ${leads.length} docstrings, clean\n`);
}
process.stdout.write(`problems: ${problems}\n`);
process.exitCode = problems === 0 ? 0 : 1;
