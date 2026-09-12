import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv/dist/2020.js";
import { applyPatch } from "fast-json-patch";
import { applyLayoutWindowConfigMutation, type LayoutWindowConfig, type LayoutWindowConfigMutation } from "../../🧬️schema/🟦️.ts";
import { applyLayoutWindowTransientMutation, type LayoutWindowTransient, type LayoutWindowTransientMutation } from "../../../🫧️transient/🧬️schema/🟦️.ts";

type WindowInstance = { id: string; windowKindId: string };
type Fixture = {
  document: unknown;
  windowInstances: WindowInstance[];
  baseConfig: LayoutWindowConfig;
  baseTransient: LayoutWindowTransient;
  configMutations: { windowId: string; windowKindId: string; mutation: LayoutWindowConfigMutation }[];
  transientMutations: { windowId: string; windowKindId: string; mutation: LayoutWindowTransientMutation }[];
  rejections: { windowId: string; claimedWindowKindId: string; code: string }[];
};

const here = fileURLToPath(new URL(".", import.meta.url));
const readJson = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const fixture = readJson(`${here}/../../🧫️fixtures/🔬️window-ownership/🔣️.json`) as Fixture;
const configSchema = readJson(`${here}/../../🧬️schema/🔣️.json`);
const transientSchema = readJson(`${here}/../../../🫧️transient/🧬️schema/🔣️.json`);

const target = (instances: WindowInstance[], id: string, expectedKind: string): void => {
  const instance = instances.find((candidate) => candidate.id === id);
  if (!instance) throw new Error("layout-window-stale");
  if (instance.windowKindId !== expectedKind) throw new Error("layout-window-kind-mismatch");
};

export function testLayoutWindowOwnershipOracle(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  assert(ajv.compile(configSchema)(fixture.baseConfig));
  assert(ajv.compile(transientSchema)(fixture.baseTransient));
  const documentBytes = JSON.stringify(fixture.document);
  const configs: Record<string, LayoutWindowConfig> = {};
  const configOracle: Record<string, LayoutWindowConfig> = {};
  for (const row of fixture.configMutations) {
    target(fixture.windowInstances, row.windowId, row.windowKindId);
    configs[row.windowId] = applyLayoutWindowConfigMutation(configs[row.windowId] ?? fixture.baseConfig, row.mutation);
    configOracle[row.windowId] = applyPatch(structuredClone(configOracle[row.windowId] ?? fixture.baseConfig), [{ op: "replace", path: "", value: row.mutation.config }], false, false).newDocument;
  }
  assert.deepEqual(configs, configOracle);
  assert.notDeepEqual(configs["layout-left"].camera, configs["layout-right"].camera);
  const transients: Record<string, LayoutWindowTransient> = {};
  const transientOracle: Record<string, LayoutWindowTransient> = {};
  for (const row of fixture.transientMutations) {
    target(fixture.windowInstances, row.windowId, row.windowKindId);
    transients[row.windowId] = applyLayoutWindowTransientMutation(transients[row.windowId] ?? fixture.baseTransient, row.mutation);
    transientOracle[row.windowId] = applyPatch(structuredClone(transientOracle[row.windowId] ?? fixture.baseTransient), [{ op: "replace", path: "", value: row.mutation.transient }], false, false).newDocument;
  }
  assert.deepEqual(transients, transientOracle);
  assert.deepEqual(JSON.parse(JSON.stringify(configs)), configs);
  assert.deepEqual({} as Record<string, LayoutWindowTransient>, {});
  assert.equal(JSON.stringify(fixture.document), documentBytes);
  for (const rejection of fixture.rejections) assert.throws(() => target(fixture.windowInstances, rejection.windowId, rejection.claimedWindowKindId), new RegExp(rejection.code));
  console.log(`[DEBUG] layout-window-ownership configs=${Object.keys(configs).length} transients=${Object.keys(transients).length} reload=config-restored-transient-reset`);
}

testLayoutWindowOwnershipOracle();
