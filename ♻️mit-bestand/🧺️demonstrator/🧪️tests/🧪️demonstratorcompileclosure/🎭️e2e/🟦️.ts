import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import ts from "typescript";
import { validateJsonSchemaSubset } from "../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🎭️ Keeps dynamic browser sources explicit while retaining native service authority. */
export async function registerE2eSourceScopeTests(vitest: NonNullable<ImportMeta["vitest"]>, workspace: string): Promise<void> {
 const { describe, it, expect } = vitest, directory = dirname(fileURLToPath(import.meta.url)), root = resolve(workspace, "♻️mit-bestand/🧺️demonstrator"), library = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"), build = join(library, "🏗️build");
 const read = (path: string) => readFileSync(path, "utf8"), fixture = JSON.parse(read(join(directory, "🧫️cases.json"))), schema = JSON.parse(read(join(directory, "🧬️schema/🔣️.json"))), parse = (path: string) => ts.createSourceFile(path, read(path), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), expand = (path: string) => resolve(root, path.replace("{projectRoot}/", "")), canonical = (path: string) => "{projectRoot}/" + relative(root, path).replaceAll("\\", "/");
 describe("demonstrator E2E source scope", () => {
  it("validates the actual owner contracts and preserves native prerequisites", () => {
   expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
   const manifest = JSON.parse(read(join(root, "📋️project.json"))), target = manifest.targets[fixture.target], semio = target.metadata.semio, projectSchema = JSON.parse(read(join(build, "🧬️schema/📋️project/🔣️.json"))), admissionSchema = JSON.parse(read(join(build, "🧬️schema/🚪️admission.json"))), targetExecutionSchema = JSON.parse(read(join(build, "🏃️execution/🧬️schema/🔣️.json"))), ownerSchema = { $defs: projectSchema.$defs, ...projectSchema.$defs.Task.properties.metadata.properties.semio }, validateOwner = new Ajv({ strict: true }).compile(ownerSchema);
   for (const value of [fixture.metadata, semio]) { expect(validateOwner(value), JSON.stringify(validateOwner.errors)).toBe(true); expect(validateJsonSchemaSubset(ownerSchema, value)).toEqual([]); }
   const invalid = { ...fixture.metadata, closure: { ...fixture.metadata.closure, version: 1 } };
   expect(validateOwner(invalid)).toBe(false); expect(validateJsonSchemaSubset(ownerSchema, invalid).length).toBeGreaterThan(0);
   const admissionTree = parse(join(build, "🕸️graph/🚪️admission/🟦️.ts")), policyTree = parse(join(build, "🕸️graph/📥️inference/🎯️target-policy/🟨️.mjs")), names = ["buildInputSourceRoleV1", "readBuildRuntimeSourceEntriesV1", "readBuildSourceResourceEntriesV1", "buildNativeSourceObservationV1", "targetExecutionKindV1"], declarations = [...admissionTree.statements, ...policyTree.statements].filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && names.includes(node.name?.text ?? ""));
   expect(declarations).toHaveLength(names.length);
   const code = ts.transpileModule(declarations.map(node => node.getText().replace(/^export\s+/, "")).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;
   const api = new Function("projectSchema", "admissionSchema", "validateJsonSchemaSubset", "targetExecutionSchema", "conformsOwnerContractV1", code + ";return {role:buildInputSourceRoleV1,entries:readBuildRuntimeSourceEntriesV1,resources:readBuildSourceResourceEntriesV1,native:buildNativeSourceObservationV1,kind:targetExecutionKindV1};")(projectSchema, admissionSchema, validateJsonSchemaSubset, targetExecutionSchema, (contract: unknown, value: unknown) => validateJsonSchemaSubset(contract as never, value, targetExecutionSchema).length === 0);
   expect(api.kind(target) ?? null).toBe(fixture.executionKind);
   for (const row of fixture.roles) { const role = api.role(true, false, [row.observation ?? undefined]); expect(role).toBe(row.inputRole); expect(api.native({ selectedSource: api.kind(target) === "source", selectedNative: api.kind(target) !== "source", requestedInputs: true, dependencyInputs: false, inputRole: role })).toBe(row.nativeObservation); }
   expect(api.role(true, false, [semio.sourceInputObservation])).toBe("freshness");
   expect(api.entries(semio.runtimeSourceEntries)).toEqual(fixture.metadata.runtimeSourceEntries); expect(api.resources(semio.closure?.resources)).toEqual(fixture.metadata.closure.resources);
   expect(target.options.command).toBe(fixture.command); expect(target.options.forwardAllArgs).toBe(false); expect(target.cache).toBe(false); expect(target.outputs).toEqual([]); expect(semio.cache).toMatchObject({ version: 1, class: "live", reason: "drives-running-process" }); expect(target.inputs).toBeUndefined(); expect(manifest.namedInputs.default).toContain("{projectRoot}/**/*");
   for (const [name, dependencies] of Object.entries(fixture.dependencies)) expect([...manifest.targets[name].dependsOn].sort(), name).toEqual([...(dependencies as string[])].sort());
   for (const name of ["serve-e2e", "prepare-e2e", "prepare-prototype", "build", "prepare-release"]) { expect(api.kind(manifest.targets[name]) ?? null, name).toBe(null); expect(manifest.targets[name].metadata?.semio?.sourceInputObservation, name).toBeUndefined(); }
   expect(manifest.targets["prepare-prototype"].dependsOn).toHaveLength(13); expect(manifest.targets["serve-e2e"].continuous).toBe(true); expect(manifest.targets["serve-e2e"].metadata.semio.continuousSharing).toBe(false);
  });
  it("discovers the actual dynamic config, spec and text resources", () => {
   const manifest = JSON.parse(read(join(root, "📋️project.json"))), entryMatch = /^bun \.\/(.+\.ts) test$/.exec(manifest.targets[fixture.target].options.command); expect(entryMatch).not.toBeNull();
   const entryPath = resolve(root, entryMatch![1]!), entry = parse(entryPath), helperImport = entry.statements.find((node): node is ts.ImportDeclaration => ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && node.importClause?.namedBindings !== undefined && ts.isNamedImports(node.importClause.namedBindings) && node.importClause.namedBindings.elements.some(binding => binding.name.text === "runTestScript"));
   expect(helperImport).toBeDefined(); const helper = parse(resolve(dirname(entryPath), (helperImport!.moduleSpecifier as ts.StringLiteral).text)); let configPath: string | undefined;
   const visitRunner = (node: ts.Node): void => { if (ts.isArrayLiteralExpression(node)) { const index = node.elements.findIndex(element => ts.isStringLiteral(element) && element.text === "--config"), next = node.elements[index + 1]; if (index >= 0 && next && ts.isCallExpression(next) && ts.isIdentifier(next.expression) && next.expression.text === "join" && next.arguments.slice(1).every(ts.isStringLiteral)) configPath = resolve(root, ...next.arguments.slice(1).map(argument => (argument as ts.StringLiteral).text)); } ts.forEachChild(node, visitRunner); };
   visitRunner(helper); expect(configPath).toBeDefined(); const config = parse(configPath!), properties = new Map<string, ts.Expression>(); let configuredRoot: string | undefined;
   const visitRoot = (node: ts.Node): void => { if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.name.text === "demonstratorDir" && node.initializer && ts.isCallExpression(node.initializer) && node.initializer.arguments.length === 2 && ts.isStringLiteral(node.initializer.arguments[1])) configuredRoot = resolve(dirname(configPath!), node.initializer.arguments[1].text); ts.forEachChild(node, visitRoot); }; visitRoot(config); expect(configuredRoot).toBe(root);
   const visitConfig = (node: ts.Node): void => { if (ts.isPropertyAssignment(node)) properties.set(node.name.getText().replace(/^["']|["']$/g, ""), node.initializer); ts.forEachChild(node, visitConfig); }; visitConfig(config);
   const testDir = properties.get("testDir"), matches = properties.get("testMatch"); expect(testDir && ts.isCallExpression(testDir)).toBe(true); expect(matches && ts.isArrayLiteralExpression(matches)).toBe(true);
   const dirCall = testDir as ts.CallExpression, files = (matches as ts.ArrayLiteralExpression).elements.map(element => { expect(ts.isStringLiteral(element)).toBe(true); return resolve(configuredRoot!, ...(dirCall.arguments.slice(1).map(argument => { expect(ts.isStringLiteral(argument)).toBe(true); return (argument as ts.StringLiteral).text; })), (element as ts.StringLiteral).text); });
   expect([canonical(configPath!), ...files.map(canonical)].sort()).toEqual([...fixture.metadata.runtimeSourceEntries].sort());
   const inputPath = join(library, "⚡️caching/📥️inference/🟨️.mjs"), tree = parse(inputPath), definition = tree.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "commandUrlReferences")!; expect(definition).toBeDefined();
   const refs = new Function("createHash", "require", "let hits=0,misses=0,bytes=0;const facts=new Map(),limit=16777216;" + definition.getText().replace(/^export\s+/, "") + ";return commandUrlReferences;")(createHash, (name: string) => { expect(name).toBe("typescript"); return ts; });
   const reads = files.flatMap(path => refs(path, read(path)).map((literal: string) => canonical(resolve(dirname(path), literal)))); expect([...new Set(reads)].sort()).toEqual([...fixture.dataReads].sort());
   const brandPath = expand(fixture.metadata.closure.resources[0]); expect(reads).toContain(canonical(brandPath)); expect(fixture.metadata.runtimeSourceEntries).not.toContain(canonical(brandPath));
   for (const path of fixture.dataReads) expect(read(expand(path)).length, path).toBeGreaterThan(0);
  });
 });
}
