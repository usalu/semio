import { readFileSync } from "node:fs";
import { validateAsset } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const doc = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as { asset: never };
for (const issue of validateAsset(doc.asset)) console.log(issue.code, issue.path, issue.message.en.slice(0, 200));
