import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { TICKET } from "./targets.ts";
import type { Decision } from "./decide.ts";
const G = join(TICKET, "🗑️generated", "uncached-outputs");
const d: Decision[] = JSON.parse(readFileSync(join(G, "decisions.json"), "utf8"));
const manifests = readFileSync(join(G, "manifests.txt"), "utf8").trim().split("\n");
const n = (f: (x: Decision) => boolean) => d.filter(f).length;
const esc = (s: string) => s.replace(/\|/g, "\\|");
const rows = (xs: Decision[]) => xs.map((x) => `| \`${x.id}\` | ${esc(x.reason)} |`).join("\n");
const md = `# result-uncached-outputs: explicit outputs for the 809 cacheable-without-outputs targets (2026-10-20)

Executor \`uncached-outputs\`. Static only: no \`bun repo\` run, no nx graph load, no cargo, no Bazel action, no test run. Inputs: \`brief-1019.md\`, \`bzl-stage-uncached-report.md\` (809 targets in 278 projects, reason \`unbounded\`), \`result-store-out.md\`, \`result-cache-{fw,s,rest}.md\`. \`C\` = the coordinator ticket folder.

## Result: counts by resolution (all 809 accounted for)

| Resolution | Targets | Manifest edit |
|---|---|---|
| verdict-only, explicit \`outputs: []\` | ${n((x) => x.resolution === "verdict")} | \`"outputs": []\` added |
| - artifact-rust standard runner (\`test\`: ${n((x) => x.basis === "std:rust-test")}, \`test-snapshot-sqlite[-native\|-source]\`: ${n((x) => x.basis === "std:rust-snapshot")}) | ${n((x) => x.basis === "std:rust-test" || x.basis === "std:rust-snapshot")} | |
| - TypeScript artifact standard runner (\`test\`) | ${n((x) => x.basis === "std:ts-test")} | |
| - custom handler, statically traced (registered class or lifted program, 0 workspace-anchored writes) | ${n((x) => x.basis === "handler")} | |
| - custom handler, hand-read (generators' check twins, stdout-only inventories, repo laws, flagged by the analysis) | ${n((x) => x.resolution === "verdict" && x.basis === "hand-read")} | |
| writes undeclared files / not a function of declared inputs: \`cache: false\` | ${n((x) => x.resolution === "uncache")} | \`"cache": false\` set |
| outputs are injected by project-assembly from a taxonomy generator contract: manifest unchanged | ${n((x) => x.resolution === "contract")} | none (see below) |
| target no longer exists (report is stale) | ${n((x) => x.resolution === "stale")} | none |
| exact non-empty \`outputs\` list | 0 | none found: every file-writing target is a generator (contract-injected), a store publisher (already receipted by store-out) or argument-dependent |
| **Total** | **${d.length}** | 795 + 9 edits in ${manifests.length} manifests; dangling \`dependsOn\` fixed (1 removal) |

Static recount (\`bun C/bzl-stage-tools/graph.ts\` reconstruction with the harness' own \`effective\` cache rule and policy defaults, \`cache === true && !Array.isArray(outputs)\`): 809 unbounded before, **4 after**: the 3 contract-injected generators below plus \`@leutwiler/realparts-of-powers-z-n:build\`, a \`repo:noop\` aggregate the real adapter always caches (\`noop ? {cache:true}\`), not in the 809.

## How each target was resolved (handler-read evidence)

Script: \`C/uncached-outputs-tools/\` (\`run.sh [--write]\` runs it end to end; every step re-runnable and deterministic, two \`decide.ts\` runs are byte-identical, a second \`apply.ts\` is a no-op).

1. \`targets.ts\` parses the report; each target is resolved to its manifest, \`options.cwd\` and \`bun ./📜️script.ts <command>\`.
2. \`walk.ts\`/\`module.ts\` read the handler: the command word selects the router registration (\`register\`, \`registerLazy\`, inline class) or, for lifted programs and standard runners, the script body; the call graph is followed by name through owner code (barrel re-exports, dynamic imports, namespace imports), foreign command branches (\`segments[0] === "<other>"\`, \`command === "<other>"\`) are stripped, \`bun test <file>\` arguments are followed into the test file. Framework infrastructure is not descended; its calls are recorded.
3. \`flow2.ts\` finds every fs write primitive in the reached functions and taints its path argument by data flow: scratch (\`mkdtemp\`, tmpdir, temp/scratch names) versus workspace-anchored (\`repoRoot\`, \`import.meta.dir\`, \`process.cwd()\`, ...). A target with any workspace-anchored write site, a suspect infra call (build/publish/install/retire/pin/store) or an unclassified program is **not** auto-resolved: it is hand-read (\`overrides.ts\`, one cited reason per row).
4. Standard runners were read once (\`⚡️caching/📦️artifacts/🦀️rust/🟦️.ts\` \`runArtifactRustPackageMain\`: \`test\` = cargo test in the leased compiler store, \`check\` = cargo check, \`test-snapshot-sqlite\` = cargo \`sqlite_snapshot_\` tests plus \`bun test\` of the listed suites; \`🟦️typescript/🟦️.ts\`: \`test\` = bundle into scratch + products store, tsc, bun suites). The 209 existing bun files they run (29 snapshot-sqlite suites, 180 TypeScript artifact suites; 1 listed file is missing, open item 2) were traced too: 5,980 reachable owner functions, 0 workspace-anchored write sites, 5 other sites (4 \`Bun.write(Bun.stdout, ...)\` stdout writes and one \`writeFileSync\` of a suite-local temporary file).
5. Rust test sources (144 artifact directories, \`rustscan.ts\`/\`rustgate.ts\`): 91 fs-write sites next to \`CARGO_MANIFEST_DIR\`/\`current_dir\`; 68 are gated by an env var or write/bless flag, the 23 ungated ones are \`#[ignore]\` blessing tests (puzzle-5d fill-run, bmp/tiff fixture derivation) or belong to the repo-dashboard journeys (uncached below). No default-run Rust test writes a tracked file.
6. \`apply.ts\` re-reads every manifest right before writing, edits by span (untouched text, inline objects and concurrent edits stay intact), proves \`parse(after) == parse(before) + intended keys\` and re-compares the file before an atomic rename. 0 targets had changed since the report (all 804 edits applied; 0 \`already-declared\`).

Per-target evidence (handler, reachable functions, write sites, spawned processes, reason): \`C/uncached-outputs-tools/evidence.md\` (809 rows).

## Targets set to \`cache: false\` (${n((x) => x.resolution === "uncache")}; the schema has no reason field, none was invented)

| Target | Reason |
|---|---|
${rows(d.filter((x) => x.resolution === "uncache"))}

## Contract-injected (manifest unchanged, ${n((x) => x.resolution === "contract")})

\`project-assembly\` (\`generatorOutputOwners\`, assembler line 162) sets \`outputs\` of every \`generatorContracts[*].target\` from the contract's \`outputRoots\`, overriding any manifest list; the static harness in \`C/bzl-stage-tools/graph.ts\` does not run that inference, hence the report. A manifest list would be a second authority that the assembler replaces anyway.

| Target | Contract |
|---|---|
${rows(d.filter((x) => x.resolution === "contract"))}

Their check twins (\`check-wgpu\`, \`check-distribution\`, \`check-generated\`) are compare-only and got \`outputs: []\`.

## Stale

\`@semio-tech/layout-js:test-renderer-contract\` and \`@semio-tech/puzzle-js:test-renderer-contract\` no longer exist in the manifests (\`procedural-js\` and \`raster-js\` still define theirs and were resolved).

## Dangling \`dependsOn\` fixed

\`✏️s/🧑‍💻dev/📋️project.json\` \`typecheck.dependsOn\`: removed \`@semio-tech/trinity-jack-lsp:wasm\`. It is not a rename: \`@semio-tech/trinity-jack-lsp-worker\` defines only \`test\`, so repointing it would still be refused by \`requireBazelDependencyV1\`. The Rust crate that produced the \`wasm\` target (\`✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust\`) is deleted at HEAD (auto-commit fd14da55f18; only an untracked \`pkg/\` remains), so there is no producer left to depend on. See open item 1. The three \`semio-hub-gis-component:component\` references in the report are harness artifacts: the crate exists (\`🌎️hub/🧩️compositions/🌍️gis/🧩️component/📦️packages/🦀️rust/Cargo.toml\`), its project and \`component\` target are inferred from Cargo.

## Hand-read verdicts (${n((x) => x.resolution === "verdict" && x.basis === "hand-read")})

| Target | Evidence |
|---|---|
${rows(d.filter((x) => x.resolution === "verdict" && x.basis === "hand-read"))}

## Changes

- Edited: ${manifests.length} \`📋️project.json\` manifests (795 \`"outputs": []\`, 9 \`"cache": false\`, 1 \`dependsOn\` removal). New tool folder \`C/uncached-outputs-tools/\` (targets, closure, module, walk, flow2, writes, files, classify, decide, overrides, jsonspan, apply, validate, report, result, rustscan, rustgate, flagged, run.sh, evidence.md). No edits to \`package.json\`/\`bun.lock\`, library code, \`🔣️taxonomy.json\`, \`policy.json\` or \`.vscode/\`.

<details><summary>Edited manifests (${manifests.length})</summary>

${manifests.map((m) => "- `" + m + "`").join("\n")}

</details>

## Verification

| Command | Result |
|---|---|
| \`bun C/uncached-outputs-tools/decide.ts\` (twice, \`cmp\`) | 809 decisions, byte-identical |
| \`bun C/uncached-outputs-tools/apply.ts\` (dry run), then \`apply.ts --write\` | 795 \`outputs-added\`, 9 \`cache-disabled\`, 1 \`dependsOn-removed\`, 278 manifests; per-manifest semantic equality proof held |
| \`bun C/uncached-outputs-tools/apply.ts\` again | 795 + 9 \`already-declared/uncached\`, 0 manifests (idempotent) |
| \`bun C/uncached-outputs-tools/validate.ts\` (repo \`validateJsonSchemaSubset\` against \`🏗️build/🧬️schema/📋️project/🔣️.json\`, plus Ajv as third-party cross-check) | 278 manifests, 0 repo-validator failures, 0 Ajv failures; 804 of 804 editable targets now declare \`outputs\` or \`cache: false\`, 0 unresolved |
| \`bun C/bzl-stage-tools/graph.ts\` static recount of \`cache && !outputs\` | 809 to 4 (3 contract generators + 1 noop aggregate outside the 809) |
| \`bun build --target=bun\` of the tool entry points | ok |
| NOT run | \`bun repo show projects\` / nx graph, real Bazel \`bazelUncachedReportV1\` on the inferred graph (expect the same 3 generators resolved by the assembler), any repo test suite, any target |

## Open items

1. **LSP worker without producer.** \`@semio-tech/trinity-jack-lsp-worker\` (\`🧠️lsp/🟦️.ts\`) still imports \`./📦️packages/🦀️rust/pkg/trinity_jack_lsp.js\`, which only exists as untracked residue because the Rust crate is absent at HEAD; \`📇️catalog.json\` (vite engines) still lists \`@semio-tech/trinity-jack-lsp\`. Owner decision: restore a producer (then re-add its \`wasm\` dependency under the new name) or retire the worker, the engines catalog entry and the \`s-dev\` typecheck import.
2. **Remodel snapshot suite path.** \`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust/📜️script.ts\` lists \`…/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧬️schema/🟦️.ts\`, which does not exist (the schema test lives at \`…/📸️snapshot/🗄️schema/🟦️.ts\`); \`remodel-remodeling-rs:test-snapshot-sqlite\` and \`-source\` fail at the bun run.
3. **Bridge \`test\` dispatch.** \`space-home-rs\`/\`space-space-rs:test-complete-app-document\` run \`bun ./📜️script.ts test\` in the mutation-bridge script whose program only handles \`check\`; \`test\` falls to the exact-coordinates branch and throws. Resolved as verdict (no write path) but it cannot pass until the command is wired.
4. **Central policy alternative.** \`⚡️caching/🔣️policy.json\` \`targetDefaults.test\` has no \`outputs\`, and \`targetDefaults\` match exact names, so \`test-*\`, \`verify-*\` and \`check-*\` variants never inherit \`outputs: []\`; adding \`"outputs": []\` to \`test\` would cover only the 255 targets named exactly \`test\`, not the 142 \`verify-*\` or the \`test-*\` variants; manifest-level declarations (this change) stay explicit and tool-neutral.
5. **TypeScript artifact \`test\`** (${n((x) => x.basis === "std:ts-test")} targets) runs \`context.build()\`, which also publishes the package into the products store and binds the \`dist\` links; declared \`outputs: []\` like project-assembly does (\`policy.outputs = []\` for artifact \`test\`), because \`build\` owns the receipt and publication is content-addressed and idempotent. A cache hit does not re-bind the links; consumers depend on \`build\`.
6. **Stdout products.** \`list-mutations\` (7), \`workspace:test-discover\` and the taxonomy/dependency verifiers print their result; nx replays terminal output on a hit, the Bazel adapter's replay of stdout for cached actions was not checked.
7. **Test sinks.** Several tests write scratch under \`SEMIO_TEST_ARTIFACT_DIR\` or the ignored \`.🧬semio/🦑️repo/⚡️cache/🧪️tests/<route>\` and \`🤖️generated\` folders (\`repoTestArtifactEnvironment\`, \`s-native-ownership\`, \`repo-lib:test-inventory-artifact-shards\` whose default points at a ticket \`🗑️generated\` folder). They are discarded sandbox output, not products; the Bazel adapter must provide the variable (\`test-catalog-publication\` throws without it).
8. **\`workspace:verify-typescript\`** keeps \`outputs: []\` although it writes the generated tsconfig and tsc logs under \`.cache/semio/typescript\` as byproducts; its sibling \`verify-typescript-project\`, whose only product is that file, is uncached.
`;
writeFileSync(join(TICKET, "result-uncached-outputs.md"), md);
console.log("written", md.length);
