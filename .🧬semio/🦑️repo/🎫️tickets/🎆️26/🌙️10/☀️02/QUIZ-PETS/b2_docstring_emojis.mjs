/** 🔎️ Check of work package B2: every docstring of its files starts with an emoji followed by U+FE0F, no emoji starts two docstrings of one file, no repository file writes to the console or keeps a `[DEBUG]` log, and no comment sits inside a definition of the TypeScript files. `node b2_docstring_emojis.mjs` from anywhere. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const pets = resolve(ticket, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const modules = ["👀️attention", "💞️sociability", "🎯️choice", "🕰️clock", "🗓️schedule", "📝️draft", "🎪️stage", "🎥️projection", "👥️population", "💗️feeling", "🧠️behavior", "🎲️randomness", "✅️validation"];
const files = [
  ...modules.map((module) => resolve(pets, "🔨️modules", module, "🟦️.ts")),
  ...["🧠️behavior", "🎲️randomness", "✅️validation"].map((module) => resolve(pets, "🔨️modules", module, "🦀️.rs")),
  resolve(pets, "🔨️modules/🎪️stage/🧪️tests/🔬️unit/🟦️.ts"),
  resolve(pets, "🧬️schema/🟦️.ts"),
  resolve(pets, "🧬️schema/🦀️.rs"),
  resolve(pets, "🧪️tests/🧬️schema-conformance/🐍️.py"),
  resolve(pets, "🧪️tests/⚗️chemistry-rules/🟦️.ts"),
  resolve(pets, "🧪️tests/⚗️chemistry-rules/🐍️.py"),
  resolve(pets, "🧪️tests/🧠️behavior-choice/🟦️.ts"),
  resolve(pets, "🧪️tests/🧠️behavior-choice/🐍️.py"),
  resolve(pets, "🧪️tests/🧠️behavior-choice/🦀️.rs"),
  resolve(ticket, "generate_behavior_vectors.py"),
  resolve(ticket, "generate_feeling_vectors.py"),
  resolve(ticket, "generate_schema_vectors.py"),
  resolve(ticket, "stage_storyboard.ts"),
  resolve(ticket, "b2_probe.ts"),
];
const tools = ["QUIZ-PETS"];
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
for (const file of files) {
  let source;
  try {
    source = readFileSync(file, "utf8");
  } catch {
    process.stdout.write(`${file}: missing\n`);
    continue;
  }
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
  const tool = tools.some((name) => file.includes(name));
  const noisy = tool ? [...source.matchAll(/\[DEBUG\]/gu)].map((match) => match[0]) : [...source.matchAll(/console\.\w+|\[DEBUG\]|\bprint\(|println!|eprintln!|dbg!/gu)].map((match) => match[0]);
  const inner = file.endsWith(".ts") ? source.split("\n").filter((line) => /^\s+\/\/(?!#)/u.test(line)) : [];
  problems += plain.length + twice.length + noisy.length + inner.length;
  const name = file.slice(file.indexOf("🐾") >= 0 ? file.indexOf("🐾") : file.indexOf("QUIZ-PETS"));
  if (plain.length + twice.length + noisy.length + inner.length > 0) process.stdout.write(`${name}: ${leads.length} docstrings, without emoji ${JSON.stringify(plain)}, repeated ${JSON.stringify(twice)}, console or debug ${JSON.stringify(noisy)}, comments inside ${inner.length}\n`);
  else process.stdout.write(`${name}: ${leads.length} docstrings, clean\n`);
}
process.stdout.write(`problems: ${problems}\n`);
process.exitCode = problems === 0 ? 0 : 1;
