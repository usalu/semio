/** ⚖️ S2-CODES: runs the seven `verify mutation-outcome-law` rules (same bundle, same shared git inventory, same `high`
 * filter) without the `verify` router, whose owned-command route table can be broken by an unrelated `📋️project.json`.
 * Prints every breach as the gate does, grouped per rule with counts, and exits 1 on any breach. */
import {
  policyDeriveGlueMountBreaches,
  policyMergePolicyParityBreaches,
  policyMutationMessageCodeBreaches,
  policyMutationOutcomeBreaches,
  policyNoCrdtVocabularyBreaches,
  policyNoValidateOverrideBreaches,
  policySeverityInfoBreaches,
} from "/Users/ueli/Documents/semio/📜️script.ts";

const repo = "/Users/ueli/Documents/semio";
const started = performance.now();
const rules = { policyMutationOutcomeBreaches, policyMutationMessageCodeBreaches, policyNoCrdtVocabularyBreaches, policyNoValidateOverrideBreaches, policySeverityInfoBreaches, policyMergePolicyParityBreaches, policyDeriveGlueMountBreaches };
let total = 0;
for (const [name, rule] of Object.entries(rules)) {
  const breaches = rule(repo).filter((breach) => breach.priority === "high");
  total += breaches.length;
  for (const breach of breaches) console.error(`[verify mutation-outcome-law] ${breach.kind}: ${breach.summary}`);
  console.log(`[s2-codes] ${name}: ${breaches.length}`);
}
console.log(`[s2-codes] ${total} breach(es) in ${Math.round(performance.now() - started)} ms`);
process.exit(total > 0 ? 1 : 0);
