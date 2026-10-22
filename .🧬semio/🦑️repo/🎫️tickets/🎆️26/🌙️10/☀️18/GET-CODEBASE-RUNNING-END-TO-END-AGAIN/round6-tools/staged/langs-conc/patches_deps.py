# 🧰 Native dependency synchronisation: Python projects get their own environment from the toolchain plan (never a shared root .venv synced with --all-packages), Go runs under the pinned local toolchain, the .NET SDK is the pinned one.
from layout import *
from patchlib import chain

DEPS = BOOT + "/📦️dependencies/🏗️native/🟦️.ts"
DEPS_TEST = CACHING + "/🧪️tests/📦️native-dependencies/🟦️.ts"
DEPS_FIXTURE = CACHING + "/🧪️tests/🧫️fixtures/📦️native-dependencies/🔣️.json"

def deps(text):
    return chain(text, [
        ("deps-import", 'import { wasmBindgenVersion } from "../../../🦀️cargo/🕸️wasm-bindgen/🟦️.ts";\n',
         'import { wasmBindgenVersion } from "../../../🦀️cargo/🕸️wasm-bindgen/🟦️.ts";\nimport { repositoryCachePartitionRootV1 } from "../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";\nimport { NATIVE_TOOLCHAINS_V1, pythonEnvironmentOfProjectV1, toolchainMessageV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";\nimport { pinnedGoEnvironmentV1 } from "../../🛠️tools/🧰️native/🟦️.ts";\nimport { systemNativeToolPortV1 } from "../../🛠️tools/🧰️native/🔌️port/🟦️.ts";\n'),
        ("deps-python", '  if (kind === "python") await run(...render("python", "sync", { scope: "all" }));\n',
         '  if (kind === "python") {\n    const plan = pythonEnvironmentOfProjectV1(workspace, "workspace", { dependencies: repositoryCachePartitionRootV1(workspace, "dependencies"), tools: repositoryCachePartitionRootV1(workspace, "tools") }), planned = { ...boundedEnvironment, ...plan.variables };\n    await run("uv", [...plan.installArguments], false, planned);\n    await run("uv", [...plan.syncArguments], false, planned);\n  }\n'),
        ("deps-go-signature", 'runner: DependencyToolRunner = runNativeDependencyToolV1,environment:NodeJS.ProcessEnv=process.env): Promise<void> {',
         'runner: DependencyToolRunner = runNativeDependencyToolV1,environment:NodeJS.ProcessEnv=process.env,goEnvironment:(base:NodeJS.ProcessEnv)=>Promise<Readonly<Record<string,string|undefined>>>=base=>pinnedGoEnvironmentV1(base,repositoryCachePartitionRootV1(workspace,"tools"),systemNativeToolPortV1(workspace,phase.authority.network.offline),phase.leaseControl.signal)): Promise<void> {'),
        ("deps-go", '  else if (kind === "go") await run(...render("go", "sync"), false, { ...process.env, GOWORK: join(workspace, "go.work") });\n',
         '  else if (kind === "go") await run(...render("go", "sync"), false, { ...(await goEnvironment(process.env)), GOWORK: join(workspace, "go.work") });\n'),
        ("deps-dotnet", '  else if (kind === "dotnet") console.log("[deps-dotnet] Build-tool project restores completed");\n',
         '  else if (kind === "dotnet") {\n    const sdk = (await run("dotnet", ["--version"], true)).trim();\n    if (sdk !== NATIVE_TOOLCHAINS_V1.dotnet.sdk) throw new Error(toolchainMessageV1("pin", { tool: ".NET SDK", actual: sdk, expected: NATIVE_TOOLCHAINS_V1.dotnet.sdk, source: NATIVE_TOOLCHAINS_V1.dotnet.file }, environment));\n    console.log(`[deps-dotnet] Pinned .NET SDK ${sdk} is ready`);\n  }\n'),
    ])

def deps_test(text):
    return chain(text, [
        ("deps-test-python-files", '    authorCargoFixture();\n    const { cargoRepositoryPackages } = await import("../../../🗂️workspaces/🦀️cargo/🟦️.ts");\n    assert.deepEqual(cargoRepositoryPackages(root), fixture.cargo.expectedMembers);\n    delete process.env.PLAYWRIGHT_BROWSERS_PATH;\n',
         '    authorCargoFixture();\n    for (const [path, contents] of Object.entries(fixture.python.files)) writeFileSync(join(root, path), contents as string);\n    const { cargoRepositoryPackages } = await import("../../../🗂️workspaces/🦀️cargo/🟦️.ts");\n    assert.deepEqual(cargoRepositoryPackages(root), fixture.cargo.expectedMembers);\n    delete process.env.PLAYWRIGHT_BROWSERS_PATH;\n'),
        ("deps-test-dotnet-capture", 'if (capture) return command==="cargo"&&args[0]==="--version"?identity(cwd,signal):"";',
         'if (capture) return command==="cargo"&&args[0]==="--version"?identity(cwd,signal):command==="dotnet"?toolchainPolicy.dotnet.sdk:"";'),
        ("deps-test-go", '        return "";\n      });\n      assert.deepEqual(calls, row.commands.map(', '        return "";\n      }, process.env, async (base: NodeJS.ProcessEnv) => ({ ...base, ...toolchainPolicy.go.variables }));\n      assert.deepEqual(calls, row.commands.map('),
        ("deps-test-import", 'import assert from "node:assert/strict";\n',
         'import assert from "node:assert/strict";\nimport toolchainPolicy from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🔣️.json" with { type: "json" };\n'),
    ])

def deps_fixture(text):
    return chain(text, [
        ("deps-fixture-python",
         '    {\n      "kind": "python",\n      "commands": [\n        [\n          "uv",\n          "sync",\n          "--locked",\n          "--all-packages",\n          "--all-groups"\n        ]\n      ]\n    },\n',
         '    {\n      "kind": "python",\n      "commands": [\n        [\n          "uv",\n          "python",\n          "install",\n          "3.14"\n        ],\n        [\n          "uv",\n          "sync",\n          "--locked",\n          "--project",\n          ".",\n          "--group",\n          "dev",\n          "--group",\n          "test"\n        ]\n      ],\n      "environment": {\n        "UV_NO_CONFIG": "1",\n        "UV_PYTHON": "3.14",\n        "UV_PYTHON_DOWNLOADS": "manual",\n        "UV_PYTHON_PREFERENCE": "only-managed"\n      }\n    },\n'),
        ("deps-fixture-go", '      "environment": {\n        "GOWORK": "{workspace}/go.work"\n      }\n', '      "environment": {\n        "GOTOOLCHAIN": "local",\n        "GOWORK": "{workspace}/go.work"\n      }\n'),
        ("deps-fixture-files", '{\n  "entry": ', '{\n  "python": {\n    "files": {\n      "pyproject.toml": "[project]\\nname = \\"probe\\"\\nversion = \\"0.0.0\\"\\nrequires-python = \\">=3.14,<3.15\\"\\ndependencies = []\\n",\n      "uv.lock": "version = 1\\nrequires-python = \\"==3.14.*\\"\\n"\n    }\n  },\n  "entry": '),
    ])

def edits():
    return [(DEPS, deps), (DEPS_TEST, deps_test), (DEPS_FIXTURE, deps_fixture)]
