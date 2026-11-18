/** 🩺 Validation issues of chosen assets in full. bun humansa-validate.ts <slug>... */
import { Glob } from "bun";
import { join, resolve } from "node:path";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const catalog = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
for (const slug of process.argv.slice(2)) {
  const asset = (await Bun.file(join(catalog, "👤️humans", slug, "🔣️.json")).json()).asset;
  const issues = validateAsset(asset);
  console.log(`== ${slug}: ${issues.length}`);
  for (const i of issues) console.log(`  ${i.code}: ${i.message.en}`);
}
