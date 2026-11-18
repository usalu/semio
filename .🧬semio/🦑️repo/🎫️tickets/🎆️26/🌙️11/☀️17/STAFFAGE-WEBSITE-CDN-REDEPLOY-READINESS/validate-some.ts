/** 🩺️ Validates the catalog assets given as `category/slug` arguments and prints every issue. */
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
for (const name of process.argv.slice(2)) {
  const issues = validateAsset((await Bun.file(join(catalog, name, "🔣️.json")).json()).asset);
  console.log(`${name}\t${issues.length}`);
  for (const issue of issues.slice(0, 12)) console.log(`   ${issue.code}: ${issue.message.en}`);
}
