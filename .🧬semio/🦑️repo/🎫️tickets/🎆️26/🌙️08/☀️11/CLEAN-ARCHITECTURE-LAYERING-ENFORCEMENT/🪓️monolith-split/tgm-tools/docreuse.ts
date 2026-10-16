import { readFileSync } from "node:fs";
import { docstringEmojiReuseOfText, docstringHitsOfText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";
for (const path of process.argv.slice(2)) {
  const text = readFileSync(path, "utf8");
  const reuse = docstringEmojiReuseOfText(path, text);
  const hits = docstringHitsOfText(path, text);
  console.log(JSON.stringify({ path: path.split("/").slice(-3).join("/"), reuse: reuse.map((r) => ({ e: r.emoji, lines: r.lines })), hits: hits.length }));
}
