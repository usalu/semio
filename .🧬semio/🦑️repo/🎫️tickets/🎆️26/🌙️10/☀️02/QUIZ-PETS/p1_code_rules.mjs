/** 🔎️ Check of work package P1 over the files it wrote or touched: every docstring starts with an emoji and no emoji starts two docstrings of one file; no line comment, no console call and no `[DEBUG]` log in product files and suites; no `innerHTML`, `outerHTML`, `cssText` or `setAttribute("style")` in product files; no `[DEBUG]` left in the ticket tools. `node p1_code_rules.mjs` from anywhere; exit code 1 on a finding. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const pets = resolve(here, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const react = "🎯️targets/⚛️react/🔨️modules";
const product = [`${react}/📡️survey/🟦️.ts`, `${react}/🖌️depiction/🟦️.ts`, `${react}/🤏️grasp/🟦️.ts`, `${react}/🫧️layer/🟦️.tsx`, "🎯️targets/⚛️react/🟦️.tsx"].map((file) => resolve(pets, file));
const suites = ["🧪️tests/📡️surface-survey/🟦️.tsx", "🧪️tests/🖌️pet-depiction/🟦️.tsx", "🧪️tests/🤏️pet-handling/🟦️.tsx", "🧪️tests/🧰️gear-depiction/🟦️.tsx", "🧪️tests/🫥️decorative-layer/🟦️.tsx"].map((file) => resolve(pets, file));
const tools = ["p1_measure.mjs", "p1_table.mjs", "p1_trace_threads.mjs", "p1_trace_paint.mjs", "p1_style_probe.mjs", "p1_write_probe.mjs", "p1_mutation_probe.mjs", "p1_gpu_probe.mjs", "p1_hand_probe.mjs", "p1_hand_check.mjs", "p1_rules_probe.mjs", "p1_species_shape.ts", "p1_code_rules.mjs"].map((file) => resolve(here, file));
const sheet = resolve(pets, "🎯️targets/⚛️react/🎨️.css");
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
const rules = readFileSync(sheet, "utf8");
const sheetFindings = [...(/\[DEBUG\]/u.test(rules) ? ["debug text"] : []), ...(rules.replace(/\/\*[\s\S]*?\*\//gu, "").includes("data-pet-cursor") ? ["a rule names data-pet-cursor"] : [])];
problems += sheetFindings.length;
process.stdout.write(`${sheetFindings.length === 0 ? "clean" : sheetFindings.join("; ")} — 🐾️pets/🎯️targets/⚛️react/🎨️.css\n`);
for (const file of [...product, ...suites, ...tools]) {
  const source = readFileSync(file, "utf8");
  const leads = [...source.matchAll(/^\s*\/\*\*\s*(\S+)/gmu)].map((match) => [...segmenter.segment(match[1])][0].segment);
  const findings = [...leads.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead)).map((lead) => `docstring without emoji: ${lead}`), ...leads.filter((lead, index) => leads.indexOf(lead) !== index).map((lead) => `emoji twice: ${lead}`)];
  for (const [index, line] of source.split("\n").entries()) {
    if (/\[DEBUG\]/u.test(line) && !tools.includes(file)) findings.push(`debug at ${index + 1}`);
    if (tools.includes(file)) continue;
    if (/^\s*\/\/(?!#(?:end)?region)/u.test(line)) findings.push(`line comment at ${index + 1}`);
    if (/console\.\w+\(/u.test(line) && !/spyOn\(console|console\[method\]/u.test(line)) findings.push(`console at ${index + 1}`);
    if (product.includes(file) && !/^\s*(?:\/\*\*|\*)/u.test(line) && /innerHTML|outerHTML|cssText|setAttribute\("style"/u.test(line)) findings.push(`forbidden at ${index + 1}: ${line.trim().slice(0, 60)}`);
  }
  if (tools.includes(file) && /\[DEBUG\] /u.test(source)) findings.push("a temporary debug log is left");
  problems += findings.length;
  process.stdout.write(`${findings.length === 0 ? "clean" : findings.join("; ")} — ${file.slice(Math.max(file.indexOf("🐾️pets"), file.indexOf("QUIZ-PETS")))}\n`);
}
process.exitCode = problems === 0 ? 0 : 1;
