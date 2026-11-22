/** 🩺️ Validates the given human assets with the library's `validateAsset` and prints every issue.
 *  bun validate.ts slug [slug ...] */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
let failing = 0;
for (const slug of process.argv.slice(2)) {
  const t0 = performance.now();
  const asset = (await Bun.file(join(catalog, slug, "🔣️.json")).json()).asset;
  const issues = validateAsset(asset);
  console.log(`${slug}\t${issues.length} issues\t${((performance.now() - t0) / 1000).toFixed(1)} s`);
  if (issues.length > 0) failing++;
  for (const issue of issues) console.log(`   ${issue.code} @ ${issue.path ?? ""}: ${issue.message.en}`.slice(0, 600));
}
console.log(`failing ${failing}`);
