import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { parseInferenceReconcilePayloadV1 } from "../../🔎️reconcile/🟦️.ts";

type Fixture = { readonly schemaVectors: readonly { readonly name: string; readonly schema: string; readonly value: unknown; readonly valid: boolean }[]; readonly reconciliationVectors: readonly { readonly name: string; readonly value: unknown; readonly valid: boolean }[] };

/** ⚖️ Validates the owner wire vectors independently with Ajv and the TypeScript recovery decoder. */
export function proveGisMapInferenceClientV1(): number {
  const owner = join(import.meta.dir, "..");
  const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8")) as Fixture;
  const validators = new Map<string, ReturnType<Ajv["compile"]>>();
  let checks = 0;
  for (const vector of fixture.schemaVectors) {
    let validate = validators.get(vector.schema);
    if (!validate) {
      validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧬️schema", vector.schema), "utf8")));
      validators.set(vector.schema, validate);
    }
    assert.equal(validate(vector.value), vector.valid, vector.name + JSON.stringify(validate.errors));
    checks++;
  }
  for (const vector of fixture.reconciliationVectors) {
    let valid = false;
    try { parseInferenceReconcilePayloadV1(vector.value); valid = true; } catch {}
    assert.equal(valid, vector.valid, vector.name);
    checks++;
  }
  assert.equal(validators.size, 11);
  assert.equal(fixture.reconciliationVectors.length, 10);
  console.log("[DEBUG] GIS owner client schemas=" + validators.size + " vectors=" + fixture.schemaVectors.length + " reconciliation=" + fixture.reconciliationVectors.length);
  return checks;
}
