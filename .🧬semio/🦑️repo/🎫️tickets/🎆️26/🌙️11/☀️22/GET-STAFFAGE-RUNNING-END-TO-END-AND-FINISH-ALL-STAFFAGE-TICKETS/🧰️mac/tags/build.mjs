import fs from "node:fs";
import { fileURLToPath } from "node:url";
const here = fileURLToPath(new URL(".", import.meta.url));
const slugs = fs.readFileSync(here + "slugs.txt", "utf8").trim().split("\n");
const lines = ["de-1.txt", "de-2.txt", "de-3.txt", "fixes.txt"].flatMap((f) => fs.readFileSync(here + f, "utf8").trim().split("\n"));
const table = new Map();
for (const line of lines) {
  const [slug, de, en] = line.split("|");
  table.set(slug, { en: en ?? slug.replaceAll("-", " "), de });
}
const missing = slugs.filter((s) => !table.has(s));
const extraOk = new Set(["test", "fixture", "wheel"]);
const extra = [...table.keys()].filter((s) => !slugs.includes(s) && !extraOk.has(s));
if (missing.length || extra.length) throw new Error(JSON.stringify({ missing, extra }));
fs.writeFileSync(here + "vocabulary.json", JSON.stringify(Object.fromEntries([...table].sort(([a], [b]) => (a < b ? -1 : 1))), null, 1) + "\n");
console.log(slugs.length, "tags");
