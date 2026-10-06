import { JsonSyntaxWorkspace } from "../../../../🎒️pack/🔤️json/📥️decode/🟦️.ts";
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020.js";
import ts from "typescript";
import { parseTree, findNodeAtLocation, type ParseError } from "jsonc-parser";

const root = resolve(import.meta.dir, "..");
const contract = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8"));
const corpus = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🔣️.json"), "utf8"));
const limits = { sourceBytes: 2000000, documentBytes: 1000000, coordinateBytes: 1000000, depth: 256, schemas: 10000, resources: 10000, references: 10000, chunk: 128 };
const control = () => ({ syntax: new JsonSyntaxWorkspace(), maximumUnits: 1000000000, maximumOwnedBytes: 4000000000, limits: { ...limits }, cancelled: () => false, progress: (_: import("../🟦️.ts").SchemaReferenceProgress) => {}, yield: async () => { await new Promise<void>(resolve => setTimeout(resolve, 0)); } });
type Position = { id: string; path: string; pointer: string };
type Edge = { origin: Position; reference: string; resourceUri: string; target: Position };
type Closure = { root: unknown; documents: { id: string; source: string }[]; edges: Edge[]; resources: { uri: string; origin: Position }[] };
type Api = { schemaReferenceClosure(request: unknown, authority: ReturnType<typeof control>): Promise<Closure> };

async function api(): Promise<Api> {
  const module = await import(resolve(root, "🟦️.ts"));
  expect(typeof module.schemaReferenceClosure).toBe("function");
  return module;
}

test("closed language-neutral corpus retains independent AJV observations and declared compiler refusal", () => {
  const validator = new Ajv({ strict: true }).addSchema(contract);
  expect(new Set(corpus.cases.map((row: { id: string }) => row.id)).size).toBe(corpus.cases.length);
  for (const row of corpus.cases) {
    expect(validator.validate(contract.$id, row.request)).toBe(true);
    if (row.oracle === null) {
      if (row.oracleFailure) {
        const oracle = new Ajv({ strict: false });
        oracle.addSchema(JSON.parse(row.request.root.source), row.request.root.baseUri);
        expect(() => oracle.getSchema(row.request.root.baseUri)).toThrow(row.oracleFailure.message);
      }
      continue;
    }
    const ajv = row.request.dialect === "2020-12" ? new Ajv2020({ strict: false }) : new Ajv({ strict: false });
    let validateInstance: ReturnType<Ajv["compile"]> | undefined;
    try {
      for (const input of row.request.resources) ajv.addSchema(JSON.parse(input.source), input.baseUri);
      ajv.addSchema(JSON.parse(row.request.root.source), row.request.root.baseUri);
      validateInstance = ajv.getSchema(row.request.root.baseUri);
    } catch {}
    expect(Boolean(validateInstance)).toBe(row.oracle.compile);
    if (validateInstance) for (const instance of row.oracle.instances) expect(Boolean(validateInstance(instance.value))).toBe(instance.valid);
  }
});

test("actual closure preserves raw inputs, each origin occurrence, and unique physical reachability", async () => {
  const owner = await api();
  const validate = new Ajv({ strict: true }).addSchema(contract).compile({ $ref: contract.$id + "#/$defs/closure" });
  for (const row of corpus.cases) {
    const before = JSON.stringify(row.request);
    if ("error" in row.expected) {
      await expect(owner.schemaReferenceClosure(row.request, control())).rejects.toMatchObject({ code: row.expected.error });
    } else {
      const result = await owner.schemaReferenceClosure(row.request, control());
      expect(validate(result)).toBe(true);
      expect(result.root).toEqual(row.request.root);
      expect(result.documents.map(input => input.id)).toEqual(row.expected.documents);
      expect(result.edges.map(edge => [edge.reference, edge.target.id, edge.target.pointer, edge.resourceUri])).toEqual(row.expected.targets);
      for (const input of result.documents) expect(input).toEqual(row.request.resources.find((candidate: { id: string }) => candidate.id === input.id));
      for (const edge of result.edges) {
        const input = [row.request.root, ...row.request.resources].find(candidate => candidate.id === edge.origin.id);
        expect(edge.origin.path).toBe(input.path);
        expect(edge.origin.pointer.endsWith("/$ref")).toBe(true);
        const target = [row.request.root, ...row.request.resources].find(candidate => candidate.id === edge.target.id);
        const errors: ParseError[] = [];
        const tree = parseTree(target.source, errors, { disallowComments: true, allowTrailingComma: false });
        expect(errors).toEqual([]);
        const at = edge.target.pointer === "" ? [] : edge.target.pointer.slice(1).split("/").map(part => part.replace(/~1/g, "/").replace(/~0/g, "~"));
        let located = tree;
        for (const part of at) located = located ? findNodeAtLocation(located, [located.type === "array" ? Number(part) : part]) : undefined;
        expect(["object", "boolean"].includes(located?.type ?? "")).toBe(true);
      }
    }
    expect(JSON.stringify(row.request)).toBe(before);
  }
});

test("explicit budgets refuse before publishing an accepted partial closure", async () => {
  const owner = await api();
  const simple = corpus.cases.find((row: { id: string }) => row.id === "repeated-external").request;
  const nested = { ...simple, resources: [], root: { ...simple.root, source: '{"allOf":[{"allOf":[true]}]}' } };
  for (const [budget, code, request] of [
    ["sourceBytes", "source-byte-budget", simple],
    ["documentBytes", "document-byte-budget", simple],
    ["coordinateBytes", "coordinate-byte-budget", simple],
    ["schemas", "schema-budget", simple],
    ["resources", "resource-budget", simple],
    ["references", "reference-budget", simple],
    ["depth", "depth-budget", nested],
  ] as const) {
    const operation = control();
    operation.limits = { ...limits, [budget]: 1 };
    await expect(owner.schemaReferenceClosure(request, operation)).rejects.toMatchObject({ code });
  }
});

test("cancellation is observed before work and during real scheduled bounded chunks", async () => {
  const owner = await api();
  const request = corpus.cases.find((row: { id: string }) => row.id === "recursive-object").request;
  let callbacks = 0;
  await expect(owner.schemaReferenceClosure(request, { ...control(), cancelled: () => true, progress: () => { callbacks++; } })).rejects.toMatchObject({ code: "cancelled" });
  expect(callbacks).toBe(0);
  const large = { ...request, root: { ...request.root, source: JSON.stringify({ allOf: Array.from({ length: 400 }, () => ({ type: "string" })) }) } };
  let cancelled = false, yielded = 0;
  const operation = { ...control(), limits: { ...limits, chunk: 16 }, cancelled: () => cancelled, yield: async () => {
    yielded++;
    setTimeout(() => { cancelled = true; }, 0);
    await new Promise<void>(resolve => setTimeout(resolve, 1));
  } };
  await expect(owner.schemaReferenceClosure(large, operation)).rejects.toMatchObject({ code: "cancelled" });
  expect(yielded).toBe(1);
});

test("progress is monotonic, source bytes are charged once, and reference repetitions remain distinct", async () => {
  const owner = await api();
  const request = corpus.cases.find((row: { id: string }) => row.id === "cyclic-external").request;
  const observed: Record<string, number>[] = [];
  await owner.schemaReferenceClosure(request, { ...control(), progress: value => { observed.push({ ...value }); } });
  expect(observed.length).toBeGreaterThan(1);
  for (let index = 1; index < observed.length; index++) for (const key of Object.keys(observed[index]!)) expect(observed[index]![key]!).toBeGreaterThanOrEqual(observed[index - 1]![key]!);
  const final = observed.at(-1)!;
  expect(final.sourceBytes).toBe([request.root, ...request.resources].reduce((sum, input) => sum + new TextEncoder().encode(input.source).length, 0));
  expect(final.inputs).toBe(3);
  expect(final.references).toBe(4);
});

test("captured input admission refuses accessor execution, aliases, sparse resources, and unsupported control", async () => {
  const owner = await api();
  const request = corpus.cases[0].request;
  let reads = 0;
  const root = { ...request.root };
  Object.defineProperty(root, "source", { enumerable: true, get() { reads++; return "true"; } });
  await expect(owner.schemaReferenceClosure({ ...request, root }, control())).rejects.toMatchObject({ code: "invalid-input" });
  expect(reads).toBe(0);
  await expect(owner.schemaReferenceClosure({ ...request, resources: new Array(1) }, control())).rejects.toMatchObject({ code: "invalid-input" });
  await expect(owner.schemaReferenceClosure(request, { ...control(), limits: { ...limits, chunk: 0 } })).rejects.toMatchObject({ code: "invalid-control" });
  const samePath = { ...request.root, id: "other", baseUri: "https://case.test/other" };
  await expect(owner.schemaReferenceClosure({ ...request, resources: [samePath] }, control())).rejects.toMatchObject({ code: "duplicate-input-path" });
});

test("owned public source API typechecks and has no product or foreign runtime imports", async () => {
  await api();
  const path = resolve(root, "🟦️.ts");
  const source = readFileSync(path, "utf8");
  const parsed = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true);
  const imports = parsed.statements.filter(ts.isImportDeclaration).map(statement => (statement.moduleSpecifier as ts.StringLiteral).text);
  expect(imports.every(specifier => specifier.startsWith(".") && !specifier.includes("🛍️products") && !specifier.includes("🖥️s") && !specifier.includes("🌎️hub"))).toBe(true);
  const program = ts.createProgram([path], { noEmit: true, strict: true, skipLibCheck: true, types: [], target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true, lib: ["lib.es2022.d.ts", "lib.dom.d.ts"] });
  expect(ts.getPreEmitDiagnostics(program).map(diagnostic => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"))).toEqual([]);
});

test("input and operation authority is captured before any awaited host callback", async () => {
  const owner = await api();
  const row = corpus.cases.find((row: { id: string }) => row.id === "explicit-external");
  const request = structuredClone(row.request), original = structuredClone(request), operation = control();
  operation.limits.chunk = 1;
  let mutated = false;
  operation.yield = async () => {
    if (!mutated) {
      mutated = true;
      request.root.source = "false";
      request.resources[0].source = '{"$ref":"missing"}';
      request.resources.length = 0;
      request.annotations.push("$ref");
      operation.limits.sourceBytes = 1;
      operation.limits.references = 1;
    }
  };
  const result = await owner.schemaReferenceClosure(request, operation);
  expect(mutated).toBe(true);
  expect(result.root).toEqual(original.root);
  expect(result.documents).toEqual(original.resources);
  expect(result.edges.map(edge => [edge.reference, edge.target.id, edge.target.pointer, edge.resourceUri])).toEqual(row.expected.targets);
});

test("annotation vocabulary cannot erase standard or dynamic reference authority", async () => {
  const owner = await api(), request = corpus.cases[0].request;
  for (const keyword of ["$ref", "$id", "$dynamicRef", "items", "const", "properties"]) await expect(owner.schemaReferenceClosure({ ...request, annotations: [keyword] }, control())).rejects.toMatchObject({ code: "invalid-vocabulary" });
});

test("wide schema indexing yields while admitting children and cannot allocate beyond schema authority", async () => {
  const owner = await api(), base = corpus.cases[0].request;
  const request = { ...base, root: { ...base.root, source: JSON.stringify({ properties: Object.fromEntries(Array.from({ length: 2000 }, (_, index) => ["field" + index, true])) }) } };
  let stopped = false;
  let observed = { schemas: 0, inputs: 0 };
  const operation = { ...control(), limits: { ...limits, chunk: 1 }, cancelled: () => stopped, progress: (value: { schemas: number; inputs: number }) => { observed = value; }, yield: async () => { if (observed.inputs === 1 && observed.schemas === 1) stopped = true; } };
  await expect(owner.schemaReferenceClosure(request, operation)).rejects.toMatchObject({ code: "cancelled" });
  expect(observed.schemas).toBe(1);
  await expect(owner.schemaReferenceClosure(request, { ...control(), limits: { ...limits, schemas: 1 } })).rejects.toMatchObject({ code: "schema-budget" });
});
