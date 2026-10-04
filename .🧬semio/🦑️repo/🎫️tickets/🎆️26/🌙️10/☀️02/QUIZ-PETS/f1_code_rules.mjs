/** 🔎️ Check of work package F1 over the files it touched: every docstring starts with an emoji followed by U+FE0F, no emoji starts two docstrings of one file, no repository file writes to the console or keeps a `[DEBUG]` log, no comment sits inside a definition of the TypeScript files, and no declared name carries a banned stem. `node f1_code_rules.mjs` from anywhere; one line per file, exit code 1 on a finding — findings `known` from the last commit or from another work package's lines are listed but not counted. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const pets = resolve(root, "🧰️framework/🛍️products/🐾️pets");
const files = [
  ...["📏️spacing", "🎥️projection", "🚶️locomotion", "🧗️climbing", "👆️gesture", "🎯️choice", "🗓️schedule", "💞️sociability", "👥️population", "👀️attention", "🕰️clock", "🎪️stage"].map((module) => resolve(pets, "🔨️modules", module, "🟦️.ts")),
  ...["🧗️climbing", "🎪️stage", "🎥️projection"].map((module) => resolve(pets, "🔨️modules", module, "🧪️tests/🔬️unit/🟦️.ts")),
  resolve(pets, "🧬️schema/🟦️.ts"),
  resolve(pets, "🧬️schema/🦀️.rs"),
  resolve(pets, "🧬️schema/🧪️tests/🔬️unit/🦀️.rs"),
  resolve(pets, "🧪️tests/🫥️decorative-layer/🟦️.tsx"),
  resolve(pets, "🧪️tests/🪜️ladder-geometry/🐍️.py"),
  resolve(pets, "🧪️tests/🪜️ladder-geometry/🟦️.ts"),
  resolve(pets, "🧪️tests/🎣️grapple-reach/🐍️.py"),
  resolve(pets, "🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx"),
  resolve(pets, "🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts"),
  resolve(pets, "🎯️targets/⚛️react/📖️stories/🟦️.tsx"),
  resolve(root, "🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🐕️pet-walk/🟦️.ts"),
  ...["stage_fuzz.ts", "generate_climbing_vectors.py", "f1_routes.ts", "f1_gear_probe.ts", "f1_aims_probe.ts", "f1_rope_story.ts", "f1_rope_rounds.ts", "f1_trace_diff.ts", "f1_frame_diff.ts", "f1_survey.mjs", "f1_survey_look.ts", "f1_emojis.mjs", "f1_before.config.ts"].map((name) => resolve(ticket, name)),
];
const banned = new Set(["core", "common", "util", "utils", "helper", "helpers", "misc", "shared", "base", "lib", "impl"]);
const known = new Set(["▭️", "base", "core", "println!", "eprintln!", "dbg!", "console.error", "console.log", "console.info", "console.warn", "console.debug", "console.trace"]);
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
  const inner = /\.(ts|tsx|mjs)$/u.test(file) ? source.split("\n").filter((line) => /^\s+\/\/(?!#)/u.test(line)) : [];
  const names = [...source.matchAll(/\b(?:const|let|function|type|interface|class|fn|def|struct|enum|mod|static)\s+([A-Za-z_]\w*)/gu)].map((match) => match[1]);
  const stems = [...new Set(names.filter((name) => name.replace(/([a-z0-9])([A-Z])/gu, "$1_$2").toLowerCase().split("_").some((part) => banned.has(part))))];
  const found = [...plain, ...twice, ...noisy, ...stems].filter((finding) => !known.has(finding)).length + inner.length;
  problems += found;
  const name = file.slice(Math.max(file.indexOf("🐾"), file.indexOf("❓"), file.indexOf("QUIZ-PETS")));
  process.stdout.write(found === 0 && plain.length + twice.length + noisy.length + stems.length === 0 ? `${name}: ${leads.length} docstrings, clean\n` : found === 0 ? `${name}: ${leads.length} docstrings, clean but for known findings of others or of the last commit ${JSON.stringify([...new Set([...plain, ...twice, ...noisy, ...stems])])}\n` : `${name}: ${leads.length} docstrings, without emoji ${JSON.stringify(plain)}, repeated ${JSON.stringify(twice)}, console or debug ${JSON.stringify(noisy)}, comments inside ${inner.length}, banned stems ${JSON.stringify(stems)}\n`);
}
process.stdout.write(`problems: ${problems}\n`);
process.exitCode = problems === 0 ? 0 : 1;
