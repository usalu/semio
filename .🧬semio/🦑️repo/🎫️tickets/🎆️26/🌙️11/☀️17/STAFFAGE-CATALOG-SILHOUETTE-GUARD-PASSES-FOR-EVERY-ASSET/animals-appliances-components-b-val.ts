/** 🩺 Validates chosen catalog assets and prints all issues: bun animals-appliances-components-b-val.ts <category/asset> ... */
import { Glob } from "bun";
import { join, resolve } from "node:path";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const catalog = resolve("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
for (const path of [...new Glob("*/*/🔣️.json").scanSync(catalog)].sort()) {
  const key = path.split("/").slice(0, 2).join("/");
  if (!process.argv.slice(2).some((a) => key.endsWith(a))) continue;
  const asset = (await Bun.file(join(catalog, path)).json()).asset;
  for (const issue of validateAsset(asset)) console.log(key, issue.code, issue.message.en.slice(0, 400));
}
console.log("done");
