/** 🩺 The validation issues of one asset: bun machinery-validate.ts <category/asset> */
import { join } from "node:path";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const catalog = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const asset = (await Bun.file(join(catalog, process.argv[2]!, "🔣️.json")).json()).asset;
for (const i of validateAsset(asset)) console.log(JSON.stringify(i));
