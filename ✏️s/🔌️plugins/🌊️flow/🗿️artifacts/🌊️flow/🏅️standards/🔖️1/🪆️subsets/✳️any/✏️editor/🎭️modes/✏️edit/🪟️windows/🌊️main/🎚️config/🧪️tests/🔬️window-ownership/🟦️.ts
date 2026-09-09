import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import { parseFlowArtifact } from "../../../../../../../../🧬️schema/🟦️.ts";
import { parseFlowSnapshot } from "../../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { parseFlowDiff } from "../../../../../../../../🧬️schema/🔺️diff/🟦️.ts";
import { applyFlowMainWindowConfigMutation, type FlowMainWindowConfig, type FlowMainWindowConfigMutation } from "../../🧬️schema/🟦️.ts";
import { applyFlowWindowTransientMutation, type FlowWindowTransient, type FlowWindowTransientMutation } from "../../../🫧️transient/🧬️schema/🟦️.ts";

type WindowInstance = { id: string; windowKindId: string };
type Fixture = {
  document: unknown;
  windowInstances: WindowInstance[];
  baseConfig: FlowMainWindowConfig;
  baseTransient: FlowWindowTransient;
  configMutations: { windowId: string; windowKindId: string; mutation: FlowMainWindowConfigMutation }[];
  transientMutations: { windowId: string; windowKindId: string; mutation: FlowWindowTransientMutation }[];
  expectedConfig: Record<string, FlowMainWindowConfig>;
  expectedTransient: Record<string, FlowWindowTransient>;
  rejections: { windowId: string; claimedWindowKindId: string; code: string }[];
  hostContext: { programContributions: unknown[] };
};

const here = fileURLToPath(new URL(".", import.meta.url));
const readJson = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const fixture = readJson(`${here}/../../🧫️fixtures/🔬️window-ownership/🔣️.json`) as Fixture;
const configSchema = readJson(`${here}/../../🧬️schema/🔣️.json`);
const transientSchema = readJson(`${here}/../../../🫧️transient/🧬️schema/🔣️.json`);
const schemaRoot = fileURLToPath(new URL("../../../../../../../../🧬️schema/", import.meta.url));
const artifactSchema = readJson(`${schemaRoot}/🔣️.json`);
const snapshotSchema = readJson(`${schemaRoot}/📸️snapshot/🔣️.json`);
const diffSchema = readJson(`${schemaRoot}/🔺️diff/🔣️.json`);
const workspace = fileURLToPath(new URL("../../../../../../../../../../../../../../../../../", import.meta.url));
const childSchema = readJson(`${workspace}/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json`);
const ioSchema = readJson(`${workspace}/🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json`);

const target = (instances: WindowInstance[], id: string, expectedKind: string): void => {
  const instance = instances.find((candidate) => candidate.id === id);
  if (!instance) throw new Error("flow-window-stale");
  if (instance.windowKindId !== expectedKind) throw new Error(expectedKind === "flow-main" ? "flow-main-window-kind-required" : "flow-window-kind-mismatch");
};

export function testFlowWindowOwnershipOracle(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  for (const keyword of ["x-semio-state", "x-semio-child-kind", "x-semio-child-standard", "x-semio-child-subset", "x-semio-formats"]) ajv.addKeyword(keyword);
  ajv.addFormat("double", true);
  ajv.addSchema(ioSchema);
  ajv.addSchema(childSchema);
  for (const schema of [configSchema, transientSchema, artifactSchema, snapshotSchema, diffSchema]) {
    const validate = ajv.compile(schema);
    const sample = schema === configSchema ? fixture.baseConfig : schema === transientSchema ? fixture.baseTransient : schema === diffSchema ? { content: (fixture.document as any).content } : fixture.document;
    assert(validate(sample), JSON.stringify(validate.errors));
  }
  parseFlowArtifact(fixture.document);
  parseFlowSnapshot(fixture.document);
  parseFlowDiff({ content: (fixture.document as any).content });
  for (const legacy of ["camera", "widgets", "synapses", "layout"]) {
    assert.throws(() => parseFlowArtifact({ ...(fixture.document as object), [legacy]: legacy === "camera" ? { x: 0, y: 0, zoom: 1 } : [] }));
  }

  const documentBytes = JSON.stringify(fixture.document);
  const configs: Record<string, FlowMainWindowConfig> = {};
  const configOracle: Record<string, FlowMainWindowConfig> = {};
  for (const entry of fixture.configMutations) {
    target(fixture.windowInstances, entry.windowId, entry.windowKindId);
    configs[entry.windowId] = applyFlowMainWindowConfigMutation(configs[entry.windowId] ?? fixture.baseConfig, entry.mutation);
    configOracle[entry.windowId] = applyPatch(structuredClone(configOracle[entry.windowId] ?? fixture.baseConfig), [{ op: "replace", path: "", value: entry.mutation.config }], false, false).newDocument;
  }
  assert.deepEqual(configs, fixture.expectedConfig);
  assert.deepEqual(configs, configOracle);
  assert.notDeepEqual(configs["flow-main-left"].camera, configs["flow-main-right"].camera);

  const transients: Record<string, FlowWindowTransient> = {};
  const transientOracle: Record<string, FlowWindowTransient> = {};
  for (const entry of fixture.transientMutations) {
    target(fixture.windowInstances, entry.windowId, entry.windowKindId);
    transients[entry.windowId] = applyFlowWindowTransientMutation(transients[entry.windowId] ?? fixture.baseTransient, entry.mutation);
    transientOracle[entry.windowId] = applyPatch(structuredClone(transientOracle[entry.windowId] ?? fixture.baseTransient), [{ op: "replace", path: "", value: entry.mutation.transient }], false, false).newDocument;
  }
  assert.deepEqual(transients, fixture.expectedTransient);
  assert.deepEqual(transients, transientOracle);
  assert.equal(JSON.stringify(fixture.document), documentBytes);

  const restoredConfigs = JSON.parse(JSON.stringify(configs));
  const reloadedTransients: Record<string, FlowWindowTransient> = {};
  assert.deepEqual(restoredConfigs, fixture.expectedConfig);
  assert.deepEqual(reloadedTransients, {});
  assert.equal(JSON.stringify(fixture.document), documentBytes);
  for (const rejected of fixture.rejections) assert.throws(() => target(fixture.windowInstances, rejected.windowId, rejected.claimedWindowKindId), new RegExp(rejected.code));
  assert(!Object.hasOwn(fixture.baseConfig, "contributionsJson") && !Object.hasOwn(fixture.baseTransient, "contributionsJson"));
  assert.equal(fixture.hostContext.programContributions.length, 1);
  console.log(`[DEBUG] flow-window-ownership windows=${Object.keys(configs).length} transients=${Object.keys(transients).length} documentBytes=${documentBytes.length} staleAndWrongKind=rejected`);
}

