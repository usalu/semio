/** 🔎️ Check of work package B4 over the files it touched: every docstring starts with an emoji followed by U+FE0F, no emoji starts two docstrings of one file, no repository file writes to the console or keeps a `[DEBUG]` log, no comment sits inside a definition of the TypeScript files, and no declared name carries a banned stem. `node b4_code_rules.mjs` from anywhere; one line per file, exit code 1 on a finding — findings `known` from the last commit or from another work package's lines are listed but not counted. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const pets = resolve(ticket, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const typescript = ["🏞️terrain", "🧗️climbing", "💗️feeling", "🎲️randomness", "📝️draft", "👀️attention", "🎪️stage", "👥️population", "🚶️locomotion", "🕰️clock", "🎯️choice", "🧠️behavior", "💞️sociability", "🎥️projection", "🪄️mischief"];
const rust = ["🏞️terrain", "💗️feeling", "🎲️randomness", "🎪️stage", "👥️population", "🧠️behavior"];
const files = [
  ...typescript.map((module) => resolve(pets, "🔨️modules", module, "🟦️.ts")),
  ...rust.map((module) => resolve(pets, "🔨️modules", module, "🦀️.rs")),
  ...["🎪️stage", "🎥️projection", "🧗️climbing", "🪄️mischief", "🎲️randomness"].map((module) => resolve(pets, "🔨️modules", module, "🧪️tests/🔬️unit/🟦️.ts")),
  ...["🎪️stage", "🎲️randomness", "🧠️behavior"].map((module) => resolve(pets, "🔨️modules", module, "🧪️tests/🔬️unit/🦀️.rs")),
  resolve(pets, "🧬️schema/🟦️.ts"),
  resolve(pets, "🧬️schema/🦀️.rs"),
  resolve(pets, "🧬️schema/🧪️tests/🔬️unit/🦀️.rs"),
  ...["🎪️stage-trace", "🧗️wall-climbing", "🎣️grapple-reach", "🪜️ladder-geometry"].map((name) => resolve(pets, "🧪️tests", name, "🟦️.ts")),
  ...["🎪️stage-trace", "🧗️wall-climbing", "🎣️grapple-reach", "🧠️behavior-choice", "💗️feeling-dynamics"].map((name) => resolve(pets, "🧪️tests", name, "🐍️.py")),
  resolve(ticket, "generate_behavior_vectors.py"),
  resolve(ticket, "generate_climbing_vectors.py"),
  resolve(ticket, "stage_fuzz.ts"),
  resolve(ticket, "b4_explore.ts"),
  resolve(ticket, "b4_debug.ts"),
  resolve(ticket, "b4_trace_digests.ts"),
];
const banned = new Set(["core", "common", "util", "utils", "helper", "helpers", "misc", "shared", "base", "lib", "impl"]);
const known = new Set(["▭️", "base", "core", "println!", "eprintln!", "dbg!", "a_cast_shows_its_core_and_fills_up_with_the_rotation", "a_visitor_keeps_one_seat_from_two_seats_on_and_the_core_takes_turns_for_the_others", "a_core_without_a_rotation_gets_every_seat_and_so_does_a_rotation_without_a_core"]);
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
  const twice = [...new Set(leads.filter((lead, index) => leads.indexOf(lead) !== index))];
  const tool = file.includes("QUIZ-PETS");
  const noisy = tool ? [...source.matchAll(/\[DEBUG\]|console\.\w+/gu)].map((match) => match[0]) : [...source.matchAll(/console\.\w+|\[DEBUG\]|^\s*print\(|println!|eprintln!|dbg!/gmu)].map((match) => match[0]);
  const inner = /\.(ts|mjs)$/u.test(file) ? source.split("\n").filter((line) => /^\s+\/\/(?!#)/u.test(line)) : [];
  const names = [...source.matchAll(/\b(?:const|let|function|type|interface|class|fn|def|struct|enum|mod|static)\s+([A-Za-z_]\w*)/gu)].map((match) => match[1]);
  const stems = [...new Set(names.filter((name) => name.replace(/([a-z0-9])([A-Z])/gu, "$1_$2").toLowerCase().split("_").some((part) => banned.has(part))))];
  const found = [...plain, ...twice, ...noisy, ...stems].filter((finding) => !known.has(finding)).length + inner.length;
  problems += found;
  const name = file.slice(file.indexOf("🐾") >= 0 ? file.indexOf("🐾") : file.indexOf("QUIZ-PETS"));
  process.stdout.write(found === 0 && plain.length + twice.length + noisy.length + stems.length === 0 ? `${name}: ${leads.length} docstrings, clean\n` : found === 0 ? `${name}: ${leads.length} docstrings, clean but for known findings of others or of the last commit ${JSON.stringify([...plain, ...twice, ...noisy, ...stems])}\n` : `${name}: ${leads.length} docstrings, without emoji ${JSON.stringify(plain)}, repeated ${JSON.stringify(twice)}, console or debug ${JSON.stringify(noisy)}, comments inside ${inner.length}, banned stems ${JSON.stringify(stems)}\n`);
}
process.stdout.write(`problems: ${problems}\n`);
process.exitCode = problems === 0 ? 0 : 1;
