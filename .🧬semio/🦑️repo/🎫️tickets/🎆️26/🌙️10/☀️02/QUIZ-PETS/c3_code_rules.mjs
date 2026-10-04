/** 🔎️ Check of work package C3 over the quiz files it touches: every docstring starts with an emoji, and the emojis that start two docstrings of one file are listed; no line comment inside the product files, no console call and no `[DEBUG]` log in product files and suites. `node c3_code_rules.mjs` from anywhere; prints one line per file and exit code 1 on a finding other than repeated emojis that were there before (`--baseline` prints them all without judging). */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const quiz = resolve(here, "../../../../../../../🧰️framework/🛍️products/❓️quiz");
const modules = "🎯️targets/⚛️react/🔨️modules";
const product = [
  `${modules}/🐾️pets/🟦️.tsx`,
  `${modules}/🎛️preferences/🟦️.tsx`,
  `${modules}/🌐️i18n/🟦️.ts`,
  `${modules}/🪟️chrome/🟦️.tsx`,
  `${modules}/📖️quiz-page/🟦️.tsx`,
  `${modules}/🗂️classification/🟦️.tsx`,
  `${modules}/↕️sorting/🟦️.tsx`,
  `${modules}/🃏️matching/🟦️.tsx`,
  `${modules}/▶️run/🟦️.tsx`,
  `${modules}/🏁️results/🟦️.tsx`,
  "🎯️targets/⚛️react/🟦️.tsx",
].map((file) => resolve(quiz, file));
const suites = ["🧪️tests/🐾️pet-companions/🟦️.tsx", "🧪️tests/🏠️home-grid/🟦️.tsx", "🧪️tests/📡️presence-client/🟦️.tsx"].map((file) => resolve(quiz, file));
const tools = ["c3_code_rules.mjs", "c3_selector_probe.ts", "c3_budget.ts", "c3_budget.vite.ts", "c3_preview_check.mjs", "quiz_pets_preview/main.tsx", "quiz_pets_preview/vite.config.ts"].map((file) => resolve(here, file));
const before = JSON.parse(process.env.C3_REPEATS ?? "{}");
const baseline = process.argv.includes("--baseline");
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
const repeats = {};
let problems = 0;
for (const file of [...product, ...suites, ...tools]) {
  const name = file.slice(file.indexOf("❓️quiz") >= 0 ? file.indexOf("❓️quiz") : file.indexOf("QUIZ-PETS"));
  const source = readFileSync(file, "utf8");
  const leads = [...source.matchAll(/^\s*\/\*\*\s*(\S+)/gmu)].map((match) => [...segmenter.segment(match[1])][0].segment);
  const findings = leads.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead)).map((lead) => `docstring without emoji: ${lead}`);
  const twice = [...new Set(leads.filter((lead, index) => leads.indexOf(lead) !== index))];
  repeats[name] = twice;
  const fresh = twice.filter((lead) => !(before[name] ?? []).includes(lead));
  if (!baseline) findings.push(...fresh.map((lead) => `emoji twice (new): ${lead}`));
  if (product.includes(file) || suites.includes(file)) {
    for (const [index, line] of source.split("\n").entries()) {
      if (product.includes(file) && /^\s*\/\/(?!#(?:end)?region)/u.test(line)) findings.push(`line comment at ${index + 1}`);
      if (/\[DEBUG\]|console\.\w+\(/u.test(line) && !/spyOn\(console|console, level/u.test(line)) findings.push(`console or debug at ${index + 1}`);
    }
  }
  problems += findings.length;
  process.stdout.write(`${findings.length === 0 ? "clean" : findings.join("; ")}${twice.length === 0 ? "" : ` (repeated: ${twice.join(" ")})`} — ${name}\n`);
}
if (baseline) process.stdout.write(`${JSON.stringify(repeats)}\n`);
process.exitCode = problems === 0 ? 0 : 1;
