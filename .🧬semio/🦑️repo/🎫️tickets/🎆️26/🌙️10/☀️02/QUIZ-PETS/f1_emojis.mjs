/** 🔤️ Ticket tool of work package F1: prints the emojis that begin the docstrings of the files named on the command line (one line per file), so a new docstring can take one no other docstring of its file has. `node f1_emojis.mjs <file>…` from anywhere. */
import { readFileSync } from "node:fs";

const segmenter = new Intl.Segmenter("en", { granularity: "grapheme" });
for (const file of process.argv.slice(2)) {
  const source = readFileSync(file, "utf8");
  const pattern = file.endsWith(".py") ? /^[ \t]*(?:"""|r""")[ \t]*(\S+)/gmu : file.endsWith(".rs") ? /^[ \t]*\/\/[/!][ \t]*(\S+)/gmu : /^[ \t]*\/\*\*[ \t]*(\S+)/gmu;
  const leads = [...source.matchAll(pattern)].map((match) => [...segmenter.segment(match[1])][0].segment);
  process.stdout.write(`${file.slice(-40)}: ${leads.join(" ")}\n`);
}
