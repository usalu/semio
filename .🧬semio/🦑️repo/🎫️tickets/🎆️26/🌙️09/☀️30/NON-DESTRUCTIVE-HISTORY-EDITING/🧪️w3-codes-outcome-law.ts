/** ⚖️ W3-CODES: runs the outcome-code rule of `verify mutation-outcome-law` alone and prints every breach grouped by path,
 * so the remap lanes can be driven to zero without waiting on the full seven-rule bundle. Exit 1 on any breach. */
import { policyMutationMessageCodeBreaches, policyMutationOutcomeBreaches } from "/Users/ueli/Documents/semio/📜️script.ts";

const repo = "/Users/ueli/Documents/semio";
const started = performance.now();
const breaches = [...policyMutationMessageCodeBreaches(repo), ...(process.argv.includes("--with-leaves") ? policyMutationOutcomeBreaches(repo) : [])].filter((breach) => breach.priority === "high");
for (const breach of breaches) console.log(`${breach.kind}: ${breach.summary}`);
console.log(`[w3-codes] ${breaches.length} breach(es) in ${Math.round(performance.now() - started)} ms`);
process.exit(breaches.length > 0 ? 1 : 0);
