/** 🩺️ Validates the machinery assets (or the given slugs) with the library's `validateAsset` and prints their issues.
 *  bun machinery-validate-some.ts [slug ...] */
import { Glob } from "bun";
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const only = process.argv.slice(2);
let failing = 0, total = 0;
for (const path of [...new Glob("🏗️machinery/*/🔣️.json").scanSync(catalog)].sort()) {
  const slug = path.split("/")[1]!;
  if (only.length > 0 && !only.includes(slug)) continue;
  total++;
  const asset = (await Bun.file(join(catalog, path)).json()).asset;
  const issues = validateAsset(asset);
  if (issues.length === 0) continue;
  failing++;
  console.log(`${slug}\t${issues.length}`);
  for (const issue of issues.slice(0, 8)) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, 300));
}
console.log(`failing ${failing} of ${total}`);
