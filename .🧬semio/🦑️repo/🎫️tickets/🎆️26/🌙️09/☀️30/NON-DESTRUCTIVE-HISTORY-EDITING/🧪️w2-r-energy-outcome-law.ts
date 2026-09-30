/** ⚖️ W2-R energy §10/§11: runs the `verify mutation-outcome-law` rule bundle rule by rule with wall time per rule, so a
 * crash in one rule cannot hide the verdict of the others and a slow rule shows itself. All seven rules share the one
 * memoized `policyMutationLawInventory` walk, so the first rule pays for it. Prints high-priority breaches per rule; exit 1
 * when any rule reports a high-priority breach or crashes. */
import { policyDeriveGlueMountBreaches, policyMergePolicyParityBreaches, policyMutationMessageCodeBreaches, policyMutationOutcomeBreaches, policyNoCrdtVocabularyBreaches, policyNoValidateOverrideBreaches, policySeverityInfoBreaches } from "/Users/ueli/Documents/semio/📜️script.ts";

const repo = "/Users/ueli/Documents/semio";
const rules = { policyMutationOutcomeBreaches, policyMutationMessageCodeBreaches, policyNoCrdtVocabularyBreaches, policyNoValidateOverrideBreaches, policySeverityInfoBreaches, policyMergePolicyParityBreaches, policyDeriveGlueMountBreaches };
let failed = false;
for (const [name, rule] of Object.entries(rules)) {
  const started = performance.now();
  try {
    const breaches = rule(repo).filter((breach) => breach.priority === "high");
    failed ||= breaches.length > 0;
    console.log(`${name}: ${breaches.length} high-priority breach(es) in ${((performance.now() - started) / 1000).toFixed(1)} s`);
    for (const breach of breaches) console.log(`  ${breach.kind}: ${breach.summary}`);
  } catch (error) {
    failed = true;
    console.log(`${name}: CRASHED after ${((performance.now() - started) / 1000).toFixed(1)} s ${(error as Error).message}`);
  }
}
process.exit(failed ? 1 : 0);
