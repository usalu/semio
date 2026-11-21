import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { Database } from "bun:sqlite";
import fixture from "./process3d-launch-inputs.json";

const workspace = process.cwd();
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const os = "🧰️framework/🛍️products/💻️os/🔨️modules";
const inference = library + "/🏗️build/🕸️graph/📥️inference";
const read = (path: string) => readFileSync(resolve(workspace, path), "utf8");
const json = (path: string) => JSON.parse(read(path));
const local = (path: string) => import(pathToFileURL(resolve(workspace, path)).href);
const require = createRequire(import.meta.url);
const toml = require("@iarna/toml");
const picomatch = require("picomatch");
const [playground, targetPolicy, runtime, composition, engines, nativeHost, provider, admission, matchers] = await Promise.all([
  local(inference + "/🎮️playground-targets/🟨️.mjs"),
  local(inference + "/🎯️target-policy/🟨️.mjs"),
  local(library + "/🕸️dependencies/🧩️runtime/🟨️.mjs"),
  local(os + "/🔌️plugin/📇️registry/🎮️playground/🧩️composition/🟨️.mjs"),
  local(os + "/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs"),
  local(library + "/🎮️playground/🖥️native-host/🟨️.mjs"),
  local("✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts"),
  local(library + "/🏗️build/🕸️graph/🚪️admission/🟦️.ts"),
  local(library + "/🏗️build/📥️inputs/🟨️.mjs"),
]);
const policy = json(library + "/⚡️caching/🔣️policy.json");
const coordinates = {
  POLICY: policy,
  PROJECT_BASENAME: "📋️project.json",
  DEFAULT_EXECUTOR: "repo:script",
  targetExecutionSchema: json(library + "/🏗️build/🏃️execution/🧬️schema/🔣️.json"),
  targetCacheSchema: json(library + "/🏗️build/🏃️execution/🧬️schema/💾️cache.json"),
  buildPath: (path: string) => path.replaceAll("\\", "/"),
};
targetPolicy.wire({ coordinates });
const nativeVisited = new Set<string>();
const wiring = {
  coordinates,
  targetPolicy,
  nativeManifests: {
    readToml: (path: string) => toml.parse(read(path)),
    nativePreparation: (path: string) => {
      nativeVisited.add(path);
      return [];
    },
  },
  bootstrap: { loaded: { ...runtime, ...composition, ...engines, ...nativeHost } },
  sourceAnalyses: {},
};
playground.wire(wiring);

const componentManifests = [fixture.hostOwner, ...fixture.extensionRoots].map(owner => owner + "/🧩️component/📦️packages/🦀️rust/Cargo.toml");
const catalog = provider.catalogBuildContributionV1({ readText: read });
const targets = await playground.playgroundPreparationTargets(componentManifests, workspace, fixture.projectRoot, {}, [catalog], { preparationControl: { advance: async () => {} } });
const sessions = playground.playgroundSessionTargets(componentManifests, workspace, [catalog]);
const session = sessions["session-" + fixture.variant];
const dependentOutputs = { dependentTasksOutputFiles: "**/*", transitive: true };
assert.equal(session.options.command, "repo ./📜️script.ts session process3d");
assert.equal(session.cache, true);
assert.deepEqual(session.dependsOn, [fixture.catalog]);
assert.ok(session.inputs.some((input: unknown) => JSON.stringify(input) === JSON.stringify(dependentOutputs)));
assert.deepEqual([...nativeVisited].sort(), componentManifests.map(dirname).sort());
assert.deepEqual(Object.values(json(fixture.react.contribution)).slice(1), fixture.react.runtime);

const { namedInputs } = json(fixture.projectRoot + "/📋️project.json");
const defaultSentinel = fixture.projectRoot + "/📜️script.ts";
const ownerSentinel = fixture.extensionRoots[0] + "/📦️packages/🦀️rust/📜️script.ts";
const defaultUnrelated = os + "/🧑‍💻dev/🧪️tests/🎚️config/🟦️.ts";
const sentinels = [defaultSentinel, defaultUnrelated, ownerSentinel, fixture.react.contribution, ...fixture.react.runtime, ...fixture.wgpu.runtime, fixture.native.entry, fixture.native.ownedPolicy, fixture.native.meshCatalog];
assert.ok(sentinels.every(path => existsSync(join(workspace, path))));
const db = new Database(":memory:");
const sqlRole = db.query("select case when ?1 then 'compile' else coalesce(?2, 'compile') end as role");
function inputRole(target: any, declared = false) {
  const field = target.metadata?.semio?.sourceInputObservation ?? null;
  const actual = admission.buildInputSourceRoleV1(true, declared, [field ?? undefined]);
  assert.equal(actual, (sqlRole.get(+declared, field) as { role: string }).role);
  return actual;
}

function patterns(inputs = ["default"]): string[] {
  return inputs.flatMap((input: unknown) => {
    if (typeof input !== "string") return [];
    if (namedInputs[input]) return patterns(namedInputs[input]);
    return input.includes("{") && !input.startsWith("{workspaceRoot}") && !input.startsWith("{projectRoot}") && !input.startsWith("!") ? [] : [
      input.replaceAll("{workspaceRoot}/", "").replaceAll("{projectRoot}", fixture.projectRoot),
    ];
  });
}

function matched(target: any) {
  const expanded = patterns(target.inputs);
  const selected = sentinels.filter(path => expanded.some(pattern => !pattern.startsWith("!") && matchers.buildInputPathMatchesV1(path, pattern)) && !expanded.some(pattern => pattern.startsWith("!") && matchers.buildInputPathMatchesV1(path, pattern.slice(1))));
  const reference = sentinels.filter(path => expanded.some(pattern => !pattern.startsWith("!") && picomatch.isMatch(path, pattern)) && !expanded.some(pattern => pattern.startsWith("!") && picomatch.isMatch(path, pattern.slice(1))));
  assert.deepEqual(selected, reference);
  return selected;
}

const evidence = [];
const expectProductionFix = process.argv.includes("--require-fixed");
const fallbackReact = [os + "/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts", os + "/🧑‍💻dev/🚀️entry/🟦️.ts"];
const fallbackWgpu = os + "/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts";

function candidateFor(target: any, renderer: "react" | "wgpu", row: { devContribution?: string; compositionConfigPath?: string }) {
  const contribution = renderer === "react" && row.devContribution ? json(row.devContribution) : undefined;
  const runtime = renderer === "react"
    ? contribution ? [contribution.viteConfig, contribution.browserEntry] : fallbackReact
    : [row.compositionConfigPath ?? fallbackWgpu];
  const runtimeSourceEntries = runtime.map(path => "{workspaceRoot}/" + path);
  assert.ok(runtime.every(path => existsSync(join(workspace, path))));
  assert.deepEqual(admission.readBuildRuntimeSourceEntriesV1(runtimeSourceEntries), runtimeSourceEntries);
  return {
    ...target,
    inputs: contribution ? ["{workspaceRoot}/" + row.devContribution] : [],
    metadata: { semio: { ...target.metadata.semio, sourceInputObservation: "freshness", runtimeSourceEntries } },
  };
}

const processRow = { devContribution: fixture.react.contribution, compositionConfigPath: fixture.wgpu.runtime[0] };
const checkNative = process.argv.includes("--native-launch");
const nativeClosure = process.argv.includes("--native-closure");
const checkNativeTarget = (all: Record<string, any>, row: { variant: string; nativeLaunchPolicy?: string; nativeHost?: { cratePath: string; target: string } }, meshCatalogs: readonly string[], profile: string, operation: "run" | "smoke") => {
  const suffix = `${row.variant}-native-${profile}`;
  const name = `${operation}-${suffix}`;
  const target = all[name];
  assert.equal(target.options.command, fixture.native.command.replace("{variant}", row.variant).replace("{profile}", profile) + (operation === "smoke" ? " --smoke" : ""));
  assert.equal(target.options.forwardAllArgs, true, name);
  assert.equal(target.cache, false, name);
  assert.equal(target.continuous, false, name);
  assert.deepEqual(target.outputs, [], name);
  assert.deepEqual(target.dependsOn, [`prepare-${suffix}`, row.nativeHost ? `${json(row.nativeHost.cratePath + "/📋️project.json").name}:${row.nativeHost.target}` : fixture.native.renderer], name);
  const fresh = [...new Set([row.nativeLaunchPolicy, ...meshCatalogs].filter(Boolean))];
  assert.deepEqual(target.inputs, [dependentOutputs, ...fresh.map(path => "{workspaceRoot}/" + path)], name);
  assert.deepEqual(matched(target), fresh, name);
  assert.equal(inputRole(target), "freshness", name);
  assert.equal(target.metadata.semio.runtimeSourceEntries, undefined, name);
};
for (const profile of fixture.profiles) {
  if (checkNative) {
    const preparation = targets[`prepare-process3d-native-${profile}`];
    assert.deepEqual(preparation.inputs, [dependentOutputs]);
    assert.deepEqual([...preparation.dependsOn].sort(), [fixture.session, ...fixture.componentTargets].sort());
    for (const operation of ["run", "smoke"] as const) checkNativeTarget(targets, { variant: fixture.variant }, [], profile, operation);
  }
  for (const renderer of ["react", "wgpu"] as const) {
    const lane = fixture[renderer];
    const suffix = `${fixture.variant}-${renderer}-${profile}`;
    const prepare = targets["prepare-" + suffix];
    const activate = targets["activate-" + suffix];
    const dev = targets["dev-" + suffix];
    const serve = targets["serve-" + suffix];
    assert.deepEqual([...prepare.dependsOn].sort(), [fixture.session, ...fixture.commonPrepare, ...lane.prepare, ...fixture.componentTargets].sort());
    assert.deepEqual(activate.dependsOn, ["prepare-" + suffix]);
    assert.deepEqual(dev.dependsOn, ["activate-" + suffix]);
    assert.deepEqual(serve.dependsOn, [renderer === "react" ? fixture.session : "activate-" + suffix]);
    assert.equal(prepare.options.command, `repo ./📜️script.ts prepare process3d ${renderer} ${profile}`);
    assert.equal(activate.options.command, `repo ./📜️script.ts activate process3d ${renderer} ${profile}`);
    assert.deepEqual(prepare.inputs, [dependentOutputs]);
    assert.deepEqual(activate.outputs, [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/runtime/${renderer}/process3d`]);
    assert.equal(activate.metadata.semio.cache.class, "deterministic");
    assert.equal(activate.cache, true);
    assert.ok(activate.inputs.some((input: unknown) => JSON.stringify(input) === JSON.stringify(dependentOutputs)));
    assert.ok(matched(activate).includes(ownerSentinel));
    const activationCandidate = {
      ...activate,
      metadata: { semio: { ...activate.metadata.semio, sourceInputObservation: "freshness" } },
    };
    assert.deepEqual(matched(activationCandidate), matched(activate));
    assert.deepEqual(activationCandidate.inputs, activate.inputs);
    for (const [operation, target] of Object.entries({ dev, serve })) {
      assert.equal(target.options.command, lane.command.replace("{profile}", profile));
      assert.equal(target.options.env.SEMIO_PLUGIN_CATALOG, join(workspace, catalog.publication.outputRoot));
      assert.equal(target.options.env.SEMIO_WGPU_COMPOSITION_PATH, renderer === "wgpu" ? fixture.wgpu.runtime[0] : undefined);
      assert.equal(target.cache, false);
      assert.equal(target.continuous, true);
      assert.deepEqual(target.outputs, []);
      const runtimeEntries = lane.runtime.map(path => "{workspaceRoot}/" + path);
      const liveCandidate = candidateFor(target, renderer, processRow);
      assert.deepEqual(liveCandidate.metadata.semio.runtimeSourceEntries, runtimeEntries);
      const candidate = { liveMatched: inputRole(liveCandidate), activationMatched: inputRole(activationCandidate), runtime: inputRole(liveCandidate, true) };
      assert.deepEqual(candidate, fixture.sourceRoles.candidate);
      assert.deepEqual(matched(liveCandidate), renderer === "react" ? [fixture.react.contribution] : []);
      assert.deepEqual(liveCandidate.dependsOn, target.dependsOn);
      assert.deepEqual(liveCandidate.options, target.options);
      if (expectProductionFix) {
        assert.deepEqual(target.inputs, liveCandidate.inputs, operation + "-" + suffix);
        assert.deepEqual(target.metadata.semio.runtimeSourceEntries, runtimeEntries);
        assert.equal(inputRole(target), candidate.liveMatched);
        assert.equal(inputRole(activate), candidate.activationMatched);
      } else {
        assert.equal(target.inputs, undefined, operation + "-" + suffix);
        assert.ok(matched(target).includes(defaultSentinel));
        assert.ok(matched(target).includes(defaultUnrelated));
        assert.deepEqual({ liveMatched: inputRole(target), activationMatched: inputRole(activate), runtime: inputRole(target, true) }, fixture.sourceRoles.asGenerated);
      }
      evidence.push({ target: operation + "-" + suffix, compiledInputMatches: matched(target).filter(path => inputRole(target) === "compile" && /\.[cm]?[jt]sx?$/.test(path)), activationMatch: ownerSentinel, runtimeEntries });
    }
  }
}
if (process.argv.includes("--all-variants") || process.argv.includes("--closure") || nativeClosure) {
  const module = await local(inference + "/🟨️.mjs");
  await module.libraryBootstrap;
  const defaultTargets = new Map<string, any>();
  if (process.argv.includes("--all-variants")) {
    const roots = catalog.publication.contributions.flatMap((owner: any) => owner.packageRoots);
    const found = Bun.spawnSync(["repo", "tools", "exec", "--light", "git", "--", "grep", "-z", "-l", "^\\[package.metadata.component\\]", "--", ...roots.map((root: string) => root + "/**/Cargo.toml")], { cwd: workspace });
    assert.equal(found.exitCode, 0, found.stderr.toString());
    const manifests = found.stdout.toString().split("\0").filter(Boolean);
    const rows = playground.collectPlaygroundCatalog(manifests, workspace, [catalog]);
    const selected = rows.map((row: any) => row.variant);
    assert.equal(new Set(selected).size, selected.length);
    assert.ok(selected.includes(fixture.variant));
    playground.wire({ ...wiring, sourceAnalyses: { prepareSourceAnalysesV1: module.prepareSourceAnalysesV1 } });
    const generic = await playground.playgroundPreparationTargets(manifests, workspace, fixture.projectRoot, {}, [catalog], { preparationControl: { advance: async () => {} } });
    let count = 0, nativeCount = 0, policyCount = 0, meshCount = 0;
    const fallback = { react: [], wgpu: [] };
    const refused = [];
    for (const row of rows) {
      if (checkNative && generic[`run-${row.variant}-native-prototype`].metadata?.semio?.runtimeInputAdmission?.status !== "refused") {
        const assets = toml.parse(read(row.cratePath + "/Cargo.toml")).package.metadata.semio.assets ?? [];
        const meshCatalogs = assets.filter(asset => asset.kind === "mesh-collection" && (asset.app === undefined || asset.app === row.app)).map(asset => asset.catalog);
        for (const profile of fixture.profiles) for (const operation of ["run", "smoke"] as const) {
          checkNativeTarget(generic, row, meshCatalogs, profile, operation);
          nativeCount++;
          if (row.nativeLaunchPolicy !== undefined) policyCount++;
          if (meshCatalogs.length) meshCount++;
        }
      }
      for (const profile of fixture.profiles) for (const renderer of ["react", "wgpu"] as const) {
        for (const operation of ["dev", "serve"] as const) {
          const name = `${operation}-${row.variant}-${renderer}-${profile}`;
          const target = generic[name];
          if (target.metadata?.semio?.runtimeInputAdmission?.status === "refused") {
            refused.push(name);
            continue;
          }
          const candidate = candidateFor(target, renderer, row);
          assert.deepEqual(candidate.dependsOn, target.dependsOn);
          assert.deepEqual(candidate.options, target.options);
          assert.equal(inputRole(candidate), "freshness");
          assert.equal(inputRole(candidate, true), "compile");
          if (expectProductionFix) {
            assert.deepEqual(target.inputs, candidate.inputs, name);
            assert.deepEqual(target.metadata.semio.runtimeSourceEntries, candidate.metadata.semio.runtimeSourceEntries, name);
            assert.equal(inputRole(target), "freshness", name);
            assert.equal(inputRole(generic[`activate-${row.variant}-${renderer}-${profile}`]), "freshness", name);
          }
          if (renderer === "react" && row.devContribution === undefined) {
            assert.deepEqual(candidate.metadata.semio.runtimeSourceEntries, fallbackReact.map(path => "{workspaceRoot}/" + path));
            fallback.react.push(row.variant);
            if (!defaultTargets.has(renderer)) defaultTargets.set(renderer, target);
          }
          if (renderer === "wgpu" && row.compositionConfigPath === undefined) {
            assert.deepEqual(candidate.metadata.semio.runtimeSourceEntries, ["{workspaceRoot}/" + fallbackWgpu]);
            fallback.wgpu.push(row.variant);
            if (!defaultTargets.has(renderer)) defaultTargets.set(renderer, target);
          }
          count++;
        }
      }
    }
    assert.ok(fallback.react.length > 0 && fallback.wgpu.length > 0);
    if (checkNative) {
      assert.equal(nativeCount, rows.length * fixture.profiles.length * 2);
      assert.equal(rows.find(row => row.variant === fixture.native.ownedPolicyVariant).nativeLaunchPolicy, fixture.native.ownedPolicy);
      assert.ok(policyCount > 0 && policyCount < nativeCount);
      assert.ok(meshCount > 0 && meshCount < nativeCount);
    }
    if (checkNative) console.log(JSON.stringify({ nativeTargets: nativeCount, withOwnedPolicy: policyCount, withMeshCatalog: meshCount }));
    console.log(JSON.stringify({ manifests: manifests.length, variants: rows.length, liveTargets: count, refused, fallback: Object.fromEntries(Object.entries(fallback).map(([side, names]) => [side, [...new Set(names)].sort()])) }, null, 2));
  }
  if (process.argv.includes("--closure")) {
    const scripts = module.createScriptInputCache();
    for (const [id, renderer, row] of [
      ["process-react", "react", processRow], ["process-wgpu", "wgpu", processRow], ["default-react", "react", {}], ["default-wgpu", "wgpu", {}],
    ] as const) {
      const processTarget = targets[`dev-process3d-${renderer}-prototype`];
      const authoredTarget = id.startsWith("process-") ? processTarget : defaultTargets.get(renderer);
      const target = (expectProductionFix && authoredTarget) || candidateFor(processTarget, renderer, row);
      const combined = process.argv.includes("--combined");
      const normalized = module.cacheInternals.targetWithDefaults(target, fixture.projectRoot, true);
      const actual = await module.cacheInternals.targetScriptSources(combined ? normalized : { ...target, options: {} }, workspace, scripts, fixture.projectRoot);
      const compiled = scripts.compiledNames.get(actual.closure);
      for (const entry of target.metadata.semio.runtimeSourceEntries) {
        assert.ok(actual.closure.includes(entry), `${id} observed runtime entry ${entry}`);
        assert.ok(compiled.has(entry), `${id} classified runtime entry ${entry}`);
      }
      if (combined) {
        const command = await module.cacheInternals.targetScriptSources({ options: normalized.options }, workspace, scripts, fixture.projectRoot);
        for (const entry of scripts.compiledNames.get(command.closure)) {
          assert.ok(compiled.has(entry), `${id} preserved command module ${entry}`);
        }
      } else {
        assert.ok(!actual.closure.includes("{workspaceRoot}/" + defaultUnrelated));
      }
      console.log(JSON.stringify({ closure: id, combined, observed: actual.closure.length, compiled: compiled.size, runtimeEntries: target.metadata.semio.runtimeSourceEntries }));
    }
  }
  if (nativeClosure) {
    const launch = await local(dirname(fixture.native.defaultPolicy) + "/🟦️.ts");
    const baseline = launch.readNativeLaunchDeclarationV1(json(fixture.native.defaultPolicy));
    assert.deepEqual(launch.selectNativeLaunchDeclarationV1(baseline, undefined, () => assert.fail("default policy has a static import")), baseline);
    const policyReads: string[] = [];
    const owned = launch.selectNativeLaunchDeclarationV1(baseline, fixture.native.ownedPolicy, path => {
      policyReads.push(path);
      return read(path);
    });
    assert.deepEqual(policyReads, [fixture.native.ownedPolicy]);
    assert.deepEqual(owned, json(fixture.native.ownedPolicy));

    const scripts = module.createScriptInputCache();
    const target = module.cacheInternals.targetWithDefaults(targets["run-process3d-native-prototype"], fixture.projectRoot, true);
    const actual = await module.cacheInternals.targetScriptSources(target, workspace, scripts, fixture.projectRoot);
    const commandOnly = await module.cacheInternals.targetScriptSources({ options: target.options }, workspace, scripts, fixture.projectRoot);
    assert.deepEqual(actual.closure, commandOnly.closure);
    const compiled = scripts.compiledNames.get(actual.closure);
    assert.deepEqual(compiled, scripts.compiledNames.get(commandOnly.closure));
    assert.ok(compiled.has("{workspaceRoot}/" + fixture.native.entry));
    assert.ok(actual.closure.includes("{workspaceRoot}/" + fixture.native.defaultPolicy));
    assert.ok(!compiled.has("{workspaceRoot}/" + fixture.native.defaultPolicy));
    assert.ok(!compiled.has("{workspaceRoot}/" + defaultUnrelated));
    console.log(JSON.stringify({ closure: "process-native", observed: actual.closure.length, compiled: compiled.size, policyReads }));
  }
}
db.close();
console.log(JSON.stringify({ session: session.dependsOn, profiles: fixture.profiles, prepare: { react: targets["prepare-process3d-react-prototype"].dependsOn, wgpu: targets["prepare-process3d-wgpu-prototype"].dependsOn }, evidence }, null, 2));
