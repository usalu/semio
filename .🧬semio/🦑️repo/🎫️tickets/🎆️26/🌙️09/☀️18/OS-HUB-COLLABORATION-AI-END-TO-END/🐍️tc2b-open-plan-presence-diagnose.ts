/** 🩺️ Names which of the six checkpoint-presence predicates refuses the neutral plan, so the red
 * `required checkpoint presence differs: present` in `space-artifact-creation-check source` can be
 * attributed to its owner instead of guessed at. Read-only: it parses the shipped fixture only. */
import { readFileSync } from "node:fs";
import { parseDocumentOpenPlanV1, parseDocumentExecutionTargetLeaseFieldsV1, leaseFieldsFromPlanV1, executionTargetLeaseFieldsAdmissible, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json", import.meta.url), "utf8"));
const plan = structuredClone(fixture.validPlan);
try {
  const parsed = parseDocumentOpenPlanV1(plan, fixture.nowMs);
  console.log("plan: accepted");
  try {
    const lease = leaseFieldsFromPlanV1(parsed, { component: 1, descriptor: 1 });
    parseDocumentExecutionTargetLeaseFieldsV1(lease);
    console.log("lease: accepted");
    console.log(`admissible: ${executionTargetLeaseFieldsAdmissible(lease, { componentMaxBytes: DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, descriptorMaxBytes: DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES })}`);
  } catch (error) {
    console.log(`lease: refused ${(error as Error).message}`);
  }
} catch (error) {
  console.log(`plan: refused ${(error as Error).message}`);
}
