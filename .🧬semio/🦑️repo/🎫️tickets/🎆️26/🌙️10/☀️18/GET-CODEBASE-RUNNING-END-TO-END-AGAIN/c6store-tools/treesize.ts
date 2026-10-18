// 🌳️ c6store module-tree sizes: for each tag of files.tsv, files and bytes reached by listModuleTree (default options) and by {tests:true}. usage: bun treesize.ts
import {readFileSync, statSync} from "node:fs";
import {resolve} from "node:path";
import {listModuleTree} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const R = "/Users/ueli/Documents/semio";
const rows = readFileSync(resolve(import.meta.dir, "files.tsv"), "utf8").split("\n").filter(Boolean).map(l => l.split("\t") as [string, string]);
for (const [tag, rel] of rows) {
  for (const tests of [false, true]) {
    try {
      const files = listModuleTree(resolve(R, rel), {tests, crates: false});
      const bytes = files.reduce((s, f) => s + statSync(f).size, 0);
      console.log(tag, "tests=" + tests, "files", files.length, "bytes", bytes);
    } catch (e) { console.log(tag, "tests=" + tests, "ERR", String((e as Error).message).slice(0, 160)); }
  }
}
