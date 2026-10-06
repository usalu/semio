import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
const path="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs";
const before=readFileSync(path,"utf8"),after=before.replace('fixture["columnsJson"].as_str().unwrap().into(), fixture["rowsJson"].as_str().unwrap().into()','fixture["columnsJson"].as_str().unwrap(), fixture["rowsJson"].as_str().unwrap()');assert.notEqual(after,before);writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify([{path,before,after}],null,2)+"\n");assert.equal(readFileSync(path,"utf8"),before);writeFileSync(path,after);console.log("[DEBUG] Curation typed TableScene constructor receives exact borrowed neutral str paths=1");
