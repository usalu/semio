/** 🔎️ Check of work package C1a over the files it wrote or touched: every docstring starts with an emoji and no emoji starts two docstrings of one file; no line comment inside code (regions aside), no console call, no temporary debug log; in product files no `innerHTML`, `outerHTML`, `cssText`, `setAttribute("style"` or `<style>`. `node c1a_code_rules.mjs` from anywhere; exit code 1 on a finding. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const pets = resolve(here, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const product = ["🎯️targets/⚛️react/🔨️modules/🤏️grasp/🟦️.ts", "🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts", "🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx", "🎯️targets/⚛️react/🟦️.tsx"].map((file) => resolve(pets, file));
const suites = ["🧪️tests/🤏️pet-handling/🟦️.tsx", "🧪️tests/🫥️decorative-layer/🟦️.tsx", "🧪️tests/📡️surface-survey/🟦️.tsx", "🧪️tests/🧰️gear-depiction/🟦️.tsx"].map((file) => resolve(pets, file));
const tools = ["c1a_harness/main.tsx", "c1a_harness/vite.config.ts", "c1a_harness/c1a_pets_recorder.ts", "c1a_drive.mjs", "c1a_probe_stage.ts", "c1a_code_rules.mjs"].map((file) => resolve(here, file));
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
for (const file of [...product, ...suites, ...tools]) {
  const source = readFileSync(file, "utf8");
  const leads = [...source.matchAll(/^\s*\/\*\*\s*(\S+)/gmu)].map((match) => [...segmenter.segment(match[1])][0].segment);
  const plain = leads.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead));
  const twice = leads.filter((lead, index) => leads.indexOf(lead) !== index);
  const findings = [...plain.map((lead) => `docstring without emoji: ${lead}`), ...twice.map((lead) => `emoji twice: ${lead}`)];
  for (const [index, line] of source.split("\n").entries()) {
    if (/^\s*\/\/(?!#(?:end)?region| #(?:end)?region)/u.test(line)) findings.push(`line comment at ${index + 1}`);
    if (/\[DEBUG\]/u.test(line) || (!tools.includes(file) && /console\.\w+\(/u.test(line))) findings.push(`console or debug at ${index + 1}`);
    if (product.includes(file) && !/^\s*(?:\/\*\*|\*)/u.test(line) && /innerHTML|outerHTML|cssText|setAttribute\("style"|"style"/u.test(line)) findings.push(`forbidden at ${index + 1}: ${line.trim().slice(0, 60)}`);
  }
  problems += findings.length;
  process.stdout.write(`${file.includes("QUIZ-PETS") ? file.slice(file.indexOf("QUIZ-PETS") + 10) : file.slice(pets.length + 1)}: ${leads.length} docstrings, ${findings.length === 0 ? "clean" : findings.join("; ")}\n`);
}
process.exitCode = problems === 0 ? 0 : 1;
