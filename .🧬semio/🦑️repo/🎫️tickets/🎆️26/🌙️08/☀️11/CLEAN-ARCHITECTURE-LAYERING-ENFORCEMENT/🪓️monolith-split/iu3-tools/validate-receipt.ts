/** 🔬️ Read-only check: validates the live image.json against the current ImageReceipt schema. */
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import imageSchema from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json" with{type:"json"};
import mountDirectorySchema from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json" with{type:"json"};
import{readFileSync}from"node:fs";
const authored=JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/image.json","utf8"));
console.log(JSON.stringify(validateJsonSchemaSubset((imageSchema as any).definitions.ImageReceipt,authored,imageSchema as any,[mountDirectorySchema as any])));
