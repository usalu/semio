import {readFileSync} from "node:fs";
import {hostPrivate, declaredNames} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
import {identifierSet} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
const dir = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/" + process.argv[2];
const host = readFileSync(dir + "/🦀️.rs", "utf8"), body = readFileSync(dir + "/🧪️tests/🔬️unit/🦀️.rs", "utf8");
console.log(JSON.stringify(hostPrivate(host, identifierSet(body), declaredNames(body), body), null, 1));
