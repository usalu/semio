import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import { applyMapWindowConfigMutation, parseMapWindowConfig, type MapWindowConfig, type MapWindowConfigMutation } from "../../🧬️schema/🟦️.ts";

type WindowInstance = { id: string; windowKindId: string };
type MutationRow = { windowId: string; windowKindId: string; mutation: MapWindowConfigMutation };
type Fixture = {
  document: unknown;
  windowInstances: WindowInstance[];
  baseConfig: MapWindowConfig;
  mutations: MutationRow[];
  undoMutations: MutationRow[];
  redoMutations: MutationRow[];
  expected: Record<string, MapWindowConfig>;
  rejections: { windowId: string; claimedWindowKindId: string; code: string }[];
};

const here = fileURLToPath(new URL(".", import.meta.url));
const fixture = JSON.parse(readFileSync(`${here}/../../🧫️fixtures/🔬️window-ownership/🔣️.json`, "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(`${here}/../../🧬️schema/🔣️.json`, "utf8"));

const target = (instances: WindowInstance[], id: string, expectedKind: string): void => {
  const instance = instances.find((candidate) => candidate.id === id);
  if (!instance) throw new Error("gis-map-window-stale");
  if (instance.windowKindId !== expectedKind) throw new Error("gis-map-window-kind");
};

const patch = (base: MapWindowConfig, mutation: MapWindowConfigMutation): MapWindowConfig => {
  const operation = (() => {
    switch (mutation.operation) {
      case "setLayerVisibility": return mutation.visible === null ? { op: "remove", path: `/layerVisibility/${mutation.layerId}` } : { op: "add", path: `/layerVisibility/${mutation.layerId}`, value: mutation.visible };
      case "setCamera": return { op: "replace", path: "/cameraJson", value: mutation.cameraJson };
      case "setRenderMode": return { op: "replace", path: "/renderMode", value: mutation.value };
      case "setVectorStyle": return { op: "replace", path: "/vectorStyle", value: mutation.value };
      case "setLodMode": return { op: "replace", path: "/lodMode", value: mutation.value };
      case "setLayerStrokeScale": return mutation.value === null ? { op: "remove", path: `/layerStrokeScale/${mutation.layerId}` } : { op: "add", path: `/layerStrokeScale/${mutation.layerId}`, value: mutation.value };
    }
  })();
  return applyPatch(structuredClone(base), [operation as Operation], false, false).newDocument;
};

export function testGisMapWindowOwnershipOracle(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword({ keyword: "x-semio-state", metaSchema: { type: "string" } });
  const validate = ajv.compile(schema);
  assert(validate(fixture.baseConfig), JSON.stringify(validate.errors));
  assert.deepEqual(parseMapWindowConfig(fixture.baseConfig), fixture.baseConfig);
  const documentBytes = JSON.stringify(fixture.document);
  const exactInstances = fixture.windowInstances.filter((instance) => instance.windowKindId === "gis2d-main");
  const actual = Object.fromEntries(exactInstances.map((instance) => [instance.id, structuredClone(fixture.baseConfig)]));
  const oracle = structuredClone(actual);
  const applyRows = (rows: MutationRow[]): void => {
    for (const row of rows) {
      target(fixture.windowInstances, row.windowId, row.windowKindId);
      actual[row.windowId] = applyMapWindowConfigMutation(actual[row.windowId], row.mutation);
      oracle[row.windowId] = patch(oracle[row.windowId], row.mutation);
    }
    assert.deepEqual(actual, oracle);
    assert.equal(JSON.stringify(fixture.document), documentBytes);
  };
  applyRows(fixture.mutations);
  assert.deepEqual(actual, oracle);
  assert.deepEqual(actual, fixture.expected);
  assert.notEqual(actual["gis-map-left"].cameraJson, actual["gis-map-right"].cameraJson);
  applyRows(fixture.undoMutations);
  assert.deepEqual(actual, Object.fromEntries(exactInstances.map((instance) => [instance.id, fixture.baseConfig])));
  applyRows(fixture.redoMutations);
  assert.deepEqual(actual, fixture.expected);
  for (const rejection of fixture.rejections) assert.throws(() => target(fixture.windowInstances, rejection.windowId, rejection.claimedWindowKindId), new RegExp(rejection.code));
  assert.throws(() => parseMapWindowConfig({ ...fixture.baseConfig, layerVisibility: { water: "false" } }));
  console.log("[DEBUG] gis-map-window-ownership windows=2 schema=ajv implementation=typescript oracle=json-patch document=stable");
}

testGisMapWindowOwnershipOracle();
