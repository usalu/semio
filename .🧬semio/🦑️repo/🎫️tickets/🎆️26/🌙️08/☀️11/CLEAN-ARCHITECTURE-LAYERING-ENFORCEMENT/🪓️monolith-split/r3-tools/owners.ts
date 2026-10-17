// 🗂️ Which crates of a subset include a given file in their lib / test closure (read-only).
import {join} from "node:path";
import {artifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import {measureRustCrate} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const [dir, needle] = process.argv.slice(2) as [string, string];
for (const c of artifactCrates(root, dir)) {
  const lib = await measureRustCrate(root, c.entry, false, new Map(), join(root, c.dir));
  const test = await measureRustCrate(root, c.entry, true, new Map(), join(root, c.dir));
  const hit = (list: readonly string[]) => list.filter(p => p.endsWith(needle));
  if (hit(lib.sourcePaths).length || hit(test.sourcePaths).length) console.log(c.name, "lib", hit(lib.sourcePaths).length, "test", hit(test.sourcePaths).length);
}
