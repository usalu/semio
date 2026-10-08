import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import Ajv from "ajv";
import { buildCatalogue, catalogue, CATALOGUE_CATEGORY_FILES, checkCatalogue, resolveCatalogueText, type CatalogueCategoryFile, type CatalogueFinding, type CatalogueQuality } from "../../🟦️.ts";

type Patch = { path: (string | number)[]; value?: unknown; delete?: boolean };
type Case = { name: string; patches: Patch[]; appendFile?: Patch[]; expected: CatalogueFinding[] };
type Fixture = {
  unexposedKernelMethods: string[];
  kernelCoverage: { method: string; kind: string }[];
  meshCoverage: { operation: string; kind: string }[];
  minimumKinds: number;
  lookups: { kind: string; label: { en: string; de: string }; quality: CatalogueQuality; inputs: string[]; outputs: string[]; defaults: Record<string, unknown> }[];
  base: unknown;
  cases: Case[];
  schemaRejections: { name: string; patches: Patch[] }[];
};

const folder = new URL("../../", import.meta.url);
const fixture = JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json", folder), "utf8")) as Fixture;
const metaSchema = JSON.parse(readFileSync(new URL("🔣️.json", folder), "utf8"));
const validate = new Ajv({ allErrors: true, strict: true, strictRequired: false }).compile(metaSchema);
const categoryFileNames = readdirSync(folder).filter(name => name.startsWith("🔣️") && name !== "🔣️.json" && name.endsWith(".json")).sort();

const patched = (source: unknown, patches: readonly Patch[]): CatalogueCategoryFile => {
  const copy = JSON.parse(JSON.stringify(source));
  for (const patch of patches) {
    const parent = patch.path.slice(0, -1).reduce((node, key) => node[key], copy);
    const key = patch.path[patch.path.length - 1]!;
    if (patch.delete) delete parent[key];
    else parent[key] = patch.value;
  }
  return copy;
};
const sorted = (findings: readonly CatalogueFinding[]) => findings.map(({ code, owner, port }) => ({ code, owner, ...(port === undefined ? {} : { port }) })).sort((left, right) => JSON.stringify(left) < JSON.stringify(right) ? -1 : 1);

test("every category file validates against the draft-07 meta-schema", () => {
  expect(categoryFileNames.length).toBeGreaterThanOrEqual(27);
  for (const name of categoryFileNames) {
    if (!validate(JSON.parse(readFileSync(new URL(name, folder), "utf8")))) throw new Error(`${name}: ${JSON.stringify(validate.errors)}`);
  }
});

test("the loader bundles exactly the category files of the folder", () => {
  expect(CATALOGUE_CATEGORY_FILES.length).toBe(categoryFileNames.length);
  const ids = new Set(CATALOGUE_CATEGORY_FILES.map(file => file.category.id));
  expect(ids.size).toBe(categoryFileNames.length);
  expect(categoryFileNames).toEqual([...ids].map(id => `🔣️${id.replace(".", "-")}.json`).sort());
});

test("the fixture base validates and every schema rejection fails independent validation", () => {
  if (!validate(fixture.base)) throw new Error(JSON.stringify(validate.errors));
  for (const rejection of fixture.schemaRejections) if (validate(patched(fixture.base, rejection.patches))) throw new Error(`schema accepted: ${rejection.name}`);
});

test("the bundled catalogue has no semantic findings", () => {
  expect(checkCatalogue(CATALOGUE_CATEGORY_FILES)).toEqual([]);
  expect(catalogue().kinds.length).toBeGreaterThanOrEqual(fixture.minimumKinds);
});

for (const testCase of fixture.cases) test(`finding law: ${testCase.name}`, () => {
  const files = [patched(fixture.base, testCase.patches)];
  if (testCase.appendFile) files.push(patched(fixture.base, testCase.appendFile));
  expect(sorted(checkCatalogue(files))).toEqual(sorted(testCase.expected));
});

test("every exposed kernel method and mesh operation has exactly one kind", () => {
  const known = catalogue();
  const claimed = new Map<string, string>();
  for (const { method, kind } of fixture.kernelCoverage) {
    if (known.kind(kind) === undefined) throw new Error(`${method} -> ${kind} is not a kind`);
    if (claimed.has(kind)) throw new Error(`${kind} claimed twice`);
    claimed.set(kind, method);
  }
  for (const { operation, kind } of fixture.meshCoverage) {
    if (known.kind(kind) === undefined) throw new Error(`${operation} -> ${kind} is not a kind`);
    if (claimed.has(kind)) throw new Error(`${kind} claimed twice`);
    claimed.set(kind, operation);
  }
  expect(fixture.kernelCoverage.length).toBe(93);
  expect(fixture.meshCoverage.length).toBe(42);
  expect(fixture.unexposedKernelMethods.length).toBe(7);
});

test("kinds and ports are found by id and name", () => {
  const known = catalogue();
  for (const lookup of fixture.lookups) {
    const kind = known.kind(lookup.kind)!;
    expect(kind.label).toEqual(lookup.label);
    expect(kind.quality).toBe(lookup.quality);
    expect(kind.inputs.map(port => port.name)).toEqual(lookup.inputs);
    expect(kind.outputs.map(port => port.name)).toEqual(lookup.outputs);
    for (const [name, value] of Object.entries(lookup.defaults)) expect(known.port(lookup.kind, name)?.default).toEqual(value);
  }
  expect(known.kind("brep.nowhere.nothing")).toBeUndefined();
  expect(known.port("brep.primitive.box", "nothing")).toBeUndefined();
  expect(known.category("brep.primitive")?.kinds.length).toBe(6);
});

test("texts resolve per locale with English as the fallback", () => {
  const label = catalogue().kind("brep.primitive.box")!.label;
  expect(resolveCatalogueText(label, "de")).toBe("Quader");
  expect(resolveCatalogueText(label, "de-CH")).toBe("Quader");
  expect(resolveCatalogueText(label, "en-GB")).toBe("Box");
  expect(resolveCatalogueText(label, "fr")).toBe("Box");
});

test("a duplicate kind id is refused when building a catalogue", () => {
  const file = patched(fixture.base, []);
  expect(() => buildCatalogue([file, file])).toThrow("duplicate kind id");
});
