import { readFileSync, writeFileSync } from "node:fs";
import { mutationSchemaDocumentIndexProjection } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-index/🟦️.ts";
const path = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json";
const projected = mutationSchemaDocumentIndexProjection(process.cwd());
const committed = readFileSync(path, "utf8");
console.log("identical", projected === committed, "bytes", committed.length, projected.length);
if (process.argv[2] === "write" && projected !== committed) { writeFileSync(path, projected); console.log("written"); }
