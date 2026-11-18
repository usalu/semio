/** 🩺 Validation issues of one asset JSON file (e.g. the HEAD copy): bun machinery-validate-file.ts <path> */
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const asset = (await Bun.file(process.argv[2]!).json()).asset;
const issues = validateAsset(asset);
for (const i of issues) console.log(`${i.code}: ${i.message.en}`.slice(0, 260));
console.log("issues", issues.length);
