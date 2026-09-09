import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import { applyWriterMainWindowConfigMutation, type WriterMainWindowConfigMutation } from "../../🧬️schema/🧬️mutations/🟦️.ts";
import { applyWriterMainWindowTransientMutation, type WriterMainWindowTransientMutation } from "../../../🫧️transient/🧬️schema/🧬️mutations/🟦️.ts";
import { testWriterPartialConstructionOracle } from "../../../🫧️transient/🧪️tests/🧩️partial-construction/🟦️.ts";

/** 🧪️ Validates exact Writer window partitions against Ajv and independent JSON Patch. */
export function testWriterWindowStateOracle(): void {
  testWriterPartialConstructionOracle();
  const fixture = JSON.parse(readFileSync(new URL("./../../🧫️fixtures/🔬️window-state-ownership/🔣️.json", import.meta.url), "utf8"));
  const configSchema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const configMutationSchema = JSON.parse(readFileSync(new URL("../../🧬️schema/🧬️mutations/🔣️.json", import.meta.url), "utf8"));
  const transientSchema = JSON.parse(readFileSync(new URL("../../../🫧️transient/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const transientMutationSchema = JSON.parse(readFileSync(new URL("../../../🫧️transient/🧬️schema/🧬️mutations/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword("x-semio-state");
  ajv.addKeyword("x-semio-owner");
  const validateConfig = ajv.compile(configSchema);
  const validateTransient = ajv.compile(transientSchema);
  const validateConfigMutation = ajv.compile(configMutationSchema);
  const validateTransientMutation = ajv.compile(transientMutationSchema);
  let configs = Object.fromEntries([fixture.leftWindowId, fixture.rightWindowId].map((id: string) => [id, structuredClone(fixture.baseConfig)]));
  let transients = Object.fromEntries([fixture.leftWindowId, fixture.rightWindowId].map((id: string) => [id, structuredClone(fixture.baseTransient)]));
  const configGenerations: Record<string, number> = Object.fromEntries(Object.keys(configs).map((id) => [id, 0]));
  const transientGenerations: Record<string, number> = Object.fromEntries(Object.keys(transients).map((id) => [id, 0]));
  for (const step of fixture.steps) {
    if (step.lane === "config") {
      assert(validateConfigMutation(step.mutation), JSON.stringify(validateConfigMutation.errors));
      const field = step.mutation.kind === "set-camera" ? "camera" : "editorSettings";
      const value = step.mutation.kind === "set-camera" ? step.mutation.camera : step.mutation.settings;
      const oracle = applyPatch(configs, [{ op: "replace", path: `/${step.windowId}/${field}`, value }], true, false).newDocument;
      configs[step.windowId] = applyWriterMainWindowConfigMutation(configs[step.windowId], step.mutation as WriterMainWindowConfigMutation);
      assert.deepEqual(configs, oracle);
      configGenerations[step.windowId] += 1;
    } else {
      assert(validateTransientMutation(step.mutation), JSON.stringify(validateTransientMutation.errors));
      const fields = { "set-editor-selection": "editorSelection", "set-lint-generation": "lintGeneration", "set-engagement-input": "engagementInput" } as const;
      const field = fields[step.mutation.kind as keyof typeof fields];
      const value = step.mutation.kind === "set-editor-selection" ? step.mutation.selection : step.mutation.value;
      const oracle = applyPatch(transients, [{ op: "replace", path: `/${step.windowId}/${field}`, value }], true, false).newDocument;
      transients[step.windowId] = applyWriterMainWindowTransientMutation(transients[step.windowId], step.mutation as WriterMainWindowTransientMutation);
      assert.deepEqual(transients, oracle);
      transientGenerations[step.windowId] += 1;
    }
  }
  for (const state of Object.values(configs)) assert(validateConfig(state), JSON.stringify(validateConfig.errors));
  for (const state of Object.values(transients)) assert(validateTransient(state), JSON.stringify(validateTransient.errors));
  assert.deepEqual(configs, fixture.expectedConfigs);
  assert.deepEqual(transients, fixture.expectedTransients);
  assert.deepEqual(configGenerations, fixture.expectedConfigGenerations);
  assert.deepEqual(transientGenerations, fixture.expectedTransientGenerations);
  console.log("[DEBUG] Writer window state oracle matched Ajv, TypeScript folds, and independent JSON Patch per-window generations");
}
