/** 🧊 Three-views sweep of an asset at a git revision. bun spread-rev.ts <rev> <asset>... */
import { execFileSync } from "node:child_process";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { authoredScales, sweep } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { assetPathsFromFiles } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { relative } from "node:path";
const [rev, ...names] = process.argv.slice(2);
for (const name of names) {
  const e = (await assetPathsFromFiles()).find((x) => x.folder === name)!;
  const a = JSON.parse(execFileSync("git", ["-C", "/Users/ueli/Documents/semio", "show", `${rev}:${relative("/Users/ueli/Documents/semio", e.path)}`], { maxBuffer: 1 << 28 }).toString("utf8")).asset as Asset;
  console.log(`${name}@${rev}: ${authoredScales(a).flatMap((sc) => sweep(a, sc)).length}`);
}
