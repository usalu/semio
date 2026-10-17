import { POLICY_SPLIT_SOURCE_ROOTS, policyModuleTreeFiles } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
const repo = "/Users/ueli/Documents/semio";
let bad = 0;
for (const p of POLICY_SPLIT_SOURCE_ROOTS) { try { const files = policyModuleTreeFiles(repo, p); if (!files.length) { bad++; console.log("EMPTY", p); } } catch (e) { bad++; console.log("ERR", p.slice(-70), String(e).slice(0, 100)); } }
console.log("roots", POLICY_SPLIT_SOURCE_ROOTS.length, "bad", bad, "shellhelpers-entry", POLICY_SPLIT_SOURCE_ROOTS.some((p) => p.includes("ShellHelpers")));
