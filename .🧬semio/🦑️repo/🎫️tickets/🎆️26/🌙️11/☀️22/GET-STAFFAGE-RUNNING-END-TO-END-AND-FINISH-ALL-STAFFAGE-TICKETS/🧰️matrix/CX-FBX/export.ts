import { exportModels } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/📦️batch/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const [id, out] = process.argv.slice(2);
const e = (await assetPathsFromFiles()).find((x) => x.folder === id)!;
const { asset } = await readStreamedAsset(e);
await exportModels({ out: out!, assets: [asset.id], views: ["light-solid", "thermal", "forces"] });
console.log("done");
