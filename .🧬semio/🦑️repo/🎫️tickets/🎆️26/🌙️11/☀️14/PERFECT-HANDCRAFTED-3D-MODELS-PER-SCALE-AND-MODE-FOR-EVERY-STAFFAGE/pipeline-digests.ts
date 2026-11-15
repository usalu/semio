import { writeFileSync } from "node:fs";
import { digestsOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️digests/🟦️.ts";
import { scaledFigure } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧊️volumes/🟦️.ts";

const digests = await digestsOf(scaledFigure());
writeFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🧊️model-digests/🔣️.json", `${JSON.stringify(digests, null, 1)}\n`);
console.log(Object.keys(digests.glb).length, Object.keys(digests.fbx).length);
