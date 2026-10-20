"""Staged pg-chain change set: anchored (old, new) rows per tracked target, plus whole new files copied from staged/."""
import os

STAGED = os.path.join(os.path.dirname(os.path.abspath(__file__)), "staged")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
I = L + "/🏗️build/🕸️graph/📥️inference"
OS = "🧰️framework/🛍️products/💻️os/🔨️modules"
BZ = L + "/🏗️build/🔧️tools/🏗️bazel/🦀️cargo"
WGPU = OS + "/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"

EDITS = {}
NEW = {
    "component-cache.ts": I + "/🧪️tests/🧩️component-cache/🟦️.ts",
    "component-cache.json": I + "/🧪️tests/🧩️component-cache/🔣️.json",
}


def edit(path, *rows):
    EDITS.setdefault(path, []).extend(rows)


edit(I + "/🧩️component-targets/🟨️.mjs",
    (r'''import { printArtifactNxOutputV1 } from "../../../../../../../📓️print/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
''', r'''import { printArtifactNxOutputV1 } from "../../../../../../../📓️print/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import { storeReceiptOutputV1 } from "../../../🧾️store-receipt/🟨️.mjs";
'''),
    (r'''let SCRIPT_BASENAME, relativeScriptInputs, executionTargetV1, DEFAULT_EXECUTOR,''', r'''let SCRIPT_BASENAME, relativeScriptInputs, executionTargetV1, withCacheDecision, DEFAULT_EXECUTOR,'''),
    (r'''  ({ executionTargetV1 } = registry.targetPolicy);''', r'''  ({ executionTargetV1, withCacheDecision } = registry.targetPolicy);'''),
    (r'''/** 🧩️ The one on-demand target of every plugin and extension crate: `component` builds exactly that component through the build driver (its Cargo pass
 * restricted to the one package, then describe, receipt, transpile, `wasm-opt`, probe and module) for the tests and tools that load it. It is uncached
 * (Cargo freshness and the stage cache are the authorities) and never part of a release graph, which compiles every component inside its one driver task.
 * @see ../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts */''', r'''/** 🧩️ The one target of every plugin and extension crate that builds it: `component` runs exactly that component through the build driver (its Cargo pass
 * restricted to the one package, then describe, receipt, transpile, `wasm-opt`, probe and module) for the playground lanes, tests and tools that load it.
 * It replays from the task cache: the crate's exact Cargo closure, the committed descriptor pair, the Rust toolchain, the pinned Binaryen manifest, the jco
 * and preview2 shim packages and the driver's own import closure are its inputs, and the receipt of the module and model directories it published into the
 * products store is its one restorable output (a hit is verified against the store). Cargo compiles one component at a time, so the target is serial.
 * It is never part of a release graph, which compiles every component inside its one driver task.
 * @see ../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts
 * @see ../../../🧾️store-receipt/🧬️schema/🔣️.json */'''),
    (r'''  const script = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts", moduleDirectory = loaded.componentDeploymentDirectoryV1(metadata.semio);
  return { component: executionTargetV1("orchestration", {
    executor: DEFAULT_EXECUTOR,
    cache: false,
    metadata: { semio: { orchestrator: true } },
    dependsOn: ["@semio-tech/os-plugin-describe-rs:build", "@semio-tech/framework-plugin-web:support", "workspace:deps-wasm-opt"],
    outputs: [componentModelOutputV1(buildPath(root)), ...(moduleDirectory === undefined ? [] : [`{workspaceRoot}/${PLUGIN_MODULES_ROOT}/${moduleDirectory}`])],
    options: { cwd: ".", command: `bun ${JSON.stringify(script)} component --manifest ${JSON.stringify(buildPath(relative(workspaceRoot, path)))}`, forwardAllArgs: false },
  }) };
}''', r'''  const script = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts", owner = buildPath(join(root, "..", "..")), toolchain = POLICY.toolchains.cargo;
  const binaryen = buildPath(relative(workspaceRoot, join(LIBRARY_ROOT, "⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/🧬️schema/🔣️.json")));
  return { component: withCacheDecision(executionTargetV1("orchestration", {
    executor: DEFAULT_EXECUTOR,
    cache: true,
    parallelism: false,
    metadata: { semio: { orchestrator: true, componentStoreV1: { version: 1 } } },
    dependsOn: ["@semio-tech/os-plugin-describe-rs:build", "@semio-tech/framework-plugin-web:support", "workspace:deps-wasm-opt"],
    inputs: ["nativeSources", "^nativeSources", { dependentTasksOutputFiles: "**/*" }, `{workspaceRoot}/${owner}/🛂️.descriptor.semio`, `{workspaceRoot}/${owner}/🔣️.json`, `{workspaceRoot}/${binaryen}`, ...toolchain.files.map((file) => `{workspaceRoot}/${file}`), ...toolchain.environment.map((env) => ({ env })), ...toolchain.commands.map((runtime) => ({ runtime })), { externalDependencies: ["@bytecodealliance/jco", "@bytecodealliance/preview2-shim"] }, { runtime: 'node -p "process.platform.concat(process.arch)"' }],
    outputs: [storeReceiptOutputV1("component")],
    options: { cwd: ".", command: `bun ${JSON.stringify(script)} component --manifest ${JSON.stringify(buildPath(relative(workspaceRoot, path)))}`, forwardAllArgs: false },
  }), "deterministic", "pure-function-of-inputs") };
}'''),
)

edit(I + "/🧱️project-assembly/🟨️.mjs",
    (r'''...Object.fromEntries(Object.entries(componentBuildTargets(root, workspaceRoot)).map(([name, target]) => [name, withCacheDecision(target, "driver-owned", "freshness-owned-by-driver")])), ''', r'''...componentBuildTargets(root, workspaceRoot), '''),
    (r'''    if (nativeTarget && nativeProject && readToml(join(workspaceRoot, nativeRoot, "Cargo.toml")).package) {''', r'''    if ((nativeTarget || policy.metadata?.semio?.componentStoreV1) && nativeProject && readToml(join(workspaceRoot, nativeRoot, "Cargo.toml")).package) {'''),
)

PG = I + "/🎮️playground-targets/🟨️.mjs"
edit(PG,
    (r'''import { observedSourceAnalysisV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
''', r'''import { observedSourceAnalysisV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { storeReceiptOutputV1 } from "../../../🧾️store-receipt/🟨️.mjs";
'''),
    (r'''let POLICY, readToml, buildPath, LIBRARY_ROOT, executionTargetV1, withCacheDecision, PROJECT_BASENAME, readPhysicalSource, loaded, nativePreparation, componentModelOutputV1, PLUGIN_MODULES_ROOT, prepareSourceAnalysesV1, revisionIdentity;''', r'''let POLICY, readToml, buildPath, LIBRARY_ROOT, executionTargetV1, withCacheDecision, PROJECT_BASENAME, readPhysicalSource, loaded, nativePreparation, prepareSourceAnalysesV1, revisionIdentity;'''),
    (r'''  ({ POLICY, buildPath, LIBRARY_ROOT, PROJECT_BASENAME, readPhysicalSource, componentModelOutputV1, PLUGIN_MODULES_ROOT } = registry.coordinates);''', r'''  ({ POLICY, buildPath, LIBRARY_ROOT, PROJECT_BASENAME, readPhysicalSource } = registry.coordinates);'''),
    (r'''/** 🎮️ Declares completed runtime prerequisites, cacheable validation and profile-specific browser producers.
 * React activation owns its restorable runtime directory. WGPU activation publishes live extensions,
 * fonts and reload notifications, so it must execute after every cacheable preparation. */''', r'''/** 🎮️ Declares completed runtime prerequisites, cacheable validation and profile-specific browser producers.
 * A lane's preparation is a cached verdict over the `component` targets of its closure (each replays from its own receipt), so no lane compiles; its
 * activation owns a restorable runtime directory and hashes the owner trees of the components it records. The production Vite bundle replays from the
 * receipt of its store output over the `viteBundleSources` contract (the framework and os-dev trees, the lane's owner roots and development contribution). */'''),
    (r'''  const closurePrerequisites=async(ids,facet)=>{const prerequisites=["@semio-tech/os-plugin-describe-rs:build",...(facet==="browser"?["workspace:deps-wasm-opt"]:[])];for(const id of ids)prerequisites.push(...await componentPrerequisites(id));return [...new Set(prerequisites)].sort();};
  const closureOutputs=(ids,facet="browser",mode="{options.env.SEMIO_BUILD_MODE}")=>[...ids].sort().flatMap(id=>{const row=components.get(id),directory=loaded.componentDeploymentDirectoryV1(row);return[componentModelOutputV1(row.cratePath,mode),...(facet==="browser"&&directory!==undefined?[`{workspaceRoot}/${PLUGIN_MODULES_ROOT}/${directory}`]:[])];});
''', r'''  const componentTargetsOf=async ids=>{const targets=[];for(const id of [...ids].sort()){await componentPrerequisites(id);targets.push(`${components.get(id).project}:component`);}return targets;};
  const ownerSources=ids=>[...new Set([...ids].map(id=>{const parts=components.get(id).cratePath.split("/");return parts.length>=4&&parts.slice(-3).join("/")==="🧩️component/📦️packages/🦀️rust"?parts.slice(0,-3).join("/"):parts.join("/");}))].sort().flatMap(owner=>[`{workspaceRoot}/${owner}/**/*`,...POLICY.generatedDirectories.map(directory=>`!{workspaceRoot}/${owner}/**/${directory}/**/*`)]);
  const transitiveOutputs={ dependentTasksOutputFiles: "**/*", transitive: true };
'''),
    (r'''    const bootPrerequisites=await closurePrerequisites(bootSelected,"browser"),selectedPrerequisites=await closurePrerequisites(selected,"browser"),nativePrerequisites=await closurePrerequisites(selected,"model");''', r'''    const bootComponents=await componentTargetsOf(bootSelected),selectedComponents=await componentTargetsOf(selected);'''),
    (r'''      result[`prepare-${playground.variant}-native-${profile}`] = withCacheDecision({
        cache: false,
        metadata: orchestrator,
        outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/native/${playground.variant}`, ...closureOutputs(selected,"model",profile)],
        dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, ...nativePrerequisites],
        options: { command: `bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/📜️script.ts publish ${playground.variant} ${profile}`, forwardAllArgs: false },
      }, "driver-owned", "freshness-owned-by-driver");''', r'''      result[`prepare-${playground.variant}-native-${profile}`] = withCacheDecision({
        cache: true,
        outputs: [storeReceiptOutputV1(`prepare-${playground.variant}-native-${profile}`)],
        inputs: [transitiveOutputs],
        dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, ...selectedComponents],
        options: { command: `bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/📜️script.ts publish ${playground.variant} ${profile}`, forwardAllArgs: false },
      }, "deterministic", "pure-function-of-inputs");'''),
    (r'''      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`prepare-${playground.variant}-react-${profile}`],''', r'''      inputs: [transitiveOutputs, ...ownerSources(bootSelected)],
      dependsOn: [`prepare-${playground.variant}-react-${profile}`],'''),
    (r'''      result[`prepare-${playground.variant}-react-${profile}`] = withCacheDecision({
      cache: false,
      metadata: orchestrator,
      outputs: closureOutputs(bootSelected),
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, "@semio-tech/framework-plugin-web:support", "semio-framework-os-infinite:fonts", ...rendererEngines, ...engines, ...bootPrerequisites],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} react ${profile}` },
      }, "driver-owned", "freshness-owned-by-driver");''', r'''      result[`prepare-${playground.variant}-react-${profile}`] = withCacheDecision({
      cache: true,
      outputs: [],
      inputs: [transitiveOutputs],
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, "@semio-tech/framework-plugin-web:support", "semio-framework-os-infinite:fonts", ...rendererEngines, ...engines, ...bootComponents],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} react ${profile}` },
      }, "deterministic", "check-without-outputs");'''),
    (r'''      result[`activate-${playground.variant}-wgpu-${profile}`] = withCacheDecision({
      cache: false,
      outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/runtime/wgpu/${playground.variant}`],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`prepare-${playground.variant}-wgpu-${profile}`],
      options: { command: `bun ./📜️script.ts activate ${playground.variant} wgpu ${profile}` },
      }, "side-effecting", "publishes-live-state");''', r'''      result[`activate-${playground.variant}-wgpu-${profile}`] = withCacheDecision({
      cache: true,
      parallelism: false,
      outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/runtime/wgpu/${playground.variant}`],
      inputs: [transitiveOutputs, ...ownerSources(bootSelected)],
      dependsOn: [`prepare-${playground.variant}-wgpu-${profile}`],
      options: { command: `bun ./📜️script.ts activate ${playground.variant} wgpu ${profile}` },
      }, "deterministic", "pure-function-of-inputs");'''),
    (r'''      result[`prepare-${playground.variant}-wgpu-${profile}`] = withCacheDecision({
      cache: false,
      metadata: orchestrator,
      outputs: closureOutputs(bootSelected),
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, "@semio-tech/framework-plugin-web:support", "semio-framework-os-infinite:fonts", `${wgpuProject.name}:wasm`, `${wgpuProject.name}:generate-browser-boot`, `${wgpuProject.name}:generate-frame-worker`, `${wgpuProject.name}:generate-renderer-boot`, ...bootPrerequisites],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} wgpu ${profile}` },
      }, "driver-owned", "freshness-owned-by-driver");''', r'''      result[`prepare-${playground.variant}-wgpu-${profile}`] = withCacheDecision({
      cache: true,
      outputs: [],
      inputs: [transitiveOutputs],
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, "@semio-tech/framework-plugin-web:support", "semio-framework-os-infinite:fonts", `${wgpuProject.name}:wasm`, `${wgpuProject.name}:generate-browser-boot`, `${wgpuProject.name}:generate-frame-worker`, `${wgpuProject.name}:generate-renderer-boot`, ...bootComponents],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} wgpu ${profile}` },
      }, "deterministic", "check-without-outputs");'''),
    (r'''...distributionEngines, ...engines, ...selectedPrerequisites, "@semio-tech/assets:build"],''', r'''...distributionEngines, ...engines, ...selectedComponents, "@semio-tech/assets:build"],'''),
    (r'''    const buildIdentity=createHash("sha256").update("os-dev:"+playground.variant+":react").digest("hex").slice(0,16);
    const buildRoot = `{workspaceRoot}/.cache/b/p/m/vite`;
    const buildOutput = `${buildRoot}/release/${buildIdentity}/out`;
''', r'''    const contribution = playground.devContribution === undefined ? undefined : JSON.parse(readFileSync(join(workspaceRoot, playground.devContribution), "utf8"));
    const contributionSources = contribution === undefined ? [] : [`{workspaceRoot}/${playground.devContribution}`, ...(typeof contribution.ownerRoot === "string" ? [`{workspaceRoot}/${buildPath(contribution.ownerRoot)}/**/*`] : []), ...(contribution.browserSessionFactories ?? []).filter(row => typeof row.engine === "string").map(row => `{workspaceRoot}/${buildPath(row.engine).replace(/^\.\//u, "")}/**/*`)];
'''),
    (r'''    result[name] = withCacheDecision({
      cache: false,
      metadata: orchestrator,
      outputs: [buildOutput],''', r'''    result[name] = withCacheDecision({
      cache: true,
      parallelism: false,
      metadata: orchestrator,
      outputs: [storeReceiptOutputV1(name)],'''),
    (r'''      inputs: ["production", "^production", { dependentTasksOutputFiles: "**/*", transitive: true }, { env: "S_HUB_URL" },''', r'''      inputs: ["production", "^production", "viteBundleSources", transitiveOutputs, ...ownerSources(selected), ...contributionSources, { env: "S_HUB_URL" },'''),
    (r'''    }, "driver-owned", "freshness-owned-by-driver");
    result[`build-${playground.variant}-react`] = {
      ...result[name], outputs: [`${buildRoot}/{options.env.SEMIO_BUILD_MODE}/${buildIdentity}/out`],''', r'''    }, "deterministic", "pure-function-of-inputs");
    result[`build-${playground.variant}-react`] = {
      ...result[name], outputs: [storeReceiptOutputV1(`build-${playground.variant}-react`)],'''),
)

edit(OS + "/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts",
    (r'''import { repositoryCargoNetworkV1 } from "@semio-tech/repo-lib/cargo";
''', ''),
    (r'''import { buildComponentClosureV1, playgroundComponentClosureV1 } from "@semio-tech/framework-plugin/build/driver-stages";
''', ''),
    (r'''/** 🧩️ Semantic activation preparation owner. */''', r'''/** 🧩️ Semantic activation preparation owner: a read-only verdict over the staged closure of one lane. Every component is built by its own cached `component` target before the lane prepares, so preparation compiles nothing and refuses a closure that is not staged.
 * @see ../../../🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts */'''),
    (r'''    await buildComponentClosureV1(repoRoot, playgroundComponentClosureV1(variant, "boot"), { facet:"browser",invocation:this.invocation,network:repositoryCargoNetworkV1(this.invocation,repoRoot), control: scriptCompilerAdmissionControlV1(this.invocation, "native:driver", 0) });
''', ''),
)

edit(OS + "/🧑‍💻dev/🚚️distribution/📜️script.ts",
    (r'''import { repositoryCargoNetworkV1 } from "../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
''', ''),
    (r'''import { buildComponentClosureV1, playgroundComponentClosureV1 } from "../../🔌️plugin/📦️packages/🦀️rust/🏗️build/🧩️driver-stages/🟦️.ts";
''', ''),
    (r'''/** 🏗️ Builds exactly the variant's components through the build driver, then bundles the browser variant. */''', r'''/** 🏗️ Bundles the browser variant over components the build tool already staged through their `component` targets. */'''),
    (r'''      await buildComponentClosureV1(workspace, playgroundComponentClosureV1(variant, "full"), { facet:"browser",invocation:this.invocation,network:repositoryCargoNetworkV1(this.invocation,workspace),control: scriptCompilerAdmissionControlV1(this.invocation, "native:driver", 0) });
''', ''),
)

edit(WGPU + "/⌨️native-entrypoint/📦️modules/📜️script.ts",
    (r'''import { repositoryCargoNetworkV1 } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
''', ''),
    (r'''import { buildComponentClosureV1 } from "../../../../../../🔌️plugin/📦️packages/🦀️rust/🏗️build/🧩️driver-stages/🟦️.ts";
''', ''),
    (r'''/** 📥️ Builds exactly the session's components through the build driver, then publishes the native runtime from their modules and the session. */''', r'''/** 📥️ Publishes the native runtime from the session and the modules its `component` targets already staged. */'''),
    (r'''      await buildComponentClosureV1(repo, new Set(session.plugins.map((plugin: { pluginId: string }) => plugin.pluginId)), { facet:"model",invocation:this.invocation,network:repositoryCargoNetworkV1(this.invocation,repo),control:phase });
''', ''),
)

edit(OS + "/🔌️plugin/📦️packages/🟦️typescript/📋️project.json",
    (r'''    "support": {
      "executor": "repo:script",
      "cache": false,
      "inputs": [
        "default",
        "^default",
        "reads",
        "production",
        "browserBundleSources"
      ],
      "outputs": [
        "{workspaceRoot}/.cache/b/p/m/browser/{options.env.SEMIO_BUILD_MODE}/modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim",
        "{workspaceRoot}/.cache/b/p/m/browser/{options.env.SEMIO_BUILD_MODE}/modules/🧵️shard"
      ],
      "options": {
        "command": "bun ./📜️script.ts support"
      },
      "metadata": {
        "semio": {
          "cache": { "version": 1, "class": "managed-output", "reason": "outputs-in-managed-store" }
        }
      },''', r'''    "support": {
      "executor": "repo:script",
      "cache": true,
      "inputs": [
        "sharedGlobals",
        "browserBundleSources",
        { "externalDependencies": ["@bytecodealliance/preview2-shim"] }
      ],
      "outputs": [
        "{projectRoot}/.🧾️store/support.json"
      ],
      "options": {
        "command": "bun ./📜️script.ts support"
      },
      "metadata": {
        "semio": {
          "cache": { "version": 1, "class": "deterministic", "reason": "pure-function-of-inputs" }
        }
      },'''),
)

edit(BZ + "/🧬️schema/🔣️.json",
    (r'''    "nativeCommand": "^bun (?:\"[^\"]+\"|\\S+) native cargo (check|build|test) --manifest (\"[^\"]+\"|\\S+)$",''', r'''    "nativeCommand": "^bun (?:\"[^\"]+\"|\\S+) native cargo (check|build|test) --manifest (\"[^\"]+\"|\\S+)$",
    "componentCommand": "^bun (?:\"[^\"]+\"|\\S+) component --manifest (\"[^\"]+\"|\\S+)$",'''),
)

edit(BZ + "/🟦️.ts",
    (r'''/** 🧮️ Names the Cargo subcommand and manifest of a native task from its authored command; every other task is not a Cargo task. */''', r'''/** 🧮️ Names the Cargo subcommand and manifest of a native task from its authored command, or the `build` of a component task (its driver compiles the one crate); every other task is not a Cargo task. */'''),
    (r''' if(kind!=="native"||typeof command!=="string")return undefined;
 const match=new RegExp(policy.nativeCommand,"u").exec(command);
 return match?{subcommand:match[1] as BazelCargoClassV1["subcommand"],manifest:match[2]!.replace(/^"|"$/gu,"")}:undefined;''', r''' if(typeof command!=="string")return undefined;
 if(kind==="orchestration"){const match=new RegExp(policy.componentCommand,"u").exec(command);return match?{subcommand:"build",manifest:match[1]!.replace(/^"|"$/gu,"")}:undefined;}
 if(kind!=="native")return undefined;
 const match=new RegExp(policy.nativeCommand,"u").exec(command);
 return match?{subcommand:match[1] as BazelCargoClassV1["subcommand"],manifest:match[2]!.replace(/^"|"$/gu,"")}:undefined;'''),
)

edit(BZ + "/🧪️tests/🔣️.json",
    (r'''  {
   "name": "source kind is no cargo task",''', r'''  {
   "name": "component driver",
   "kind": "orchestration",
   "command": "bun \"🧰️x/🏗️build/🧩️driver-stages/📜️script.ts\" component --manifest \"🌎️y/🧩️component/📦️packages/🦀️rust/Cargo.toml\"",
   "expected": { "subcommand": "build", "manifest": "🌎️y/🧩️component/📦️packages/🦀️rust/Cargo.toml" }
  },
  {
   "name": "component driver unquoted",
   "kind": "orchestration",
   "command": "bun a/script.ts component --manifest p/Cargo.toml",
   "expected": { "subcommand": "build", "manifest": "p/Cargo.toml" }
  },
  {
   "name": "component command of a native task is no cargo task",
   "kind": "native",
   "command": "bun \"a/script.ts\" component --manifest \"p/Cargo.toml\"",
   "expected": null
  },
  {
   "name": "native cargo command of an orchestration is no cargo task",
   "kind": "orchestration",
   "command": "bun \"a/script.ts\" native cargo build --manifest \"p/Cargo.toml\"",
   "expected": null
  },
  {
   "name": "component command with a flag is no cargo task",
   "kind": "orchestration",
   "command": "bun \"a/script.ts\" component --manifest \"p/Cargo.toml\" --release",
   "expected": null
  },
  {
   "name": "source kind is no cargo task",'''),
)


def template(name, old_class, old_reason, new_class, new_reason):
    pad = '        '
    old = f'"name": "{name}",\n{pad}"class": "{old_class}",\n{pad}"reason": "{old_reason}",\n{pad}"cache": false,'
    new = f'"name": "{name}",\n{pad}"class": "{new_class}",\n{pad}"reason": "{new_reason}",\n{pad}"cache": true,'
    return (old, new)


edit(I + "/🧪️tests/💾️cache/🔣️.json",
    template("prepare-{variant}-native-{profile}", "driver-owned", "freshness-owned-by-driver", "deterministic", "pure-function-of-inputs"),
    template("prepare-{variant}-react-{profile}", "driver-owned", "freshness-owned-by-driver", "deterministic", "check-without-outputs"),
    template("prepare-{variant}-wgpu-{profile}", "driver-owned", "freshness-owned-by-driver", "deterministic", "check-without-outputs"),
    template("activate-{variant}-wgpu-{profile}", "side-effecting", "publishes-live-state", "deterministic", "pure-function-of-inputs"),
    template("prepare-{variant}-react", "driver-owned", "freshness-owned-by-driver", "deterministic", "check-without-outputs"),
    template("prepare-{variant}-wgpu", "driver-owned", "freshness-owned-by-driver", "deterministic", "check-without-outputs"),
    template("build-{variant}-react-release", "driver-owned", "freshness-owned-by-driver", "deterministic", "pure-function-of-inputs"),
    template("build-{variant}-react", "driver-owned", "freshness-owned-by-driver", "deterministic", "pure-function-of-inputs"),
)

edit("✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🏗️preparation/🟦️.ts",
    (r'''    assert.equal(prepare.cache, false, `${row.variant} ${profile}: a preparation builds through the driver and answers to Cargo freshness`);
    assert.equal(prepare.metadata?.semio?.orchestrator, true);
    assert.deepEqual([...prepare.outputs].sort(), moduleOutputs(bootExpected), `${row.variant} ${profile}: boot prepare publishes exactly the appScoped host closure`);
    for (const prerequisite of vectors.prerequisites) assert.ok(prepare.dependsOn.includes(prerequisite), `${row.variant} ${profile}: prepare needs ${prerequisite}`);
    assert.ok(!prepare.dependsOn.some((id: string) => /:(?:materialize|describe|component)$/u.test(id)));
    const native = targets[`prepare-${row.variant}-native-${profile}`];
    assert.equal(native.dependsOn[0], `@semio-tech/plugin-registry:session-${row.variant}`);
    assert.equal(native.cache, false);
    assert.deepEqual(native.outputs.slice(1).sort(), moduleOutputs(fullExpected));
    assert.ok(!native.dependsOn.some((id: string) => /renderer.*:(wasm|generate-)|:fonts$|:(?:materialize|describe|component)$/.test(id)));''', r'''    const componentsOf = (target: any) => target.dependsOn.filter((id: string) => id.endsWith(":component")).sort();
    assert.equal(prepare.cache, true, `${row.variant} ${profile}: a preparation is a cached verdict over the staged closure`);
    assert.deepEqual(prepare.outputs, [], `${row.variant} ${profile}: a verdict publishes nothing`);
    assert.deepEqual([prepare.metadata?.semio?.cache?.class, prepare.metadata?.semio?.cache?.reason], ["deterministic", "check-without-outputs"]);
    assert.deepEqual(componentsOf(prepare), bootExpected.map((id: string) => `${projects.get(id)}:component`).sort(), `${row.variant} ${profile}: boot prepare waits for exactly the appScoped host closure`);
    for (const prerequisite of vectors.prerequisites) assert.ok(!prepare.dependsOn.includes(prerequisite), `${row.variant} ${profile}: ${prerequisite} belongs to the component targets`);
    const native = targets[`prepare-${row.variant}-native-${profile}`];
    assert.equal(native.dependsOn[0], `@semio-tech/plugin-registry:session-${row.variant}`);
    assert.equal(native.cache, true);
    assert.deepEqual(native.outputs, [`{projectRoot}/.🧾️store/prepare-${row.variant}-native-${profile}.json`]);
    assert.deepEqual(componentsOf(native), fullExpected.map((id: string) => `${projects.get(id)}:component`).sort());
    assert.ok(!native.dependsOn.some((id: string) => /renderer.*:(wasm|generate-)|:fonts$|:(?:materialize|describe)$/.test(id)));
    const activate = targets[`activate-${row.variant}-react-${profile}`], wgpuActivate = targets[`activate-${row.variant}-wgpu-${profile}`];
    for (const target of [activate, wgpuActivate]) {
      assert.equal(target.cache, true);
      assert.ok(target.inputs.some((input: unknown) => typeof input === "string" && input.startsWith("{workspaceRoot}/") && input.endsWith("/**/*") && !input.startsWith("{workspaceRoot}/!")), `${row.variant} ${profile}: an activation hashes the owner trees it records`);
    }'''),
    (r'''      assert.equal(build?.cache, false, `${name}: production builds its closure through the driver`);
      assert.equal(build.metadata?.semio?.orchestrator, true);''', r'''      assert.equal(build?.cache, true, `${name}: the Vite bundle replays from its store receipt`);
      assert.equal(build.metadata?.semio?.orchestrator, true);
      assert.deepEqual(componentsOf(build), fullExpected.map((id: string) => `${projects.get(id)}:component`).sort());'''),
    (r'''      const distDir = row.distDir;
      assert.deepEqual(build.outputs, [distDir ? `{workspaceRoot}/${distDir}/release` : `{projectRoot}/dist/build-${row.variant}-react/release`]);
''', r'''      assert.deepEqual(build.outputs, [`{projectRoot}/.🧾️store/${name}.json`]);
      assert.ok(build.inputs.includes("viteBundleSources"));
'''),
    (r'''      for (const prerequisite of vectors.prerequisites) assert.ok(build.dependsOn.includes(prerequisite));
      assert.ok(!build.dependsOn.some((target: string) => /:?(?:activate|dev|serve)-|:(?:materialize|describe|component)$/.test(target)));''', r'''      assert.ok(!build.dependsOn.some((target: string) => /:?(?:activate|dev|serve)-|:(?:materialize|describe)$/.test(target)));'''),
    (r'''const targets = cacheInternals.playgroundPreparationTargets(paths, workspace, dev,{},[catalogBuildContributionV1({readText:path=>readFileSync(join(workspace,path),"utf8")})]),''', r'''const targets = await cacheInternals.playgroundPreparationTargets(paths, workspace, dev,{},[catalogBuildContributionV1({readText:path=>readFileSync(join(workspace,path),"utf8")})],{workspaceRoot:workspace,preparationControl:{advance:async()=>{}}}),'''),
    (r'''    const project = manifest.package.name;''', r'''    const projectFile = join(workspace, dirname(path), "📋️project.json"), project = existsSync(projectFile) ? JSON.parse(readFileSync(projectFile, "utf8")).name : manifest.package.name;'''),
    (r'''import {readFileSync} from "node:fs";''', r'''import {existsSync,readFileSync} from "node:fs";'''),
    (r'''    const bootExpected = oracle(components, [{ id: component.pluginId, appScoped: true }]),''', r'''    const bootExpected = oracle(components, [{ id: component.pluginId, appScoped: true }]) as string[],'''),
)

edit(L + "/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
    (r'''        assert.equal(preparation.cache, false, `${targetName} builds its closure through the driver`);
        assert.equal(preparation.metadata?.semio?.orchestrator, true);''', r'''        assert.equal(preparation.cache, true, `${targetName} is a cached verdict over its staged component closure`);
        assert.deepEqual(preparation.outputs, []);'''),
    (r'''        assert.equal(wgpuPreparation.cache, false, `${wgpuTargetName} builds its closure through the driver`);
        assert.deepEqual(wgpuPreparation.outputs, preparation.outputs, `${wgpuTargetName} publishes the same boot closure modules`);''', r'''        assert.equal(wgpuPreparation.cache, true, `${wgpuTargetName} is a cached verdict over its staged component closure`);
        assert.deepEqual(wgpuPreparation.dependsOn.filter((id: string) => id.endsWith(":component")), preparation.dependsOn.filter((id: string) => id.endsWith(":component")), `${wgpuTargetName} waits for the same boot closure components`);'''),
    (r'''        assert.equal(wgpuActivationTarget.cache, false, `${wgpuActivationName} publishes live extensions and reload notifications on every activation`);''', r'''        assert.equal(wgpuActivationTarget.cache, true, `${wgpuActivationName} publishes into a variant/profile-scoped runtime root and is safe to replay`);'''),
    (r'''        const hostDirectory = moduleOf(playground.pluginId) && toml.parse(readFileSync(join(root, moduleOf(playground.pluginId)!.root, "Cargo.toml"), "utf8")).package.metadata.semio["deployment-directory"];
        if (hostDirectory !== undefined) assert.ok(preparation.outputs.some((output: string) => output.endsWith(`/${hostDirectory}`)), `${targetName}: boot prepare must publish the host module`);''', r'''        const bootComponents = preparation.dependsOn.filter((id: string) => id.endsWith(":component"));
        assert.ok(bootComponents.includes(`${moduleOf(playground.pluginId)!.name}:component`), `${targetName}: boot prepare must wait for the host component`);'''),
    (r'''        assert.ok(preparation.outputs.length <= fullSession.length, `${targetName}: boot prepare must not exceed the full session closure`);
        if (fullSession.length > preparation.outputs.length + 5) assert.ok(preparation.outputs.length < fullSession.length / 2, `${targetName}: host boot prepare must skip the catalog fanout`);
        for (const prerequisite of vectors.closurePreparation.prerequisites) assert.ok(preparation.dependsOn.includes(prerequisite));''', r'''        assert.ok(bootComponents.length <= fullSession.length, `${targetName}: boot prepare must not exceed the full session closure`);
        if (fullSession.length > bootComponents.length + 5) assert.ok(bootComponents.length < fullSession.length / 2, `${targetName}: host boot prepare must skip the catalog fanout`);
        for (const prerequisite of vectors.closurePreparation.prerequisites) assert.ok(!preparation.dependsOn.includes(prerequisite), `${targetName}: ${prerequisite} belongs to the component targets`);'''),
)


edit(OS + "/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json",
    (r'''      "sharedGlobals"
    ],
    "stagingRootSources": [''', r'''      "sharedGlobals"
    ],
    "viteBundleSources": [
      "{workspaceRoot}/🧰️framework/**/*",
      "!{workspaceRoot}/🧰️framework/**/node_modules/**/*",
      "!{workspaceRoot}/🧰️framework/**/.git/**/*",
      "!{workspaceRoot}/🧰️framework/**/.nx/**/*",
      "!{workspaceRoot}/🧰️framework/**/.cache/**/*",
      "!{workspaceRoot}/🧰️framework/**/target/**/*",
      "!{workspaceRoot}/🧰️framework/**/dist/**/*",
      "!{workspaceRoot}/🧰️framework/**/📤️dist/**/*",
      "!{workspaceRoot}/🧰️framework/**/build/**/*",
      "!{workspaceRoot}/🧰️framework/**/out/**/*",
      "!{workspaceRoot}/🧰️framework/**/coverage/**/*",
      "!{workspaceRoot}/🧰️framework/**/test-results/**/*",
      "!{workspaceRoot}/🧰️framework/**/playwright-report/**/*",
      "!{workspaceRoot}/🧰️framework/**/storybook-static/**/*",
      "!{workspaceRoot}/🧰️framework/**/.venv/**/*",
      "!{workspaceRoot}/🧰️framework/**/__pycache__/**/*",
      "!{workspaceRoot}/🧰️framework/**/bin/**/*",
      "!{workspaceRoot}/🧰️framework/**/obj/**/*",
      "!{workspaceRoot}/🧰️framework/**/⚡️cache/**/*",
      "!{workspaceRoot}/🧰️framework/**/🗑️generated/**/*",
      "!{workspaceRoot}/🧰️framework/**/.lake/**/*",
      "!{workspaceRoot}/🧰️framework/**/pkg/**/*",
      "!{workspaceRoot}/🧰️framework/**/.🧾️store/**/*",
      "!{workspaceRoot}/🧰️framework/**/🧪️tests/**/*",
      "{workspaceRoot}/✏️s/🧑‍💻dev/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/node_modules/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/.git/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/.nx/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/.cache/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/target/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/dist/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/📤️dist/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/build/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/out/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/coverage/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/test-results/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/playwright-report/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/storybook-static/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/.venv/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/__pycache__/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/bin/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/obj/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/⚡️cache/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/🗑️generated/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/.lake/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/pkg/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/.🧾️store/**/*",
      "!{workspaceRoot}/✏️s/🧑‍💻dev/**/🧪️tests/**/*",
      "{workspaceRoot}/package.json",
      "{workspaceRoot}/bun.lock",
      "{workspaceRoot}/bunfig.toml",
      "{workspaceRoot}/tsconfig.json"
    ],
    "stagingRootSources": ['''),
)
