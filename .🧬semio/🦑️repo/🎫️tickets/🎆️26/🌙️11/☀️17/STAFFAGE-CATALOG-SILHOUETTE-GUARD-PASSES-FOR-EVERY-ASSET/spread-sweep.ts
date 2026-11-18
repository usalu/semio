/** 🧊 The three-views sweep findings per asset. bun spread-sweep.ts <asset>... */
import { authoredScales, sweep } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { loadAsset } from "./humansa-lib.ts";
for (const name of process.argv.slice(2)) {
  const a = await loadAsset(name);
  const f = authoredScales(a).flatMap((sc) => sweep(a, sc));
  console.log(`${name}: ${f.length}`);
  for (const x of f) console.log(`  [${x.check}] ${x.message.slice(0, 260)}`);
}
