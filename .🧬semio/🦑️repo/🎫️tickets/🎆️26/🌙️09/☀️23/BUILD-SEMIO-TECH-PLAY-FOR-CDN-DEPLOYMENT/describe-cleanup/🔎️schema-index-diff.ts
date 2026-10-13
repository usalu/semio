/** 🔎️ Prints the lines the schema document index generator would change, without writing it. */
import { readFileSync } from "node:fs";
import { mutationSchemaDocumentIndexProjection } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📜️script.ts";
const repoRoot = new URL("../../../../../../../../", import.meta.url).pathname;
const committed = readFileSync(`${repoRoot}🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json`, "utf8").split("\n");
const projected = mutationSchemaDocumentIndexProjection(repoRoot).split("\n");
const left = new Set(committed), right = new Set(projected);
for (const line of committed) if (!right.has(line)) console.log(`- ${line}`);
for (const line of projected) if (!left.has(line)) console.log(`+ ${line}`);
