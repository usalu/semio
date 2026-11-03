import { readFileSync } from "node:fs";
import { validateAsset } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const walker = JSON.parse(readFileSync("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🚶️reference-walker/🔣️.json", "utf8"));
for (const issue of validateAsset(walker)) console.log(issue.code, issue.path, issue.message.en);
console.log(JSON.stringify(walker.dimensions), JSON.stringify(walker.representations["500"]));
