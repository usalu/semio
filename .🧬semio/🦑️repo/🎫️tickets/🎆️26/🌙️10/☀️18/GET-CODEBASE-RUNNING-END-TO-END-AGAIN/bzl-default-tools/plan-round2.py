"""Round 2 groups of build-plan.py (executed in its namespace): probe, closure, nx, manifest2."""
STAGED_NEW = os.path.join(HERE, "staged", "new")
newfiles = []


def new_file(group, path):
    content = open(os.path.join(STAGED_NEW, path), encoding="utf-8", newline="").read()
    newfiles.append({"group": group, "path": path, "sha256": sha(content)})


# ---------------------------------------------------------------- probe: the Bazel cache probe behind cache-verify
edit("probe", adapter, "const BAZEL_MODULE='module(name = \"semio_tasks\", version = \"0.0.0\")\\n';", "export const BAZEL_MODULE='module(name = \"semio_tasks\", version = \"0.0.0\")\\n';")
edit("probe", orchestration, 'import { BundleScript } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";', 'import { BundleScript, createIssuedResourceControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";')
edit("probe", orchestration, 'import probeData from "../🔣️probe.json" with { type: "json" };', 'import probeData from "../🔣️probe.json" with { type: "json" };\nimport buildPolicy from "../../../🏗️build/🔣️policy.json" with { type: "json" };')
old_probe = '''    const selected = process.env.SEMIO_CACHE_PROBE_TOOL ?? "nx";
    if (selected !== "nx") throw new Error(`No cache probe implements build tool ${selected}`);
    if (validateJsonSchemaSubset(probeSchema, probeSpecification).length) throw new Error("Closed cache probe specification required");
    return (await import("../../🚀️bootstrap/🛠️tools/🔁️probe/🟦️.ts")).nxCacheProbeV1(this.repoRoot);
'''
new_probe = '''    const selected = process.env.SEMIO_CACHE_PROBE_TOOL ?? process.env.SEMIO_BUILD_TOOL ?? buildPolicy.defaultTool;
    if (validateJsonSchemaSubset(probeSchema, probeSpecification).length) throw new Error("Closed cache probe specification required");
    if (selected === "nx") {
      await (await import("../../🚀️bootstrap/🛠️tools/🟦️.ts")).prepareNxToolingStorageV1(this.repoRoot, this.invocation, createIssuedResourceControlV1(this.invocation, "cache-verify:nx", 0));
      return (await import("../../🚀️bootstrap/🛠️tools/🔁️probe/🟦️.ts")).nxCacheProbeV1(this.repoRoot);
    }
    if (selected !== "bazel") throw new Error(`No cache probe implements build tool ${selected}`);
    const [{ withBazelSetupStorageV1 }, adapter, bootstrap, { bazelCacheProbeV1 }] = await Promise.all([import("../../../🏗️build/🔧️tools/🏗️bazel/💾️storage/🟦️.ts"), import("../../../🏗️build/🔧️tools/🏗️bazel/📜️script.ts"), import("../../🚀️bootstrap/🛠️tools/🏗️bazel/📜️script.ts"), import("../../../🏗️build/🔧️tools/🏗️bazel/🔁️probe/🟦️.ts")]);
    return withBazelSetupStorageV1(this.repoRoot, this.invocation, createIssuedResourceControlV1(this.invocation, "cache-verify:bazel", 0), async (storage, phase) => {
      const executable = await bootstrap.prepareBazel(this.repoRoot, storage.tools, phase, () => {}, false), modules = await adapter.prepareBazelModulesV1(this.repoRoot, executable, this.invocation, storage, phase);
      return bazelCacheProbeV1({ version: bootstrap.BAZEL_VERSION, executable, installBase: join(storage.paths.install, bootstrap.BAZEL_VERSION, `${process.platform}-${process.arch}`), module: adapter.BAZEL_MODULE, lock: modules.lock, repository: modules.repository });
    });
'''
edit("probe", orchestration, old_probe, new_probe)
PROBE = B + "/🔧️tools/🏗️bazel/🔁️probe"
for rel in ["/🟦️.ts", "/🏭️runner/🟦️.ts", "/🧪️tests/🟦️.ts", "/🧪️tests/🧫️fixtures/🔣️.json"]:
    new_file("probe", PROBE + rel)

# ---------------------------------------------------------------- closure: closure-scoped JavaScript install (inert until SEMIO_JS_INSTALL=closure or the policy value)
PARSERS = B + "/🚀️bootstrap/📦️parsers"
new_file("closure", PARSERS + "/🟨️.mjs")
new_file("closure", B + "/🚀️bootstrap/🧪️tests/🎯️closure/🟦️.ts")
new_file("closure", B + "/🚀️bootstrap/🧪️tests/🎯️closure/🔣️.json")
INF = B + "/🕸️graph/📥️inference"
HELPER = 'import { parserRequireV1 } from "../../../🚀️bootstrap/📦️parsers/🟨️.mjs";'
RQ_OLD, RQ_NEW = 'createRequire(import.meta.url)("typescript")', 'parserRequireV1(import.meta.url)("typescript")'
IMP = 'import { createRequire } from "node:module";'
edit("closure", INF + "/🔎️owner-discovery/🟨️.mjs", IMP, HELPER)
edit("closure", INF + "/🔎️owner-discovery/🟨️.mjs", RQ_OLD, RQ_NEW)
edit("closure", INF + "/🦀️native-manifests/🟨️.mjs", IMP, HELPER)
edit("closure", INF + "/🦀️native-manifests/🟨️.mjs", "}))(createRequire(import.meta.url));", "}))(parserRequireV1(import.meta.url));")
edit("closure", INF + "/🧩️component-targets/🟨️.mjs", IMP, HELPER)
edit("closure", INF + "/🧩️component-targets/🟨️.mjs", RQ_OLD, RQ_NEW)
edit("closure", INF + "/🧭️script-inputs/🟨️.mjs", 'import { createRequire, isBuiltin } from "node:module";', 'import { isBuiltin } from "node:module";\n' + HELPER)
edit("closure", INF + "/🧭️script-inputs/🟨️.mjs", RQ_OLD, RQ_NEW)
edit("closure", INF + "/🧱️project-assembly/🟨️.mjs", IMP, HELPER)
edit("closure", INF + "/🧱️project-assembly/🟨️.mjs", RQ_OLD, RQ_NEW)
edit("closure", INF + "/🧶️bun-lock/🟨️.mjs", IMP, HELPER)
edit("closure", INF + "/🧶️bun-lock/🟨️.mjs", RQ_OLD, RQ_NEW)
edit("closure", L + "/⚡️caching/📥️inference/🟨️.mjs", IMP, 'import { parserRequireV1 } from "../../🏗️build/🚀️bootstrap/📦️parsers/🟨️.mjs";')
edit("closure", L + "/⚡️caching/📥️inference/🟨️.mjs", "const require = createRequire(import.meta.url);", "const require = parserRequireV1(import.meta.url);")

DEPS = L + "/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage"
edit("closure", DEPS + "/🟦️.ts", 'export type JavascriptDependencyStoreSelectionV1=Readonly<{kind:"all"}>|Readonly<{kind:"scope";owner:string}>;', 'export type JavascriptDependencyStoreSelectionV1=Readonly<{kind:"all"}>|Readonly<{kind:"scope";owner:string}>|Readonly<{kind:"scopes";owners:readonly string[]}>;')
edit("closure", DEPS + "/🟦️.ts", 'requested=selection.kind==="all"?projection.plans.map(row=>row.scope):[selection.owner]', 'requested=selection.kind==="all"?projection.plans.map(row=>row.scope):selection.kind==="scopes"?[...selection.owners]:[selection.owner]')
edit("closure", DEPS + "/🟦️.ts", 'requested=selection.kind==="all"?projection.owners.scopes.map(row=>row.scope):[selection.owner]', 'requested=selection.kind==="all"?projection.owners.scopes.map(row=>row.scope):selection.kind==="scopes"?[...selection.owners]:[selection.owner]')
edit("closure", DEPS + "/🧬️schema/📥️install.json", '"owner": {"type": "string", "minLength": 1, "maxLength": 256}}}]},', '"owner": {"type": "string", "minLength": 1, "maxLength": 256}}}, {"type": "object", "additionalProperties": false, "required": ["kind", "owners"], "properties": {"kind": {"const": "scopes"}, "owners": {"type": "array", "minItems": 1, "maxItems": 512, "uniqueItems": true, "items": {"type": "string", "minLength": 1, "maxLength": 256}}}}]},')

edit("closure", B + "/🔣️policy.json", '  "remoteCache": {\n    "version": 1,\n    "environment": {', '  "javascriptInstall": "all",\n  "javascriptInstallRoots": [],\n  "remoteCache": {\n    "version": 1,\n    "environment": {')
edit("closure", B + "/🧬️schema/🔣️.json", '"defaultTool": {\n          "$ref": "#/$defs/BuildTool"\n        }\n      },\n      "additionalProperties": true', '"defaultTool": {\n          "$ref": "#/$defs/BuildTool"\n        },\n        "javascriptInstall": {\n          "enum": [\n            "all",\n            "closure"\n          ]\n        },\n        "javascriptInstallRoots": {\n          "type": "array",\n          "uniqueItems": true,\n          "items": {\n            "type": "string",\n            "minLength": 1\n          }\n        }\n      },\n      "additionalProperties": true')

BOOT = B + "/🚀️bootstrap/🟦️.ts"
edit("closure", BOOT, 'import {createHash} from "node:crypto";', 'import {createHash} from "node:crypto";\nimport {fileURLToPath} from "node:url";\nimport buildPolicy from "../🔣️policy.json" with {type:"json"};\nimport type {JavascriptDependencyStoreSelectionV1} from "../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts";')
edit("closure", BOOT, "},publish:(root:string)=>void=publishBuildBootstrapSourcesV1):Promise<void> {", "},publish:(root:string)=>void=publishBuildBootstrapSourcesV1,requestedScope?:string):Promise<void> {")
edit("closure", BOOT, "const bootstrap=dependencyInstallSchema.$defs.SourceOwnerBootstrapProtocol.const,workspace=", 'const bootstrap=dependencyInstallSchema.$defs.SourceOwnerBootstrapProtocol.const,scope=requestedScope??bootstrap.scope,base=scope==="."?root:join(root,...scope.split("/")),workspace=')
edit("closure", BOOT, 'const manifestBytes=bounded(join(root,"package.json"),8388608)', 'const manifestBytes=bounded(join(base,"package.json"),8388608)')
edit("closure", BOOT, 'if(!bounded(join(root,"package.json"),8388608).equals(manifestBytes)', 'if(!bounded(join(base,"package.json"),8388608).equals(manifestBytes)')
edit("closure", BOOT, "{version:1,scopes:custody.scopes},bootstrap.scope)", "{version:1,scopes:custody.scopes},scope)")
V = "generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,"
edit("closure", BOOT, "fast=await " + V + "bootstrap.scope)", "fast=await " + V + "scope)")
edit("closure", BOOT, "confirmed=walk&&!cached?await confirmWalk():await " + V + "bootstrap.scope)", "confirmed=walk&&!cached?await confirmWalk():await " + V + "scope)")
edit("closure", BOOT, "settled=walk&&!cached?await confirmWalk():await " + V + "bootstrap.scope)", "settled=walk&&!cached?await confirmWalk():await " + V + "scope)")
edit("closure", BOOT, 'if(!lstatSync(join(root,"node_modules")).isSymbolicLink())', 'if(!lstatSync(join(base,"node_modules")).isSymbolicLink())')
edit("closure", BOOT, 'const target=requireNativeStoragePathV1(store.root,realpathSync(join(root,"node_modules")))', 'const target=requireNativeStoragePathV1(store.root,realpathSync(join(base,"node_modules")))')
edit("closure", BOOT, 'if(binding.scope!=="."||binding.generation!==owner.receipts.find(row=>row.scope===".")?.generation', 'if(binding.scope!==scope||binding.generation!==owner.receipts.find(row=>row.scope===scope)?.generation')
edit("closure", BOOT, 'row.sourceDirectory===join(root,"node_modules")', 'row.sourceDirectory===join(base,"node_modules")')
edit("closure", BOOT, 'const require=createRequire(join(root,"package.json")),parsers=', 'const require=createRequire(join(base,"package.json")),parsers=')
adopted = 'export function adoptedCandidateV1(status:"published"|"present"|"deferred"|"stale"|"failed"):boolean{return status==="published"||status==="present";}'
closure_code = open(os.path.join(HERE, "closure-code.ts"), encoding="utf-8").read()
edit("closure", BOOT, adopted, adopted + closure_code)
edit("closure", script, 'if(buildOperationRequiresV1(command)==="dependencies"){const {ensureBuildEnvironmentV1}=await import("./🚀️bootstrap/🟦️.ts");await ensureBuildEnvironmentV1(root,invocation,phaseControl);}', 'if(buildOperationRequiresV1(command)==="dependencies"){const {ensureBuildEnvironmentV1,buildEnvironmentPlanV1}=await import("./🚀️bootstrap/🟦️.ts"),environmentPlan=buildEnvironmentPlanV1(root,selected.tool,process.env);await ensureBuildEnvironmentV1(root,invocation,phaseControl,environmentPlan.install,undefined,environmentPlan.scope);}')
edit("closure", script, 'if(untouched){console.log("[bazel] no affected tasks");return;}', 'if(untouched){console.log("[bazel] no affected tasks");return;}\n    {const {ensureBuildClosureScopesV1}=await import("./🚀️bootstrap/🟦️.ts");await ensureBuildClosureScopesV1(root,invocation,artifactPhase,()=>Object.values(adapter.planBazelTasksV1(graph,request).tasks).map(task=>task.root),process.env);}')

# ---------------------------------------------------------------- nx: Nx modules resolve from the pinned installation, never from the workspace root
T0P = T0 + "/🔁️probe"
INSTALL = ".nx/installation/package.json"
edit("nx", T0P + "/🟦️.ts", 'const sdk=openNxSdkV1(join(repositoryRoot,"package.json"));', 'const sdk=openNxSdkV1(join(repositoryRoot,"' + INSTALL + '"));')
edit("nx", T0P + "/🧪️tests/🟦️.ts", 'openNxSdkV1(join(root,"package.json")).packageDirectory()', 'openNxSdkV1(join(root,"' + INSTALL + '")).packageDirectory()')
edit("nx", T0P + "/🧪️tests/🟦️.ts", 'createRequire(join(root,"package.json")).resolve("nx/bin/nx.js")', 'createRequire(join(root,"' + INSTALL + '")).resolve("nx/bin/nx.js")')
edit("nx", T0P + "/🧪️tests/🟦️.ts", 'createRequire(join(root,"package.json")).resolve("nx/package.json")', 'createRequire(join(root,"' + INSTALL + '")).resolve("nx/package.json")')
edit("nx", T0 + "/🌡️host-capacity/🧪️tests/🟦️.ts", 'createRequire(join(root,"package.json")).resolve("nx/bin/nx")', 'createRequire(join(root,"' + INSTALL + '")).resolve("nx/bin/nx")')
edit("nx", T0 + "/🔌️sdk/🧪️tests/🟦️.ts", 'const installed=createRequire(join(root,"package.json")),port=', 'const installed=createRequire(join(root,"' + INSTALL + '")),port=')
edit("nx", T0 + "/🔌️sdk/🧪️tests/🟦️.ts", 'const base=join(root,"package.json"),installed=createRequire(base),sdk=openNxSdkV1(base);', 'const base=join(root,"' + INSTALL + '"),installed=createRequire(base),sdk=openNxSdkV1(base);')
edit("nx", M + "/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts", 'join(root, "node_modules/nx/schemas/nx-schema.json")', 'join(root, ".nx/installation/node_modules/nx/schemas/nx-schema.json")')
NXREQ = 'createRequire(join(process.cwd(), "' + INSTALL + '"))'
CATALOG = "✏️s/🧑‍💻dev/🧩️catalog/🧪️tests"
CTG = '("nx/src/tasks-runner/create-task-graph")'
edit("nx", CATALOG + "/🎯️selected-runtime/🟦️.ts", 'require' + CTG, NXREQ + CTG)
edit("nx", CATALOG + "/🧩️native-prerequisites/🟦️.ts", 'require' + CTG, NXREQ + CTG, bulk=True)
edit("nx", CATALOG + "/🧩️native-prerequisites/🟦️.ts", 'createRequire(import.meta.url)' + CTG, NXREQ.replace("process.cwd(), ", "process.cwd(),") + CTG, bulk=True)
edit("nx", CATALOG + "/🧩️native-prerequisites/🟦️.ts", 'createRequire(import.meta.url)("nx/src/executors/run-commands/run-commands.impl")', NXREQ + '("nx/src/executors/run-commands/run-commands.impl")')
edit("nx", "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔌️components/🧪️tests/🌐️production-browser-artifacts/🟦️.ts", 'join(dirname(require.resolve("nx/package.json")), require("nx/package.json").bin.nx)', '(nxRequire => join(dirname(nxRequire.resolve("nx/package.json")), nxRequire("nx/package.json").bin.nx))(' + NXREQ + ')')
edit("nx", "nx.json", '"$schema": "./node_modules/nx/schemas/nx-schema.json",', '"$schema": "./.nx/installation/node_modules/nx/schemas/nx-schema.json",')

# ---------------------------------------------------------------- manifest2: parser package, its workspace row, Nx leaves the root scope (R keeps Nx for the Nx contract tests); run `bun repo dependencies lock --all` afterwards
new_file("manifest2", PARSERS + "/package.json")
tools_row = '    "' + T0 + '",\n'
edit("manifest2", "package.json", tools_row, tools_row + '    "' + PARSERS + '",\n')
edit("manifest2", "package.json", '    "@iarna/toml": "2.2.5",\n    "@nx/devkit": "23.2.0",\n    "@nx/js": "23.2.0",\n    "@nxlv/python": "23.0.2",\n    "@playwright/test"', '    "@iarna/toml": "2.2.5",\n    "@playwright/test"')
edit("manifest2", "package.json", '    "npm-run-path": "^6.0.0",\n    "nx": "23.2.0",\n    "opentype.js"', '    "npm-run-path": "^6.0.0",\n    "opentype.js"')

# ---------------------------------------------------------------- setup follow-up: the install guard is catalogue-driven now (`requires`), so the setup law reads the catalogue instead of an array literal
STORAGE_TESTS = BZ + "/💾️storage/🧪️tests/🟦️.ts"
edit("setup", STORAGE_TESTS, 'import fixture from "./🧫️fixtures/🔣️.json";', 'import fixture from "./🧫️fixtures/🔣️.json";\nimport catalogue from "../../../../⚙️operations/🔣️catalog.json";')
edit("setup", STORAGE_TESTS, 'guard=branches.find(node=>node.thenStatement.getText().includes("ensureBuildEnvironmentV1"))!;', 'guard=branches.find(node=>node.expression.getText().includes("buildOperationRequiresV1"))!;')
old_law = ''' const skipped:string[]=[],collect=(node:ts.Node)=>{if(ts.isArrayLiteralExpression(node))skipped.push(...node.elements.filter(ts.isStringLiteral).map(element=>element.text));ts.forEachChild(node,collect);};collect(guard.expression);
 expect(skipped.sort()).toEqual([...fixture.setup.installSkipped].sort());expect(guard.expression.getText()).toMatch(/^!\\[.*\\]\\.includes\\(command\\)$/u);
'''
new_law = ''' const required=new Map(catalogue.operations.map(row=>[row.name,row.requires]));
 for(const name of fixture.setup.installSkipped)expect(required.get(name),name).toBe("none");
 expect(guard.expression.getText()).toBe('buildOperationRequiresV1(command)==="dependencies"');expect(guard.thenStatement.getText()).toContain("ensureBuildEnvironmentV1");
'''
edit("setup", STORAGE_TESTS, old_law, new_law)
