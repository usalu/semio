/** 🔎️ Check of work package C3b over the files it wrote or edited: every docstring starts with an emoji, no emoji starts two docstrings of one file, no line comment (but `#region` markers) in product files, no console call and no `[DEBUG]` anywhere. `node c3b_code_rules.mjs` from anywhere; one line per file, exit code 1 on any finding. Repeated emojis of the package barrel, which many tickets share, are listed with the docstrings they start so their owner can tell them apart. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const react = resolve(here, "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react");
const product = ["🔨️modules/🐾️pets/🎪️stage/🟦️.tsx", "🔨️modules/🐾️pets/🟦️.tsx", "🔨️modules/🎛️preferences/🟦️.tsx", "🟦️.tsx"].map((file) => resolve(react, file));
const tools = ["c3b_code_rules.mjs", "c3b_budget.ts", "c3b_budget.vite.ts"].map((file) => resolve(here, file));
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
for (const file of [...product, ...tools]) {
  const name = file.slice(file.indexOf("⚛️react") >= 0 ? file.indexOf("⚛️react") : file.indexOf("QUIZ-PETS"));
  const source = readFileSync(file, "utf8");
  const docstrings = [...source.matchAll(/^\s*\/\*\*\s*(\S+)([^\n]*)/gmu)].map((match) => ({ lead: [...segmenter.segment(match[1])][0].segment, text: `${match[1]}${match[2]}`.slice(0, 60) }));
  const leads = docstrings.map((docstring) => docstring.lead);
  const findings = docstrings.filter((docstring) => !/\p{Extended_Pictographic}/u.test(docstring.lead) || docstring.lead.startsWith("@")).map((docstring) => `docstring without emoji: ${docstring.text}`);
  const twice = [...new Set(leads.filter((lead, index) => leads.indexOf(lead) !== index))];
  findings.push(...twice.map((lead) => `emoji twice: ${docstrings.filter((docstring) => docstring.lead === lead).map((docstring) => docstring.text).join(" | ")}`));
  for (const [index, line] of source.split("\n").entries()) {
    if (product.includes(file) && /^\s*\/\/(?!#(?:end)?region)/u.test(line)) findings.push(`line comment at ${index + 1}`);
    if (/\[DEBUG\]|console\.\w+\(/u.test(line) && file !== tools[0]) findings.push(`console or debug at ${index + 1}`);
  }
  problems += findings.length;
  process.stdout.write(`${findings.length === 0 ? "clean" : findings.join("\n    ")} — ${name}\n`);
}
process.exitCode = problems === 0 ? 0 : 1;
