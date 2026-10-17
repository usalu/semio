import {readdirSync, existsSync} from "node:fs";
import {measureRustCrate} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const SS = `${ROOT}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets`;
for (const s of readdirSync(SS)) {
  const f = `${SS}/${s}/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`;
  if (!existsSync(f)) continue;
  const m = await measureRustCrate(ROOT, f, false, new Map(), SS);
  console.log(s, m.files, m.bytes, m.lines);
}
