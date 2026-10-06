import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
const path="/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts",before=readFileSync(path,"utf8");
assert.equal(before.split("../../../../../🧫️fixtures/🔁️hub-tail-after-check-in/").length,4);
const after=before.replaceAll("../../../../../🧫️fixtures/🔁️hub-tail-after-check-in/","../../../../🧫️fixtures/🔁️hub-tail-after-check-in/");
writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify([{path,before,after}],null,2)+"\n");assert.equal(readFileSync(path,"utf8"),before);writeFileSync(path,after);console.log("[DEBUG] Writer current checkpoint Source fixture imports resolve four exact domain ancestors paths=1");
