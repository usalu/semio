/** 🔏️ Parses the document-execution-target-lease fixture through the owned parser to surface the
 * exact refusal the worker's catch clause replaces with `document open: invalid execution target`. */
import { readFileSync } from "node:fs";
import { parseDocumentExecutionTargetLeaseFieldsV1, parseDocumentOpenPlanV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../../../../../../🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json", import.meta.url), "utf8"));
for (const [name, parse, value] of [
  ["manifest", parseDocumentExecutionTargetLeaseFieldsV1, fixture.manifest],
  ["plan", parseDocumentOpenPlanV1, fixture.plan],
] as const) {
  try {
    parse(value);
    console.log(`${name}: ok`);
  } catch (error) {
    console.log(`${name}: ${(error as Error).message}`);
  }
}
