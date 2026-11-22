/** 🩺️ Validates the HW-18 humans (or the given slugs, or `--file <asset.json>`) with the library's `validateAsset` and prints every issue.
 *  bun validate.ts [slug ...] [--file path] */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const args = process.argv.slice(2);
const files = args[0] === "--file" ? args.slice(1) : (args.length ? args : ["worker-with-pallet-jack", "person-carrying-ladder", "gardener-with-wheelbarrow"]).map((s) => join(catalog, s, "🔣️.json"));
let failing = 0;
for (const path of files) {
  const t0 = performance.now();
  const asset = (await Bun.file(path).json()).asset;
  const issues = validateAsset(asset);
  console.log(`${asset.id}\t${issues.length} issues\t${((performance.now() - t0) / 1000).toFixed(1)} s`);
  if (issues.length) failing++;
  for (const issue of issues.slice(0, 60)) console.log(`   ${issue.code}: ${issue.message.en}`.slice(0, 400));
}
console.log(`failing ${failing} of ${files.length}`);
