/** 🩺️ Validates HW-05 humans (or the given slugs, or a file via FILE=) with the library's `validateAsset` and prints every issue.
 *  bun validate.ts [slug ...] */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const slugs = process.argv.slice(2);
const list = slugs.length ? slugs : ["walking-toddler", "schoolchild-with-backpack", "teenager-with-headphones"];
let failing = 0;
for (const slug of list) {
  const path = slug.endsWith(".json") ? slug : join(catalog, slug, "🔣️.json");
  const t0 = performance.now();
  const asset = (await Bun.file(path).json()).asset;
  const issues = validateAsset(asset);
  const ms = Math.round(performance.now() - t0);
  console.log(`${slug}\t${issues.length} issues\t${ms} ms`);
  if (issues.length) failing++;
  for (const issue of issues.slice(0, Number(process.env.LIMIT ?? 60))) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, Number(process.env.WIDTH ?? 400)));
}
console.log(`failing ${failing} of ${list.length}`);
