import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const g = (n: string) => JSON.parse(readFileSync(join(import.meta.dir, "../🗑️generated", n), "utf8"));
const plan = g("cache-fw-plan.json"), b = g("cache-fw-census-before.json"), a = g("cache-fw-census-after.json");
const CL = ["deterministic-cacheable", "deterministic-but-cache-off", "unclassified-cache-off", "continuous", "side-effecting", "test-against-live", "diagnostic-uncached", "aggregate"];
const cnt = (c: any) => { const m: Record<string, number> = Object.fromEntries(CL.map((k) => [k, 0])); for (const t of c.targets.filter((t: any) => t.owner === "🧰️framework")) m[t.classProposal]++; return m; };
const cb = cnt(b), ca = cnt(a);
const flags = (c: any) => { const f: Record<string, number> = {}; for (const t of c.targets.filter((t: any) => t.owner === "🧰️framework")) for (const x of t.flags) f[x] = (f[x] ?? 0) + 1; return f; };
const fb = flags(b), fa = flags(a);
const rows = (xs: any[][]) => xs.map((x) => `| ${x[0]} | ${String(x[2] ?? x[1]).replaceAll("|", "/")} |`).join("\n");
const manual = plan.keptOff.filter((x: any[]) => x[1] === "manual"), orch = plan.keptOff.filter((x: any[]) => x[1] === "orchestrator"), pol = plan.keptOff.filter((x: any[]) => x[1] === "policy");
const byClass = plan.keptOff.filter((x: any[]) => x[2] === "census-class");
const doc = `# result-cache-fw: cache coverage for 🧰️framework project manifests (2026-10-19)

Executor \`cache-fw\`. Static only: no \`bun repo\` run, no nx graph load, no cargo, no activation. Inputs: \`brief-1019.md\`, \`audit-cache-coverage-1019.md\`, \`result-bzl-man.md\`, \`audit-bazel-state-1019.md\`, plus the plugin source (\`🏗️build/🕸️graph/📥️inference/🧱️project-assembly\`, \`🧭️script-inputs\`, \`🕸️dependency-edges\`) to learn what the assembled graph adds on its own.

## Result

Framework scope (1673 targets, 259 manifests with targets, census replica \`cache-fw-tools/census.ts\`):

| Class | Before | After |
|---|---|---|
${CL.map((k) => `| ${k} | ${cb[k]} | ${ca[k]} |`).join("\n")}
| Total | ${Object.values(cb).reduce((x, y) => x + y)} | ${Object.values(ca).reduce((x, y) => x + y)} |

Raw manifest flags (framework): \`cache: true\` 237 to ${a.targets.filter((t: any) => t.owner === "🧰️framework" && t.cache === "true").length}; \`cache: false\` 964 to ${a.targets.filter((t: any) => t.owner === "🧰️framework" && t.cache === "false").length}; \`cache\` absent 472 to ${a.targets.filter((t: any) => t.owner === "🧰️framework" && t.cache === "absent").length}. Flag \`no-inputs\` ${fb["no-inputs"]} to ${fa["no-inputs"]}, \`no-outputs\` ${fb["no-outputs"]} to ${fa["no-outputs"]}, \`cache-off-deterministic\` ${fb["cache-off-deterministic"]} to ${fa["cache-off-deterministic"]}, \`needs-decision\` ${fb["needs-decision"]} to ${fa["needs-decision"]}.

The ${fa["cache-off-deterministic"]} deterministic-but-cache-off and ${fa["needs-decision"]} unclassified-cache-off that remain are deliberate and listed below with reasons (host-dependent infrastructure tests, live journeys, launchers, writers, orchestration drivers, policy-pinned). The remaining \`no-inputs\` are generator-contract targets whose inputs/outputs the assembly derives from \`taxonomy.json\` \`generatorContracts\`, and a few orchestrators.

## Method

1. What the assembly already does (read from \`🧱️project-assembly\`): for every target with an effective \`cache: true\`, the assembler appends the literal import closure of the command script (\`commandSourcesN\` groups); Cargo targets get \`nativeSources\`/\`nativeTestSources\` plus lock inputs and the explicit \`inputs\` are replaced; nx dependency edges are inferred from TS imports, Cargo path dependencies (not dev) and package.json. So manifests must declare what is NOT reachable that way: project/data reads outside the project default, test files spawned by path, tool/toolchain files, env.
2. Static analyzer \`cache-fw-tools/analyze.ts\` (string-literal relative paths in each project's scripts, their import closure, minus the script closure and minus what \`default\` plus the dependency closure already cover) gave per project the extra read roots; they are collapsed to module roots and written once per project as the project-level named input \`reads\` (not into \`default\`, so they do not leak into every dependent through \`^default\`; for Cargo projects whose \`nativeTestSources\` is not owner-defined it is extended with \`reads\` because the plugin replaces explicit inputs there). Rust path dev-dependencies (not edges) are added to \`reads\` too. Print product targets read the whole print product.
3. Target decisions \`cache-fw-tools/decide.ts\` + \`manual.ts\`; edits are span-based (\`jsonspan.ts\`), so untouched text, inline objects and concurrent edits stay intact. Every file was re-read and compared right before writing (0 stale).
4. Inputs written per flipped or implicit target: policy default by name (\`production\`,\`^production\` for build families, else \`default\`,\`^default\`), plus project groups \`testEnv\` (the three \`SEMIO_*\` test env inputs for every \`test*\`), \`cargoToolchain\` (workspace \`Cargo.toml\`, \`Cargo.lock\`, \`.cargo/config.toml\`, \`rust-toolchain.toml\`, \`rustfmt.toml\`) for Rust projects, \`goToolchain\` (\`go.work\`, \`go.work.sum\`) for Go projects, \`reads\`, and \`{dependentTasksOutputFiles}\` where the target consumes a dependency (not tool installers). Owner-authored input lists on formerly cache-off targets are kept and unioned with these (never narrowed). Targets that were already \`cache: true\` with their own inputs were not touched.
5. Outputs: \`[]\` for tests/checks/lint/verify/contract/source/native/oracle families and for Rust \`build\` (artifacts live in the leased compiler store, same accepted boundary as proctor in \`result-bzl-man.md\`); existing declared outputs were kept. Generator-contract targets and their check targets only got \`cache: true\`; the assembly injects their inputs and outputs. In-tree generators (\`generate*\`, \`typegen\`, \`channel-version-generate\`) use \`outputs: []\` following the policy default for \`generate\`; store-publishing builders (\`print:build\`, Rust builds) use \`outputs: []\` for the same reason as the Vite/cargo boundary.
6. Executor conversion requested by the coordinator (message during the run): the 32 framework targets of \`rules-fix-tools/non-repo-script-targets.json\` now carry \`"executor": "repo:script"\` (10 manifests). Only \`semio-framework-os-infinite:{build,check,test}\` called a foreign driver directly; they now call \`bun ./📜️script.ts build|check|test\` and \`♾️infinite/📦️packages/🦀️rust/📜️script.ts\` implements \`BuildScript\` (\`buildAdmittedCargoArtifactsV1\`, as \`pets-rs\`), \`CheckScript\` (\`runAdmittedCargoCommandV1\`, as \`tool-run-rs\`) and \`TestScript\` (\`runAdmittedCargoPackageTestsV1\`, as \`test-wgpu-world-terrain\` there). \`ui-styling-py:deps\` keeps \`bun ../../🏗️builder/🐍️python/📜️script.ts deps\` (already a \`📜️script.ts <command>\` call). Framework now has 0 targets without \`repo:script\`/\`repo:noop\`.

## Targets kept at cache false (schema has no reason field; \`Task\` has no \`additionalProperties: false\` and no neutral reason key, so none was invented)

### Deliberate, explicit \`cache: false\` written or confirmed (${manual.length + pol.length})

| Target | Reason |
|---|---|
${rows(manual)}
${rows(pol)}

### Orchestration drivers left as authored (${orch.length}; children are cached)

| Target | Reason |
|---|---|
${rows(orch)}

### Census classes continuous / side-effecting / live / diagnostic left untouched (${byClass.length}; policy owns them)

${byClass.map((x: any[]) => `\`${x[0]}\` (${x[1]})`).join(", ")}

### Skipped, plugin- or owner-controlled (${plan.skipped.length})

| Target | Reason |
|---|---|
${rows(plan.skipped)}

Note: the plugin forces \`cache=false\` for Rust \`wasm\`/\`wasm-release\` targets (except renderer-wgpu) and for \`role:artifact\` TypeScript \`build\`/\`test\`; raw manifests with \`cache: true\` on those names (e.g. \`framework-editor-rs:wasm\`) are therefore not effective until the plugin changes. \`node-graph-rs:wasm\` was the only one with \`cache\` absent; it now says \`false\`.

## Verification

| Check | Command | Result |
|---|---|---|
| Neutral schema, repo validator | \`bun cache-fw-tools/validate.ts\` (calls \`validateJsonSchemaSubset\` with \`🏗️build/🧬️schema/📋️project/🔣️.json\` on every framework manifest with targets) | 259 manifests, 0 errors |
| Neutral schema, third party | same script, Ajv draft-07 (\`allErrors\`) | 259 manifests, 0 errors |
| JSON well-formedness and no stale writes | \`decide.ts --apply\` parses every result before writing | 256 files written, 0 stale |
| Census replica before/after | \`cache-fw-tools/census.ts\` (copy of \`aud-cachecov-tools/census.ts\`, tolerant of files deleted mid-run) and \`counts.ts\` | table above |
| Infinite script | \`bun build --target=bun ♾️infinite/📦️packages/🦀️rust/📜️script.ts\` | bundles (652 modules); not executed |
| Executor conformance | \`cache-fw-tools/execcount.ts\` | 1673 framework targets, 0 without repo:script/repo:noop |
| Not run | \`bun repo show projects\`, nx graph load, any task execution, cargo, \`--tool=bazel\`, the cache-contract test suite | not run per brief; the assembled inputs (command-source groups, native replacement) are inferred from reading the plugin, not observed |

## Open items

1. First real graph load must confirm: project-level named inputs \`reads\`, \`testEnv\`, \`cargoToolchain\`, \`goToolchain\` survive \`projectInputs\` (the plugin generates its own \`commandSourcesN\` groups; no collisions were found by grep), and no target trips the output/input overlap negation.
2. Projects whose owners define \`nativeTestSources\`/\`default\` without \`{projectRoot}\` (for example \`semio-framework-os-infinite\`) do not consume \`reads\` on native tests; their Rust sources come from compiler dep-info. TS-side test files reached only by path stay unhashed there until the owner includes \`reads\`.
3. \`reads\` is a static string-literal analysis (module-root granularity). Tests that scan directories at run time (source-text laws, workspace walkers; 43 flagged projects in \`aud\` output) are covered only as far as their referenced roots; shared infra scanners (workspaces, native-build, leases) are execution environment, not inputs.
4. Store-published builders (Rust builds, \`print:build\`) and in-tree generators use \`outputs: []\`. A pruned store or deleted untracked generated file is not repaired by a cache hit; consumers re-run the producing step themselves today. Declaring store-aware outputs is the same open item as in \`result-bzl-man.md\` (item 2).
5. Orchestration drivers and the host-dependent infra tests listed above stay uncached; if the Bazel exemption list should be tool-readable, the schema needs a reason field (not added).
6. Bazel adapter still defaults absent \`inputs\` to \`{projectRoot}/**/*\`; all flipped framework targets now have explicit inputs, the 170 raw-absent remainder are contract targets, orchestrators, uncached classes.

## Files changed

256 framework \`📋️project.json\` manifests (cache, inputs, outputs, project-level \`namedInputs\`; list: \`git diff --name-only -- 🧰️framework | grep project.json\`, other fleets also edit there), plus 10 of them for the executor conversion, and \`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts\` (build/check/test commands). Tools: \`C/cache-fw-tools/\` (\`census.ts\`, \`analyze.ts\`, \`decide.ts\`, \`manual.ts\`, \`jsonspan.ts\`, \`validate.ts\`, \`executor.ts\`, helpers).
`;
writeFileSync(join(import.meta.dir, "../result-cache-fw.md"), doc);
console.log(doc.length);
