/** ✅ Validates the owner registry through the repository's own admission reader. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
const adm = await import(join(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts"));
const reg = adm.readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(process.cwd(), "📋️build-owners.json"), "utf8")));
console.log("registry valid, owners:", reg.owners.length, "providers:", reg.catalogProviders.length);
