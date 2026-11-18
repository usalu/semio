/** 🩺️ Validates the catalog assets named as arguments (category/slug) and prints every issue. */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const catalog = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
for (const name of process.argv.slice(2)) {
  const asset = (await Bun.file(join(catalog, name, "🔣️.json")).json()).asset;
  const issues = validateAsset(asset);
  console.log(`== ${name}: ${issues.length}`);
  for (const issue of issues) console.log(`  ${issue.code}: ${issue.message.en}`);
}
