import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {createRequire} from "node:module";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
import {test} from "bun:test";
import { testDemonstratorBrowserCatalog } from "../📇️catalog/🧪️tests/🟦️.ts";
import * as runtime from "../🟦️.ts";
import { demonstratorRuntimeAssetSources } from "../📦️assets/🟦️.ts";
/** 🎪️ Keeps Vite's runtime description independent of build commands and brand implementations. */
export async function testDemonstratorRuntime(workspace: string): Promise<void> {
  await testDemonstratorBrowserCatalog(workspace);
  const require = createRequire(import.meta.url), directory = join(workspace, "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime");
  const catalog = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8")), fixture = JSON.parse(readFileSync(join(directory, "🧫️cases.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(directory, "🧬️schema/🔣️.json"), "utf8")), validator = new (require("ajv").default)();
  validator.addSchema(schema);
  const contract = fixture.catalogTest;
  assert.ok(validator.validate({ $ref: schema.$id + "#/$defs/DemonstratorRuntimeTestContract" }, contract));
  const project = JSON.parse(readFileSync(join(directory, "../../📋️project.json"), "utf8")), target = project.targets[contract.target], semio = target.metadata?.semio;
  const executionSchema = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/🔣️.json"), "utf8"));
  const validateExecution = new (require("ajv").default)({ strict: true }).compile({ ...executionSchema, $ref: "#/definitions/Source" });
  assert.equal(validateExecution({ kind: "source" }), false);
  assert.ok(validateExecution(semio?.execution));
  assert.equal(semio.execution.version, contract.executionVersion);
  assert.equal(semio.execution.kind, contract.executionKind);
  assert.equal(semio.sourceInputObservation, contract.observation);
  assert.deepEqual(semio.runtimeSourceEntries, contract.runtimeSourceEntries);
  assert.deepEqual(target.dependsOn, contract.dependsOn);
  assert.equal(target.options.command, contract.command);
  const compiler = require("typescript"), routerPath = join(directory, "../../📜️script.ts"), router = compiler.createSourceFile(routerPath, readFileSync(routerPath, "utf8"), compiler.ScriptTarget.Latest, true);
  const helperPath = join(directory, "../..", contract.runtimeSourceEntries[0].replace("{projectRoot}/", "")), helper = compiler.createSourceFile(helperPath, readFileSync(helperPath, "utf8"), compiler.ScriptTarget.Latest, true);
  const runtimeImports = helper.statements.filter((node: any) => compiler.isImportDeclaration(node) && compiler.isStringLiteral(node.moduleSpecifier) && contract.runtimeImports.includes(node.moduleSpecifier.text)).map((node: any) => node.moduleSpecifier.text);
  assert.deepEqual(runtimeImports.sort(), [...contract.runtimeImports].sort());
  const routes: string[] = [], entries: string[] = [];
  const visitRoutes = (node: any): void => {
    if (compiler.isCallExpression(node) && compiler.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && node.arguments.length === 2 && compiler.isStringLiteral(node.arguments[0]) && node.arguments[0].text === contract.route && compiler.isIdentifier(node.arguments[1])) routes.push(node.arguments[1].text);
    compiler.forEachChild(node, visitRoutes);
  };
  visitRoutes(router);
  assert.equal(routes.length, 1);
  const visitClasses = (node: any): void => {
    if (compiler.isClassDeclaration(node) && node.name?.text === routes[0]) {
      const visitCommand = (child: any): void => {
        if (compiler.isCallExpression(child) && compiler.isIdentifier(child.expression) && child.expression.text === "runOwnedCommand") {
          assert.ok(compiler.isPropertyAccessExpression(child.arguments[0]) && child.arguments[0].expression.getText(router) === "process" && child.arguments[0].name.text === "execPath");
          const argumentsNode = child.arguments[1];
          assert.ok(compiler.isArrayLiteralExpression(argumentsNode) && argumentsNode.elements.length === 2 && compiler.isStringLiteral(argumentsNode.elements[0]) && argumentsNode.elements[0].text === "test");
          const entry = argumentsNode.elements[1];
          assert.ok(compiler.isCallExpression(entry) && compiler.isIdentifier(entry.expression) && entry.expression.text === "join" && entry.arguments.length === 2 && compiler.isPropertyAccessExpression(entry.arguments[0]) && entry.arguments[0].expression.kind === compiler.SyntaxKind.ThisKeyword && entry.arguments[0].name.text === "root" && compiler.isStringLiteral(entry.arguments[1]));
          entries.push("{projectRoot}/" + entry.arguments[1].text);
        }
        compiler.forEachChild(child, visitCommand);
      };
      visitCommand(node);
    }
    compiler.forEachChild(node, visitClasses);
  };
  visitClasses(router);
  assert.deepEqual(entries, contract.runtimeSourceEntries);
  assert.ok(validator.validate(schema.$id, catalog));

  const bundle = await require("esbuild").build({ entryPoints: [join(directory, "🟦️.ts")], absWorkingDir: workspace, bundle: true, write: false, platform: "node", format: "esm", packages: "external", metafile: true, logLevel: "silent" });
  const inputs = Object.keys(bundle.metafile.inputs);
  assert.ok(inputs.every(path => !path.endsWith("📜️script.ts") && !path.endsWith("🪧️brand.ts")), "Runtime descriptions must not load task or brand implementations");
  assert.deepEqual(runtime.demonstratorRuntimeBuildVariants(fixture.primary), fixture.additionalBuildVariants);
  assert.equal(new Set(catalog.panes.map((row: any) => row.variant)).size, catalog.panes.length);
  for (const row of catalog.panes) assert.equal(runtime.demonstratorPaneRuntimeVariant(row.variant), row.runtimeVariant);
  for (const variant of fixture.invalidVariants) assert.throws(() => runtime.demonstratorPaneRuntimeVariant(variant), /variant/);
  const producerRows = await import(pathToFileURL(join(workspace, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🧩️plugins/🟦️.ts")).href);
  const declarations = new Map<string, string>();
  for (const row of [...producerRows.PLUGIN_BUILD_TARGETS, ...producerRows.EXTENSION_TARGETS]) {
    const text = readFileSync(join(workspace, row.cratePath, "Cargo.toml"), "utf8"), independent = require("@iarna/toml").parse(text), firstParty = Bun.TOML.parse(text);
    assert.deepEqual(firstParty, independent);
    const semio = independent.package.metadata.semio;
    assert.equal(semio["component-kind"], row.role);
    assert.equal(semio["deployment-directory"], row.directoryName);
    assert.equal(independent.package.metadata.component.package, "semio:" + row.pluginId);
    declarations.set(row.pluginId, semio["deployment-directory"]);
  }
  const names = (ids: string[]) => ids.map(id => { assert.ok(declarations.has(id), id); return declarations.get(id)!; });
  const layout = runtime.demonstratorRuntimeModuleLayout([...new Set<string>(runtime.DEMONSTRATOR_RUNTIME_TARGETS.map((row: any) => row.pluginId))]);
  assert.deepEqual(layout.pluginModuleDirNames, [...fixture.supportDirectories, ...names(fixture.pluginIds)]);
  assert.deepEqual(layout.extensionModuleDirNames, names(fixture.extensionIds));
  const pipeline = JSON.parse(readFileSync(join(directory, "🧫️pipeline.json"), "utf8"));
  assert.ok(validator.validate({ $ref: schema.$id + "#/$defs/DemonstratorPipelineContract" }, pipeline));
  assert.deepEqual(runtime.DEMONSTRATOR_RUNTIME_TARGETS.map((row: any) => row.variant).sort(), pipeline.variants);

  const releasePrerequisites=["prepare-release",...pipeline.wasmTargets,...pipeline.site.artifactTargets],independentProject=compiler.parseConfigFileTextToJson("project.json",readFileSync(join(directory,"../../📋️project.json"),"utf8"));
  assert.equal(independentProject.error,undefined);
  assert.deepEqual(project.targets.build.dependsOn,releasePrerequisites);
  assert.deepEqual(independentProject.config.targets.build.dependsOn,releasePrerequisites);
  const owners=JSON.parse(readFileSync(join(workspace,"📋️build-owners.json"),"utf8")).owners;
  for(const identity of pipeline.site.artifactTargets){const separator=identity.lastIndexOf(":"),owner=owners.find((row:any)=>row.name===identity.slice(0,separator));assert.ok(owner,identity);const producer=JSON.parse(readFileSync(join(workspace,owner.manifest),"utf8"));assert.ok(producer.targets[identity.slice(separator+1)],identity);assert.equal(typeof producer.targets[identity.slice(separator+1)].options.command,"string");}
  assert.equal(project.targets.prepare, undefined, "Only profile-qualified preparation targets exist");
  assert.deepEqual(project.targets.build.outputs, ["{projectRoot}/.🧾️store/build.json"]);
  assert.equal(project.targets.build.options.command, `bun ./${pipeline.site.command} build`);
  assert.equal(project.targets.build.options.env?.SEMIO_BUILD_MODE, pipeline.site.profile, "Site publication consumes its declared profile, independent of the caller environment");
  assert.equal(project.targets.build.defaultConfiguration, pipeline.site.profile);
  assert.equal(project.targets.build.metadata.semio.buildMode.fixed, pipeline.site.profile);
  assert.deepEqual(project.targets["activate-prototype"]?.dependsOn, ["prepare-prototype", pipeline.development.activationTarget]);
  assert.deepEqual(project.targets["activate-prototype"]?.outputs, ["{projectRoot}/dist/♻️activation/prototype"], "Demonstrator union receipt must restore from the build cache");
  assert.equal(project.targets["activate-prototype"]?.cache, true, "the union receipt is a pure function of the lane receipts");
  assert.equal(project.targets["activate-lanes-prototype"]?.executor, "repo:noop", "the lane gate is an ordering node without a process");
  for (const name of ["serve", "dev", "prepare-e2e", "serve-e2e"]) assert.ok(project.targets[name]?.inputs?.some((input: any) => input?.dependentTasksOutputFiles === "**/*" && input.transitive === true), `${name} restores the transitive closure it serves from`);
  for (const name of ["serve", "dev"]) {
    assert.equal(project.targets[name]?.continuous, true);
    assert.equal(project.targets[name]?.cache, false);
    assert.deepEqual(project.targets[name]?.dependsOn, ["activate-prototype"]);
    assert.equal(project.targets[name]?.options.command, `bun ./${pipeline.command} serve`);
  }
  assert.deepEqual(project.targets["prepare-e2e"]?.dependsOn, ["activate-prototype", pipeline.e2e.browserTarget]);
  assert.deepEqual(project.targets["serve-e2e"]?.dependsOn, ["prepare-e2e"]);
  assert.equal(project.targets["serve-e2e"]?.continuous, true);
  assert.equal(project.targets["serve-e2e"]?.metadata.semio.continuousSharing, false);
  assert.deepEqual(project.targets["test-e2e"]?.dependsOn, ["serve-e2e"]);
  assert.equal(project.targets["test-e2e"]?.cache, false);
  assert.equal(project.targets["test-e2e"]?.options.command, `bun ./${pipeline.e2e.testCommand} test`);
  const testCommand = readFileSync(join(directory, "../..", pipeline.e2e.testCommand), "utf8");
  for (const forbidden of pipeline.forbiddenOrchestration) assert.equal(testCommand.includes(forbidden), false, `E2E hides ${forbidden}`);
  const workspaceProject = JSON.parse(readFileSync(join(workspace, "📋️project.json"), "utf8"));
  assert.ok(!workspaceProject.targets["deps-browsers"].dependsOn?.includes("deps-javascript"), "Browser preparation must not replace dependencies used by running build consumers");
  const siteCommand = readFileSync(join(directory, "../..", pipeline.site.command), "utf8");
  for (const forbidden of [...pipeline.forbiddenCommandImports, ...pipeline.forbiddenOrchestration]) assert.equal(siteCommand.includes(forbidden), false, `Site build hides ${forbidden}`);
  for (const profile of pipeline.profiles) {
    const preparation = project.targets[`prepare-${profile}`];
    assert.ok(preparation, `Missing Demonstrator ${profile} preparation`);
    assert.equal(preparation.cache, true, `prepare-${profile} only validates already build-materialized bytes and writes nothing`);
    assert.deepEqual(preparation.dependsOn, [...pipeline.variants.map((variant: string) => `${pipeline.preparationProject}:prepare-${variant}-react-${profile}`), ...pipeline.wasmTargets]);
  }

  for (const profile of pipeline.profiles) {
    const sources = demonstratorRuntimeAssetSources(workspace, profile);
    const components = sources.filter((row: any) => row.owner.endsWith(`:browser:${profile}`)), support = sources.filter((row: any) => !components.includes(row));
    assert.equal(components.length, fixture.pluginIds.length + fixture.extensionIds.length);
    assert.deepEqual(support.slice(0, 3).map((row: any) => row.owner), [`browser-support:${profile}:preview2`, `browser-support:${profile}:shard`, "infinite:fonts"]);
    assert.ok(support.length > 3 && support.slice(3).every((row: any) => row.owner.startsWith("canvas:")), "Every source after the browser support is a staged canvas asset");
    assert.equal(new Set(sources.map((row: any) => row.root)).size, sources.length);
    assert.equal(sources.filter((row: any) => row.shimDirectory !== undefined).length, fixture.extensionIds.length);
    for (const row of sources) assert.ok(row.owner === "infinite:fonts" || row.owner.startsWith("canvas:") || row.owner.includes(`:${profile}`), row.owner);
    assert.equal(sources.some((row: any) => row.root.includes("/runtime/") || row.root.includes("🏪️store")), false);
  }
  const command = readFileSync(join(directory, "../..", pipeline.command), "utf8");
  for (const forbidden of [...pipeline.forbiddenCommandImports, ...pipeline.forbiddenOrchestration]) assert.equal(command.includes(forbidden), false, `Demonstrator hides ${forbidden}`);
}


test("the actual demonstrator receives its declared runtime catalog and modules",()=>testDemonstratorRuntime(process.env.SEMIO_WORKSPACE_ROOT??process.cwd()));
