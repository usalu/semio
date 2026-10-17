// 🔭️ Prints what `hostPrivate` finds for a test file against its host (why the planner does not detach it).
import { readFileSync } from "node:fs";
import { hostPrivate, declaredNames } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
import { identifierSet } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
const [hostFile, testPath] = process.argv.slice(2);
const host = readFileSync(hostFile!, "utf8");
const test = readFileSync(testPath!, "utf8");
const hostPath = process.argv[4] ? process.argv[4].split(",") : null;
const r = hostPrivate(host, identifierSet(test), declaredNames(test), test, hostPath);
console.log(JSON.stringify(r, null, 1).slice(0, 3000));
