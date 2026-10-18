import { analyze } from "./ast.ts";
import { build, moduleGraph, write, type Plan } from "./split.ts";
export const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
export const plan: Plan = {
  source: `${L}/🟦️.ts`,
  baseDir: L,
  modules: [
    { dir: "🏗️|build/📥️|inputs/🧊️|stable-file", ranges: [["readStableBuildFile", "readStableBuildFile"]] },
    { dir: "🗂️|workspaces/📁️|data-directories", ranges: [["getHubDataDir", "getMapCacheDir"]] },
    { dir: "🚨️|policy/🔖️|breach", ranges: [["BreachPriority", "BreachRecord"]] },
    { dir: "🚨️|policy/🖥️|cli", ranges: [["fixtureItemsOf", "getLibRoot"]] },
    { dir: "🚨️|policy/🧩️|linters", ranges: [["NODE_QUERY", "defineLint"]] },
    { dir: "🚨️|policy/🛡️|dependency-boundary", ranges: [["ADAPTER_MARKERS", "dependencyBoundaryBreachesForBundleDir"]] },
    { dir: "🚨️|policy/🔒️|allowlist", ranges: [["POLICY_DISCOVERY_SKIPPED_DIRS", "policyDiscoveredAllowlist"]] },
    { dir: "🚨️|policy/🏃️|runner", ranges: [["LintScriptModule", "runPolicyExit"]] },
    { dir: "🚨️|policy/🧭️|entrypoints", ranges: [["dispatchPolicyArgv", "dispatchSubcommand"]] },
    { dir: "🧪️|testing/🎚️|levels", ranges: [["levelsAbove", "goLevelTestArgs"], ["vitestLevelArgs", "playwrightTestTimeoutMs"]] },
    { dir: "🧪️|testing/🐹️|go-plan", ranges: [["CanonicalGoPlan", "runCanonicalGoBuild"]] },
    { dir: "🧪️|testing/🎛️|repository-command", ranges: [["RepositoryTestCommandOptionsV1", "capturedTestFailureDiagnostics"]] },
    { dir: "🧪️|testing/📊️|coverage", ranges: [["coverageEnabled", "dotnetCoverageArgs"], ["CoverageSummary", "enforceCoverageThreshold"]] },
    { dir: "🧪️|testing/🗄️|lcov", ranges: [["LcovFileRecord", "goProfileToLcov"]] },
    { dir: "🧪️|testing/🧪️|vitest/📋️|arguments", ranges: [["vitestRunArguments", "vitestRunArguments"]] },
    { dir: "🧪️|testing/🎭️|playwright", ranges: [["runPlaywright", "runPlaywright"]] },
    { dir: "🏃️|process/🦀️|cargo/🔍️|package-names", ranges: [["CrateIndexRecord", "resolveCargoPackageNames"]] },
    { dir: "🏃️|process/👹️|daemon", ranges: [["killBudgetTree", "killBudgetTree"], ["SpawnDaemonHandle", "waitForHttpUrl"]] },
    { dir: "🏃️|process/🦀️|cargo/🧹️|lint", ranges: [["runCargoLint", "runCargoLint"]] },
    { dir: "🏃️|process/🦀️|cargo/🏃️|run", ranges: [["runCargo", "runCargo"]] },
    { dir: "🏃️|process/🍞️|bun", ranges: [["viteConfigLoader", "spawnBun"], ["runViteBunxDevPlain", "runViteBunxDevPlain"]] },
    { dir: "🏃️|process/🧭️|routing/🔗️|script-url", ranges: [["scriptPathFromUrl", "scriptPathFromUrl"]] },
    { dir: "🎮️|playground/🖥️|asset-server", ranges: [["SEMIO_ASSET_SERVER_PORT", "SEMIO_ASSET_BASE_URL_ENV"]] },
    { dir: "🎮️|playground/📋️|example-argv", ranges: [["consumePlaygroundExampleArgv", "consumePlaygroundExampleArgv"]] },
    { dir: "🎮️|playground/🧊️|wgpu-dev", ranges: [["WGPU_DEV_LEGACY_ENTRY_PATH", "stopTrunkDevPort"]] },
    { dir: "🎛️|commands/📝️|commit/🌳️|git", ranges: [], names: ["gitRepoRoot", "gitTrackedPaths", "gitDir", "GIT_INDEX_LOCK_RE", "GIT_INDEX_LOCK_MAX_ATTEMPTS", "GIT_INDEX_LOCK_RETRY_DELAY_MS", "git", "gitCachedNames", "gitEmail"] },
    { dir: "🎛️|commands/📝️|commit/📊️|metrics/🏷️|types", ranges: [["MetricKindId", "MetricsRunner"]] },
    { dir: "🎛️|commands/📝️|commit/📊️|metrics/🛤️|path-rules", ranges: [["METRICS_LOCK_FILES", "langMetricsEmoji"]] },
    { dir: "🎛️|commands/📝️|commit/📊️|metrics/🔢️|uloc", ranges: [["countJsonKeysValue", "countUnifiedLocForFile"]] },
    { dir: "🎛️|commands/📝️|commit/📊️|metrics/💾️|scan", ranges: [["METRICS_CACHE_VERSION", "metricsRunnerFromUloc"]] },
    { dir: "🎛️|commands/📝️|commit/📊️|metrics/🔀️|deltas", ranges: [["splitGitNumstatDelta", "gitDeltaLineTotal"]], names: ["pathsFromNumstatRow"] },
    { dir: "🎛️|commands/📝️|commit/📊️|metrics/🎨️|format", ranges: [["appendGitDeltaSuffix", "formatUlocMetricsBody"]] },
    { dir: "🎛️|commands/📝️|commit/📊️|metrics/🧮️|summary", ranges: [["countUnifiedLocUnderPathPrefixes", "validateCommitMetricsDeltaSum"]] },
    { dir: "🎛️|commands/📝️|commit/🔣️|emoji-presentation", ranges: [], names: ["EMOJI_PRESENTATION_SELECTOR_RE", "normalizeEmojiPresentation"] },
    { dir: "🎛️|commands/📝️|commit/👤️|contributor", ranges: [], names: ["MicroCommitLevel", "Contributor", "CurrentBranch", "currentBranch", "branchValidationError", "branchAllowed", "findContributor", "loadLevel"] },
    { dir: "🎛️|commands/📝️|commit/🔢️|counter", ranges: [], names: ["COUNTER_RE", "BUNDLE_TAG_RE", "NUMERIC_COUNTER_RE", "pad2", "pad3", "COUNTER_LOG_DEPTH", "canonicalWipLine1Base", "extractCounterFromSubject", "extractNumericCounterFromSubject", "line1BaseFromBundleTag", "contributorWipLine1Base", "bumpCounterFromHistory", "bumpCounterFromSubject", "nextCounter", "formatSecond"] },
    { dir: "🎛️|commands/📝️|commit/🔸️|bullets", ranges: [], names: ["preparedBulletsPath", "EMOJI_LEAD_RE", "MICRO_COMMIT_BULLET_RE", "TIMESTAMP_LINE_RE", "RESERVED_BULLET_LEAD_EMOJIS", "bulletLeadEmoji", "formatMicroCommitBulletLine", "MICRO_COMMIT_METRIC_ROW_RE", "isCommitMetricLine", "isMicroCommitUlocLine", "normalizeBulletLines", "validateBulletSpacing", "bulletEmojiValidationError", "validateBulletEmojis", "writePreparedBullets", "readPreparedBullets", "STAGED_CHANGE_AREAS", "isInsignificantStagedPath", "pathTokensForBulletCoverage", "bulletsMentionPathTokens", "bulletsCoverArea", "uncoveredStagedAreas", "validateBulletsAgainstStaged", "readDiffBulletsInput", "listCachedPaths", "listAddedTicketPaths", "TICKET_JSON_RE", "ticketBullets"] },
    { dir: "🎛️|commands/📝️|commit/📄️|prepared-message", ranges: [], names: ["digestMicroCommitMessage", "preparedDigestPath", "preparedActivePath", "markPrepareActive", "isPrepareActive", "GK_TEMPLATE_BASENAME", "GK_COMMIT_TEMPLATE_FILE", "GIT_COMMIT_DRAFT_FILES", "buildMicroCommitMessage", "writeMicroCommitTemplates", "shouldRefreshPreparedCommitMessage", "removeGitDirPrefixed", "removeGitKrakenTemplateFiles", "resetGitCommitTemplateState", "clearGitCommitDraftState", "clearMicroCommitTemplatesOnly", "clearStaleTemplatesBeforePrepare", "wipeAfterCommit", "handlePrepareCommitMsg"] },
    { dir: "🎛️|commands/📝️|commit/🪝️|hooks", ranges: [["MICRO_COMMIT_BUN_PIN", "installMicroCommitGitHooks"]], names: ["MICRO_COMMIT_POST_WIPE_HOOKS"] },
    { dir: "🎛️|commands/📝️|commit/🪙️|micro-commit", ranges: [], names: ["resetMicroCommitTemplates", "emitPrepareStdout", "stageMicroCommitChanges", "runMicroCommit"] },
    { dir: "🎛️|commands/📝️|commit/🪜️|steps", ranges: [["CommitLevel", "CommitSteps"], ["parseCommitSteps", "isCommitPrepareOnly"]] },
    { dir: "🎛️|commands/📝️|commit/📋️|prepare-commands", ranges: [["shSingleQuote", "formatCommitPrepareAgentReply"]] },
    { dir: "🎛️|commands/📝️|commit/🧾️|bundle-model", ranges: [["CommitBundleDateSection", "BUNDLE_DATE_SECTION_RE"]], names: ["GitDeltaSum"] },
    { dir: "🎛️|commands/📝️|commit/🏷️|labels", ranges: [["EMOJI_CLUSTER_RE", "LABEL_TOKEN_BLOCKLIST"], ["leadingEmojiClusterCount", "bundleScopeLabelError"]], names: ["isBundleScopeLine"] },
    { dir: "🎛️|commands/📝️|commit/🔖️|tags", ranges: [["findLastBundleWipCommit", "formatBundleSubject"]] },
    { dir: "🎛️|commands/📝️|commit/🧭️|path-prefixes", ranges: [["changedPathsInRange", "assignChangedPathsToBundles"], ["buildBundlePathPrefixSets", "pathMatchesBundleIndex"]], names: ["resolveBundleIndicesForNumstatRow"] },
    { dir: "🎛️|commands/📝️|commit/📅️|dates", ranges: [["BUNDLE_COMMIT_TIMESTAMP_LINE_RE", "extractBundleDateLineFromCommit"], ["normalizeBundleDateLine", "formatBundleDateLine"]] },
    { dir: "🎛️|commands/📝️|commit/📈️|attribution", ranges: [["addGitDeltaSums", "addGitDeltaSums"], ["BundleDateDeltasMap", "formatBundleHeaderLine"]], names: ["sortCommitBundlesByEditTotal", "formatGitDeltaSumBrief", "assertGitDeltaSumsEqual", "rangeGitDeltaTotal", "partitionRangeDeltasByBundle", "partitionRangeSizeDeltasByBundle", "validateBundleDayDeltasAttribution", "validateBundleCommitAttribution"] },
    { dir: "🎛️|commands/📝️|commit/📜️|body", ranges: [], names: ["validateBulletLine", "parseCommitBundleBody", "commitBundleBodyError"] },
    { dir: "🎛️|commands/📝️|commit/📤️|emit", ranges: [["COMMIT_DIFF_MAX_BYTES", "emitCommitAnalysis"]] },
    { dir: "🎛️|commands/📝️|commit/🔀️|run-commit", ranges: [], names: ["buildCommitMessage", "readBodyInput", "assertCleanWorktree", "emitStdout", "runCommit"] },
  ],
};
if (import.meta.main && process.argv.length === 2) {
  const a = analyze(plan.source);
  const built = build(plan, a);
  for (const b of built) console.log(String(b.text.split("\n").length).padStart(5), b.file.slice(L.length + 1));
  console.log(JSON.stringify(moduleGraph(built)));
}
if (import.meta.main && process.argv[2] === "--preview") {
  const { mkdirSync, writeFileSync } = await import("node:fs");
  const { dirname, join } = await import("node:path");
  const a = analyze(plan.source);
  const built = build(plan, a);
  for (const b of built) {
    const out = join(process.argv[3], b.file);
    mkdirSync(dirname(out), { recursive: true });
    writeFileSync(out, b.text);
  }
}
if (import.meta.main && process.argv[2] === "--surface") {
  const { exportMap } = await import("./exports.ts");
  const { surfaceOf } = await import("./split.ts");
  const a = analyze(plan.source);
  const built = build(plan, a);
  const { map, warnings } = surfaceOf(plan, a, built, exportMap);
  console.log("names", map.size);
  for (const w of warnings) console.log("WARN", w);
}
export const patches: { dir: string; from: string; to: string }[] = [
  { dir: "🚨️|policy/🖥️|cli", from: 'resolve(__dirname, "📦️packages")', to: 'resolve(__dirname, "../../📦️packages")' },
];
if (import.meta.main && process.argv[2] === "--write") {
  const { seg } = await import("./split.ts");
  const a = analyze(plan.source);
  const built = build(plan, a);
  for (const p of patches) {
    const b = built.find((x) => x.mod.dir === p.dir)!;
    if (b.text.split(p.from).length !== 2) throw new Error(`patch anchor not unique: ${p.from}`);
    b.text = b.text.replace(p.from, p.to);
  }
  write(built, false);
  const { writeFileSync } = await import("node:fs");
  writeFileSync(process.argv[3], built.map((b) => b.file).join("\n") + "\n");
  console.log("written", built.length);
}
