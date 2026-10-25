import { readFileSync, writeFileSync } from "node:fs";
import { docstringHitsOfText, docstringEmojiReuseOfText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts";
const [listPath, outPath] = process.argv.slice(2);
const paths = readFileSync(listPath, "utf8").split("\n").filter(Boolean);
const first: unknown[] = [];
const reuse: unknown[] = [];
for (const p of paths) {
  let t: string;
  try { t = readFileSync("/Users/ueli/Documents/semio/" + p, "utf8"); } catch { continue; }
  first.push(...docstringHitsOfText(p, t));
  reuse.push(...docstringEmojiReuseOfText(p, t));
}
writeFileSync(outPath, JSON.stringify({ first, reuse }));
console.log(paths.length, first.length, reuse.length);
