import {readFileSync} from "node:fs";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/📐️projection/🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const manifest=JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/package.json","utf8"));
console.log(JSON.stringify(validateJsonSchemaSubset(schema.definitions.Recipe,manifest,schema)).slice(0,1500));
console.log(JSON.stringify(schema.definitions.Recipe).slice(0,1500));
