import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

type Fixture = { request: Record<string, unknown>; complete: Result; stepped: Result; faulted: Result };
type Result = {
  complete: boolean;
  progress: { completed: number; total: number; fraction: number };
  computed: number;
  cacheHits: number;
  fuelUsed: number;
  faulted: number;
  cursor: { digest: string; next: number } | null;
  widgets: { id: string; dep: string; quality: string; fault: { code: string; en: string; de: string; port: string | null } | null; outputs: { port: string; kind: string; detail: string }[] }[];
};

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const fixture = read("../../🧫️fixtures/🔣️.json") as Fixture;
const ajv = new Ajv({ allErrors: true, strict: true, strictRequired: false, allowUnionTypes: true });
const validateRequest = ajv.compile(read("../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🛰️service/📥️request.json"));
const validateResult = ajv.compile(read("../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🛰️service/📤️result.json"));
const snapshotSchema = read("../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json");
const snapshotOracle = new Ajv({ allErrors: true, strict: false });
snapshotOracle.addSchema(read("../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json"));
const validateSnapshot = snapshotOracle.compile(snapshotSchema);

const fails = (validate: ReturnType<typeof ajv.compile>, value: unknown): boolean => !validate(value);

test("the committed request validates against the published request schema and its snapshot against the snapshot schema", () => {
  if (!validateRequest(fixture.request)) throw new Error(JSON.stringify(validateRequest.errors));
  if (!validateSnapshot(fixture.request.snapshot)) throw new Error(JSON.stringify(validateSnapshot.errors));
  console.log("[DEBUG] Host geometry request and typed snapshot agree with the independent JSON Schema oracle");
});

test("the request schema refuses both carriers at once, neither, unknown members and an empty generation", () => {
  const snapshot = fixture.request.snapshot;
  expect(fails(validateRequest, { snapshot, document: { pack: "AA==", spr: "AA==" } })).toBe(true);
  expect(fails(validateRequest, {})).toBe(true);
  expect(fails(validateRequest, { snapshot, targets: [] })).toBe(true);
  expect(fails(validateRequest, { snapshot, generation: "" })).toBe(true);
  expect(fails(validateRequest, { document: { pack: "AA==", spr: "AA==" }, generation: "g1" })).toBe(false);
});

for (const name of ["complete", "stepped", "faulted"] as const) {
  test(`the ${name} result validates against the published result schema and states a consistent run`, () => {
    const result = fixture[name];
    if (!validateResult(result)) throw new Error(JSON.stringify(validateResult.errors));
    expect(result.progress.completed).toBeLessThanOrEqual(result.progress.total);
    expect(result.complete).toBe(result.cursor === null);
    expect(result.complete).toBe(result.progress.completed === result.progress.total);
    expect(result.faulted).toBe(result.widgets.filter(widget => widget.fault !== null).length);
    expect(result.widgets.map(widget => widget.id)).toEqual([...result.widgets.map(widget => widget.id)].sort());
    for (const widget of result.widgets) {
      if (widget.fault) {
        expect(widget.fault.en.length).toBeGreaterThan(0);
        expect(widget.fault.de.length).toBeGreaterThan(0);
        expect(widget.fault.en).not.toBe(widget.fault.de);
        expect(widget.outputs).toEqual([]);
      }
    }
  });
}

test("the result schema refuses an unknown quality, a malformed fault code and a missing cursor", () => {
  const broken = (change: (result: Result) => void): Result => {
    const copy = structuredClone(fixture.complete);
    change(copy);
    return copy;
  };
  expect(fails(validateResult, broken(result => { result.widgets[0]!.quality = "perfect"; }))).toBe(true);
  expect(fails(validateResult, broken(result => { result.widgets[0]!.fault = { code: "Bad Code", en: "x", de: "y", port: null }; }))).toBe(true);
  expect(fails(validateResult, broken(result => { delete (result as Partial<Result>).cursor; }))).toBe(true);
  expect(fails(validateResult, broken(result => { result.progress.fraction = 1.5; }))).toBe(true);
});

test("the stepped result is an unfinished prefix of the complete one: same dependency hashes for the widgets it finished", () => {
  const done = new Map(fixture.complete.widgets.map(widget => [widget.id, widget.dep]));
  expect(fixture.stepped.complete).toBe(false);
  for (const widget of fixture.stepped.widgets) expect(done.get(widget.id)).toBe(widget.dep);
  expect(fixture.stepped.widgets.length).toBe(fixture.stepped.progress.completed);
});
