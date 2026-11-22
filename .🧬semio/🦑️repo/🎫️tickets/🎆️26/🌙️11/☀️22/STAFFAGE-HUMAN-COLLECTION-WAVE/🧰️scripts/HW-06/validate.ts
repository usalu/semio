/** 🩺️ Validates the given human slugs (default: the three HW-06 assets) with the library's `validateAsset` and prints their issues.
 *  bun validate.ts [slug ...] */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const args = process.argv.slice(2);
const full = args.includes("--all");
const slugs = args.filter((a) => !a.startsWith("--"));
let failing = 0;
for (const slug of slugs.length ? slugs : ["elderly-person-knitting", "mobility-scooter-user", "carer-pushing-wheelchair"]) {
  const t0 = performance.now();
  const asset = (await Bun.file(join(catalog, slug, "🔣️.json")).json()).asset;
  const issues = validateAsset(asset);
  console.log(`${slug}\t${issues.length}\t${((performance.now() - t0) / 1000).toFixed(1)}s`);
  if (issues.length) failing++;
  for (const issue of full ? issues : issues.slice(0, 40)) console.log(`   ${issue.code} @${issue.path ?? ""}: ${issue.message.en}`.slice(0, 400));
}
console.log(`failing ${failing}`);
