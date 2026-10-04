/** 🔎️ Check of work package A5: every docstring of its files starts with an emoji, no emoji starts two docstrings of one file, no `@emoji` start, nothing writes to the console in product code, no `[DEBUG]` is left and no comment sits inside a definition of product code. `node a5_docstring_check.mjs`. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const pets = resolve(here, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const product = ["🔨️modules/🚧️clearance/🟦️.ts", "🔨️modules/🚧️clearance/🧪️tests/🔬️unit/🟦️.ts", "🧪️tests/🚧️clearance-proof/🟦️.ts", "🧪️tests/🚧️clearance-proof/🐍️.py"].map((file) => resolve(pets, file));
const ticket = ["clearance_world.ts", "clearance_vectors_check.ts", "clearance_trace_check.py", "generate_clearance_vectors.py", "a5_docstring_check.mjs"].map((file) => resolve(here, file));
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
for (const file of [...product, ...ticket]) {
  const source = readFileSync(file, "utf8");
  const python = file.endsWith(".py");
  const leads = [...source.matchAll(python ? /^[ \t]*[ru]?"""(\S+)/gmu : /^\s*\/\*\*\s*(\S+)/gmu)].map((match) => [...segmenter.segment(match[1])][0].segment).filter((lead) => lead !== '"' && lead !== "\\");
  const plain = leads.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead));
  const twice = leads.filter((lead, index) => leads.indexOf(lead) !== index);
  const at = leads.filter((lead) => lead.startsWith("@"));
  const noisy = product.includes(file) ? [...source.matchAll(/console\.\w+|\[DEBUG\]|print\(/gu)].map((match) => match[0]) : file.endsWith("a5_docstring_check.mjs") ? [] : [...source.matchAll(/\[DEBUG\]/gu)].map((match) => match[0]);
  const inside = product.includes(file) && !python ? source.split("\n").filter((line) => /^\s{2,}\/\/(?!#)/.test(line)) : [];
  problems += plain.length + twice.length + at.length + noisy.length + inside.length;
  process.stdout.write(`${file.split(/[\\/]/).slice(-3).join("/")}: ${leads.length} docstrings, without emoji ${JSON.stringify(plain)}, repeated ${JSON.stringify(twice)}, @ ${JSON.stringify(at)}, console or debug ${JSON.stringify(noisy)}, comments inside ${inside.length}\n`);
}
process.exitCode = problems === 0 ? 0 : 1;
