import { createHash } from "node:crypto";
import { readRequiredCatalogProjectionV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { playgroundSessionPublicationV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts";

const [catalog, ...variants] = process.argv.slice(2);
const projection = readRequiredCatalogProjectionV1(catalog);
const sessions = Object.fromEntries(variants.map(variant => {
  const files = playgroundSessionPublicationV1(variant, projection);
  return [variant, Object.fromEntries(files.map(file => [file.path, createHash("sha256").update(file.text).digest("hex")]))];
}));
console.log(JSON.stringify(sessions));
