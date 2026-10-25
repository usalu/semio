import {loadClassifiedBaseline} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/📋️orchestration/🟦️.ts";
import {readFileSync,writeFileSync} from "node:fs";
const root="/Users/ueli/Documents/semio";
const sorted=loadClassifiedBaseline(root);
const p=root+"/🔒️dependencies.json";
const raw=JSON.parse(readFileSync(p,"utf8"));
writeFileSync(p,`${JSON.stringify({...raw,schemaVersion:2,entries:sorted},null,2)}\n`);
console.log("[prof] entries",sorted.length);
