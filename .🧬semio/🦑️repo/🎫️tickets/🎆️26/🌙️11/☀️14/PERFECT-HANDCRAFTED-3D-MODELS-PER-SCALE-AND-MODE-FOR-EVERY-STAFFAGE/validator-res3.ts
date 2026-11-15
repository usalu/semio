import { readFileSync } from "node:fs";
const { measureAsset } = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes-next/🌓️silhouette/🟦️.ts`);
const a = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/👤️sample-figure/🔣️.json", "utf8")).asset;
measureAsset(a, 200, undefined, Number(process.argv[2]), 0.03);
