/** 🧭️ W2-R energy §11: proves the gate's git-visible `policyMutationLawInventory().mutationsDirs` finds every `🧬️mutations`
 * directory the taxonomy source admission (`policyFindAllMutationsDirs`) finds that rule 1 can judge — one holding at least
 * one direct `🔺️diff/🦀️.rs` leaf — and prints both directions of the difference with wall times. Exit 1 on a rule-1-relevant
 * directory only the admission sees. */
import { existsSync } from "node:fs";
import { join } from "node:path";
import { policyListMutationDirs } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📇️direct-owner-index/🟦️.ts";
import { policyFindAllMutationsDirs } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📸️captured-source/🟦️.ts";
import { policyMutationLawInventory } from "/Users/ueli/Documents/semio/📜️script.ts";

const repo = "/Users/ueli/Documents/semio";
const judged = (mutationsRel: string) => policyListMutationDirs(repo, mutationsRel).some((name) => existsSync(join(repo, mutationsRel, name, "🔺️diff", "🦀️.rs")));
let started = performance.now();
const inventory = new Set(policyMutationLawInventory(repo).mutationsDirs);
console.log(`inventory: ${inventory.size} dirs in ${((performance.now() - started) / 1000).toFixed(1)} s`);
started = performance.now();
const admission = new Set(policyFindAllMutationsDirs(repo).map((path) => path.normalize("NFC")));
console.log(`admission: ${admission.size} dirs in ${((performance.now() - started) / 1000).toFixed(1)} s`);
const onlyAdmission = [...admission].filter((path) => !inventory.has(path));
const onlyInventory = [...inventory].filter((path) => !admission.has(path));
const missed = onlyAdmission.filter(judged);
console.log(`only admission: ${onlyAdmission.length} (${missed.length} with diff leaves rule 1 would judge)`);
for (const path of onlyAdmission) console.log(`  ${judged(path) ? "MISSED" : "inert "} ${path}`);
console.log(`only inventory: ${onlyInventory.length}`);
for (const path of onlyInventory) console.log(`  ${judged(path) ? "judged" : "inert "} ${path}`);
process.exit(missed.length > 0 ? 1 : 0);
