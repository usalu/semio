// 🌳 OSD: dumps the module-tree source (parent first) of the old (snapshot) and new schema root and facade root for the source-text reader comparison. Writes only to the given directory.
import {writeFileSync, mkdirSync} from "node:fs";
import {readModuleTreeSource} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const M = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory";
const O = import.meta.dir + "/orig/";
const out = process.argv[2]!;
mkdirSync(out, {recursive: true});
for (const [name, rel] of [["schema", "🧬️schema/🦀️.rs"], ["root", "🦀️.rs"]] as const) {
  writeFileSync(`${out}/${name}-old.rs`, readModuleTreeSource(`${O}${rel}`));
  writeFileSync(`${out}/${name}-new.rs`, readModuleTreeSource(`${M}/${rel}`));
}
