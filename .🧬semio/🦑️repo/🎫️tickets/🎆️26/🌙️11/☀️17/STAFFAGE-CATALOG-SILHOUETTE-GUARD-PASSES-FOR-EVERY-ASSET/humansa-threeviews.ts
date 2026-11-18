/** 🧊 The sweep of the three-views test for the current and the HEAD version of chosen assets: findings per scale. bun humansa-threeviews.ts <asset>... */
import { execFileSync } from "node:child_process";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { authoredScales, sweep } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { loadAsset } from "./humansa-lib.ts";
for (const name of process.argv.slice(2)) {
  const cur = await loadAsset(name);
  const head = JSON.parse(execFileSync("git", ["-C", "/Users/ueli/Documents/semio", "show", `HEAD:🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${name}/🔣️.json`], { maxBuffer: 1 << 28 }).toString("utf8")).asset as Asset;
  for (const [label, a] of [["HEAD", head], ["now", cur]] as const) {
    const f = authoredScales(a).flatMap((sc) => sweep(a, sc));
    console.log(`${name} ${label}: ${f.length}`);
    for (const x of f.slice(0, 4)) console.log(`    [${x.check}] ${x.message.slice(0, 170)}`);
  }
}
