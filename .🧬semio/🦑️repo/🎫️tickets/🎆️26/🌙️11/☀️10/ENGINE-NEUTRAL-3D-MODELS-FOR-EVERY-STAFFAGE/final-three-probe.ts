import { authoredScales, catalog, consistency, sweep } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const ids = process.argv.slice(2);
for (const asset of catalog().filter((a) => ids.includes(a.id))) {
  for (const scale of authoredScales(asset)) {
    for (const f of sweep(asset, scale)) console.log("SWEEP", asset.id, scale, f.check, f.message);
    for (const f of consistency(asset, scale)) console.log("CONS", asset.id, scale, f.check, f.message);
  }
}
