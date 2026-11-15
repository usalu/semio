import { readFileSync } from "node:fs";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const a = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
for (const i of validateAsset(a)) console.log(i.code, i.path, i.message.en.slice(0, 140));
