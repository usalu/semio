import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import { applyEquationGraphWindowConfigMutation, type EquationGraphWindowConfigMutation } from "../../🧬️schema/🧬️mutations/🟦️";

/** 🧪️ Validates exact Equation graph-window partitions with Ajv and independent JSON Patch. */
export function testEquationGraphWindowConfigOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
  const configSchema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const mutationSchema = JSON.parse(readFileSync(new URL("../../🧬️schema/🧬️mutations/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword("x-semio-state");
  ajv.addKeyword("x-semio-owner");
  const validateConfig = ajv.compile(configSchema);
  const validateMutation = ajv.compile(mutationSchema);
  let windows = Object.fromEntries([fixture.leftWindowId, fixture.rightWindowId].map((id: string) => [id, structuredClone(fixture.base)]));
  const generations: Record<string, number> = Object.fromEntries(Object.keys(windows).map((id) => [id, 0]));
  for (const row of fixture.cases) {
    assert(validateMutation(row.mutation), JSON.stringify(validateMutation.errors));
    const oracle = applyPatch(windows, [{ op: "replace", path: `/${row.windowId}/camera`, value: row.mutation.camera }], true, false).newDocument;
    windows[row.windowId] = applyEquationGraphWindowConfigMutation(windows[row.windowId], row.mutation as EquationGraphWindowConfigMutation);
    generations[row.windowId] += 1;
    assert.deepEqual(windows, oracle);
    assert.deepEqual(windows, row.expected);
  }
  for (const state of Object.values(windows)) assert(validateConfig(state), JSON.stringify(validateConfig.errors));
  assert.deepEqual(generations, fixture.expectedGenerations);
  assert.deepEqual(fixture.expectedAppConfig, {});
  console.log("[DEBUG] Equation graph-window config matched Ajv, TypeScript folds, and independent JSON Patch partitions");
}
