import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const a = (await Bun.file(`/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/${process.argv[2]}/🔣️.json`).json()).asset;
const i = validateAsset(a); console.log("[DEBUG]", i.length); for (const x of i.slice(0, 30)) console.log("[DEBUG]", x.code, x.message.en.slice(0, 200));
