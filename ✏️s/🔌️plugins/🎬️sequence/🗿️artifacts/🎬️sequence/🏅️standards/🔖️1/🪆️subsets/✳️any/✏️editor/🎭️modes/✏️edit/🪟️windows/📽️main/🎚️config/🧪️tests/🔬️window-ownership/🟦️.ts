import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv/dist/2020.js";
import { applyPatch } from "fast-json-patch";
import { applySequenceMainWindowConfigMutation, type SequenceMainWindowConfig, type SequenceMainWindowConfigMutation } from "../../🧬️schema/🟦️.ts";
import { applySequenceScriptWindowTransientMutation, type SequenceScriptWindowTransient, type SequenceScriptWindowTransientMutation } from "../../../../📜️script/🫧️transient/🧬️schema/🟦️.ts";

type WindowInstance = { id: string; windowKindId: string };
type Fixture = {
  document: unknown;
  childDocument: { steps: { id: string }[] };
  childEditAfterReload: { kind: string; x: number; y: number; expectedNodeDelta: number; expectedLane: "child" };
  windowInstances: WindowInstance[];
  baseConfig: SequenceMainWindowConfig;
  baseTransient: SequenceScriptWindowTransient;
  configMutations: { windowId: string; windowKindId: string; mutation: SequenceMainWindowConfigMutation }[];
  transientMutations: { windowId: string; windowKindId: string; mutation: SequenceScriptWindowTransientMutation }[];
  rejections: { windowId: string; claimedWindowKindId: string; code: string }[];
};

const here = fileURLToPath(new URL(".", import.meta.url));
const readJson = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const fixture = readJson(`${here}/../../🧫️fixtures/🔬️window-ownership/🔣️.json`) as Fixture;
const configSchema = readJson(`${here}/../../🧬️schema/🔣️.json`);
const transientSchema = readJson(`${here}/../../../../📜️script/🫧️transient/🧬️schema/🔣️.json`);

const target = (instances: WindowInstance[], id: string, expectedKind: string): void => {
  const instance = instances.find((candidate) => candidate.id === id);
  if (!instance) throw new Error("sequence-window-stale");
  if (instance.windowKindId !== expectedKind) throw new Error("sequence-window-kind-mismatch");
};

export function testSequenceWindowOwnershipOracle(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  assert(ajv.compile(configSchema)(fixture.baseConfig));
  assert(ajv.compile(transientSchema)(fixture.baseTransient));
  const documentBytes = JSON.stringify(fixture.document);
  const configs: Record<string, SequenceMainWindowConfig> = {};
  const configOracle: Record<string, SequenceMainWindowConfig> = {};
  for (const row of fixture.configMutations) {
    target(fixture.windowInstances, row.windowId, row.windowKindId);
    configs[row.windowId] = applySequenceMainWindowConfigMutation(configs[row.windowId] ?? fixture.baseConfig, row.mutation);
    configOracle[row.windowId] = applyPatch(structuredClone(configOracle[row.windowId] ?? fixture.baseConfig), [{ op: "replace", path: "", value: row.mutation.config }], false, false).newDocument;
  }
  assert.deepEqual(configs, configOracle);
  assert.notDeepEqual(configs["sequence-main-left"], configs["sequence-main-right"]);
  const transients: Record<string, SequenceScriptWindowTransient> = {};
  const transientOracle: Record<string, SequenceScriptWindowTransient> = {};
  for (const row of fixture.transientMutations) {
    target(fixture.windowInstances, row.windowId, row.windowKindId);
    transients[row.windowId] = applySequenceScriptWindowTransientMutation(transients[row.windowId] ?? fixture.baseTransient, row.mutation);
    transientOracle[row.windowId] = applyPatch(structuredClone(transientOracle[row.windowId] ?? fixture.baseTransient), [{ op: "replace", path: "", value: row.mutation.transient }], false, false).newDocument;
  }
  assert.deepEqual(transients, transientOracle);
  assert.deepEqual(JSON.parse(JSON.stringify(configs)), configs);
  assert.deepEqual({} as Record<string, SequenceScriptWindowTransient>, {});
  const childBefore = structuredClone(fixture.childDocument);
  const childAfter = structuredClone(childBefore);
  childAfter.steps.push({ id: `step-${childAfter.steps.length + 1}` });
  const childOracle = applyPatch(structuredClone(childBefore), [{ op: "add", path: "/steps/-", value: { id: `step-${childBefore.steps.length + 1}` } }], false, false).newDocument;
  assert.deepEqual(childAfter, childOracle);
  assert.equal(childAfter.steps.length - childBefore.steps.length, fixture.childEditAfterReload.expectedNodeDelta);
  assert.equal(fixture.childEditAfterReload.expectedLane, "child");
  assert.equal(JSON.stringify(fixture.document), documentBytes);
  for (const rejection of fixture.rejections) assert.throws(() => target(fixture.windowInstances, rejection.windowId, rejection.claimedWindowKindId), new RegExp(rejection.code));
  console.log(`[DEBUG] sequence-window-ownership configs=${Object.keys(configs).length} transients=${Object.keys(transients).length} reload=config-restored-transient-reset child-edit=${fixture.childEditAfterReload.expectedLane}`);
}

testSequenceWindowOwnershipOracle();
