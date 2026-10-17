import { docstringHitsOfText, docstringEmojiReuseOfText, docstringOpenersOfText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts";
import { readFileSync } from "node:fs";
const paths = (await Bun.stdin.text()).split("\n").filter(Boolean);
let hits = 0, reuse = 0;
for (const p of paths) {
  const t = readFileSync("/Users/ueli/Documents/semio/" + p, "utf8");
  for (const h of docstringHitsOfText(p, t)) { hits++; console.log("HIT", h.rule, p.slice(-60), h.line, h.text); }
  const firstLine = docstringOpenersOfText(p, t)[0]?.line; for (const r of docstringEmojiReuseOfText(p, t)) { if (process.argv[2] === "first" && r.lines[0] !== firstLine) continue; reuse++; console.log("REUSE", p.slice(-60), r.emoji, r.lines.join(",")); }
}
console.log("files", paths.length, "hits", hits, "reuse", reuse);
