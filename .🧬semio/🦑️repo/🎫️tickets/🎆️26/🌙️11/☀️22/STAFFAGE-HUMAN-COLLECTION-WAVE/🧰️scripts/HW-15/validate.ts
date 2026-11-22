/** 🩺️ Validates the given human assets (default: the HW-15 slugs) with the library's `validateAsset` and prints their issues.
 *  bun validate.ts [slug ...] */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const args = process.argv.slice(2);
const slugs = args.length > 0 ? args : ["person-with-guide-dog", "person-signing", "person-with-arm-sling"];
let failing = 0;
for (const slug of slugs) {
  const category = slug === "labrador-dog" ? "🐕️animals" : "👤️humans";
  const t0 = performance.now();
  const asset = (await Bun.file(join(catalog, category, slug, "🔣️.json")).json()).asset;
  const issues = validateAsset(asset);
  const s = ((performance.now() - t0) / 1000).toFixed(1);
  console.log(`${slug}\t${issues.length} issues\t${s}s`);
  if (issues.length > 0) failing++;
  for (const issue of issues.slice(0, Number(process.env.MAX ?? 40))) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, Number(process.env.W ?? 400)));
}
console.log(`failing ${failing} of ${slugs.length}`);
