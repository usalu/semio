# Current Forms Package Ownership Follow-Up

Read-only architecture audit; previous CURRENT-FRAMEWORK-IMPORT-ENFORCEMENT.md and full receipts retained unchanged. No source edits, generation, tests, Cargo or native commands.

## Correction to earlier package gate conclusion

The full shared artifact TypeScript helper ALREADY implements an explicit package gate: test builds artifact-root ../../🟦️.ts, imports Bun.resolveSync(packageName,packageRoot), creates a package-name TypeScript consumer and checks it with skipLibCheck=false, then runs declared suites. Forms package Script forwards build/check/test to this helper. Prior report only inspected relative consumer suites and incorrectly implied that package-resolution verification had to be created from scratch. Retain this existing gate and strengthen exact named APIs/private-subpath rejection in Forms-owned tests if required. Runtime/test execution has not been performed by this audit.

## Ownership respecting deletion

Forms is a specific artifact with public runtime/type exports and private:true publication flag. Remove its stale infrastructureCases entry; retain the exact command-only no-exports/main/module/types assertion for genuine infrastructure. DO NOT add actual Forms to generic Repo publicExportCases: that test reads each row.owner/package.json directly, so deleting Forms would break general enforcement. Existing authored publicExportCases also depend on concrete specific owners; convert public-package mechanism tests to self-contained synthetic manifests, roots and targets inside ticket-owned test sandboxes. Generic schema describes a public-artifact case shape, not a named Forms exception. Generic authority census should validate only PRESENT optional specific owners, while synthetic deletion tests verify absence is accepted.

Forms-owned gate lives under the actual Forms artifact tree and is invoked by Forms' own Nx build/check/test tasks. Deleting Forms removes its own gate and must leave Repo synthetic architecture/public-package tests runnable. Publication visibility and architectural dependency direction remain separate: neither public exports nor a synthetic case lets neutral code depend on specific Forms/product APIs.

## Precise bounded test-first cohort

1. Generic schema 🧬️schema/🧱️dependency-direction/🔣️.json: define closed synthetic public-package fixture inputs (manifest name/private/type/exports and declared target contents, conditions and expected resolution). Generic fixture 🧫️fixtures/🧱️dependency-direction/🔣️.json: retire Forms from infrastructureCases and replace actual-owner publicExportCases with synthetic positive root/subpath/import/types and negative undeclared-subpath cases. Retain private infrastructure assertions and closed schema rejection vectors.
2. Generic 🧪️tests/🧱️dependency-direction/🟦️.ts: first add a synthetic no-Forms/deleted-specific workspace case; public resolution test writes provided manifest/targets instead of reading specific repo owners. Reuse independent dependency-cruiser oracle, strict TypeScript resolution, and existing removal sandbox. Existing hardcoded infrastructure roster still contains other specific owners; full deleting-specific independence needs generic present-owner validation or synthetic command-only manifest/router/project cases, not simply removal of Forms. Bound this to the same fixture/schema/test cohort.
3. Forms-owned existing suite 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪶️consumers/🟦️.ts: add explicit package-name API and missing-subpath tests under the already-built package test gate. Assert expected snapshot/SQLite/JSON named exports and typed FormsSnapshot/Definition/Response surface. Retain existing helper rather than add a second builder or new script file.
4. Forms package.json: correct stale description; add build/check scripts calling existing Nx tasks if command-list truthfulness requires full executable exposure (currently only test is listed despite build/check/test Nx+router). Existing Forms project and script already own all three commands. Register any missing GUI rows in existing order only after census; do not blindly add duplicates. Generic Repo must not contain Forms-specific executable strings or paths after cohort.

RED must precede production edits, then run closed fixture schema + generic dependency/removal/public resolution oracle suite and Forms-owned Nx build/check/test separately. Deletion check runs generic suite against synthetic workspace without Forms. This audit provides a cohort, not a source-ready patch or passing claim.

## Sync proposal current-byte recheck
Executed Python assertions confirm exact two-file proposal: one basic header + two backlog headers; inverses recover the full originals; both fixture.dsl files remain byte exact. At this final recheck both current edits.ops match PROPOSED exactly (Root published concurrently after the first read). No source write.

```json
[
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧫️fixtures/basic-remote-ops/edits.ops",
    "currentState": "proposed",
    "headerCount": 1,
    "inverseExact": true,
    "beforeSha256": "9e298c60b48d10ac1e0bc3d0a83d971c2931930eb9ba4846d1ce962b4b31becb",
    "currentSha256": "90fb8a4d5b4809ebb7bf0a3654d9d8243bb85c5f99f33d36d82fb20cc0e92ff5"
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧫️fixtures/remote-ops-backlog/edits.ops",
    "currentState": "proposed",
    "headerCount": 2,
    "inverseExact": true,
    "beforeSha256": "d7ba3835e7388e4aa84716bfd6d85287cb98b0eba689206a1c7076b53f2aa34a",
    "currentSha256": "7739f8a1c706fb4ef6d54fb887b0638696289fa23012ca04a290a4053e08cd8d"
  }
]
```

## Full current inputs

### ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🟦️typescript/package.json

SHA-256 `b372654987daea2fb5fc2c3646f1b461e5a109344979ed299fe9682c714a8d93`; 1522 bytes.

```
{
  "$schema": "../../../../../../../node_modules/nx/schemas/project-schema.json",
  "projectType": "library",
  "name": "@semio-tech/forms-js",
  "version": "0.1.0",
  "description": "@semio-tech/forms-js authored owner test infrastructure.",
  "type": "module",
  "private": true,
  "main":"./dist/🟦️.js",
  "types":"./dist/🟦️.d.ts",
  "exports":{".":{"types":"./dist/🟦️.d.ts","import":"./dist/🟦️.js"}},
  "scripts": {
    "test": "bun nx run @semio-tech/forms-js:test"
  },
  "dependencies": {
    "@semio-tech/s-3d-js": "workspace:*",
    "@semio-tech/framework": "workspace:*",
    "@semio-tech/ui-react": "workspace:*",
    "@semio-tech/ui-styling": "workspace:*",
    "@semio-tech/infinite-world-r3f": "workspace:*",
    "@semio-tech/cad-js-module-spatial-shape": "workspace:*",
    "@semio-tech/cad-js-module-aec-building": "workspace:*",
    "@semio-tech/cad-js-module-aec-building-energy": "workspace:*",
    "@semio-tech/cad-js-module-aec-building-structure": "workspace:*"
  },
  "devDependencies": {
    "fast-check": "3.23.2",
    "typescript": "^5.9.3"
  },
  "author": {
    "name": "Ueli Saluz",
    "email": "ueli@semio-tech.com",
    "url": "https://github.com/usalu"
  },
  "license": "LGPL-3.0-or-later",
  "repository": {
    "type": "git",
    "url": "https://github.com/usalu/semio.git",
    "directory": "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🟦️typescript"
  },
  "bundleKind": "repo",
  "nx": {
    "includedScripts": []
  }
}

```

### ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🟦️typescript/📜️script.ts

SHA-256 `20bfcda1a6d52bd732f9c497dbc1194677235c93bc2a9919c99a040062cf7933`; 1379 bytes.

```
#!/usr/bin/env bun
/** 📋️ Forms TypeScript contract and authoring verification. */
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import{runArtifactTypeScriptPackageMain}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
const publicOptions={suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts","🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪶️consumers/🟦️.ts"]};
class PublicScript extends BundleScript{async run():Promise<void>{await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/forms-js",publicOptions);}}
class TestScript extends BundleScript {
  async run(): Promise<void> {
    await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/forms-js",publicOptions);

  }
}
const router = new ScriptRouter(import.meta.dir).register("build",PublicScript).register("check",PublicScript).register("test", TestScript);
await runScriptMain(router, { defaultCommand: "test" });

```

### ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🟦️typescript/📋️project.json

SHA-256 `26b0f4e77b99ff985a80b1cb5808772f8bf42f74d1f164aa73c1cc4e4734224b`; 922 bytes.

```
{
  "name": "@semio-tech/forms-js",
  "$schema": "../../../../../../../node_modules/nx/schemas/project-schema.json",
  "targets": {
    "build":{"executor":"nx:run-commands","options":{"cwd":"✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🟦️typescript","command":"bun ./📜️script.ts build","forwardAllArgs":true},"outputs":["{projectRoot}/dist"]},
    "check":{"executor":"nx:run-commands","options":{"cwd":"✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🟦️typescript","command":"bun ./📜️script.ts check","forwardAllArgs":true},"cache":false},
    "test": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts test",
        "forwardAllArgs": true
      }
    }
  }
}

```

### ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🟦️.ts

SHA-256 `816f0af9bcacbd6a8d974b1de85f9c69351f90474a73068ca358b01f80615cf9`; 1224 bytes.

```
/** 📋️ Forms public owned document, semantic SQLite and intrinsic value types. */
export{parseFormsSnapshot,FORMS_SQLITE_SCHEMA,formsSnapshotToSqliteDatabase,formsSnapshotFromSqliteDatabase,validateFormsSnapshotSqliteDialect}from"./🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export type{FormsSnapshot}from"./🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export type{FormQuestion,FormStep,FormExpr,FormQuestionOption,FormVectorField,DslValue}from"./🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";
export type{FormsResponse,FormsAnswer}from"./🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🟦️.ts";
export type{FormsDefinition}from"./🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📝️definition/🟦️.ts";
export{parseFormsJsonArtifact,parseFormsJsonDiff,parseFormsJsonDefinition,parseFormsJsonResponse,parseFormsJsonQuestion,parseFormsJsonCondition,parseFormsJsonValue,formsArtifactJson,formsValueJson}from"./🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌱️value/🔣️json/🟦️.ts";

```

### ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪶️consumers/🟦️.ts

SHA-256 `e62e21c43463a164f6fb5bd52479fddd8623cde200f7391fef024144d97c1bf8`; 2018 bytes.

```
/** 🧪️ All owning Forms authoring and transport laws share the strict public package gate. */
import{test}from"bun:test";
import{testFormsDocumentContractOracle,testFormsDesignImport,testFormsMutationSchemas}from"./../🪪️document/🟦️.ts";
import{testFormsInspectionAuthoring}from"../../../✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️authoring/🟦️.ts";
import{testFormsQuestionPatches}from"../../../✏️editor/❓️questions/🧪️tests/🔬️patches/🟦️.ts";
import{testFormsQuestionPlacement}from"../../../✏️editor/❓️questions/📍️placement/🧪️tests/🟦️.ts";
import{testFormsExtensionInputs}from"../../../✏️editor/❓️questions/🧩️extensions/🧪️tests/🟦️.ts";
import{testFormsVisibility}from"../../../✏️editor/❓️questions/🫥️visibility/🧪️tests/🔬️editing/🟦️.ts";
import{testFormsPersistence}from"../💾️persistence/🟦️.ts";
import{testFormsResponses,testFormsSubmission}from"../../📨️response/🧪️tests/🔬️events/🟦️.ts";
import{testFormsValidation}from"../../✅️validation/🧪️tests/🟦️.ts";
import{testFormsResponseExport}from"../../📨️response/📤️export/🧪️tests/🟦️.ts";
import"../🧪️change-block-field/🟦️.ts";
test("testFormsDocumentContractOracle",testFormsDocumentContractOracle);
test("testFormsDesignImport",testFormsDesignImport);
test("testFormsMutationSchemas",testFormsMutationSchemas);
test("testFormsInspectionAuthoring",testFormsInspectionAuthoring);
test("testFormsQuestionPatches",testFormsQuestionPatches);
test("testFormsQuestionPlacement",testFormsQuestionPlacement);
test("testFormsExtensionInputs",testFormsExtensionInputs);
test("testFormsVisibility",testFormsVisibility);
test("testFormsPersistence",testFormsPersistence);
test("testFormsResponses",testFormsResponses);
test("testFormsSubmission",testFormsSubmission);
test("testFormsValidation",testFormsValidation);
test("testFormsResponseExport",testFormsResponseExport);

```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts

SHA-256 `247f37a9a1b8d445700c18ebb35dde84d461d425b823d04e5e8dd24fab5ec945`; 8325 bytes.

```
#!/usr/bin/env bun
import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { createRequire } from "node:module";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { runRepositoryCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧪️ The artifact's own TypeScript suites, relative to the artifact root: `check` type-checks them and `test` also runs
 * them with `bun test`, so a suite is registered here once instead of being discovered by a filename convention. */
export interface ArtifactTypeScriptPackageOptions {
  readonly suites?: readonly string[];
}

/** 🟦️ Builds and resolves a declaration-only TypeScript artifact package from its taxonomy source. */
export async function runArtifactTypeScriptPackageMain(packageRoot: string, packageName: string, options: ArtifactTypeScriptPackageOptions = {}): Promise<void> {
  const source = resolve(packageRoot, "../../🟦️.ts"), output = resolve(packageRoot, "dist");
  const typeScript = async (entry: string, args: string[], skipLibraries = true): Promise<void> => {
    await runRepositoryCommand(process.execPath, ["x", "tsc", entry, ...args, "--allowImportingTsExtensions", "--module", "ESNext", "--moduleResolution", "Bundler", "--resolveJsonModule", "--allowSyntheticDefaultImports", "--strict", ...(skipLibraries ? ["--skipLibCheck"] : []), "--target", "ES2022"], getWorkspaceRoot(), `artifact-typescript:${packageName}:tsc`, 120_000);
  };
  const suites = (options.suites ?? []).map((suite) => {
    const path = resolve(packageRoot, "../..", suite);
    if (!existsSync(path)) throw new Error(`${packageName} registers the missing suite ${suite}`);
    return path;
  });
  const checkSuites = async (): Promise<void> => {
    for (const suite of suites) await typeScript(suite, ["--noEmit", "--allowImportingTsExtensions"]);
  };
  const typeRoot = join(output, "🧬️types");
  const copyDeclarationAssets = (): number => {
    const compiler = createRequire(import.meta.url)("typescript"), repositoryRoot = getWorkspaceRoot();
    const files: string[] = [], directories = [typeRoot];
    while (directories.length) { const directory = directories.pop()!; for (const entry of readdirSync(directory, { withFileTypes: true })) { const path = join(directory, entry.name); if (entry.isDirectory()) directories.push(path); else if (/\.d\.[cm]?ts$/.test(entry.name)) files.push(path); } }
    const visited = new Set<string>();
    for (let index = 0; index < files.length; index++) {
      const declaration = files[index]!;
      if (visited.has(declaration)) continue;
      visited.add(declaration);
      let text = readFileSync(declaration, "utf8");
      const originalDirectory = join(repositoryRoot, dirname(relative(typeRoot, declaration)));
      for (const imported of compiler.preProcessFile(text, true, true).importedFiles) {
        if (!imported.fileName.startsWith(".")) continue;
        const sourceImport = resolve(originalDirectory, imported.fileName);
        const candidates = imported.fileName.endsWith(".json") || /\.d\.[cm]?ts$/.test(imported.fileName) ? [sourceImport] : /\.[cm]?[jt]s$/.test(imported.fileName) ? [sourceImport.replace(/\.([cm])?[jt]s$/, (_: string, kind: string | undefined) => `.d.${kind ?? ""}ts`)] : [`${sourceImport}.d.ts`, join(sourceImport, "index.d.ts")];
        for (const from of candidates) {
          if (!existsSync(from)) continue;
          const local = relative(repositoryRoot, from);
          if (local === ".." || local.startsWith(`..${sep}`)) throw new Error(`Declaration asset escapes ${packageName}: ${imported.fileName}`);
          const to = join(typeRoot, local);
          if (!existsSync(to)) { mkdirSync(dirname(to), { recursive: true }); copyFileSync(from, to); }
          if (/\.d\.[cm]?ts$/.test(to)) files.push(to);
          break;
        }
      }
      const syntax = compiler.createSourceFile(declaration, text, compiler.ScriptTarget.Latest, true), edits: { start: number; end: number; value: string }[] = [];
      const visit = (node: any): void => {
        const literal = (compiler.isImportDeclaration(node) || compiler.isExportDeclaration(node)) ? node.moduleSpecifier : compiler.isImportTypeNode(node) && compiler.isLiteralTypeNode(node.argument) ? node.argument.literal : compiler.isExternalModuleReference(node) ? node.expression : undefined;
        if (literal && compiler.isStringLiteral(literal) && literal.text.startsWith(".") && !/\.d\.[cm]?ts$/.test(literal.text) && /\.[cm]?ts$/.test(literal.text)) edits.push({ start: literal.getStart(syntax), end: literal.getEnd(), value: JSON.stringify(literal.text.replace(/\.([cm])?ts$/, (_: string, kind: string | undefined) => `.${kind ?? ""}js`)) });
        compiler.forEachChild(node, visit);
      };
      visit(syntax);
      for (const edit of edits.sort((a, b) => b.start - a.start)) text = text.slice(0, edit.start) + edit.value + text.slice(edit.end);
      writeFileSync(declaration, text);
    }
    const entry = relative(output, join(typeRoot, relative(repositoryRoot, source).replace(/\.ts$/, ".js"))).split(sep).join("/");
    writeFileSync(join(output, "🟦️.d.ts"), `export * from ${JSON.stringify(`./${entry}`)};\n`);
    return visited.size;
  };
  const build = async (): Promise<void> => {
    rmSync(output, { recursive: true, force: true });
    mkdirSync(output, { recursive: true });
    const result = await Bun.build({ entrypoints: [source], outdir: output, naming: "🟦️.js", target: "bun", format: "esm", minify: false });
    if (!result.success) throw new AggregateError(result.logs, `Failed to build ${packageName}`);
    await typeScript(source, ["--declaration", "--emitDeclarationOnly", "--rootDir", getWorkspaceRoot(), "--outDir", typeRoot]);
    const assets = copyDeclarationAssets();
    console.log(`[artifact-typescript] built ${packageName} outputs=${result.outputs.length + 1 + assets}`);
  };
  class BuildScript extends BundleScript { async run(): Promise<void> { await build(); } }
  class CheckScript extends BundleScript {
    async run(): Promise<void> {
      const result = await Bun.build({ entrypoints: [source], target: "bun", format: "esm" });
      if (!result.success) throw new AggregateError(result.logs, `Failed to check ${packageName}`);
      await typeScript(source, ["--noEmit"]);
      await checkSuites();
      console.log(`[artifact-typescript] checked ${packageName} suites=${suites.length}`);
    }
  }
  class TestScript extends BundleScript {
    async run(): Promise<void> {
      await build();
      const artifact = await import(Bun.resolveSync(packageName, packageRoot));
      const probe = join(output, "🧪️consumer.ts"), typeRoots = join(output, "🧪️types");
      mkdirSync(typeRoots);
      const assertion = Object.hasOwn(artifact, "definition") ? "const definitionId: typeof artifact.definition.id = artifact.definition.id;\nvoid definitionId;" : `const artifactModule: typeof import(${JSON.stringify(packageName)}) = artifact;\nvoid artifactModule;`;
      writeFileSync(probe, `import * as artifact from ${JSON.stringify(packageName)};\n${assertion}\n`);
      try { await typeScript(probe, ["--noEmit", "--typeRoots", typeRoots], false); } finally { rmSync(probe, { force: true }); rmSync(typeRoots, { recursive: true, force: true }); }
      assert.equal(typeof artifact, "object", `${packageName} did not resolve as an ES module`);
      await checkSuites();
      for (const suite of suites) await runRepositoryCommand(process.execPath, ["test", `./${relative(getWorkspaceRoot(), suite)}`], getWorkspaceRoot(), `artifact-typescript:${packageName}:suite`, 300_000);
      console.log(`[artifact-typescript] tested ${packageName} exports=${Object.keys(artifact).length} suites=${suites.length}`);
    }
  }
  const router = new ScriptRouter(packageRoot).register("build", BuildScript).register("check", CheckScript).register("test", TestScript);
  const segments = process.argv.slice(2);
  await router.run(segments.length ? segments : ["test"]);
}

```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️dependency-direction/🟦️.ts

SHA-256 `c19580238f2395b24972db35cd7d0249971b19738e6023348d865b10897a3bf5`; 37603 bytes.

```
import { expect, test } from "bun:test";
import { cruise } from "dependency-cruiser";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import glob from "fast-glob";
import ts from "typescript";
import { dependencyDirectionEdges, dependencyDirectionSourceInventory, dependencyDirectionWorkspacePackages, type DependencyDirectionGraphScope } from "../../🕸️dependencies/🧭️direction/🟦️.ts";

type Edge = Readonly<{ id: string; target: string; reference: "relative" | "package" | "package-subpath" | "unresolved-package"; syntax: "static" | "type" | "dynamic" | "export" | "require"; package?: string; installation?: string }>;
type Case = Readonly<{ id: string; from: string; comments?: readonly string[]; dependencies: readonly Edge[]; forbidden: readonly string[]; forbiddenRules?: Readonly<Record<string, readonly string[]>> }>;
type Rule = Readonly<{ name: string; severity: string; from: { path: string[]; pathNot?: string[] }; to: { path: string[]; pathNot?: string[] } }>;
type Graph = Readonly<{ modules: readonly { source: string; dependencies: readonly { module: string; resolved: string; couldNotResolve?: boolean }[] }[]; summary: { violations: readonly { from: string; to: string; rule: { name: string } }[] } }>;
const library = resolve(import.meta.dir, "../.."), repo = resolve(library, "../../../../..");
const read = (path: string): unknown => JSON.parse(readFileSync(join(library, path), "utf8"));
type RemovabilityCase = Readonly<{ id: string; directories: readonly string[]; manifests: Readonly<Record<string, string | object>>; expectedPackages: readonly string[]; expectedPlugins: readonly string[]; accept: boolean; members?: readonly string[]; nextManifests?: Readonly<Record<string, object>>; nextExpectedPackages?: readonly string[] }>;
const fixture = read("🧫️fixtures/🧱️dependency-direction/🔣️.json") as { schemaVersion: number; infrastructureCases: readonly { owner: string; name: string; commands: readonly string[] }[]; typeCases: readonly { id: string; source: string; name: string; value: unknown; accept: boolean }[]; publicExportCases: readonly { id: string; owner: string; specifier: string; accept: boolean; conditions: readonly string[]; target?: string }[]; removabilityCases: readonly RemovabilityCase[]; cases: readonly Case[]; graphScope: DependencyDirectionGraphScope; resolutionCases: readonly { id: string; specifier: string; owner: string; from: string; forbidden: number; authored: boolean; accept: boolean }[]; inventoryCases: readonly { id: string; accept: boolean; roots: readonly string[]; files: readonly string[]; links: readonly { path: string; target: string }[]; expectedSources: readonly string[] }[]; graphCases: readonly { id: string; accept: boolean }[] };
const schema = read("🧬️schema/🧱️dependency-direction/🔣️.json");
type PolicyOwner = Readonly<{ owner: string; name: string; role?: string }>;
const policyOwners = (fixture as typeof fixture & { policyOwners: readonly PolicyOwner[] }).policyOwners;
const policyProviders = ["🔣️taxonomy.json", "🕸️dependencies/🧭️direction/🟦️.ts", "🕸️dependencies/🧭️direction/🚀️bootstrap/🟨️.cjs", "🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs", "🗂️workspaces/🟦️bun/🟦️.ts", "🗂️workspaces/🟦️bun/🟨️.cjs", "🗂️workspaces/📦️payload/🟦️.ts", "🗂️workspaces/📦️payload/🟨️.cjs"];
const config = portablePolicy();
const rules = config.forbidden.filter((rule) => ["framework-no-implementation", "io-renderer-independent", "repo-no-implementation", "s-modules-no-plugins", "plugin-no-extension-or-artifact-📐️cad", "framework-modules-no-products"].includes(rule.name)).map((rule) => {
  const patterns = (value: string | string[]): string[] => typeof value === "string" ? [value] : value;
  return { ...rule, from: { path: patterns(rule.from.path), ...(rule.from.pathNot ? { pathNot: patterns(rule.from.pathNot) } : {}) }, to: { path: patterns(rule.to.path), ...(rule.to.pathNot ? { pathNot: patterns(rule.to.pathNot) } : {}) } };
});
const matches = (patterns: readonly string[], candidate: string): boolean => patterns.some((pattern) => new RegExp(pattern, "u").test(candidate));
function write(root: string, path: string, content: string): void { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
function writePolicyAuthority(root: string): void {
  write(root, "nx.json", "{}");
  write(root, "📋️project.json", JSON.stringify({ metadata: { semio: { taxonomy: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json" } } }));
  for (const source of policyProviders) write(root, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/${source}`, readFileSync(join(library, source), "utf8"));
}

/** 🧪️ Loads the actual policy generator over portable owners verified against their current authored manifests. */
function portablePolicy(): { forbidden: readonly Rule[]; options: { enhancedResolveOptions: { conditionNames: readonly string[] } } } {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-policy-")));
  const boundary = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs";
  try {
    write(root, boundary, readFileSync(join(repo, boundary), "utf8"));
    writePolicyAuthority(root);
    const members = policyOwners.map((row) => row.owner);
    write(root, "package.json", JSON.stringify({ workspaces: members, semio: { workspace: { schemaVersion: 1, members, owners: [] } } }));
    for (const row of policyOwners) {
      const manifest = JSON.parse(readFileSync(join(repo, row.owner, "package.json"), "utf8"));
      if (manifest.name !== row.name || manifest.semio?.dependencyRole !== row.role) throw new Error(`Portable policy owner drift: ${row.owner}`);
      write(root, `${row.owner}/package.json`, JSON.stringify(manifest));
    }
    return createRequire(import.meta.url)(join(root, boundary));
  } finally { rmSync(root, { recursive: true, force: true }); }
}

/** 🧭️ Renders the same neutral dependency intent into the JavaScript parser's supported import forms. */
function render(edge: Edge, from: string): { specifier: string; source: string } {
  const local = relative(dirname(from), edge.target).split("\\").join("/");
  const specifier = edge.reference === "relative" ? local.startsWith(".") ? local : `./${local}` : `${edge.package}${edge.reference === "package-subpath" ? "/schema" : ""}`;
  const literal = JSON.stringify(specifier);
  const sources = {
    static: `import { value } from ${literal}; export const used = value;`,
    type: `import type { Value } from ${literal}; export type Used = Value;`,
    dynamic: `export const used = import(${literal});`,
    export: `export { value } from ${literal};`,
    require: `export const used = require(${literal});`,
  };
  return { specifier, source: sources[edge.syntax] };
}

test("schema defines unique neutral dependency cases and exact expected edges", () => {
  const validate = new Ajv({ strict: true }).compile(schema as object);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.cases.map((row) => row.id)).size).toBe(fixture.cases.length);
  expect(new Set(policyOwners.map((row) => row.name)).size).toBe(policyOwners.length);
  expect(new Set(policyOwners.map((row) => row.owner)).size).toBe(policyOwners.length);
  for (const row of fixture.cases) {
    expect(new Set(row.dependencies.map((edge) => edge.id)).size).toBe(row.dependencies.length);
    expect(row.forbidden.every((id) => row.dependencies.some((edge) => edge.id === id))).toBe(true);
    expect(Object.keys(row.forbiddenRules ?? {}).every((id) => row.forbidden.includes(id))).toBe(true);
  }
});

test("declared semantic roles satisfy the portable schema and package contributions", () => {
  const taxonomy = JSON.parse(readFileSync(join(library, "🔣️taxonomy.json"), "utf8"));
  const policySchema = (schema as { $defs: { DependencyDirections: object } }).$defs.DependencyDirections;
  const validate = new Ajv({ strict: true }).compile(policySchema);
  expect(validate(taxonomy.dependencyDirections), JSON.stringify(validate.errors)).toBe(true);
  for (const rule of Object.values(taxonomy.dependencyDirections.rules) as { fromRoles: string[]; toRoles: string[] }[]) {
    expect([...rule.fromRoles, ...rule.toRoles].every((role) => taxonomy.dependencyDirections.roles[role])).toBe(true);
  }
  const manifest = JSON.parse(readFileSync(join(repo, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json"), "utf8"));
  expect(manifest.semio.dependencyRole).toBe("ui");
});

test("strict taxonomy direction covers paths, package aliases, subpaths, tests and scripts", () => {
  expect(rules).toHaveLength(6);
  expect(rules.every((rule) => rule.severity === "error")).toBe(true);
  for (const row of fixture.cases) {
    const forbidden = row.dependencies.filter((edge) => rules.some((rule) => matches(rule.from.path, row.from) && !matches(rule.from.pathNot ?? [], row.from) && matches(rule.to.path, edge.reference === "relative" ? edge.target : `${edge.package}${edge.reference === "package-subpath" ? "/schema" : ""}`))).map((edge) => edge.id);
    expect(forbidden, row.id).toEqual([...row.forbidden]);
    for (const edge of row.dependencies) if (row.forbiddenRules?.[edge.id]) expect(rules.filter((rule) => matches(rule.from.path, row.from) && !matches(rule.from.pathNot ?? [], row.from) && matches(rule.to.path, edge.reference === "relative" ? edge.target : `${edge.package}${edge.reference === "package-subpath" ? "/schema" : ""}`)).map((rule) => rule.name).sort(), row.id).toEqual([...row.forbiddenRules[edge.id]!].sort());
  }
});

test("dependency-cruiser independently parses and resolves every neutral fixture verdict", async () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const cwd = realpathSync(mkdtempSync(join(output, "dependency-direction-")));
  try {
    const cases = fixture.cases.map((row, index) => {
      const from = row.from.includes("/") ? `${dirname(row.from)}/case-${index}/${row.from.split("/").at(-1)}` : `case-${index}.ts`;
      expect(rules.filter((rule) => matches(rule.from.path, from) && !matches(rule.from.pathNot ?? [], from)).map((rule) => rule.name), row.id).toEqual(rules.filter((rule) => matches(rule.from.path, row.from) && !matches(rule.from.pathNot ?? [], row.from)).map((rule) => rule.name));
      const inputs = row.dependencies.map((edge) => ({ edge, ...render(edge, from) }));
      write(cwd, from, [...(row.comments ?? []).map((comment) => `// ${comment}`), ...inputs.map((input) => input.source), "export {};"].join("\n"));
      write(cwd, `${dirname(from)}/package.json`, JSON.stringify({ type: "module", dependencies: Object.fromEntries(row.dependencies.filter((edge) => edge.package).map((edge) => [edge.package, "*"])) }));
      for (const { edge } of inputs) {
        const target = "export interface Value {}\nexport const value = 1;\n";
        write(cwd, edge.target, target);
        if (edge.reference === "package" || edge.reference === "package-subpath") {
          const packageRoot = edge.installation ?? `${dirname(from)}/node_modules/${edge.package}`;
          write(cwd, `${packageRoot}/package.json`, JSON.stringify({ name: edge.package, type: "module", exports: { ".": "./entry.ts", "./schema": "./schema.ts" } }));
          write(cwd, `${packageRoot}/entry.ts`, target);
          write(cwd, `${packageRoot}/schema.ts`, target);
          if (edge.installation) {
            const link = join(cwd, `${dirname(from)}/node_modules/${edge.package}`);
            mkdirSync(dirname(link), { recursive: true });
            symlinkSync(join(cwd, packageRoot), link, process.platform === "win32" ? "junction" : "dir");
          }
        }
      }
      return { row, from, inputs };
    });
    const unresolved = (row: Case): boolean => row.dependencies.some((edge) => edge.reference === "unresolved-package" && edge.package?.startsWith("@semio-tech/"));
    for (const refused of [false, true]) {
      const group = cases.filter(({ row }) => unresolved(row) === refused);
      if (!group.length) continue;
      const oracle = await cruise(group.map(({ from }) => from), { baseDir: cwd, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", tsPreCompilationDeps: true, combinedDependencies: true, doNotFollow: { path: "node_modules" } }, { exportsFields: ["exports"], conditionNames: ["import", "require", "node", "default"], modules: ["node_modules"], restrictions: [cwd], bustTheCache: true });
      expect(oracle.exitCode).toBe(0);
      const graph = (typeof oracle.output === "string" ? JSON.parse(oracle.output) : oracle.output) as Graph;
      const scope = { ...fixture.graphScope, workspaceRoots: ["🧰️framework", "🌎️hub", "✏️s", "♻️mit-bestand", "🏢️semio-tech"], expectedSources: group.map(({ from }) => from) };
      if (refused) expect(() => dependencyDirectionEdges(graph, rules, scope)).toThrow();
      else expect(dependencyDirectionEdges(graph, rules, scope)).toHaveLength(group.reduce((count, { row }) => count + row.forbidden.reduce((count, id) => count + (row.forbiddenRules?.[id]?.length ?? 1), 0), 0));
      for (const { row, from, inputs } of group) {
      const module = graph.modules.find((module) => module.source === from);
      expect(module, row.id).toBeDefined();
      expect(module!.dependencies, row.id).toHaveLength(inputs.length);
      const actual = inputs.filter(({ specifier, edge }) => {
        const dependency = module!.dependencies.find((dependency) => dependency.module === specifier);
        expect(dependency, row.id).toBeDefined();
        if (edge.installation) expect(dependency!.resolved, row.id).toBe(`${edge.installation}/${edge.reference === "package-subpath" ? "schema" : "entry"}.ts`);
        expect(dependency!.couldNotResolve === true, row.id).toBe(edge.reference === "unresolved-package");
        const verdicts = graph.summary.violations.filter((violation) => violation.from === from && violation.to === dependency!.resolved && rules.some((rule) => violation.rule.name === rule.name));
        if (row.forbiddenRules?.[edge.id]) expect(verdicts.map((violation) => violation.rule.name).sort(), row.id).toEqual([...row.forbiddenRules[edge.id]!].sort());
        return verdicts.length > 0;
      }).map(({ edge }) => edge.id);
      expect(actual, row.id).toEqual([...row.forbidden]);
    }
    }
  } finally { rmSync(cwd, { recursive: true, force: true }); }
}, 45_000);


test("resolver graph validation rejects omissions, corrupt totals and contradictory verdicts", () => {
  const from = "🧰️framework/🔨️modules/🧵️job/🟦️.ts", peer = "🧰️framework/🔨️modules/🧬️schema/🟦️.ts", implementation = "🌎️hub/🟦️.ts";
  for (const row of fixture.graphCases) {
    const graph: any = { modules: [{ source: from, dependencies: [{ module: "./peer", resolved: peer }] }, { source: peer, dependencies: [] }], summary: { totalCruised: 2, totalDependenciesCruised: 1, error: 0, violations: [], ruleSetUsed: { forbidden: rules } } };
    if (row.id === "complete-forbidden" || row.id === "unreported-forbidden-edge") {
      graph.modules[0].dependencies[0].resolved = implementation;
      graph.modules.push({ source: implementation, dependencies: [] });
      graph.summary.totalCruised++;
    }
    if (row.id === "complete-forbidden" || row.id === "phantom-violation") { graph.summary.violations = [{ from, to: implementation, rule: { name: rules[0]!.name } }]; graph.summary.error = 1; }
    if (row.id === "missing-modules") delete graph.modules;
    if (row.id === "empty-modules") { graph.modules = []; graph.summary.totalCruised = 0; }
    if (row.id === "wrong-module-total") graph.summary.totalCruised++;
    if (row.id === "wrong-dependency-total") graph.summary.totalDependenciesCruised++;
    if (row.id === "missing-resolved-path") delete graph.modules[0].dependencies[0].resolved;
    if (row.id === "missing-import-specifier") delete graph.modules[0].dependencies[0].module;
    if (row.id === "unknown-policy") graph.summary.ruleSetUsed.forbidden = [{ name: "other" }];
    if (row.id === "duplicate-source") graph.modules[1].source = from;
    if (row.id === "complete-two-rules") {
      const io = "🧰️framework/🔨️modules/🚪️io/🟦️.ts", ui = "🧰️framework/🔨️modules/🖱️ui/🟦️.ts";
      graph.modules[0] = { source: io, dependencies: [{ module: "react", resolved: ui }] };
      graph.modules[1] = { source: from, dependencies: [{ module: "hub", resolved: implementation }] };
      graph.modules.push({ source: peer, dependencies: [] }, { source: ui, dependencies: [] }, { source: implementation, dependencies: [] });
      graph.summary.totalCruised = graph.modules.length;
      graph.summary.totalDependenciesCruised = 2;
      graph.summary.error = 2;
      graph.summary.violations = [{ from: io, to: ui, rule: { name: "io-renderer-independent" } }, { from, to: implementation, rule: { name: "framework-no-implementation" } }];
    }
    let scope = fixture.graphScope;
    if (row.id === "missing-followed-local-source" || row.id === "fake-nonfollowed-local-source") {
      graph.modules.pop(); graph.summary.totalCruised--;
      scope = { ...scope, expectedSources: [from] };
      if (row.id === "fake-nonfollowed-local-source") Object.assign(graph.modules[0].dependencies[0], { followable: false, matchesDoNotFollow: true });
    }
    if (row.id === "missing-disconnected-root") scope = { ...scope, expectedSources: [...scope.expectedSources, "🧰️framework/orphan.ts"] };
    if (row.id === "unresolved-local-source") {
      Object.assign(graph.modules[0].dependencies[0], { couldNotResolve: true });
      graph.modules.pop(); graph.summary.totalCruised--;
      scope = { ...scope, expectedSources: [from] };
    }
    if (row.id === "missing-source-inventory") scope = { ...scope, expectedSources: [] };
    if (row.id === "missing-scope-metadata") scope = undefined as unknown as DependencyDirectionGraphScope;
    if (row.id === "invalid-scope-pattern") scope = { ...scope, excludedPaths: ["["] };
    const terminal: Record<string, { module: string; resolved: string; couldNotResolve?: boolean; coreModule?: boolean; followable?: boolean }> = {
      "terminal-vendor": { module: "vendor", resolved: "node_modules/vendor/entry.ts", followable: false },
      "terminal-builtin": { module: "node:fs", resolved: "fs", coreModule: true, followable: false },
      "terminal-unresolved-relative": { module: "./absent.json", resolved: "./absent.json", couldNotResolve: true, followable: false },
      "terminal-unresolved-package": { module: "absent-package", resolved: "absent-package", couldNotResolve: true, followable: false },
      "terminal-json": { module: "./schema.json", resolved: "🧰️framework/schema.json", followable: false },
      "terminal-policy-exclusion": { module: "./generated.ts", resolved: "🧰️framework/🗑️generated/leaf.ts" },
      "terminal-policy-nonfollow": { module: "./nonfollow.ts", resolved: "🧰️framework/nonfollowed/leaf.ts" },
      "unresolved-workspace-alias": { module: "@neutral/framework", resolved: "@neutral/framework", couldNotResolve: true, followable: false },
      "unresolved-workspace-subpath": { module: "@neutral/framework/schema", resolved: "@neutral/framework/schema", couldNotResolve: true, followable: false },
    };
    if (terminal[row.id]) graph.modules[0].dependencies[0] = terminal[row.id];
    if (row.id === "multiple-unresolved-local-sources") {
      graph.modules[0].dependencies = [{ module: "./missing-first.ts", resolved: "./missing-first.ts", couldNotResolve: true }, { module: "./missing-second.ts", resolved: "./missing-second.ts", couldNotResolve: true }];
      graph.summary.totalDependenciesCruised = 2;
      for (const target of ["missing-first.ts", "missing-second.ts"]) expect(() => dependencyDirectionEdges(graph, rules, scope), target).toThrow(target);
    }
    if (row.id === "identical-leaf-records") { graph.modules.push({ source: peer, dependencies: [] }); graph.summary.totalCruised++; }
    if (row.accept) expect(() => dependencyDirectionEdges(graph, rules, scope), row.id).not.toThrow();
    else expect(() => dependencyDirectionEdges(graph, rules, scope), row.id).toThrow();
  }
});

test("dependency-cruiser unresolved local terminals fail and authored aliases retain ownership", async () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-resolution-")));
  try {
    const entries = fixture.resolutionCases.map((row, index) => { const entry = `${dirname(row.from)}/resolution-${index}/${row.from.split("/").at(-1)}`; write(root, entry, `import ${JSON.stringify(row.specifier)}; export {};`); return { row, entry }; });
    const result = await cruise(entries.map(({ entry }) => entry), { baseDir: root, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", combinedDependencies: true }, { modules: [join(root, "node_modules")], restrictions: [root], bustTheCache: true });
    const report = typeof result.output === "string" ? JSON.parse(result.output) : result.output;
    expect(result.exitCode).toBe(0);
    expect(report.modules.map((module: { source: string }) => module.source).sort()).toEqual([...entries.map(({ entry }) => entry), ...new Set(entries.map(({ row }) => row.specifier))].sort());
    expect(report.summary.totalCruised).toBe(report.modules.length);
    expect(report.summary.totalDependenciesCruised).toBe(entries.length);
    for (const { row, entry } of entries) {
      const module = report.modules.find((module: { source: string }) => module.source === entry);
      expect(module.dependencies, row.id).toHaveLength(1);
      expect(module.dependencies[0].couldNotResolve, row.id).toBe(true);
      const violations = report.summary.violations.filter((violation: { from: string }) => violation.from === entry);
      const terminal = report.modules.find((leaf: { source: string }) => leaf.source === module.dependencies[0].resolved);
      expect(terminal.dependencies, row.id).toEqual([]);
      const graph = { ...report, modules: [module, terminal], summary: { ...report.summary, totalCruised: 2, totalDependenciesCruised: 1, violations, error: violations.length } };
      const scope = { ...fixture.graphScope, workspacePackages: row.authored ? [{ name: row.specifier.startsWith("@") ? row.specifier.split("/").slice(0, 2).join("/") : row.specifier.split("/")[0]!, owner: row.owner, exports: [".", "./schema"] }] : [], expectedSources: [entry] };
      expect(violations.length, row.id).toBe(row.forbidden);
      if (row.accept) expect(dependencyDirectionEdges(graph, rules, scope), row.id).toHaveLength(row.forbidden);
      else expect(() => dependencyDirectionEdges(graph, rules, scope), row.id).toThrow();
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);


test("independent source inventory and dependency-cruiser reject self-consistent root and target omissions", async () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-completeness-")));
  try {
    for (const [index, row] of fixture.inventoryCases.entries()) {
      const cwd = join(root, String(index));
      for (const path of row.files) write(cwd, path, path.endsWith(".json") ? "{}" : "export {};\n");
      for (const link of row.links) {
        mkdirSync(dirname(join(cwd, link.path)), { recursive: true });
        symlinkSync(join(cwd, link.target), join(cwd, link.path), process.platform === "win32" ? "junction" : "dir");
      }
      if (!row.accept) {
        expect(() => dependencyDirectionSourceInventory(cwd, row.roots, { ...fixture.graphScope, expectedSources: row.expectedSources }), row.id).toThrow();
        await expect(cruise([...row.roots], { baseDir: cwd, outputType: "json", exclude: { path: [...fixture.graphScope.excludedPaths] } }, { bustTheCache: true })).rejects.toThrow();
        continue;
      }
      const entry = row.expectedSources[0]!, peer = row.expectedSources[1]!;
      const target = relative(dirname(entry), peer).replaceAll("\\", "/");
      write(cwd, entry, `import "./${target}"; import "node:fs"; import "./schema.json"; export {};`);
      const scope = { ...fixture.graphScope, expectedSources: row.expectedSources };
      expect(dependencyDirectionSourceInventory(cwd, row.roots, scope), row.id).toEqual([...row.expectedSources]);
      const result = await cruise([...row.roots], { baseDir: cwd, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", tsPreCompilationDeps: true, combinedDependencies: true, exclude: { path: [...scope.excludedPaths] }, doNotFollow: { path: scope.nonFollowedPaths.join("|") } }, { bustTheCache: true });
      const graph = typeof result.output === "string" ? JSON.parse(result.output) : result.output;
      expect(result.exitCode).toBe(0);
      expect(dependencyDirectionEdges(graph, rules, scope)).toEqual([]);
      expect(graph.modules.map((module: { source: string }) => module.source).filter((source: string) => row.expectedSources.includes(source)).sort()).toEqual([...row.expectedSources]);
      for (const missing of row.expectedSources.slice(1)) {
        const incomplete = structuredClone(graph);
        incomplete.modules = incomplete.modules.filter((module: { source: string }) => module.source !== missing);
        incomplete.summary.totalCruised = incomplete.modules.length;
        incomplete.summary.totalDependenciesCruised = incomplete.modules.reduce((count: number, module: { dependencies: unknown[] }) => count + module.dependencies.length, 0);
        expect(() => dependencyDirectionEdges(incomplete, rules, scope), `${row.id}: missing ${missing}`).toThrow();
        if (missing === peer) expect(() => dependencyDirectionEdges(incomplete, rules, { ...scope, expectedSources: [entry] }), "followed target closure independently of inventory").toThrow();
      }
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);


test("portable removability cases agree with actual boundary policy loading", async () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-removability-")));
  const boundary = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs";
  const owner = "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript";
  try {
    const oracleRows: { entry: string; policy: { forbidden: readonly Rule[] } }[] = [];
    for (const [index, row] of fixture.removabilityCases.entries()) {
      const cwd = join(root, String(index));
      write(cwd, boundary, readFileSync(join(repo, boundary), "utf8"));
      write(cwd, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", readFileSync(join(library, "🔣️taxonomy.json"), "utf8"));
      writePolicyAuthority(cwd);
      const members = row.members ?? [owner, "✏️s/🔌️plugins/🧪️example/🗿️artifacts/🧪️example/📦️packages/🟦️typescript"];
      write(cwd, "package.json", JSON.stringify({ workspaces: members, semio: { workspace: { schemaVersion: 1, members, owners: [] } } }));
      for (const directory of row.directories) mkdirSync(join(cwd, directory), { recursive: true });
      for (const [directory, manifest] of Object.entries(row.manifests)) write(cwd, `${directory}/package.json`, typeof manifest === "string" ? manifest : JSON.stringify(manifest));
      const load = (): { forbidden: readonly Rule[] } => createRequire(import.meta.url)(join(cwd, boundary));
      if (!row.accept) {
        expect(() => dependencyDirectionWorkspacePackages(cwd, []), row.id).toThrow();
        expect(load, row.id).toThrow();
        continue;
      }
      expect(dependencyDirectionWorkspacePackages(cwd, []).map((pkg) => pkg.name), row.id).toEqual([...row.expectedPackages]);
      const policy = load();
      expect(policy.forbidden.filter((rule) => rule.name.startsWith("plugin-no-extension-or-artifact-")).map((rule) => rule.name.slice("plugin-no-extension-or-artifact-".length)), row.id).toEqual([...row.expectedPlugins]);
      const neutralRule = policy.forbidden.find((rule) => rule.name === "framework-no-implementation")!;
      expect(neutralRule.severity, row.id).toBe("error");
      write(cwd, "🧰️framework/🟦️.ts", "export {};\n");
      oracleRows.push({ entry: `${index}/🧰️framework/🟦️.ts`, policy });
      if (row.nextManifests) {
        for (const [directory, manifest] of Object.entries(row.nextManifests)) write(cwd, `${directory}/package.json`, JSON.stringify(manifest));
        const expected = glob.sync(members.map((pattern) => `${pattern}/package.json`), { cwd, onlyFiles: true, followSymbolicLinks: false }).map((path) => JSON.parse(readFileSync(join(cwd, path), "utf8")).name).sort();
        expect(expected, row.id).toEqual([...row.nextExpectedPackages!].sort());
        expect(dependencyDirectionWorkspacePackages(cwd, []).map((pkg) => pkg.name).sort(), row.id).toEqual(expected);
      }
    }
    const oracle = await cruise(oracleRows.map(({ entry }) => entry), { baseDir: root, validate: true, ruleSet: { forbidden: oracleRows.flatMap(({ policy }, index) => policy.forbidden.map((rule) => ({ ...rule, name: `case-${index}-${rule.name}`, severity: rule.severity as "error" | "warn" }))) }, outputType: "json" }, { modules: [join(root, "node_modules")], restrictions: [root], bustTheCache: true });
    const graph = typeof oracle.output === "string" ? JSON.parse(oracle.output) : oracle.output;
    expect(oracle.exitCode).toBe(0);
    expect(graph.modules.map((module: { source: string }) => module.source).sort()).toEqual(oracleRows.map(({ entry }) => entry).sort());
    expect(graph.summary.totalDependenciesCruised).toBe(0);
    expect(graph.summary.violations).toEqual([]);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

/** 🏷️ Authored public subpaths resolve independently while private and pseudo-package names fail. */
test("authored owner exports agree with independent package resolution", async () => {
  const policy = config;
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-public-exports-")));
  try {
    const entries = fixture.publicExportCases.map((row, index) => {
      const from = `♻️mit-bestand/public-${index}/consumer.ts`;
      const manifest = JSON.parse(readFileSync(join(repo, row.owner, "package.json"), "utf8"));
      const owner = `${dirname(from)}/node_modules/${manifest.name}`;
      write(root, `${owner}/package.json`, JSON.stringify(manifest));
      const targets = (entry: unknown): string[] => typeof entry === "string" ? [entry] : entry && typeof entry === "object" ? Object.values(entry).flatMap(targets) : [];
      for (const target of targets(manifest.exports)) write(root, `${owner}/${target}`, "export const value = 1;\n");
      write(root, from, `import { value } from ${JSON.stringify(row.specifier)}; export const used = value;`);
      return { row, from, manifest, owner };
    });
    for (const conditions of new Map(entries.map(({ row }) => [JSON.stringify(row.conditions), row.conditions])).values()) {
      const group = entries.filter(({ row }) => JSON.stringify(row.conditions) === JSON.stringify(conditions));
      const result = await cruise(group.map(({ from }) => from), { baseDir: root, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", tsPreCompilationDeps: true, combinedDependencies: true, doNotFollow: { path: "node_modules" } }, { exportsFields: ["exports"], conditionNames: [...conditions], modules: ["node_modules"], restrictions: [root], bustTheCache: true });
      const report = typeof result.output === "string" ? JSON.parse(result.output) : result.output;
      expect(result.exitCode).toBe(0);
      expect(report.summary.totalDependenciesCruised).toBe(group.length);
      expect(report.summary.totalCruised).toBe(report.modules.length);
      for (const { row, from, manifest, owner } of group) {
        const module = report.modules.find((module: { source: string }) => module.source === from);
        expect(module.dependencies, row.id).toHaveLength(1);
        const dependency = module.dependencies[0];
        expect(dependency.couldNotResolve !== true, row.id).toBe(row.accept);
        if (row.target) {
          expect(policy.options.enhancedResolveOptions.conditionNames, row.id).toEqual([...row.conditions]);
          expect(dependency.resolved, row.id).toBe(`${owner}/${row.target.slice(2)}`);
        }
        const terminal = report.modules.find((leaf: { source: string }) => leaf.source === dependency.resolved);
        expect(terminal.dependencies, row.id).toEqual([]);
        const violations = report.summary.violations.filter((violation: { from: string }) => violation.from === from);
        const graph = { ...report, modules: [module, terminal], summary: { ...report.summary, totalCruised: 2, totalDependenciesCruised: 1, violations, error: violations.length } };
        const scope = { ...fixture.graphScope, workspaceRoots: [...fixture.graphScope.workspaceRoots, "♻️mit-bestand", "✏️s"], workspacePackages: [{ name: manifest.name, owner: row.owner, exports: Object.keys(manifest.exports) }], expectedSources: [from] };
        if (row.accept) expect(dependencyDirectionEdges(graph, rules, scope), row.id).toEqual([]);
        else expect(() => dependencyDirectionEdges(graph, rules, scope), row.id).toThrow();
      }
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

/** 🔬️ An independent compiler checks authored wire declarations against portable JSON values. */
test("authored schema type declarations accept the portable wire values", () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-wire-types-")));
  try {
    for (const [index, row] of fixture.typeCases.entries()) {
      const entry = join(root, `${index}.ts`);
      writeFileSync(entry, `import type { ${row.name} } from ${JSON.stringify(join(repo, row.source))}; const value: ${row.name} = ${JSON.stringify(row.value)}; void value;`);
      const program = ts.createProgram([entry], { noEmit: true, strict: true, allowImportingTsExtensions: true, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, target: ts.ScriptTarget.ES2022, types: [] });
      const diagnostics = ts.getPreEmitDiagnostics(program);
      expect(diagnostics.length === 0, `${row.id}: ${diagnostics.map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, " ")).join("; ")}`).toBe(row.accept);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

/** 🧪️ Private owner infrastructure routes agree with the independent TypeScript registration parser. */
test("private owner infrastructure exposes truthful registered commands", () => {
  for (const row of fixture.infrastructureCases) {
    const manifest = JSON.parse(readFileSync(join(repo, row.owner, "package.json"), "utf8"));
    const source = ts.createSourceFile("📜️script.ts", readFileSync(join(repo, row.owner, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const registered = new Set<string>();
    const visit = (node: ts.Node): void => {
      if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) registered.add(node.arguments[0].text);
      ts.forEachChild(node, visit);
    };
    visit(source);
    expect(manifest.name, row.name).toBe(row.name);
    expect(manifest.private, row.name).toBe(true);
    for (const field of ["exports", "main", "module", "types"]) expect(manifest[field], `${row.name}: ${field}`).toBeUndefined();
    expect(existsSync(join(repo, row.owner, "🟦️.ts")), row.name).toBe(false);
    expect(manifest.scripts, row.name).toEqual(Object.fromEntries(row.commands.map((command) => [command, `bun nx run ${row.name}:${command}`])));
    for (const command of row.commands) expect(registered.has(command), `${row.name}: ${command}`).toBe(true);
  }
});

```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️dependency-direction/🔣️.json

SHA-256 `2a7fa6385291bb2364e7c137aa187eb594dce536baff2c9220e0dc3f81a39df8`; 51801 bytes.

```
{
  "schemaVersion": 1,
  "cases": [
    {
      "id": "framework-peer",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🧬️schema/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "product-framework-peer",
      "from": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🧬️schema/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "implementation-consumes-framework",
      "from": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🧬️schema/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "implementation-composition",
      "from": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/🔌️plugins/📐️cad/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "framework-relative-hub",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-dynamic-hub",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
          "reference": "relative",
          "syntax": "dynamic"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-type-hub",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
          "reference": "relative",
          "syntax": "type"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-export-hub",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
          "reference": "relative",
          "syntax": "export"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-require-hub",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
          "reference": "relative",
          "syntax": "require"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-package-hub",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
          "reference": "package",
          "syntax": "static",
          "package": "os-hub-ts"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-package-subpath-hub",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
          "reference": "package-subpath",
          "syntax": "static",
          "package": "os-hub-ts"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-unresolved-package-hub",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
          "reference": "unresolved-package",
          "syntax": "static",
          "package": "os-hub-ts"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-unresolved-package-plugin",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/🔌️plugins/📐️cad/🟦️.ts",
          "reference": "unresolved-package",
          "syntax": "static",
          "package": "@semio-tech/cad-cad-rs"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-package-peer",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🧬️schema/🟦️.ts",
          "reference": "package",
          "syntax": "static",
          "package": "@semio-tech/framework"
        }
      ],
      "forbidden": []
    },
    {
      "id": "framework-test-hub",
      "from": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-bootstrap-hub",
      "from": "🧰️framework/🔨️modules/🧵️job/📜️script.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🌎️hub/💡️inference/🧬️schema/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-♻️mit-bestand",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "♻️mit-bestand/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-🏢️semio-tech",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🏢️semio-tech/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-👴️leutwiler",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "👴️leutwiler/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "framework-🎓️teaching",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🎓️teaching/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "comments-and-false-generated-banner",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "comments": [
        "GENERATED FILE — do not edit",
        "import \"os-hub-ts\";",
        "import \"🌎️hub/💡️inference/🧬️schema/🟦️.ts\";",
        "import \"✏️s/🔌️plugins/📐️cad/🟦️.ts\";"
      ],
      "dependencies": [],
      "forbidden": []
    },
    {
      "id": "same-count-different-forbidden-target",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/🔌️plugins/📐️cad/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "io-relative-ui",
      "from": "🧰️framework/🔨️modules/🚪️io/🔬️probe/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "io-dynamic-renderer",
      "from": "🧰️framework/🔨️modules/🚪️io/🔬️probe/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🟦️.ts",
          "reference": "relative",
          "syntax": "dynamic"
        }
      ],
      "forbidden": [
        "edge"
      ],
      "forbiddenRules": {
        "edge": [
          "io-renderer-independent",
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "io-type-only-ui-alias",
      "from": "🧰️framework/🔨️modules/🚪️io/🔬️probe/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx",
          "reference": "package",
          "syntax": "type",
          "package": "@semio-tech/ui-react"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "io-ui-alias-subpath",
      "from": "🧰️framework/🔨️modules/🚪️io/🔬️probe/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx",
          "reference": "package-subpath",
          "syntax": "static",
          "package": "@semio-tech/ui-react"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "io-react-vendor",
      "from": "🧰️framework/🔨️modules/🚪️io/🔬️probe/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx",
          "reference": "package",
          "syntax": "static",
          "package": "react"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "io-three-vendor-dynamic",
      "from": "🧰️framework/🔨️modules/🚪️io/🔬️probe/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx",
          "reference": "package",
          "syntax": "dynamic",
          "package": "three"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "io-fiber-vendor-unresolved",
      "from": "🧰️framework/🔨️modules/🚪️io/🔬️probe/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx",
          "reference": "unresolved-package",
          "syntax": "type",
          "package": "@react-three/fiber"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "renderer-consumes-io",
      "from": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🚪️io/🔬️probe/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "io-consumes-neutral-schema",
      "from": "🧰️framework/🔨️modules/🚪️io/🔬️probe/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🧰️framework/🔨️modules/🧬️schema/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "io-comment-ui-reference",
      "from": "🧰️framework/🔨️modules/🚪️io/🔬️probe/🟦️.ts",
      "comments": [
        "import \"react\";",
        "import \"@semio-tech/ui-react\";"
      ],
      "dependencies": [],
      "forbidden": []
    },
    {
      "id": "repo-source-implementation",
      "from": "📜️script.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript/🟦️.ts",
          "syntax": "type",
          "reference": "relative"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "s-module-plugin",
      "from": "✏️s/🔨️modules/🧪️neutral/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/🔌️plugins/📐️cad/🧪️tests/🔬️boundary/🟦️.ts",
          "syntax": "dynamic",
          "reference": "relative"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "s-module-plugin-subpath",
      "from": "✏️s/🔨️modules/🧪️neutral/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/🔌️plugins/📐️cad/🧪️tests/🔬️boundary/🟦️.ts",
          "syntax": "type",
          "reference": "package-subpath",
          "package": "@semio-tech/cad-cad-rs"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "plugin-owner-artifact",
      "from": "✏️s/🔌️plugins/📐️cad/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🟦️.ts",
          "syntax": "type",
          "reference": "relative"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "plugin-owner-extension-subpath",
      "from": "✏️s/🔌️plugins/📐️cad/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🟦️.ts",
          "syntax": "export",
          "reference": "package-subpath",
          "package": "@semio-tech/cad-js-module-spatial-shape"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "artifact-consumes-plugin",
      "from": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/🔌️plugins/📐️cad/🧪️tests/🔬️boundary/🟦️.ts",
          "syntax": "static",
          "reference": "relative"
        }
      ],
      "forbidden": []
    },
    {
      "id": "extension-consumes-plugin",
      "from": "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/🔌️plugins/📐️cad/🧪️tests/🔬️boundary/🟦️.ts",
          "syntax": "static",
          "reference": "relative"
        }
      ],
      "forbidden": []
    },
    {
      "id": "installed-external-implementation-root",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🏢️semio-tech/node_modules/.bun/vendor@1.0.0/node_modules/vendor/index.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "installed-external-nested-workspace",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/node_modules/vendor/index.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "installed-external-scoped",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🌎️hub/node_modules/@external/neutral/index.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "installed-external-pnpm",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "♻️mit-bestand/node_modules/.pnpm/vendor@1/node_modules/vendor/index.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "installed-external-root-source",
      "from": "📜️script.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🏢️semio-tech/node_modules/vendor/index.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "source-node-modules-lookalike",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🏢️semio-tech/node_modules-lookalike/index.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "source-nested-implementation",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🏢️semio-tech/apps/example/index.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "installed-authored-alias",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/entry.ts",
          "reference": "package",
          "syntax": "static",
          "package": "os-hub-ts",
          "installation": "🏢️semio-tech/node_modules/.bun/os-hub-ts@workspace/node_modules/os-hub-ts"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "installed-authored-direct-path",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🏢️semio-tech/node_modules/.bun/os-hub-ts@workspace/node_modules/os-hub-ts/index.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "installed-external-package",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/entry.ts",
          "reference": "package",
          "syntax": "static",
          "package": "vendor",
          "installation": "🏢️semio-tech/node_modules/.bun/vendor@1/node_modules/vendor"
        }
      ],
      "forbidden": []
    },
    {
      "id": "installed-external-io-role",
      "from": "🧰️framework/🔨️modules/🚪️io/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "🏢️semio-tech/node_modules/.bun/react@19/node_modules/react/index.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "installed-authored-public-subpath",
      "from": "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "dependencies": [
        {
          "id": "edge",
          "target": "✏️s/entry.ts",
          "reference": "package-subpath",
          "syntax": "type",
          "package": "os-hub-ts",
          "installation": "🏢️semio-tech/node_modules/.bun/os-hub-ts@workspace/node_modules/os-hub-ts"
        }
      ],
      "forbidden": [
        "edge"
      ]
    },
    {
      "id": "general-module-product-relative-static",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "upward",
          "target": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "upward"
      ],
      "forbiddenRules": {
        "upward": [
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "general-module-product-relative-type",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "upward",
          "target": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
          "reference": "relative",
          "syntax": "type"
        }
      ],
      "forbidden": [
        "upward"
      ],
      "forbiddenRules": {
        "upward": [
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "general-module-product-relative-dynamic",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "upward",
          "target": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
          "reference": "relative",
          "syntax": "dynamic"
        }
      ],
      "forbidden": [
        "upward"
      ],
      "forbiddenRules": {
        "upward": [
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "general-module-product-relative-export",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "upward",
          "target": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
          "reference": "relative",
          "syntax": "export"
        }
      ],
      "forbidden": [
        "upward"
      ],
      "forbiddenRules": {
        "upward": [
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "general-module-product-relative-require",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "upward",
          "target": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
          "reference": "relative",
          "syntax": "require"
        }
      ],
      "forbidden": [
        "upward"
      ],
      "forbiddenRules": {
        "upward": [
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "general-module-product-permanent-tooling",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts",
      "dependencies": [
        {
          "id": "upward",
          "target": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "upward"
      ],
      "forbiddenRules": {
        "upward": [
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "general-module-product-native-oracle",
      "from": "🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "upward",
          "target": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "upward"
      ],
      "forbiddenRules": {
        "upward": [
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "general-module-product-alias",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "upward",
          "target": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
          "reference": "package",
          "syntax": "static",
          "package": "@semio-tech/repo-lib"
        }
      ],
      "forbidden": [
        "upward"
      ],
      "forbiddenRules": {
        "upward": [
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "general-module-product-subpath",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "upward",
          "target": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
          "reference": "package-subpath",
          "syntax": "type",
          "package": "@semio-tech/repo-lib"
        }
      ],
      "forbidden": [
        "upward"
      ],
      "forbiddenRules": {
        "upward": [
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "general-module-installed-authored-product",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "upward",
          "target": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
          "reference": "package",
          "syntax": "type",
          "package": "@semio-tech/repo-lib",
          "installation": "🧰️framework/🛍️products/🦑️repo/node_modules/.bun/@semio-tech+repo-lib@0.0.0/node_modules/@semio-tech/repo-lib"
        }
      ],
      "forbidden": [
        "upward"
      ],
      "forbiddenRules": {
        "upward": [
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "general-module-installed-external-product-storage",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "external",
          "target": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
          "reference": "package",
          "syntax": "static",
          "package": "neutral-installed-product-vendor",
          "installation": "🧰️framework/🛍️products/🦑️repo/node_modules/.bun/neutral-installed-product-vendor@0.0.0/node_modules/neutral-installed-product-vendor"
        }
      ],
      "forbidden": []
    },
    {
      "id": "general-module-product-storage-lookalike",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "upward",
          "target": "🧰️framework/🛍️products/🦑️repo/node_modules-lookalike/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": [
        "upward"
      ],
      "forbiddenRules": {
        "upward": [
          "framework-modules-no-products"
        ]
      }
    },
    {
      "id": "product-consumes-general-module",
      "from": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
      "dependencies": [
        {
          "id": "downward",
          "target": "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    },
    {
      "id": "general-module-consumes-general-module",
      "from": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️boundary/🟦️.ts",
      "dependencies": [
        {
          "id": "peer",
          "target": "🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts",
          "reference": "relative",
          "syntax": "static"
        }
      ],
      "forbidden": []
    }
  ],
  "graphCases": [
    {
      "id": "complete-allowed",
      "accept": true
    },
    {
      "id": "complete-forbidden",
      "accept": true
    },
    {
      "id": "missing-modules",
      "accept": false
    },
    {
      "id": "empty-modules",
      "accept": false
    },
    {
      "id": "wrong-module-total",
      "accept": false
    },
    {
      "id": "wrong-dependency-total",
      "accept": false
    },
    {
      "id": "missing-resolved-path",
      "accept": false
    },
    {
      "id": "missing-import-specifier",
      "accept": false
    },
    {
      "id": "unknown-policy",
      "accept": false
    },
    {
      "id": "unreported-forbidden-edge",
      "accept": false
    },
    {
      "id": "phantom-violation",
      "accept": false
    },
    {
      "id": "duplicate-source",
      "accept": false
    },
    {
      "id": "complete-two-rules",
      "accept": true
    },
    {
      "id": "missing-followed-local-source",
      "accept": false
    },
    {
      "id": "missing-disconnected-root",
      "accept": false
    },
    {
      "id": "unresolved-local-source",
      "accept": false
    },
    {
      "id": "fake-nonfollowed-local-source",
      "accept": false
    },
    {
      "id": "terminal-vendor",
      "accept": true
    },
    {
      "id": "terminal-builtin",
      "accept": true
    },
    {
      "id": "terminal-unresolved-package",
      "accept": true
    },
    {
      "id": "terminal-json",
      "accept": true
    },
    {
      "id": "terminal-policy-exclusion",
      "accept": true
    },
    {
      "id": "terminal-policy-nonfollow",
      "accept": true
    },
    {
      "id": "identical-leaf-records",
      "accept": true
    },
    {
      "id": "missing-source-inventory",
      "accept": false
    },
    {
      "id": "terminal-unresolved-relative",
      "accept": false
    },
    {
      "id": "unresolved-workspace-alias",
      "accept": false
    },
    {
      "id": "unresolved-workspace-subpath",
      "accept": false
    },
    {
      "id": "missing-scope-metadata",
      "accept": false
    },
    {
      "id": "invalid-scope-pattern",
      "accept": false
    },
    {
      "id": "multiple-unresolved-local-sources",
      "accept": false
    }
  ],
  "graphScope": {
    "workspaceRoots": [
      "🧰️framework",
      "🌎️hub"
    ],
    "excludedPaths": [
      "(^|/)🗑️generated(/|$)",
      "(^|/)\\.next(/|$)"
    ],
    "nonFollowedPaths": [
      "(^|/)node_modules(/|$)",
      "^🧰️framework/nonfollowed/"
    ],
    "expectedSources": [
      "🧰️framework/🔨️modules/🧵️job/🟦️.ts",
      "🧰️framework/🔨️modules/🧬️schema/🟦️.ts"
    ],
    "workspacePackages": [
      {
        "name": "@neutral/framework",
        "owner": "🧰️framework",
        "exports": [
          ".",
          "./schema"
        ]
      },
      {
        "name": "@neutral/implementation",
        "owner": "🌎️hub",
        "exports": [
          ".",
          "./schema"
        ]
      }
    ]
  },
  "inventoryCases": [
    {
      "id": "complete-supported-roots",
      "roots": [
        "🧰️framework"
      ],
      "files": [
        "🧰️framework/a.ts",
        "🧰️framework/orphan.mjs",
        "🧰️framework/nested/b.tsx",
        "🧰️framework/nested/types.d.mts",
        "🧰️framework/schema.json",
        "🧰️framework/styles.css",
        "🧰️framework/🗑️generated/excluded.ts",
        "🧰️framework/nonfollowed/leaf.ts",
        "🧰️framework/node_modules/vendor/entry.js",
        "🧰️framework/.next/types/routes.d.ts"
      ],
      "links": [
        {
          "path": "🧰️framework/🗑️generated/loop",
          "target": "🧰️framework"
        }
      ],
      "expectedSources": [
        "🧰️framework/a.ts",
        "🧰️framework/nested/b.tsx",
        "🧰️framework/nested/types.d.mts",
        "🧰️framework/orphan.mjs"
      ],
      "accept": true
    },
    {
      "id": "complete-repository-source-roots",
      "roots": [
        "📜️script.ts",
        "🧰️framework"
      ],
      "files": [
        "📜️script.ts",
        "schema.json",
        "🧰️framework/a.ts"
      ],
      "links": [],
      "expectedSources": [
        "📜️script.ts",
        "🧰️framework/a.ts"
      ],
      "accept": true
    },
    {
      "id": "nonexcluded-linked-root-refused",
      "accept": false,
      "roots": [
        "🧰️framework"
      ],
      "files": [
        "🧰️framework/a.ts"
      ],
      "links": [
        {
          "path": "🧰️framework/loop",
          "target": "🧰️framework"
        }
      ],
      "expectedSources": [
        "🧰️framework/a.ts"
      ]
    }
  ],
  "resolutionCases": [
    {
      "id": "missing-relative-source",
      "specifier": "./absent.ts",
      "authored": false,
      "accept": false,
      "owner": "",
      "forbidden": 0,
      "from": "🧰️framework/a.ts"
    },
    {
      "id": "missing-relative-data",
      "specifier": "./absent.json",
      "authored": false,
      "accept": false,
      "owner": "",
      "forbidden": 0,
      "from": "🧰️framework/a.ts"
    },
    {
      "id": "missing-workspace-alias",
      "specifier": "@neutral/framework",
      "authored": true,
      "accept": false,
      "owner": "🧰️framework",
      "forbidden": 0,
      "from": "🧰️framework/a.ts"
    },
    {
      "id": "missing-workspace-subpath",
      "specifier": "@neutral/framework/schema",
      "authored": true,
      "accept": false,
      "owner": "🧰️framework",
      "forbidden": 0,
      "from": "🧰️framework/a.ts"
    },
    {
      "id": "unknown-external-package",
      "specifier": "neutral-external",
      "authored": false,
      "accept": true,
      "owner": "",
      "forbidden": 0,
      "from": "🧰️framework/a.ts"
    },
    {
      "id": "unreported-authored-implementation-owner",
      "specifier": "@neutral/implementation/schema",
      "authored": true,
      "owner": "🌎️hub",
      "accept": false,
      "forbidden": 0,
      "from": "🧰️framework/a.ts"
    },
    {
      "id": "unknown-authored-subpath",
      "specifier": "@neutral/framework/absent",
      "authored": true,
      "owner": "🧰️framework",
      "accept": false,
      "from": "🧰️framework/a.ts",
      "forbidden": 0
    },
    {
      "id": "unresolved-upward-authored-alias",
      "specifier": "os-hub-ts/schema",
      "authored": true,
      "owner": "🌎️hub",
      "accept": false,
      "from": "🧰️framework/a.ts",
      "forbidden": 1
    },
    {
      "id": "unresolved-io-ui-authored-alias",
      "specifier": "@semio-tech/ui-react/schema",
      "authored": true,
      "owner": "🧰️framework/🔨️modules/🖱️ui",
      "accept": false,
      "from": "🧰️framework/🔨️modules/🚪️io/a.ts",
      "forbidden": 1
    },
    {
      "id": "invalid-authored-traversal-subpath",
      "specifier": "@neutral/framework/schema/../absent",
      "authored": true,
      "owner": "🧰️framework",
      "accept": false,
      "from": "🧰️framework/a.ts",
      "forbidden": 0
    },
    {
      "id": "explicit-unfollowed-generated-reference",
      "specifier": ".next/types/routes.d.ts",
      "authored": false,
      "owner": "",
      "accept": true,
      "from": "🧰️framework/a.ts",
      "forbidden": 0
    },
    {
      "id": "deleted-owned-package",
      "specifier": "@semio-tech/deleted-plugin/schema",
      "authored": false,
      "owner": "",
      "from": "🧰️framework/a.ts",
      "forbidden": 0,
      "accept": false
    }
  ],
  "removabilityCases": [
    {
      "id": "removed-application",
      "directories": [],
      "manifests": {},
      "expectedPackages": [],
      "expectedPlugins": [],
      "accept": true
    },
    {
      "id": "removed-plugin",
      "directories": [
        "✏️s/🔌️plugins"
      ],
      "manifests": {},
      "expectedPackages": [],
      "expectedPlugins": [],
      "accept": true
    },
    {
      "id": "removed-artifact",
      "directories": [
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript"
      ],
      "manifests": {
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript": {
          "name": "@semio-tech/example",
          "exports": {
            ".": "./🟦️.ts"
          }
        }
      },
      "expectedPackages": [
        "@semio-tech/example"
      ],
      "expectedPlugins": [
        "🧪️example"
      ],
      "accept": true
    },
    {
      "id": "new-owner-contribution",
      "directories": [
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript"
      ],
      "manifests": {
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript": {
          "name": "@semio-tech/example",
          "exports": {
            ".": "./🟦️.ts"
          }
        }
      },
      "expectedPackages": [
        "@semio-tech/example"
      ],
      "expectedPlugins": [
        "🧪️example"
      ],
      "accept": true
    },
    {
      "id": "present-manifest-missing",
      "directories": [
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript"
      ],
      "manifests": {},
      "expectedPackages": [],
      "expectedPlugins": [],
      "accept": false
    },
    {
      "id": "present-malformed-manifest",
      "directories": [
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript"
      ],
      "manifests": {
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript": "{"
      },
      "expectedPackages": [],
      "expectedPlugins": [],
      "accept": false
    },
    {
      "id": "present-manifest-missing-name",
      "directories": [
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript"
      ],
      "manifests": {
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript": {}
      },
      "expectedPackages": [],
      "expectedPlugins": [],
      "accept": false
    },
    {
      "id": "source-glob-new-owner-with-opaque-storage",
      "members": [
        "✏️s/**",
        "!**/node_modules/**",
        "!**/🧫️fixtures/**"
      ],
      "directories": [
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript"
      ],
      "manifests": {
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript": {
          "name": "@semio-tech/example",
          "exports": {
            ".": "./🟦️.ts"
          }
        },
        "✏️s/node_modules/vendor": "{",
        "✏️s/🧫️fixtures/unreadable": "{"
      },
      "expectedPackages": [
        "@semio-tech/example"
      ],
      "expectedPlugins": [
        "🧪️example"
      ],
      "accept": true,
      "nextManifests": {
        "✏️s/🔌️plugins/🧪️example/🗿️artifacts/🧪️example/📦️packages/🟦️typescript": {
          "name": "@semio-tech/example-artifact",
          "exports": {
            ".": "./🟦️.ts"
          }
        },
        "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript": {
          "name": "@semio-tech/example-current",
          "exports": {
            ".": "./🟦️.ts"
          }
        }
      },
      "nextExpectedPackages": [
        "@semio-tech/example-artifact",
        "@semio-tech/example-current"
      ]
    }
  ],
  "publicExportCases": [
    {
      "id": "owner-public-spec-subpath",
      "owner": "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript",
      "specifier": "@semio-tech/mit-bestand-praesentation-projektetage/spec",
      "accept": true,
      "conditions": [
        "import",
        "require",
        "node",
        "default"
      ]
    },
    {
      "id": "owner-undeclared-private-subpath",
      "owner": "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript",
      "specifier": "@semio-tech/mit-bestand-praesentation-projektetage/private",
      "accept": false,
      "conditions": [
        "import",
        "require",
        "node",
        "default"
      ]
    },
    {
      "id": "owner-pseudo-package-refused",
      "owner": "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript",
      "specifier": "@semio-tech/mit-bestand-praesentation-projektetage-spec",
      "accept": false,
      "conditions": [
        "import",
        "require",
        "node",
        "default"
      ]
    },
    {
      "id": "ui-authored-chrome-subpath",
      "owner": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript",
      "specifier": "@semio-tech/ui-react/chrome",
      "accept": true,
      "conditions": [
        "import",
        "require",
        "node",
        "default"
      ]
    },
    {
      "id": "ui-authored-i18n-subpath",
      "owner": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript",
      "specifier": "@semio-tech/ui-react/i18n",
      "accept": true,
      "conditions": [
        "import",
        "require",
        "node",
        "default"
      ]
    },
    {
      "id": "puzzle-artifact-owned-session",
      "owner": "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🟦️typescript",
      "specifier": "@semio-tech/puzzle-2d",
      "accept": true,
      "conditions": [
        "import",
        "require",
        "node",
        "default"
      ]
    },
    {
      "id": "sequence-authored-source-condition",
      "owner": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🟦️typescript",
      "specifier": "@semio-tech/sequence-sequence",
      "accept": true,
      "conditions": [
        "semio-source",
        "import",
        "require",
        "node",
        "default"
      ],
      "target": "./🟦️.ts"
    }
  ],
  "typeCases": [
    {
      "id": "jack-query-result-valid-table",
      "source": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌳️ast/🟦️.ts",
      "name": "QueryResult",
      "value": {
        "kind": "table",
        "columns": [
          "value"
        ],
        "rows": [
          [
            1,
            null,
            "value",
            true,
            {
              "nested": [
                "value"
              ]
            }
          ]
        ]
      },
      "accept": true
    },
    {
      "id": "jack-query-result-invalid-kind",
      "source": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌳️ast/🟦️.ts",
      "name": "QueryResult",
      "value": {
        "kind": "missing",
        "columns": [],
        "rows": []
      },
      "accept": false
    },
    {
      "id": "jack-query-result-invalid-row",
      "source": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌳️ast/🟦️.ts",
      "name": "QueryResult",
      "value": {
        "kind": "table",
        "columns": [],
        "rows": [
          1
        ]
      },
      "accept": false
    }
  ],
  "infrastructureCases": [
    {
      "owner": "✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript",
      "name": "@semio-tech/writer-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript",
      "name": "@semio-tech/mathematical-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🀄️wfc/📦️packages/🟦️typescript",
      "name": "@semio-tech/wfc-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript",
      "name": "@semio-tech/procedural-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript",
      "name": "@semio-tech/flow-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript",
      "name": "@semio-tech/gis-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript",
      "name": "@semio-tech/vcs-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript",
      "name": "@semio-tech/animate-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🎥️shooting/📦️packages/🟦️typescript",
      "name": "@semio-tech/shooting-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript",
      "name": "@semio-tech/fem-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript",
      "name": "@semio-tech/architect-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript",
      "name": "@semio-tech/process-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript",
      "name": "@semio-tech/lowpoly-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript",
      "name": "@semio-tech/reasoning-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🟦️typescript",
      "name": "@semio-tech/forms-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript",
      "name": "@semio-tech/layout-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript",
      "name": "@semio-tech/norm-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript",
      "name": "@semio-tech/playbook-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript",
      "name": "@semio-tech/imperative-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript",
      "name": "@semio-tech/remodel-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🔋️energy/📦️packages/🟦️typescript",
      "name": "@semio-tech/energy-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript",
      "name": "@semio-tech/trinity-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript",
      "name": "@semio-tech/dag-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript",
      "name": "@semio-tech/draw-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript",
      "name": "@semio-tech/raster-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🟦️typescript",
      "name": "@semio-tech/note-note-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript",
      "name": "@semio-tech/puzzle-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript",
      "name": "@semio-tech/block-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript",
      "name": "@semio-tech/space-js",
      "commands": [
        "test"
      ]
    },
    {
      "owner": "✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript",
      "name": "@semio-tech/sourcing-js",
      "commands": [
        "test"
      ]
    }
  ],
  "policyOwners": [
    {
      "owner": "🌎️hub/📦️packages/🟦️typescript",
      "name": "os-hub-ts"
    },
    {
      "owner": "🧰️framework/📦️packages/🟦️typescript",
      "name": "@semio-tech/framework"
    },
    {
      "owner": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript",
      "name": "@semio-tech/ui-react",
      "role": "ui"
    },
    {
      "owner": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust",
      "name": "@semio-tech/cad-cad-rs"
    },
    {
      "owner": "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript",
      "name": "@semio-tech/cad-js-module-spatial-shape"
    },
    {
      "owner": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript",
      "name": "@semio-tech/repo-lib"
    }
  ]
}

```

### 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️dependency-direction/🔣️.json

SHA-256 `6db379910eb0c363d2f555fbe7d4120bd7df231752a7131e70890ab7ce1eda3f`; 16554 bytes.

```
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "schemaVersion",
    "cases",
    "graphCases",
    "graphScope",
    "inventoryCases",
    "resolutionCases",
    "removabilityCases",
    "publicExportCases",
    "typeCases",
    "infrastructureCases",
    "policyOwners"
  ],
  "properties": {
    "schemaVersion": {
      "const": 1
    },
    "cases": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "id",
          "from",
          "dependencies",
          "forbidden"
        ],
        "properties": {
          "id": {
            "type": "string",
            "minLength": 1
          },
          "from": {
            "type": "string",
            "minLength": 1
          },
          "comments": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "forbidden": {
            "type": "array",
            "uniqueItems": true,
            "items": {
              "type": "string"
            }
          },
          "dependencies": {
            "type": "array",
            "items": {
              "type": "object",
              "additionalProperties": false,
              "required": [
                "id",
                "target",
                "reference",
                "syntax"
              ],
              "properties": {
                "id": {
                  "type": "string",
                  "minLength": 1
                },
                "target": {
                  "type": "string",
                  "minLength": 1
                },
                "reference": {
                  "enum": [
                    "relative",
                    "package",
                    "package-subpath",
                    "unresolved-package"
                  ]
                },
                "syntax": {
                  "enum": [
                    "static",
                    "type",
                    "dynamic",
                    "export",
                    "require"
                  ]
                },
                "package": {
                  "type": "string",
                  "minLength": 1
                },
                "installation": {
                  "type": "string",
                  "pattern": "(?:^|/)node_modules/.*[^/]$"
                }
              },
              "allOf": [
                {
                  "if": {
                    "properties": {
                      "reference": {
                        "enum": [
                          "package",
                          "package-subpath",
                          "unresolved-package"
                        ]
                      }
                    }
                  },
                  "then": {
                    "properties": {
                      "package": {
                        "type": "string"
                      }
                    },
                    "required": [
                      "package"
                    ]
                  }
                }
              ]
            }
          },
          "forbiddenRules": {
            "type": "object",
            "additionalProperties": {
              "type": "array",
              "minItems": 1,
              "uniqueItems": true,
              "items": {
                "type": "string",
                "minLength": 1
              }
            }
          }
        }
      }
    },
    "graphCases": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "id",
          "accept"
        ],
        "properties": {
          "id": {
            "enum": [
              "complete-allowed",
              "complete-forbidden",
              "missing-modules",
              "empty-modules",
              "wrong-module-total",
              "wrong-dependency-total",
              "missing-resolved-path",
              "missing-import-specifier",
              "unknown-policy",
              "unreported-forbidden-edge",
              "phantom-violation",
              "duplicate-source",
              "complete-two-rules",
              "missing-followed-local-source",
              "missing-disconnected-root",
              "unresolved-local-source",
              "fake-nonfollowed-local-source",
              "terminal-vendor",
              "terminal-builtin",
              "terminal-unresolved-package",
              "terminal-json",
              "terminal-policy-exclusion",
              "terminal-policy-nonfollow",
              "identical-leaf-records",
              "missing-source-inventory",
              "terminal-unresolved-relative",
              "unresolved-workspace-alias",
              "unresolved-workspace-subpath",
              "missing-scope-metadata",
              "invalid-scope-pattern",
              "multiple-unresolved-local-sources"
            ]
          },
          "accept": {
            "type": "boolean"
          }
        }
      }
    },
    "graphScope": {
      "$ref": "#/$defs/GraphScope"
    },
    "inventoryCases": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "id",
          "roots",
          "files",
          "links",
          "expectedSources",
          "accept"
        ],
        "properties": {
          "id": {
            "type": "string",
            "minLength": 1
          },
          "roots": {
            "type": "array",
            "uniqueItems": true,
            "items": {
              "type": "string",
              "minLength": 1
            }
          },
          "files": {
            "type": "array",
            "uniqueItems": true,
            "items": {
              "type": "string",
              "minLength": 1
            }
          },
          "expectedSources": {
            "type": "array",
            "uniqueItems": true,
            "items": {
              "type": "string",
              "minLength": 1
            }
          },
          "links": {
            "type": "array",
            "items": {
              "type": "object",
              "additionalProperties": false,
              "required": [
                "path",
                "target"
              ],
              "properties": {
                "path": {
                  "type": "string",
                  "minLength": 1
                },
                "target": {
                  "type": "string",
                  "minLength": 1
                }
              }
            }
          },
          "accept": {
            "type": "boolean"
          }
        }
      }
    },
    "resolutionCases": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "id",
          "specifier",
          "authored",
          "accept",
          "owner",
          "from",
          "forbidden"
        ],
        "properties": {
          "id": {
            "type": "string",
            "minLength": 1
          },
          "specifier": {
            "type": "string",
            "minLength": 1
          },
          "authored": {
            "type": "boolean"
          },
          "accept": {
            "type": "boolean"
          },
          "owner": {
            "type": "string"
          },
          "from": {
            "type": "string",
            "minLength": 1
          },
          "forbidden": {
            "type": "integer",
            "minimum": 0
          }
        }
      }
    },
    "removabilityCases": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "id",
          "directories",
          "manifests",
          "expectedPackages",
          "expectedPlugins",
          "accept"
        ],
        "properties": {
          "id": {
            "type": "string",
            "minLength": 1
          },
          "directories": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "manifests": {
            "type": "object",
            "additionalProperties": {
              "anyOf": [
                {
                  "type": "object"
                },
                {
                  "type": "string"
                }
              ]
            }
          },
          "expectedPackages": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "expectedPlugins": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "accept": {
            "type": "boolean"
          },
          "members": {
            "type": "array",
            "minItems": 1,
            "uniqueItems": true,
            "items": {
              "type": "string",
              "minLength": 1
            }
          },
          "nextManifests": {
            "type": "object",
            "additionalProperties": {
              "anyOf": [
                {
                  "type": "object"
                },
                {
                  "type": "string"
                }
              ]
            }
          },
          "nextExpectedPackages": {
            "type": "array",
            "items": {
              "type": "string"
            }
          }
        }
      }
    },
    "publicExportCases": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "id",
          "owner",
          "specifier",
          "accept",
          "conditions"
        ],
        "properties": {
          "id": {
            "type": "string",
            "minLength": 1
          },
          "owner": {
            "type": "string",
            "minLength": 1
          },
          "specifier": {
            "type": "string",
            "minLength": 1
          },
          "accept": {
            "type": "boolean"
          },
          "conditions": {
            "type": "array",
            "minItems": 1,
            "uniqueItems": true,
            "items": {
              "type": "string",
              "minLength": 1
            }
          },
          "target": {
            "type": "string",
            "minLength": 1
          }
        }
      }
    },
    "typeCases": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "id",
          "source",
          "name",
          "value",
          "accept"
        ],
        "properties": {
          "id": {
            "type": "string",
            "minLength": 1
          },
          "source": {
            "type": "string",
            "minLength": 1
          },
          "name": {
            "type": "string",
            "minLength": 1
          },
          "value": {},
          "accept": {
            "type": "boolean"
          }
        }
      }
    },
    "infrastructureCases": {
      "type": "array",
      "minItems": 1,
      "uniqueItems": true,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "owner",
          "name",
          "commands"
        ],
        "properties": {
          "owner": {
            "type": "string",
            "minLength": 1
          },
          "name": {
            "type": "string",
            "minLength": 1
          },
          "commands": {
            "type": "array",
            "minItems": 1,
            "uniqueItems": true,
            "items": {
              "type": "string",
              "minLength": 1
            }
          }
        }
      }
    },
    "policyOwners": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "owner",
          "name"
        ],
        "properties": {
          "owner": {
            "type": "string",
            "minLength": 1
          },
          "name": {
            "type": "string",
            "minLength": 1
          },
          "role": {
            "type": "string",
            "minLength": 1
          }
        }
      }
    }
  },
  "$defs": {
    "DependencyDirections": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "roles",
        "rules"
      ],
      "properties": {
        "roles": {
          "type": "object",
          "minProperties": 1,
          "additionalProperties": {
            "type": "object",
            "additionalProperties": false,
            "required": [
              "ownerPaths",
              "externalPackages"
            ],
            "properties": {
              "ownerPaths": {
                "type": "array",
                "uniqueItems": true,
                "items": {
                  "type": "string",
                  "minLength": 1
                }
              },
              "externalPackages": {
                "type": "array",
                "uniqueItems": true,
                "items": {
                  "type": "string",
                  "minLength": 1
                }
              }
            }
          }
        },
        "rules": {
          "type": "object",
          "minProperties": 1,
          "additionalProperties": {
            "type": "object",
            "additionalProperties": false,
            "required": [
              "fromRoles",
              "toRoles"
            ],
            "properties": {
              "fromRoles": {
                "type": "array",
                "uniqueItems": true,
                "items": {
                  "type": "string",
                  "minLength": 1
                },
                "minItems": 1
              },
              "toRoles": {
                "type": "array",
                "uniqueItems": true,
                "items": {
                  "type": "string",
                  "minLength": 1
                },
                "minItems": 1
              }
            }
          }
        }
      }
    },
    "GraphScope": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "workspaceRoots",
        "excludedPaths",
        "nonFollowedPaths",
        "expectedSources",
        "workspacePackages"
      ],
      "properties": {
        "workspaceRoots": {
          "type": "array",
          "uniqueItems": true,
          "items": {
            "type": "string",
            "minLength": 1
          },
          "minItems": 1
        },
        "excludedPaths": {
          "type": "array",
          "uniqueItems": true,
          "items": {
            "type": "string",
            "minLength": 1
          }
        },
        "nonFollowedPaths": {
          "type": "array",
          "uniqueItems": true,
          "items": {
            "type": "string",
            "minLength": 1
          }
        },
        "expectedSources": {
          "type": "array",
          "uniqueItems": true,
          "items": {
            "type": "string",
            "minLength": 1
          },
          "minItems": 1
        },
        "workspacePackages": {
          "type": "array",
          "uniqueItems": true,
          "items": {
            "type": "object",
            "additionalProperties": false,
            "required": [
              "name",
              "owner",
              "exports"
            ],
            "properties": {
              "name": {
                "type": "string",
                "minLength": 1
              },
              "owner": {
                "type": "string",
                "minLength": 1
              },
              "exports": {
                "type": "array",
                "minItems": 1,
                "uniqueItems": true,
                "items": {
                  "type": "string",
                  "pattern": "^\\.(?:/.*)?$"
                }
              }
            }
          }
        }
      }
    }
  }
}

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧫️fixtures/basic-remote-ops/edits.ops

SHA-256 `90fb8a4d5b4809ebb7bf0a3654d9d8243bb85c5f99f33d36d82fb20cc0e92ff5`; 56 bytes.

```
edit basic-1 started="0" actor=peer line=[]
  set-n n=5

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧫️fixtures/basic-remote-ops/🔣️fixture.dsl

SHA-256 `f128d390f5472bb98abd2076f9e16a8ac4003bdeec64de118f971c34d24c0675`; 174 bytes.

```
name=basic-remote-ops
schema=demo/v1
documentId=fixture-basic
expectedEvent=remoteMutations
inbound.0.kind=externalEdits
inbound.0.opsFile=edits.ops
expectedEditId=basic-1#0

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧫️fixtures/remote-ops-backlog/edits.ops

SHA-256 `7739f8a1c706fb4ef6d54fb887b0638696289fa23012ca04a290a4053e08cd8d`; 116 bytes.

```
edit backlog-1 started="0" actor=peer line=[]
  set-n n=2
edit backlog-2 started="0" actor=peer line=[]
  set-n n=3

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧫️fixtures/remote-ops-backlog/🔣️fixture.dsl

SHA-256 `772dc1c20285877a199d8c74c7d6e398aea80d1092dbbf8f9af636bdea5e3973`; 207 bytes.

```
name=remote-ops-backlog
schema=demo/v1
documentId=fixture-backlog
expectedEvent=remoteMutations
inbound.0.kind=externalEdits
inbound.0.opsFile=edits.ops
expectedEditId=backlog-1#0
expectedEditId=backlog-2#0

```

## Actual Registered Current Baseline

Root executed unchanged Bun/Nx @semio-tech/repo-lib:test-dependency-direction --skip-nx-cache. It selected all11 tests:10 passed/one failed with997 expectations; Bun runtime7.71s, Nx8.4s. Actual failure is the stale private-infrastructure assertion for Forms public exports. Complete terminal log/digest/status: 🗑️generated/goal-root/dependency-direction-current-baseline-1/terminal.json and terminal.log. No production enforcement proposal is mounted. This establishes the current registered failure; synthetic deletion successor and independent oracle are still pending.
