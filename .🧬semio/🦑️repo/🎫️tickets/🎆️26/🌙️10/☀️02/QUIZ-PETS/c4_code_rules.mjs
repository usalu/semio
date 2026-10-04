/** 🔎️ Check of work package C4 over the files it wrote: every docstring starts with an emoji followed by U+FE0F, no emoji starts two docstrings of one file, no line comment, no console call and no `[DEBUG]` log in the suites, and the ensemble is valid JSON. `node c4_code_rules.mjs` from anywhere; prints one line per file and exit code 1 on a finding. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const site = resolve(here, "../../../../../../../🎓️teaching/🏛️architecture");
const suites = ["❓️quiz/🧪️tests/🐾️pet-cast/🟦️.ts", "❓️quiz/🧪️tests/🐕️pet-walk/🟦️.ts"].map((file) => resolve(site, file));
const tools = ["c4_species_digest.ts", "c4_validate_menagerie.ts", "c4_pet_cast_mutants.ts", "c4_pet_cast_mutants.config.ts", "c4_probe.mjs", "c4_move_speed.mjs", "c4_code_rules.mjs"].map((file) => resolve(here, file));
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
for (const file of [...suites, ...tools]) {
  const source = readFileSync(file, "utf8");
  const leads = [...source.matchAll(/^\s*\/\*\*\s*(\S+)/gmu)].map((match) => [...segmenter.segment(match[1])][0].segment);
  const findings = leads.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead) || !lead.endsWith("\u{FE0F}")).map((lead) => `docstring without emoji and U+FE0F: ${lead}`);
  findings.push(...[...new Set(leads.filter((lead, index) => leads.indexOf(lead) !== index))].map((lead) => `emoji twice: ${lead}`));
  if (suites.includes(file)) {
    for (const [index, line] of source.split("\n").entries()) {
      if (/^\s*\/\/|[;{)]\s*\/\/ /u.test(line)) findings.push(`line comment at ${index + 1}`);
      if (/\[DEBUG\]|console\.\w+\(/u.test(line)) findings.push(`console or debug at ${index + 1}`);
    }
  }
  problems += findings.length;
  process.stdout.write(`${findings.length === 0 ? "clean" : findings.join("; ")} — ${file.slice(file.indexOf("🎓️") >= 0 ? file.indexOf("🎓️") : file.indexOf("QUIZ-PETS"))} (${leads.length} docstrings)\n`);
}
JSON.parse(readFileSync(resolve(site, "🐾️pets/🔣️.json"), "utf8"));
process.stdout.write("ensemble: valid JSON\n");
process.exitCode = problems === 0 ? 0 : 1;
