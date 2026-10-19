import { readFileSync } from "node:fs";
import { policyExtractRustSchemaFields } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🦀️rust/🟦️.ts";
const p = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs";
console.log("[DEBUG]", JSON.stringify(policyExtractRustSchemaFields(readFileSync(p, "utf8"), "Rewriting Artifact")).slice(0, 300));
