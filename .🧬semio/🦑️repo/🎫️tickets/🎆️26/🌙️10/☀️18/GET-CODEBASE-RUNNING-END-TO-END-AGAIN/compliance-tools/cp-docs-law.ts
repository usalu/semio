/** ⚖️ The repo's own docstring censuses (openers, emoji reuse, [DEBUG] tag) over the listed files of a root: bun cp-docs-law.ts <root> <list>. Exit 1 on any finding. */
import { readFileSync } from "node:fs";
const live = "/Users/ueli/Documents/semio";
const docs = await import(`${live}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts`);
const [root, list] = process.argv.slice(2) as [string, string];
let bad = 0;
for (const p of readFileSync(list, "utf8").split("\n").filter(Boolean)) {
  if (!/\.(m?ts|rs)$/u.test(p) || p.endsWith(".d.ts")) continue;
  const text = readFileSync(`${root}/${p}`, "utf8");
  for (const h of docs.docstringHitsOfText(p, text)) { bad++; console.log("FAIL opener", p.slice(-60), h.line, h.text); }
  for (const r of docs.docstringEmojiReuseOfText(p, text)) { bad++; console.log("FAIL reuse", p.slice(-60), r.emoji, r.lines.join(",")); }
  for (const h of docs.debugTagHitsOfText(p, text)) { bad++; console.log("FAIL debug", p.slice(-60), h.line); }
}
console.log("cp-docs-law: findings", bad);
process.exit(bad === 0 ? 0 : 1);
