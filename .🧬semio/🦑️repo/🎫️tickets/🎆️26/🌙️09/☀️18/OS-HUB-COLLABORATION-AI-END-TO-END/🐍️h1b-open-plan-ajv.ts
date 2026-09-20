/** 🔬️ H1b session 5: runs the two AJV exports the creation gate uses over the checkpoint-present
 * plan and prints the failing keyword paths, to attribute the gate's refusal precisely. */
import Ajv from "ajv";
const repoRoot = "/Users/ueli/Documents/semio";
const { readFileSync } = await import("node:fs");
const { join } = await import("node:path");
const base = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema");
const directory = JSON.parse(readFileSync(join(base, "🔣️.json"), "utf8"));
const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), "utf8"));
const s: any = await import(join(base, "🟦️.ts"));
const plan = structuredClone(fixture.validPlan);
const lease = s.leaseFieldsFromPlanV1(s.parseDocumentOpenPlanV1(plan, fixture.nowMs), { component: 1, descriptor: 1 });
for (const [name, def, value] of [["DocumentOpenPlanV1", directory.$defs.DocumentOpenPlanV1, plan], ["DocumentExecutionTargetLeaseFieldsV1", directory.$defs.DocumentExecutionTargetLeaseFieldsV1, lease]] as const) {
  const ajv = new Ajv({ strict: true, allErrors: true });
  let validate;
  try {
    validate = ajv.compile({ ...(def as object), $defs: directory.$defs });
  } catch (error) {
    console.log(`${name}: COMPILE FAILED ${error instanceof Error ? error.message : String(error)}`);
    continue;
  }
  const ok = validate(value);
  console.log(`${name}: ${ok ? "accepted" : "REFUSED"}`);
  if (!ok) for (const e of validate.errors ?? []) console.log(`   ${e.instancePath} ${e.keyword} ${e.message} ${JSON.stringify(e.params)}`);
}
