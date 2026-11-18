import { authoredScales, catalog, consistency, sweep } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
for (const asset of catalog().filter((a) => process.argv.slice(2).includes(a.id)))
  for (const scale of authoredScales(asset)) for (const f of [...sweep(asset, scale), ...consistency(asset, scale)]) console.log(`[${f.check}] ${f.message}`);
