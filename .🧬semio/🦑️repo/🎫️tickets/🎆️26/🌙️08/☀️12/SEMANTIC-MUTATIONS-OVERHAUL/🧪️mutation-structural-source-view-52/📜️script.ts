import Ajv2020 from "ajv/dist/2020";
import { createHash } from "node:crypto";
import { lstatSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import ts from "typescript";

//#region 🧪️Paths
const scriptDirectory = dirname(fileURLToPath(import.meta.url));
function workspaceFromTicket(start: string): string {
  let current = start;
  for (;;) {
    const state = lstatSync(current);
    if (!state.isDirectory() || state.isSymbolicLink()) throw new Error(`unsafe ticket ancestor ${current}`);
    if (basename(current) === ".🧬semio") return dirname(current);
    const parent = dirname(current);
    if (parent === current) throw new Error("ticket is not beneath .🧬semio");
    current = parent;
  }
}
const workspace = workspaceFromTicket(scriptDirectory);
const rootScript = resolve(workspace, "📜️script.ts");
const discovery = resolve(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts");
const fixtureSchema = resolve(scriptDirectory, "🧫️fixtures/🔣️schema.json");
const fixtureVectors = resolve(scriptDirectory, "🧫️fixtures/🔣️vectors.json");
const controller = fileURLToPath(import.meta.url);
const runDirectory = resolve(scriptDirectory, "🧫️runs");
//#endregion 🧪️Paths

//#region 🧪️Safety
function assertSafe(path: string): void {
  const rel = relative(workspace, path).replaceAll("\\", "/");
  if (!rel || rel === ".." || rel.startsWith("../") || rel.split("/").some((part) => part.normalize("NFC").toLocaleLowerCase("en-US") === "compose")) throw new Error(`unsafe controller input ${path}`);
  let current = workspace;
  for (const part of rel.split("/")) {
    current = resolve(current, part);
    if (lstatSync(current).isSymbolicLink()) throw new Error(`symlinked controller input ${rel}`);
  }
}
function read(path: string): string { assertSafe(path); return readFileSync(path, "utf8"); }
function digest(path: string): string { return createHash("sha256").update(read(path)).digest("hex"); }
//#endregion 🧪️Safety

//#region 🧪️Assertions
let assertions = 0;
function expect(value: unknown, message: string): asserts value { assertions += 1; if (!value) throw new Error(message); }
function declaration(source: string, name: string): string {
  const ast = ts.createSourceFile(rootScript, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const found: ts.FunctionDeclaration[] = [];
  const visit = (node: ts.Node): void => { if (ts.isFunctionDeclaration(node) && node.name?.text === name) found.push(node); ts.forEachChild(node, visit); };
  visit(ast);
  expect(found.length === 1, `expected one actual declaration ${name}, got ${found.length}`);
  return found[0]!.getText(ast);
}
function body(source: string, name: string): string { return declaration(source, name); }
function compile(source: string): string { return ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText.replace(/^export\s+/gmu, ""); }
type NodeKind = "file" | "directory" | "symlink";
function closedFs(root: string, entries: ReadonlyMap<string, { readonly kind: NodeKind; readonly text?: string }>): { readonly reads: string[]; readonly stats: string[]; readonly readFileSync: (path: string) => string; readonly lstatSync: (path: string) => { isFile: () => boolean; isDirectory: () => boolean; isSymbolicLink: () => boolean }; readonly existsSync: (path: string) => boolean; readonly readdirSync: (path: string, options?: unknown) => readonly { readonly name: string; isDirectory: () => boolean }[] } {
  const reads: string[] = [], stats: string[] = [];
  const node = (path: string) => {
    stats.push(path);
    const entry = entries.get(path);
    if (!entry) throw Object.assign(new Error(`ENOENT ${path}`), { code: "ENOENT" });
    return entry;
  };
  return {
    reads, stats,
    readFileSync: (path) => { reads.push(path); const entry = node(path); if (entry.kind !== "file") throw new Error(`not a file ${path}`); return entry.text ?? ""; },
    lstatSync: (path) => { const entry = node(path); return { isFile: () => entry.kind === "file", isDirectory: () => entry.kind === "directory", isSymbolicLink: () => entry.kind === "symlink" }; },
    existsSync: (path) => entries.has(path),
    readdirSync: (path) => {
      const entry = node(path);
      if (entry.kind !== "directory") throw new Error(`not a directory ${path}`);
      const prefix = path.endsWith("/") ? path : `${path}/`;
      const names = new Map<string, NodeKind>();
      for (const [candidate, child] of entries) {
        if (!candidate.startsWith(prefix)) continue;
        const rest = candidate.slice(prefix.length);
        if (!rest || rest.includes("/")) continue;
        names.set(rest, child.kind);
      }
      return [...names].map(([name, kind]) => ({ name, isDirectory: () => kind === "directory" }));
    },
  };
}
function entries(root: string, values: Readonly<Record<string, { readonly kind: NodeKind; readonly text?: string }>>): ReadonlyMap<string, { readonly kind: NodeKind; readonly text?: string }> {
  const output = new Map<string, { readonly kind: NodeKind; readonly text?: string }>([[root, { kind: "directory" }]]);
  for (const [rel, value] of Object.entries(values)) {
    let current = root;
    for (const segment of rel.split("/").slice(0, -1)) { current = join(current, segment); if (!output.has(current)) output.set(current, { kind: "directory" }); }
    output.set(join(root, rel), value);
  }
  return output;
}
//#endregion 🧪️Assertions

//#region 🧪️ActualReaders
function actualDirectoryReader(source: string, fs: ReturnType<typeof closedFs>): (repoRoot: string, rel: string) => string[] {
  const js = compile(`const POLICY_SKIP_DIRS = new Set(["compose", "node_modules"]);\n${declaration(source, "policyReaddirSafe")}\n${declaration(source, "policyListMutationDirs")}`);
  return new Function("readdirSync", "join", `${js}\nreturn policyListMutationDirs;`)(fs.readdirSync, join) as (repoRoot: string, rel: string) => string[];
}
function actualDescriptorReader(source: string, fs: ReturnType<typeof closedFs>): (repoRoot: string, rel: string) => { descriptor?: unknown; problem?: string } {
  const helpers = ["jsonSchemaSubsetObject", "jsonSchemaSubsetValueEquals", "jsonSchemaSubsetTypeMatches", "jsonSchemaSubsetErrors", "validateJsonSchemaSubset", "policyReadFileSafe", "policyMutationDescriptorSchema", "policyMutationDescriptor"].map((name) => declaration(source, name)).join("\n");
  const js = compile(`const MUTATION_DESCRIPTOR_SCHEMA_REL = "schema/🔣️mutation.json"; let mutationDescriptorSchema; const WORKSPACE_ROOT = "/workspace-must-not-be-read"; const canonicalJson = (value) => JSON.stringify(value);\n${helpers}`);
  return new Function("readFileSync", "existsSync", "join", `${js}\nreturn policyMutationDescriptor;`)(fs.readFileSync, fs.existsSync, join) as (repoRoot: string, rel: string) => { descriptor?: unknown; problem?: string };
}
async function actualTestReader(source: string, fs: ReturnType<typeof closedFs>): Promise<(repoRoot: string, leaf: string, rust: string) => boolean> {
  const actual = await import(pathToFileURL(discovery).href);
  const js = compile(["policyMutationLeafOwnedRustSource", "policyMutationLeafTestModulePath", "policyMutationLeafHasRunnableTest"].map((name) => declaration(source, name)).join("\n"));
  return new Function("relative", "isAbsolute", "sep", "resolve", "lstatSync", "join", "readFileSync", "dirname", "inspectRustRunnableTests", `${js}\nreturn policyMutationLeafHasRunnableTest;`)(relative, isAbsolute, sep, resolve, fs.lstatSync, join, fs.readFileSync, dirname, actual.inspectRustRunnableTests) as (repoRoot: string, leaf: string, rust: string) => boolean;
}
async function actualOriginReader(source: string, fs: ReturnType<typeof closedFs>): Promise<(repoRoot: string, mutations: string, rootSource: string, leaves: readonly string[], rust: string) => readonly { readonly mounted: boolean; readonly wrapped: boolean; readonly reason: string | null }[]> {
  const actual = await import(pathToFileURL(discovery).href);
  const helpers = ["policyStripEmoji", "policyKebabToPascal", "inspectMutationRootReachability"].map((name) => declaration(source, name)).join("\n");
  const js = compile(helpers);
  return new Function("isAbsolute", "join", "lstatSync", "readFileSync", "workspaceAuthorityPath", "noFollowDirectoryAncestry", "taxonomyRelativePathIsExcluded", "inspectRustModuleGraphFacts", "inspectRustStructure", "inspectRustMutationMetadataFacts", "mutationTaxonomyCompare", `${js}\nreturn inspectMutationRootReachability;`)(isAbsolute, join, fs.lstatSync, fs.readFileSync, (root: string) => root, () => undefined, actual.taxonomyRelativePathIsExcluded, actual.inspectRustModuleGraphFacts, actual.inspectRustStructure, actual.inspectRustMutationMetadataFacts, (left: string, right: string) => Buffer.from(left).compare(Buffer.from(right))) as (repoRoot: string, mutations: string, rootSource: string, leaves: readonly string[], rust: string) => readonly { readonly mounted: boolean; readonly wrapped: boolean; readonly reason: string | null }[];
}
async function actualStructuralScanner(source: string, fs: ReturnType<typeof closedFs>): Promise<(repoRoot: string, roots: readonly string[]) => unknown> {
  const actual = await import(pathToFileURL(discovery).href);
  const local = ["policyReadFileSafe", "policyMutationStructuralBreach", "policyMutationBinaryTag", "policyMutationEnumVariantNames", "policyMutationRootPurityBreaches", "policyStripEmoji", "policyKebabToPascal", "policyLeadingEmojiPrefix", "policyMutationStructuralBreaches"].map((name) => declaration(source, name)).join("\n");
  const js = compile(local);
  const taxonomy = { componentFileKinds: { "🦀️rust": "rust" }, mutationDescriptorFileKindId: "descriptor", schemaFormats: { "🔗️graphql": { fileKindId: "graphql" }, "🛰️protobuf": { fileKindId: "proto" }, "🔣️jsonschema": { fileKindId: "json" } }, testContributionFileKindId: "test" };
  const filename = (kind: string) => kind === "descriptor" ? "🧬️schema/🔣️.json" : "🦀️.rs";
  return new Function("join", "lstatSync", "readFileSync", "existsSync", "policyFindAllMutationsDirs", "policyMutationValidatedRoots", "policyMutationDirectOwnerBreaches", "inspectRustStructure", "createRustMutationInputInspector", "createRustMutationCodecOwnershipInspector", "inspectRustSourceIdentities", "loadTaxonomy", "canonicalPrimaryFilenameForKind", "inspectMutationRootReachability", "policyListMutationDirs", "policyMutationDescriptor", "policyMutationLeafHasRunnableTest", "mutationPayloadSchemaRelativePath", "POLICY_RS_COMPONENT_LEAF_NAME", "POLICY_TS_COMPONENT_LEAF", "MUTATION_DESCRIPTOR_SCHEMA_REL", `${js}\nreturn policyMutationStructuralBreaches;`)(join, fs.lstatSync, fs.readFileSync, fs.existsSync, () => [], (_repoRoot: string, roots: readonly string[]) => [...roots], () => [], actual.inspectRustStructure, actual.createRustMutationInputInspector, actual.createRustMutationCodecOwnershipInspector, actual.inspectRustSourceIdentities, () => taxonomy, filename, () => [{ leafName: "➕add", mounted: true, wrapped: true, reason: null }], () => ["➕add"], () => ({ descriptor: { owner: "owner/🧬️mutations/➕add", semanticKind: "add", emoji: "➕", aggregateVariant: "Add", payloadSchema: "🦀️.rs#AddMutation", requiredLanguageSurfaces: [], textOpcode: null, binaryTag: null } }), () => true, () => "🧬️schema/🔣️.json", "🦀️.rs", "🟦️.ts", "schema/🔣️mutation.json") as (repoRoot: string, roots: readonly string[]) => unknown;
}
//#endregion 🧪️ActualReaders

//#region 🧪️RedCases
async function redGate(vectors: { root: string; mutations: string; leaf: string; descriptor: string; test: string; codec: string }): Promise<Record<string, unknown>> {
  const source = read(rootScript);
  const direct = `${vectors.mutations}/${vectors.leaf}`;
  const leafRust = `${direct}/🦀️.rs`;
  const aggregate = `${vectors.mutations}/🦀️.rs`;
  const data = entries(vectors.root, {
    [aggregate]: { kind: "file", text: "#[path = \"➕add/🦀️.rs\"] pub mod add; pub use add::AddMutation; pub enum DemoMutation { Add(AddMutation) }" },
    [leafRust]: { kind: "file", text: "#[path = \"🧪️tests/🦀️.rs\"] mod tests; pub struct AddMutation;" },
    [vectors.descriptor]: { kind: "file", text: JSON.stringify({ owner: direct, semanticKind: "add" }) },
    [vectors.test]: { kind: "file", text: "#[test] fn unadmitted_but_runnable() {}" },
    [vectors.codec]: { kind: "file", text: "const BINARY_TAG: u8 = 7;" },
    "schema/🔣️mutation.json": { kind: "file", text: JSON.stringify({ type: "object", required: ["owner", "semanticKind"], properties: { owner: { type: "string" }, semanticKind: { type: "string" } }, additionalProperties: false }) },
  });
  const fs = closedFs(vectors.root, data);
  const dirs = actualDirectoryReader(source, fs);
  const listed = dirs(vectors.root, vectors.mutations);
  expect(listed.includes(vectors.leaf), "actual direct-child reader did not expose the unadmitted leaf");
  const descriptor = actualDescriptorReader(source, fs)(vectors.root, vectors.descriptor);
  expect(descriptor.descriptor !== undefined, `actual descriptor loader did not accept unadmitted descriptor: ${descriptor.problem ?? "unknown"}`);
  const testReader = await actualTestReader(source, fs);
  expect(testReader(vectors.root, direct, "🦀️.rs"), "actual runnable-test reader did not accept unadmitted leaf test");
  const originReader = await actualOriginReader(source, fs);
  const reachability = originReader(vectors.root, vectors.mutations, data.get(join(vectors.root, aggregate))!.text!, [vectors.leaf], "🦀️.rs");
  expect(reachability.length === 1 && reachability[0]!.mounted && reachability[0]!.wrapped, `actual readOrigin did not accept mounted unadmitted leaf: ${JSON.stringify(reachability)}`);
  const scanner = await actualStructuralScanner(source, fs);
  scanner(vectors.root, [vectors.mutations]);
  const descriptorSchema = body(source, "policyMutationDescriptorSchema");
  expect(descriptorSchema.includes("WORKSPACE_ROOT"), "actual descriptor loader no longer shows the fixture-to-workspace fallback red");
  expect(fs.reads.includes(join(vectors.root, vectors.descriptor)), "unadmitted descriptor was not actually read");
  expect(fs.reads.includes(join(vectors.root, vectors.test)), "unadmitted test was not actually read");
  expect(fs.reads.includes(join(vectors.root, leafRust)), "unadmitted origin leaf was not actually read");
  expect(fs.reads.includes(join(vectors.root, vectors.codec)), "actual structural scanner did not read the unadmitted codec contribution");
  return { listed, reachability, reads: fs.reads, stats: fs.stats, defects: ["unadmitted-direct-child", "unadmitted-descriptor", "unadmitted-test", "unadmitted-origin", "workspace-schema-fallback", "scanner-codec-filesystem-seam"] };
}
//#endregion 🧪️RedCases

//#region 🧪️Entrypoint
if (process.argv[2] !== "red") throw new Error("usage: 📜️script.ts red");
const inputs = [rootScript, discovery, fixtureSchema, fixtureVectors, controller];
const before = Object.fromEntries(inputs.map((path) => [path, digest(path)]));
const schema = JSON.parse(read(fixtureSchema));
const vectors = JSON.parse(read(fixtureVectors));
const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
expect(validate(vectors), `neutral structural-view vector invalid: ${JSON.stringify(validate.errors)}`);
const observed = await redGate(vectors);
const after = Object.fromEntries(inputs.map((path) => [path, digest(path)]));
expect(JSON.stringify(before) === JSON.stringify(after), "structural-view input drift during red capture");
const result = { command: "red", assertions, hashes: before, observed };
mkdirSync(runDirectory, { recursive: true });
writeFileSync(resolve(runDirectory, "red.json"), `${JSON.stringify(result, null, 2)}\n`);
console.log(JSON.stringify(result, null, 2));
//#endregion 🧪️Entrypoint
