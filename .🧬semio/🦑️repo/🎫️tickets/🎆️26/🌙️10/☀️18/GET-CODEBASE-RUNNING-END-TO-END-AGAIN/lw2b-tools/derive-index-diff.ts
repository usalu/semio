import { writeFileSync } from "node:fs";
import { mutationSchemaDocumentIndexProjection } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-index/🟦️.ts";
writeFileSync(process.argv[2]!, mutationSchemaDocumentIndexProjection(process.cwd()));
