/** 🔎️ Ticket tool of work package A1: every docstring of the split stage — its eleven parts in both languages and their unit suites — starts with an emoji, no emoji starts two docstrings of one file, and nothing writes to the console or keeps a `[DEBUG]` log.
 *
 * Run from the repository root: `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/a1_docstring_check.ts`.
 *
 * @see ./wp_q_docstring_emojis.mjs — the same rule for the TypeScript suites of work package Q
 */
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const V = "️";
const named = (emoji: string, slug: string): string => `${emoji.replaceAll(V, "")}${V}${slug}`;
const MODULES = join(resolve(import.meta.dir, "../../../../../../.."), named("🧰", "framework"), named("🛍", "products"), named("🐾", "pets"), named("🔨", "modules"));
const PARTS = [named("🎪", "stage"), named("📝", "draft"), named("📏", "spacing"), named("🗓", "schedule"), named("👀", "attention"), named("🚶", "locomotion"), named("💞", "sociability"), named("🎯", "choice"), named("👥", "population"), named("🕰", "clock"), named("🎥", "projection")];
const FILES = [named("🟦", ".ts"), named("🦀", ".rs")];
const UNIT = join(named("🧪", "tests"), named("🔬", "unit"));
const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });

/** 📜️ The first word of every docstring of a source: `/**` blocks in TypeScript, runs of `//!` and `///` lines in Rust. */
function leads(source: string, rust: boolean): string[] {
  if (!rust) return [...source.matchAll(/^\s*\/\*\*\s*(\S+)/gmu)].map((match) => match[1]!);
  const found: string[] = [];
  let previous = "";
  for (const line of source.split("\n")) {
    const marker = /^\s*(\/\/[!/])\s*(\S*)/u.exec(line);
    const kind = marker === null ? "" : marker[1]!;
    if (marker !== null && kind !== previous && marker[2] !== "") found.push(marker[2]!);
    previous = kind;
  }
  return found;
}

let problems = 0;
let files = 0;
for (const part of PARTS) {
  for (const file of FILES) {
    for (const path of [join(MODULES, part, file), join(MODULES, part, UNIT, file)]) {
      if (!existsSync(path)) continue;
      files++;
      const source = readFileSync(path, "utf8");
      const firsts = leads(source, file.endsWith(".rs")).map((lead) => [...segmenter.segment(lead)][0]!.segment);
      const plain = firsts.filter((lead) => !/\p{Extended_Pictographic}/u.test(lead));
      const twice = firsts.filter((lead, index) => firsts.indexOf(lead) !== index);
      const noisy = [...source.replace(/"(?:[^"\\\n]|\\.)*"/gu, '""').matchAll(/console\.\w+|\[DEBUG\]|println!|eprintln!|dbg!/gu)].map((match) => match[0]);
      problems += plain.length + twice.length + noisy.length;
      process.stdout.write(`${path.slice(MODULES.length + 1)}: ${firsts.length} docstrings, without emoji ${JSON.stringify(plain)}, repeated ${JSON.stringify(twice)}, console or debug ${JSON.stringify(noisy)}\n`);
    }
  }
}
process.stdout.write(`${problems} problem(s) in ${files} file(s)\n`);
process.exitCode = problems === 0 ? 0 : 1;
