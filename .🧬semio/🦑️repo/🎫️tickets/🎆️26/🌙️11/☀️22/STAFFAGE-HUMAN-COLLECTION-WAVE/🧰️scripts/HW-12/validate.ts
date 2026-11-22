/** 🎻️ Validates HW-12 humans (or the given slugs, or a file via --file) with the library's `validateAsset` and prints every issue.
 *  bun validate.ts [slug ...] | --file <path> */
import { join } from "node:path";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans";
const args = process.argv.slice(2);
const files = args[0] === "--file" ? args.slice(1) : (args.length ? args : ["standing-violinist", "painter-at-easel", "dancer"]).map((s) => join(catalog, s, "🔣️.json"));
let failing = 0;
for (const file of files) {
  const t0 = performance.now();
  const asset = (await Bun.file(file).json()).asset;
  const issues = validateAsset(asset);
  console.log(`${asset.id}\t${issues.length} issues\t${((performance.now() - t0) / 1000).toFixed(1)} s`);
  if (issues.length) failing++;
  for (const issue of issues) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, 400));
}
console.log(`failing ${failing} of ${files.length}`);
