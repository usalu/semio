import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv/dist/2020.js";
import { applyPatch, compare } from "fast-json-patch";
import { applyFormsTryWindowConfigMutation, type FormsTryWindowConfig } from "../../🧬️schema/🟦️.ts";
import { applyFormsTryWindowTransientMutation, type FormsTryWindowTransient } from "../../../🫧️transient/🧬️schema/🟦️.ts";

type Lease = {
  windowId: string;
  windowKindId: string;
  windowGeneration: number;
  documentGeneration: number;
};

type Fixture = {
  document: { bytes: string };
  windows: Record<"left" | "right", { lease: Lease; config: FormsTryWindowConfig; transient: FormsTryWindowTransient }>;
  leftMutations: {
    advance: { kind: "snapshot"; config: FormsTryWindowConfig };
    stage: { kind: "snapshot"; transient: FormsTryWindowTransient };
    resetConfig: { kind: "snapshot"; config: FormsTryWindowConfig };
    resetTransient: { kind: "snapshot"; transient: FormsTryWindowTransient };
  };
  continuations: Record<"left" | "right", Lease & Record<"appId" | "documentId" | "operationId" | "baseRevision", string> & { generation: number }>;
};

const here = fileURLToPath(new URL(".", import.meta.url));
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔬️window-ownership/🔣️.json", import.meta.url), "utf8")) as Fixture;
const configSchema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const transientSchema = JSON.parse(readFileSync(new URL("../../../🫧️transient/🧬️schema/🔣️.json", import.meta.url), "utf8"));

export function testFormsTryWindowOwnership(): void {
  assert(here.length > 0);
  const ajv = new Ajv({ strict: false, allErrors: true });
  const validateConfig = ajv.compile(configSchema);
  const validateTransient = ajv.compile(transientSchema);
  for (const window of Object.values(fixture.windows)) {
    assert(validateConfig(window.config), JSON.stringify(validateConfig.errors));
    assert(validateTransient(window.transient), JSON.stringify(validateTransient.errors));
  }

  const documentBefore = structuredClone(fixture.document);
  const rightBefore = structuredClone(fixture.windows.right);
  const left = structuredClone(fixture.windows.left);
  const advanced = applyFormsTryWindowConfigMutation(left.config, fixture.leftMutations.advance);
  const staged = applyFormsTryWindowTransientMutation(left.transient, fixture.leftMutations.stage);

  const leftConfigPatch = compare(left.config, advanced);
  const leftTransientPatch = compare(left.transient, staged);
  left.config = applyPatch(structuredClone(left.config), leftConfigPatch, true, false).newDocument;
  left.transient = applyPatch(structuredClone(left.transient), leftTransientPatch, true, false).newDocument;
  assert.deepEqual(left.config, { currentStepIndex: 1 });
  assert.deepEqual(left.transient, { tryValues: { "left-answer": ["\"Ada\""] } });
  assert.deepEqual(fixture.windows.right, rightBefore);
  assert.deepEqual(fixture.document, documentBefore);

  const reloadedConfig = JSON.parse(JSON.stringify(left.config)) as FormsTryWindowConfig;
  assert(validateConfig(reloadedConfig), JSON.stringify(validateConfig.errors));
  assert.deepEqual(reloadedConfig, advanced);
  assert.notDeepEqual(fixture.continuations.left, fixture.continuations.right);
  assert.notEqual(fixture.continuations.left.windowId, fixture.continuations.right.windowId);
  assert.notEqual(fixture.continuations.left.windowGeneration, fixture.continuations.right.windowGeneration);

  left.config = applyFormsTryWindowConfigMutation(left.config, fixture.leftMutations.resetConfig);
  left.transient = applyFormsTryWindowTransientMutation(left.transient, fixture.leftMutations.resetTransient);
  assert.deepEqual(left.config, { currentStepIndex: 0 });
  assert.deepEqual(left.transient, { tryValues: {} });
  assert.deepEqual(fixture.windows.right, rightBefore);
  assert.deepEqual(fixture.document, documentBefore);
}

if (import.meta.main) {
  testFormsTryWindowOwnership();
  console.log("forms-try-window-ownership ajv=valid json-patch=valid two-window-isolation=valid reload=valid reset=valid");
}
