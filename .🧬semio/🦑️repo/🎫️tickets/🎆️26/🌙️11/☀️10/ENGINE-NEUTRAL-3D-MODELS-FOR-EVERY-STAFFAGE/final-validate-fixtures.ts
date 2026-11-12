import { readFileSync } from "node:fs";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
for (const n of ["🚶️reference-walker", "👤️sample-figure", "🦶️sparse-smooth-walk"]) {
  try {
    const a = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/" + n + "/🔣️.json", "utf8"));
    for (const i of validateAsset(a, {})) console.log(n, i.code, i.path, String(i.message?.en ?? i.message).slice(0, 220));
    console.log(n, "checked");
  } catch (e) { console.log(n, "ERR", (e as Error).message); }
}
