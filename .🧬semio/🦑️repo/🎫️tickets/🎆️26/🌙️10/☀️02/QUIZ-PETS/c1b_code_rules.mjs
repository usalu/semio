/** 🔎️ Check of work package C1b over the files it wrote or touched: every docstring starts with an emoji and no emoji starts two docstrings of one file; no line comment, no console call and no `[DEBUG]` log in product files and suites; no `innerHTML`, `outerHTML`, `cssText` or `setAttribute("style")` in product files. `node c1b_code_rules.mjs` from anywhere; exit code 1 on a finding. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const pets = resolve(here, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const react = "🎯️targets/⚛️react/🔨️modules";
const product = [`${react}/🪞️lifting/🟦️.ts`, `${react}/📡️survey/🟦️.ts`, `${react}/🖌️depiction/🟦️.ts`, `${react}/🫧️layer/🟦️.tsx`, `${react}/🧰️gear/🟦️.ts`, `${react}/✨️effects/🟦️.ts`, "🎯️targets/⚛️react/🟦️.tsx"].map((file) => resolve(pets, file));
const suites = ["🧪️tests/🪞️fixture-lifting/🟦️.tsx", "🧪️tests/📡️surface-survey/🟦️.tsx", "🧪️tests/🖌️pet-depiction/🟦️.tsx"].map((file) => resolve(pets, file));
const tools = ["c1b_browser_check.mjs", "c1b_code_rules.mjs", "c1b_module_names.ts", "c1b_harness/main.ts", "c1b_harness/vite.config.ts", "c1b_harness/pets_barrel.ts"].map((file) => resolve(here, file));
const python = [resolve(here, "generate_survey_vectors.py")];
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
const report = (file, findings) => {
  problems += findings.length;
  process.stdout.write(`${findings.length === 0 ? "clean" : findings.join("; ")} — ${file.slice(file.indexOf("🐾️pets") >= 0 ? file.indexOf("🐾️pets") : file.indexOf("QUIZ-PETS"))}\n`);
};
for (const file of [...product, ...suites, ...tools, ...python]) {
  const source = readFileSync(file, "utf8");
  const pattern = python.includes(file) ? /^\s*(?:def .*\n\s*)?"""(\S+)/gmu : /^\s*\/\*\*\s*(\S+)/gmu;
  const leads = [...source.matchAll(pattern)].map((match) => [...segmenter.segment(match[1])][0].segment);
  const findings = [...leads.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead)).map((lead) => `docstring without emoji: ${lead}`), ...leads.filter((lead, index) => leads.indexOf(lead) !== index).map((lead) => `emoji twice: ${lead}`)];
  if (product.includes(file) || suites.includes(file)) {
    for (const [index, line] of source.split("\n").entries()) {
      if (/^\s*\/\/(?!#(?:end)?region)/u.test(line)) findings.push(`line comment at ${index + 1}`);
      if (/\[DEBUG\]|console\.\w+\(/u.test(line) && !/spyOn\(console/u.test(line)) findings.push(`console or debug at ${index + 1}`);
    }
  }
  if (product.includes(file)) {
    for (const [index, line] of source.split("\n").entries()) {
      if (/^\s*(?:\/\*\*|\*)/u.test(line)) continue;
      if (/innerHTML|outerHTML|cssText|setAttribute\("style"/u.test(line)) findings.push(`forbidden at ${index + 1}: ${line.trim().slice(0, 60)}`);
    }
  }
  report(file, findings);
}
process.exitCode = problems === 0 ? 0 : 1;
