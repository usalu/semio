import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import { parseNormResultsWindowConfig, type NormResultsWindowConfig } from "../../🧬️schema/🟦️.ts";
import { applyNormResultsWindowConfigMutation, type NormResultsWindowConfigMutation } from "../../🧬️schema/🧬️mutations/🟦️.ts";

type WindowInstance = { readonly id: string; readonly windowKindId: string };
type MutationRow = {
  readonly windowId?: string;
  readonly focusedWindowId?: string;
  readonly claimedWindowKindId: string;
  readonly mutation: NormResultsWindowConfigMutation;
};
type RejectionRow = Omit<MutationRow, "mutation"> & { readonly code: string };
type Fixture = {
  readonly document: unknown;
  readonly resultsWindowKinds: readonly string[];
  readonly windowInstances: readonly WindowInstance[];
  readonly baseConfig: NormResultsWindowConfig;
  readonly mutations: readonly MutationRow[];
  readonly undoMutations: readonly MutationRow[];
  readonly redoMutations: readonly MutationRow[];
  readonly expected: Readonly<Record<string, NormResultsWindowConfig>>;
  readonly rejections: readonly RejectionRow[];
  readonly validConfigs: readonly unknown[];
  readonly invalidConfigs: readonly unknown[];
};

const here = fileURLToPath(new URL(".", import.meta.url));
const fixture = JSON.parse(readFileSync(`${here}/../../🧫️fixtures/🔬️window-ownership/🔣️.json`, "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(`${here}/../../🧬️schema/🔣️.json`, "utf8"));

const target = (instances: readonly WindowInstance[], row: Pick<MutationRow, "windowId" | "focusedWindowId" | "claimedWindowKindId">): string => {
  const id = row.windowId ?? row.focusedWindowId;
  if (id === undefined) throw new Error("norm-results-window-required");
  const instance = instances.find((candidate) => candidate.id === id);
  if (instance === undefined) throw new Error("norm-results-window-stale");
  if (instance.windowKindId !== row.claimedWindowKindId) throw new Error("norm-results-window-kind-required");
  return id;
};

const patch = (base: NormResultsWindowConfig, mutation: NormResultsWindowConfigMutation): NormResultsWindowConfig => {
  const index = mutation.ChangeSelectedCheckIndex.index;
  const operations: Operation[] = index === null || index === undefined
    ? (base.selectedCheckIndex === undefined ? [] : [{ op: "remove", path: "/selectedCheckIndex" }])
    : [{ op: base.selectedCheckIndex === undefined ? "add" : "replace", path: "/selectedCheckIndex", value: index }];
  return applyPatch(structuredClone(base), operations, false, false).newDocument;
};

export function testNormResultsWindowOwnershipOracle(): void {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword({ keyword: "x-semio-state", metaSchema: { type: "string" } });
  const validate = ajv.compile(schema);
  for (const value of fixture.validConfigs) {
    assert(validate(value), JSON.stringify(validate.errors));
    assert.equal(JSON.stringify(parseNormResultsWindowConfig(value)), JSON.stringify(value));
  }
  for (const value of fixture.invalidConfigs) {
    assert.equal(validate(value), false, `Ajv admitted ${JSON.stringify(value)}`);
    assert.throws(() => parseNormResultsWindowConfig(value));
  }
  assert.equal(fixture.resultsWindowKinds.length, 15);
  assert.equal(new Set(fixture.resultsWindowKinds).size, 15);
  assert(fixture.resultsWindowKinds.every((kind) => /^norm-(?:din|en|iso|vdi)[0-9]+-results$/.test(kind)));
  const documentBytes = JSON.stringify(fixture.document);
  const exactInstances = fixture.windowInstances.filter((instance) => instance.windowKindId === "norm-en1996-results");
  const actual = Object.fromEntries(exactInstances.map((instance) => [instance.id, structuredClone(fixture.baseConfig)]));
  const oracle = structuredClone(actual);
  const applyRows = (rows: readonly MutationRow[]): void => {
    for (const row of rows) {
      const id = target(fixture.windowInstances, row);
      actual[id] = applyNormResultsWindowConfigMutation(actual[id]!, row.mutation);
      oracle[id] = patch(oracle[id]!, row.mutation);
    }
    assert.deepEqual(actual, oracle);
    assert.equal(JSON.stringify(fixture.document), documentBytes);
  };
  applyRows(fixture.mutations);
  assert.deepEqual(actual, fixture.expected);
  assert.notDeepEqual(actual["norm-results-left"], actual["norm-results-right"]);
  applyRows(fixture.undoMutations);
  assert.deepEqual(actual, Object.fromEntries(exactInstances.map((instance) => [instance.id, fixture.baseConfig])));
  applyRows(fixture.redoMutations);
  assert.deepEqual(actual, fixture.expected);
  for (const rejection of fixture.rejections) assert.throws(() => target(fixture.windowInstances, rejection), new RegExp(rejection.code));
  console.log("[DEBUG] norm-results-window-ownership families=15 same-kind-windows=2 focus=inspection schema=ajv implementation=typescript oracle=json-patch document=stable");
}

testNormResultsWindowOwnershipOracle();
