import { readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { validateAsset } = await import(`${L}/🔨️modules/🗂️catalog/🟦️.ts`);
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/👤️humans/${process.argv[2]}/🔣️.json`, "utf8")).asset;
for (const i of validateAsset(a)) if (i.code === process.argv[3]) console.log(i.message.en, "\n");
