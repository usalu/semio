import {mutationSchemaDocumentIndexProjection} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-index/🟦️.ts";
import {writeFileSync} from "node:fs";
const root=process.cwd();
const text=mutationSchemaDocumentIndexProjection(root);
writeFileSync(process.argv[2],text);
console.log("bytes",text.length);
