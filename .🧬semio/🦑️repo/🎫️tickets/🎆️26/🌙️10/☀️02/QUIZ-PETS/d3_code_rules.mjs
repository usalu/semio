/** 🔎️ Check of work package D3 over the files it wrote: every docstring starts with an emoji followed by U+FE0F (never `@emoji`), no emoji starts two docstrings of one file, no product file writes to the console or keeps a `[DEBUG]` log, no comment sits inside a definition (Rust: a `//` line that is neither a docstring nor a region mark), no line ends in a carriage return, and no declared name carries a banned stem. `node d3_code_rules.mjs` from anywhere; one line per file, exit code 1 on a finding. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const pets = resolve(ticket, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const product = [
  ...["💗️feeling", "✨️effects", "🪄️mischief"].flatMap((module) => [resolve(pets, "🔨️modules", module, "🦀️.rs"), resolve(pets, "🔨️modules", module, "🧪️tests/🔬️unit/🦀️.rs")]),
  ...["💗️feeling-dynamics", "⚗️chemistry-rules", "✨️particle-motion", "🪄️mischief-choice"].map((name) => resolve(pets, "🧪️tests", name, "🦀️.rs")),
  resolve(pets, "🦀️.rs"),
];
const tools = ["d3_scratch.ts", "d3_dump_bits.ts", "d3_check_bits.rs", "d3_rehearse_adapters.ts", "d3_code_rules.mjs"].map((file) => resolve(ticket, file));
const banned = new Set(["core", "common", "util", "utils", "helper", "helpers", "misc", "shared", "base", "lib", "impl"]);
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
let problems = 0;
for (const file of [...product, ...tools]) {
  const source = readFileSync(file, "utf8");
  const rust = file.endsWith(".rs");
  const leads = [];
  if (rust) {
    let previous = false;
    for (const line of source.split("\n")) {
      const match = /^[ \t]*\/\/[/!][ \t]*(\S+)/u.exec(line);
      if (match !== null && !previous) leads.push(match[1]);
      previous = /^[ \t]*\/\/[/!]/u.test(line) || (previous && /^[ \t]*#\[/u.test(line));
    }
  } else for (const match of source.matchAll(/^[ \t]*\/\*\*[ \t]*(\S+)/gmu)) leads.push(match[1]);
  const firsts = leads.map((lead) => [...segmenter.segment(lead)][0].segment);
  const plain = firsts.filter((lead, index) => !/\p{Extended_Pictographic}/u.test(lead) || !lead.endsWith("️") || leads[index].startsWith("@"));
  const twice = [...new Set(firsts.filter((lead, index) => firsts.indexOf(lead) !== index))];
  const noisy = product.includes(file) ? [...source.matchAll(/console\.\w+\(|\[DEBUG\]|println!\(|eprintln!\(|dbg!\(|print!\(/gu)].map((match) => match[0]) : [];
  const inner = rust ? source.split("\n").filter((line) => /^\s*\/\/(?![/!#])/u.test(line)) : source.split("\n").filter((line) => /^\s+\/\/(?!#)/u.test(line));
  const returns = source.includes("\r") ? ["CRLF"] : [];
  const names = [...source.matchAll(/\b(?:const|let|function|type|interface|class|fn|struct|enum|mod|static|trait)\s+([A-Za-z_]\w*)/gu)].map((match) => match[1]);
  const stems = [...new Set(names.filter((name) => name.replace(/([a-z0-9])([A-Z])/gu, "$1_$2").toLowerCase().split("_").some((part) => banned.has(part))))];
  const found = plain.length + twice.length + noisy.length + inner.length + returns.length + stems.length;
  problems += found;
  const name = file.slice(file.indexOf("🐾") >= 0 ? file.indexOf("🐾") : file.indexOf("QUIZ-PETS"));
  process.stdout.write(found === 0 ? `${name}: ${leads.length} docstrings, clean\n` : `${name}: ${leads.length} docstrings, without emoji ${JSON.stringify(plain)}, repeated ${JSON.stringify(twice)}, console or debug ${JSON.stringify(noisy)}, comments inside ${JSON.stringify(inner)}, ${returns.join("")} banned stems ${JSON.stringify(stems)}\n`);
}
process.stdout.write(`problems: ${problems}\n`);
process.exitCode = problems === 0 ? 0 : 1;
