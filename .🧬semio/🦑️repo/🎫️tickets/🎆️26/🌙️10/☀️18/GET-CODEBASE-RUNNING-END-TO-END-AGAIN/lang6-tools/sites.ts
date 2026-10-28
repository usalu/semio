/** 🔎️ Lists the prose-literal sites of files: bun sites.ts <root> <rel>... */
import { readFileSync } from "node:fs";
const LIVE = "/Users/ueli/Documents/semio";
const language = await import(`${LIVE}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts`);
const [root, ...rels] = process.argv.slice(2);
for (const rel of rels) {
  const text = readFileSync(`${root}/${rel}`, "utf8");
  console.log(`== ${rel.slice(-70)} total=${language.proseSitesV1(text)}`);
  text.split("\n").forEach((line, i) => { if (language.proseSitesV1(line) > 0) console.log(`${i + 1}: ${line.trim().slice(0, 300)}`); });
}
