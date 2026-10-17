import { readFileSync } from "node:fs";
import { rustTokens } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
const [file, name] = process.argv.slice(2);
const tokens = rustTokens(readFileSync(file!, "utf8"));
const hits = tokens.map((t, i) => [t, i] as const).filter(([t]) => t.text === name);
console.log(hits.length, "occurrences");
for (const [, i] of hits.slice(0, 5)) console.log(tokens.slice(Math.max(0, i - 3), i + 3).map((t) => t.text).join(" "));
for (let i = 0; i < tokens.length; i += 1) {
  if (tokens[i]!.text !== "for") continue;
  for (let at = i + 1; at < tokens.length && tokens[at]!.text !== "in" && tokens[at]!.text !== "{" && at < i + 24; at += 1) if (tokens[at]!.text === name) console.log("for-rule hit at", i, tokens.slice(i - 2, i + 10).map((t) => t.text).join(" "));
}
