/** 🔎️ Ticket probe: prints the quiz schema scope as the repo's schema inventory sees it (formats, exports, findings). */
import { inventorySchemaScopes } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = new URL("../../../../../../../", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/u, "$1");
const inventory = inventorySchemaScopes(decodeURIComponent(repoRoot));
const modules = inventory.modules.filter((module) => module.modulePath.includes("❓️quiz"));
console.log(JSON.stringify(modules.map((module) => ({ modulePath: module.modulePath, scopeId: module.scopeId, level: module.level, formats: module.formats, exports: module.documents.flatMap((document) => document.exports).length })), null, 2));
const scope = Object.entries(inventory.catalog.scopes).filter(([, entry]) => entry.path.includes("❓️quiz"));
console.log(JSON.stringify(scope.map(([id, entry]) => ({ id, path: entry.path, formats: entry.formats, exports: Object.keys(entry.exports).length })), null, 2));
console.log(JSON.stringify([...inventory.diagnostics, ...inventory.placement].filter((finding) => finding.path.includes("❓️quiz") || finding.detail.includes("❓️quiz")), null, 2));
