import { entryOf } from "./volumes-lib.ts";
import { wholeExtents } from "./volumes-silhouette.ts";
const [spec, sc] = process.argv.slice(2);
const { asset } = entryOf(spec!);
console.log(JSON.stringify(wholeExtents(asset, Number(sc))), JSON.stringify(asset.representations[sc!].pens));
