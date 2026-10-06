import {readFileSync,writeFileSync} from "node:fs";
import assert from "node:assert/strict";
const path="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs";
const before=readFileSync(path,"utf8"),after=before.replaceAll("\\uFE0F","\\u{FE0F}");
assert.equal(before.split("\\uFE0F").length-1,6);
writeFileSync(import.meta.dir+"/guarded-pairs.json",JSON.stringify([{path,before,after}],null,2)+"\n");
assert.equal(readFileSync(path,"utf8"),before);
writeFileSync(path,after);
console.log("[DEBUG] Workflow test-only Rust neutral and System observer path spelling repaired paths=1");
