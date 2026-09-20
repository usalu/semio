/** 🔬️ H1b session 5: isolates which of the six admission predicates in
 * `proveSpaceArtifactCreationContractV1`'s `checkpointPresenceCases` loop refuses the
 * checkpoint-present plan, so a peer's in-flight change is not mistaken for this slice's. */
const repoRoot = "/Users/ueli/Documents/semio";
const { readFileSync } = await import("node:fs");
const { join } = await import("node:path");
const s: any = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts"));
const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), "utf8"));
const plan = structuredClone(fixture.validPlan);
const parsed = s.parseDocumentOpenPlanV1(plan, fixture.nowMs);
const lease = s.leaseFieldsFromPlanV1(parsed, { component: 1, descriptor: 1 });
console.log("plan parentDialect:", JSON.stringify(plan.parentDialect));
for (const [name, fn] of [
  ["parseDocumentOpenPlanV1", () => s.parseDocumentOpenPlanV1(plan, fixture.nowMs)],
  ["parseDocumentExecutionTargetLeaseFieldsV1", () => s.parseDocumentExecutionTargetLeaseFieldsV1(lease)],
  ["documentOpenNeutralStructure", () => s.documentOpenNeutralStructure(plan, fixture.nowMs)],
  ["executionTargetLeaseFieldsAdmissible", () => {
    const ok = s.executionTargetLeaseFieldsAdmissible(lease, { componentMaxBytes: s.DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, descriptorMaxBytes: s.DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES });
    if (!ok) throw new Error("returned false");
    return ok;
  }],
] as const) {
  try {
    const value = fn();
    console.log(`${name}: accepted${typeof value === "boolean" ? ` (${value})` : ""}`);
  } catch (error) {
    console.log(`${name}: REFUSED ${error instanceof Error ? error.message : String(error)}`);
  }
}
