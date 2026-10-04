/** 🔎️ Check of work package A9 over the files it wrote or touched: every docstring starts with an emoji and no emoji starts two docstrings of one file; no line comment, no console call, no `[DEBUG]` log, no `id` attribute, `<defs>`, `<use>`, `innerHTML`, `cssText` or `<style>` in the product files. `node wp_a9_code_rules.mjs` from anywhere; exit code 1 on a finding. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const pets = resolve(here, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const product = ["🎯️targets/⚛️react/🔨️modules/🧰️gear/🟦️.ts", "🎯️targets/⚛️react/🔨️modules/✨️effects/🟦️.ts", "🎯️targets/⚛️react/🔨️modules/🖌️depiction/🟦️.ts"].map((file) => resolve(pets, file));
const suites = ["🧪️tests/🧰️gear-depiction/🟦️.tsx", "🧪️tests/🎆️effect-painting/🟦️.tsx"].map((file) => resolve(pets, file));
const tools = ["render_species_preview.mjs", "render_species_preview.entry.ts", "wp_a9_gear_sheet.mjs", "wp_a9_gear_sheet.entry.ts", "wp_a9_mutants.ts", "wp_a9_vitest.config.ts", "wp_a9_code_rules.mjs"].map((file) => resolve(here, file));
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
for (const file of [...product, ...suites, ...tools]) {
  const source = readFileSync(file, "utf8");
  const leads = [...source.matchAll(/^\s*\/\*\*\s*(\S+)/gmu)].map((match) => [...segmenter.segment(match[1])][0].segment);
  const plain = leads.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead));
  const twice = leads.filter((lead, index) => leads.indexOf(lead) !== index);
  const findings = [...plain.map((lead) => `docstring without emoji: ${lead}`), ...twice.map((lead) => `emoji twice: ${lead}`)];
  if (product.includes(file) || suites.includes(file)) {
    for (const [index, line] of source.split("\n").entries()) {
      if (/^\s*\/\/(?!#(?:end)?region)/u.test(line)) findings.push(`line comment at ${index + 1}`);
      if (/\[DEBUG\]|console\.\w+\(/u.test(line)) findings.push(`console or debug at ${index + 1}`);
    }
  }
  if (product.includes(file)) {
    for (const [index, line] of source.split("\n").entries()) {
      if (/^\s*(?:\/\*\*|\*)/u.test(line)) continue;
      if (/innerHTML|outerHTML|cssText|"defs"|"use"|"style"|\["id"|setAttribute\("id"|Math\.(?:sin|cos|atan2?|tan)\(/u.test(line)) findings.push(`forbidden at ${index + 1}: ${line.trim().slice(0, 60)}`);
    }
  }
  problems += findings.length;
  process.stdout.write(`${file.slice(file.lastIndexOf("QUIZ-PETS") > 0 ? file.lastIndexOf("QUIZ-PETS") + 10 : pets.length + 1)}: ${leads.length} docstrings, ${findings.length === 0 ? "clean" : findings.join("; ")}\n`);
}
process.exitCode = problems === 0 ? 0 : 1;
