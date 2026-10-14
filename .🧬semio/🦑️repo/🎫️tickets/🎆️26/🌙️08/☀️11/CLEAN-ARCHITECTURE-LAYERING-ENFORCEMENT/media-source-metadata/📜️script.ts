import { strict as assert } from "node:assert";
import { readFileSync, existsSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import ts from "typescript";
import Ajv from "ajv";
import { parse as parseToml } from "@iarna/toml";
import corpus from "./🔣️.json";

const mode = process.argv[2];
if (process.argv.length !== 3 || mode !== "preflight" && mode !== "verify") throw Error("Expected preflight or verify");
const root = resolve(import.meta.dir, "../../../../../../../.."), library = resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const source = readFileSync(resolve(library, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"), "utf8"), ast = ts.createSourceFile("inference.mjs", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
const names = ["targetExecutionKindV1", "targetWithDefaults", "targetPolicy", "projectWithDefaults", "withNativePreparation", "withWasmTooling"];
const declarations = names.map(name => {
  const selected = ast.statements.filter(ts.isFunctionDeclaration).filter(node => node.name?.text === name);
  assert.equal(selected.length, 1);
  return selected[0]!.getText(ast);
});
const projectSchema = JSON.parse(readFileSync(resolve(library, "🏗️build/🧬️schema/📋️project/🔣️.json"), "utf8"));
const executionSchema = JSON.parse(readFileSync(resolve(library, "🏗️build/🏃️execution/🧬️schema/🔣️.json"), "utf8"));
const validateProject = new Ajv({ strict: true }).compile(projectSchema), validateExecution = new Ajv({ strict: true }).compile(executionSchema);
const policy = JSON.parse(readFileSync(resolve(library, "🏗️build/🔣️policy.json"), "utf8"));
const routingPath = resolve(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const validatorPath = resolve(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts");
const routing = await import(pathToFileURL(routingPath).href), validator = await import(pathToFileURL(validatorPath).href);
const { targetBuildModes } = await import(pathToFileURL(resolve(library, "🏗️build/🎚️mode/🟨️.mjs")).href);
const envelope = routing.createScriptProcessEnvelope({ version: 1, owner: "media-source-metadata-probe", maximumElapsedMilliseconds: 30000 }, { workspaceRoot: root, projects: corpus.cases.map(row => row.project) }, Date.now());
await routing.withScriptProcessEnvelope(envelope, async (invocation: any) => {
  for (const row of corpus.cases) {
    routing.checkScriptInvocation(invocation);
    const project = JSON.parse(readFileSync(resolve(root, row.owner, "📋️project.json"), "utf8")), target = project.targets[row.target];
    assert.equal(project.name, row.project);
    assert.equal(validateProject(project), true, JSON.stringify(validateProject.errors));
    assert.deepEqual(validator.validateJsonSchemaSubset(projectSchema, project), []);
    const calls = { native: 0, projectInputs: 0 }, scripts = { context: { workspaceRoot: root, invocation }, files: new Map(), closures: new Map(), commandDependencies: new Map(), modules: new Set(), resources: new Set() };
    const forbid = (): never => { calls.native++; throw Error("Probe forbids native classification or preparation"); };
    const parameters: Record<string, any> = {
      TARGET_EXECUTION_SCHEMA: "execution-contract", targetExecutionSchema: executionSchema, DEFAULT_EXECUTOR: "repo:script", POLICY: policy,
      SCRIPT_BASENAME: "📜️script.ts", LIBRARY_ROOT: library, WORKSPACE_OWNERSHIP: {},
      existsSync, dirname, isAbsolute, join, relative, resolve,
      readToml: (path: string) => parseToml(readFileSync(path, "utf8")),
      createScriptInputCache: () => scripts,
      cargoTargets: async () => ({}), componentBuildTargets: () => ({}), printDocumentTargets: async () => ({}),
      rootCommandTargets: () => ({}), generatorOutputOwners: () => [], genericCommandFallbackInputs: () => [],
      matchesCommand: () => false, matchesUncached: () => false, mutatingName: () => false, liveName: () => false, cacheableFamily: () => false, verifyCommand: () => false,
      targetBuildModes,
      nativeTargetCommandInputs: forbid, nativePreparation: forbid, nativeDependencyRoots: forbid, targetScriptClosure: forbid,
      genericTargetCommandInputs: forbid, nativeLockInputs: forbid, relativeScriptInputs: forbid,
      projectInputs: async (_json: any, _owner: string, _workspace: string, _facts: any, context: any) => {
        calls.projectInputs++; assert.equal(context, scripts); assert.equal(context.context.invocation, invocation); assert.equal(context.context.invocation.control, invocation.control);
        routing.checkScriptInvocation(invocation); return {};
      },
    };
    const normalizer = new Function(...Object.keys(parameters), declarations.join("\n") + ";return {projectWithDefaults,withNativePreparation,targetExecutionKindV1};")(...Object.values(parameters));
    if (mode === "preflight") {
      assert.equal(target.metadata?.semio?.execution, undefined);
      await assert.rejects(normalizer.projectWithDefaults({ ...project, targets: { [row.target]: target } }, row.owner, resolve(root, row.owner), root, {}, new Map(), undefined, scripts, undefined, "source", [row.target]), /forbids native/);
      assert.equal(calls.native, 1);
      console.log(`[DEBUG] RED actual defining normalizer classifies unmarked ${row.project}:${row.target} through forbidden native branch`);
      continue;
    }
    assert.deepEqual(target.metadata.semio.execution, row.expectedExecution);
    assert.equal(validateExecution(target.metadata.semio.execution), true);
    for (const malformed of [{ version: 1, kind: "source", scope: "test" }, { version: 1, kind: "source", extra: true }, { version: 2, kind: "source" }]) {
      assert.equal(validateExecution(malformed), false);
      assert.throws(() => normalizer.targetExecutionKindV1({ metadata: { semio: { execution: malformed } } }), /execution contract refused/);
    }
    const normalized = await normalizer.projectWithDefaults({ ...project, targets: { [row.target]: target } }, row.owner, resolve(root, row.owner), root, {}, new Map(), undefined, scripts, undefined, "source", [row.target]);
    const prepared = normalizer.withNativePreparation(normalized, root, {}, new Map(), new Map(), new Map()), actual = prepared.targets[row.target];
    assert.equal(calls.native, 0); assert.equal(calls.projectInputs, 1); assert.equal(normalizer.targetExecutionKindV1(actual), "source");
    assert.deepEqual(actual.inputs, row.expectedInputs); assert.deepEqual(actual.outputs, row.expectedOutputs); assert.deepEqual(actual.dependsOn, row.expectedDependsOn); assert.equal(actual.cache, row.expectedCache);
    assert.equal(actual.options.command, row.expectedOptions.command); assert.equal(actual.options.nativeOwnerCommand, undefined); assert.equal(actual.options.ownerArguments, undefined);
    assert.equal(actual.options.cwd, row.owner);
    assert.equal(actual.defaultConfiguration, "prototype"); assert.equal(actual.configurations.prototype.env.SEMIO_BUILD_MODE, "prototype"); assert.equal(actual.configurations.release.env.SEMIO_BUILD_MODE, "release");
    assert.equal(JSON.stringify(actual).includes("nativeTestSources"), false);
    const unmarked = structuredClone(normalized); delete unmarked.targets[row.target].metadata.semio.execution;
    assert.throws(() => normalizer.withNativePreparation(unmarked, root, {}, new Map(), new Map(), new Map()), /forbids native/);
    assert.equal(calls.native, 1);
    console.log(`[DEBUG] GREEN actual defining normalizer + native-preparation gate preserve ${row.project}:${row.target} explicit inputs/options/dependencies/configurations under the same original probe context; missing marker negative reaches denied native preparation`);
  }
});
console.log(`[DEBUG] ${mode}: independent strict Ajv project/execution schemas and first-party validator agree; no actual graph publication/factory/init/frontend/native execution`);
