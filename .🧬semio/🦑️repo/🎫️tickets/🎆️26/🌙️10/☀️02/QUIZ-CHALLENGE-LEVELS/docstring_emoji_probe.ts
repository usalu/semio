/** 🔎️ Lists docstrings whose leading emoji repeats within one file, and docstrings that start without an emoji, for the files this agent touched. Usage: `bun docstring_emoji_probe.ts <file>...`. */
import { readFileSync } from "node:fs";

const LEADING = /\/\*\*\s*(\p{Extended_Pictographic}[\u{FE0F}\u{200D}\p{Extended_Pictographic}\u{1F3FB}-\u{1F3FF}]*)/gu;

for (const file of process.argv.slice(2)) {
  const source = readFileSync(file, "utf8");
  const seen = new Map<string, number[]>();
  for (const match of source.matchAll(LEADING)) {
    const emoji = match[1]!.replaceAll("\u{FE0F}", "");
    const line = source.slice(0, match.index).split("\n").length;
    seen.set(emoji, [...(seen.get(emoji) ?? []), line]);
  }
  const bare = [...source.matchAll(/\/\*\*\s*([^\s*])/gu)].filter((match) => !/\p{Extended_Pictographic}/u.test(match[1]!)).map((match) => source.slice(0, match.index).split("\n").length);
  for (const [emoji, lines] of seen) if (lines.length > 1) console.log(`${file}: ${emoji} repeats on lines ${lines.join(", ")}`);
  for (const line of bare) console.log(`${file}: docstring without emoji on line ${line}`);
}
