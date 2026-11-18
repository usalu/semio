/** 🩺 All validation issues of vegetation assets: bun vegetation-validate.ts <slug>... (no slug: all vegetation) */
import { Glob } from "bun";
import { join, resolve } from "node:path";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const catalog = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const only = process.argv.slice(2);
void resolve;
for (const path of [...new Glob("🌳️vegetation/*/🔣️.json").scanSync(catalog)].sort()) {
  const slug = path.split("/")[1]!;
  if (only.length > 0 && !only.includes(slug)) continue;
  const asset = (await Bun.file(join(catalog, path)).json()).asset;
  const issues = validateAsset(asset);
  console.log(`${slug}\t${issues.length}`);
  for (const i of issues) console.log(`   ${i.code} ${i.path ?? ""}: ${i.message.en.slice(0, 260)}`);
}
