const root = process.argv[2]!;
const checks: [string, string[]][] = [
 ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🏗️build/🧩️driver-stages/🟦️.ts", ["buildReleasePipelineV1"]],
 ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts", ["repositoryCargoNetworkV1"]],
 ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts", ["renderCatalogFiles"]],
 ["🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts", ["withScriptInvocationSignalV1","scriptInvocationEnvironment","receiveScriptProcessInvocation","ScriptRouter","BundleScript"]],
 ["🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts", ["runOwnedCommand"]],
 ["🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts", ["startToolV1","toolExitV1"]],
 ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts", ["readNativeCallerAuthorityV1","withRepositoryArtifactV1","stageRepositoryArtifacts"]],
 ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts", ["buildViteArtifact","parseServiceReadinessBudgetV1","readServiceSession","waitForServiceReady","serviceReadyProgressLine","serviceReadyProgressLanguage"]],
 ["🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/📦️assets/🟦️.ts", ["playRuntimeAssetSources"]],
 ["🏢️semio-tech/🎡️play/🔨️modules/📦️site/📄pages/📦️publication/📋️contract/🟦️.ts", ["readPlayPageCohortV1","requirePlayPageCohortMemberV1"]],
];
let failed = 0;
for (const [file, names] of checks) {
  try { const m = await import(`${root}/${file}`); for (const n of names) if (!(n in m)) { failed++; console.log(`MISSING ${n} in ${file}`); } console.log(`ok ${file.split("/").slice(-3).join("/")}`); }
  catch (e) { failed++; console.log(`FAIL ${file}\n  ${String((e as Error).message).split("\n").slice(0,3).join("\n  ")}`); }
}
for (const spec of ["@semio-tech/repo-lib/storage","@semio-tech/repo-lib/artifacts","@semio-tech/repo-lib/dev","@semio-tech/repo-lib/testing","@semio-tech/framework-os-dev/builder/vite/artifacts","@semio-tech/process/routing","@semio-tech/process/leases","@semio-tech/process/compiler-capacity","@semio-tech/schema-validator-rs/index","@semio-tech/framework-plugin/browser-bundle/distribution","@semio-tech/framework-plugin/describe/component-receipt","@semio-tech/plugin-registry/deployment","@semio-tech/assets/published-page"]) {
  try { await import(spec); console.log(`ok ${spec}`); } catch (e) { failed++; console.log(`FAIL ${spec} ${String((e as Error).message).split("\n")[0]}`); }
}
const s = await import("@semio-tech/repo-lib/storage"), a = await import("@semio-tech/repo-lib/artifacts"), d = await import("@semio-tech/repo-lib/dev");
for (const [m, names] of [[s,["playPagesArtifactCohortV1","withRepositoryCacheStoreV1"]],[a,["withRepositoryArtifactGroupV1","withRepositoryArtifactGroupTaskV1","withArtifactScratchV1","withArtifactDestinationTaskV1","withRepositoryArtifactV1","withRepositoryArtifactTaskV1","withRepositoryArtifactDestinationV1","stageRepositoryArtifacts","readNativeCallerAuthorityV1","artifactsStorageSchema"]],[d,["viteStorageLayoutV1"]]] as const) for (const n of names) if (!(n in (m as any))) { failed++; console.log(`MISSING ${n}`); }
const l = await import("@semio-tech/process/leases"); for (const n of ["withResourceLeases","checkResourceLeaseControl"]) if (!(n in l)) { failed++; console.log(`MISSING ${n}`); }
process.exit(failed ? 1 : 0);
