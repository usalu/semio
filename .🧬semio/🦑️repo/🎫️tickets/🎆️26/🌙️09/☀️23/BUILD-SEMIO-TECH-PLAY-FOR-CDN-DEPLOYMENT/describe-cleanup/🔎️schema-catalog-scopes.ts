/** 🔎️ Prints the projected schema-catalog scopes whose id or path names `reasoning` or a hub plugin-identity, without writing the catalog. */
import { inventorySchemaScopes } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const repoRoot = new URL("../../../../../../../../", import.meta.url).pathname.replace(/\/$/u, "");
const { catalog, diagnostics } = inventorySchemaScopes(repoRoot);
for (const [id, scope] of Object.entries(catalog.scopes)) if (/reasoning|plugin-identity/u.test(id + scope.path)) console.log(id, JSON.stringify(scope));
for (const finding of diagnostics) if (/reasoning|plugin-identity/u.test(JSON.stringify(finding))) console.log("diagnostic", JSON.stringify(finding));
