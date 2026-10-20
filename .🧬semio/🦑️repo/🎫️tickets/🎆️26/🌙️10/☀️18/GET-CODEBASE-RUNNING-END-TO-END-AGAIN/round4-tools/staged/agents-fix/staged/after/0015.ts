/** 🧪️ Shared map transport and composition agree with independent schema and JSON Patch evaluators. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import fixture from "../../🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import { valueSchema } from "../../../../../🌱️value/📦️packages/🟦️typescript/🧬️schema/🟦️.ts";
import { applyMapDelta, inverseMapDelta, composeMapDelta, parseMapDelta, MapDeltaApplyError, type MapEntryDelta } from "../../🧬️schema/🟦️.ts";
import type { DslValue } from "../../../../../🌱️value/📦️packages/🟦️typescript/🧬️schema/🟦️.ts";

function patch(entry: MapEntryDelta<DslValue> & { key: string }, base: Record<string, DslValue>): Record<string, DslValue> {
  const present = Object.hasOwn(base, entry.key);
  if (entry.precondition === "never" || entry.precondition === "present" && !present || entry.precondition === "absent" && present) throw new Error("independent presence precondition refused");
  const path = "/" + entry.key.replaceAll("~", "~0").replaceAll("/", "~1");
  const operation = entry.operation;
  if (operation.kind === "reject") throw new Error("explicit rejected transformation");
  if (operation.kind === "remove" && !present) return base;
  const op: Operation = operation.kind === "set" ? { op: "add", path, value: operation.value } : { op: "remove", path };
  return applyPatch(base, [op], true, false).newDocument;
}

/** 🧾️ Neutral examples and exhaustive single-key triples verify partial-function composition. */
export function testSharedMapDeltaOracle(): void {
  const ajv = new Ajv({ strict: false, allErrors: true }).addSchema(valueSchema);
  const validate = ajv.compile(schema);

  const fields = fixture.dslFields;
  assert.deepEqual(fields.map.map(field => field.key).sort(), Object.keys(schema.properties).sort());
  assert.deepEqual(fields.entry.map(field => field.key).sort(), Object.keys(schema.$defs.MapEntryDelta.properties).sort());
  for (const [record, expected] of Object.entries(fields)) {
    assert.deepEqual(expected.map(field => field.id), expected.map((_, index) => index));
    assert.equal(expected.filter(field => field.optional).length, record === "operation" ? 1 : 0);
  }

  for (const vector of fixture.cases) {
    assert.equal(validate(vector.compact), true, JSON.stringify(validate.errors));
    const compact = parseMapDelta(vector.compact);
    assert.deepEqual(compact, vector.compact);
    let composed = parseMapDelta({ entries: {} });
    for (const source of vector.source) composed = composeMapDelta(composed, parseMapDelta({ entries: { [source.key]: { precondition: source.precondition, operation: source.operation } } }));
    assert.deepEqual(composed, compact, vector.name);
    const base = structuredClone(vector.before) as Record<string, DslValue>;
    if ("error" in vector) {
      assert.throws(() => vector.source.reduce((state, entry) => patch(entry as MapEntryDelta<DslValue> & { key: string }, state), base));
      assert.throws(() => applyMapDelta(compact, base), (error) => error instanceof MapDeltaApplyError && error.code === vector.error, vector.name);
      assert.deepEqual(base, vector.before);
    } else {
      const expected = vector.source.reduce((state, entry) => patch(entry as MapEntryDelta<DslValue> & { key: string }, state), base);
      assert.deepEqual(expected, vector.after, vector.name);
      assert.deepEqual(applyMapDelta(compact, base), expected, vector.name);
    }
  }
  for (const vector of fixture.inverseCases) {
    const base = vector.before as Record<string,DslValue>;
    const delta = parseMapDelta(vector.diff);
    const inverse = inverseMapDelta(delta,base);
    const combined = {...applyMapDelta(delta,base),...vector.collaborator} as Record<string,DslValue>;
    const oracle = Object.entries(inverse.entries).reduce((state,[key,entry])=>patch({key,...entry},state),combined);
    assert.deepEqual(oracle,vector.expected,vector.name);
    assert.deepEqual(applyMapDelta(inverse,combined),oracle,vector.name);
  }  for (const value of fixture.invalid) {
    assert.equal(validate(value), false);
    assert.throws(() => parseMapDelta(value));
  }
  const changes = (["any", "present", "absent"] as const).flatMap((precondition) => [
    parseMapDelta({ entries: { key: { precondition, operation: { kind: "set", value: null } } } }),
    parseMapDelta({ entries: { key: { precondition, operation: { kind: "remove" } } } }),
  ]);
  const outcome = (run: () => unknown): unknown => { try { return { value: run() }; } catch { return { rejected: true }; } };
  const freshBases = (): Record<string, DslValue>[] => [{}, { key: "old" }, { key: null }];
  for (const a of changes) for (const b of changes) for (const c of changes) {
    assert.deepEqual(composeMapDelta(composeMapDelta(a, b), c), composeMapDelta(a, composeMapDelta(b, c)));
    for (const base of freshBases()) {
      const expected = outcome(() => [a,b,c].flatMap(delta => Object.entries(delta.entries).map(([key,entry])=>({key,...entry}))).reduce((state, entry) => patch(entry, state), base));
      assert.deepEqual(outcome(() => applyMapDelta(composeMapDelta(composeMapDelta(a, b), c), base)), expected);
    }
  }
  assert.throws(() => parseMapDelta({ entries: [] }));
}
