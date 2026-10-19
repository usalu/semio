import { generateCollectionCatalogV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🗂️collection-catalog/🟦️.ts";
const schema = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json";
console.log(JSON.stringify(generateCollectionCatalogV1(schema, process.argv[2] === "write" ? "write" : "check"), null, 1));
