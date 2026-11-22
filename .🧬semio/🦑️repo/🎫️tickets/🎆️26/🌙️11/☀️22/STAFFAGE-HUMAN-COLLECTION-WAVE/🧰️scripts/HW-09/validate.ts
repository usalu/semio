/** 🩺️ Validates the given human assets (default: the HW-09 slugs) with the library's `validateAsset` and prints their issues.
 *  bun validate.ts [slug ...] [--all] */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const args = process.argv.slice(2);
const all = args.includes("--all");
const slugs = args.filter((a) => !a.startsWith("--"));
const only = slugs.length > 0 ? slugs : ["e-scooter-rider", "traveller-with-rolling-suitcase", "person-pushing-bicycle"];
let failing = 0;
for (const slug of only) {
  const t0 = performance.now();
  const asset = (await Bun.file(join(catalog, slug, "🔣️.json")).json()).asset;
  const issues = validateAsset(asset);
  const ms = Math.round(performance.now() - t0);
  console.log(`${slug}\t${issues.length} issues\t${ms} ms`);
  if (issues.length > 0) failing++;
  for (const issue of all ? issues : issues.slice(0, 40)) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, 400));
}
console.log(`failing ${failing} of ${only.length}`);
