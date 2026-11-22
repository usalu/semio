/** 🩺️ Validates HW-13 humans (or the given slugs) with the library's `validateAsset` and prints their issues.
 *  bun validate.ts [slug ...] */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const args = process.argv.slice(2);
const slugs = args.length > 0 ? args : ["person-sweeping", "person-hanging-laundry", "person-carrying-shopping-bags"];
let failing = 0;
for (const slug of slugs) {
  const file = slug.endsWith(".json") ? slug : join(catalog, slug, "🔣️.json");
  const asset = (await Bun.file(file).json()).asset;
  const t0 = performance.now();
  const issues = validateAsset(asset);
  console.log(`${slug}\t${issues.length} issues\t${((performance.now() - t0) / 1000).toFixed(1)} s`);
  if (issues.length) failing++;
  for (const issue of issues.slice(0, 60)) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, 400));
}
console.log(`failing ${failing} of ${slugs.length}`);
