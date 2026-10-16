# Result: Root `📜️script.ts` split into domain modules

Slice ROOT-SCRIPT-SPLIT (prefix `rss`), Sonnet executor, 2026-10-16. `F` = this folder (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`). `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Tools, logs and sandboxes: `F/🗑️generated/rss-*` (tools in `rss-tools/`).

## Verdict

DONE with two caveats. `📜️script.ts` went from 25,739 to 166 lines; the command logic lives in 56 plain `🟦️.ts` modules in the repo library taxonomy; no importer of the root script is left; no production gate imports a test module. Evidence (section 7): the CLI surface is identical in old-versus-new runs (31 of 31 fast commands, every slow command that runs at all), the `tsc` error set is unchanged, the bun failure sets are identical or smaller, 79 of the 82 comparable policy rules return identical rows, all launch JSON files parse. Caveats: (1) the interactivity lanes other than `apps` and `history-closure`, `verify layering`, and the raw-spawn and null-budget rules cannot run on this tree (existing defects, identical before and after), so they are covered by the type-check and the module-load check only; (2) the new folders are not registered in the taxonomy (same unresolved class as their siblings).

## 1. What was split

`📜️script.ts` had 25,739 lines and 1,042 top-level statements: 98 imports, 19 command classes (`NativeOsScript`, `SetupScript`, `DevScript`, `GenerateScript`, `LintScript`, `VerifyScript`, `FormatScript`, `TestScript`, `BenchScript`, `StdioScript`, `PublishScript`, `MicroCommitScript`, `CommitScript`, `OsScript`, `SemioScript`, `ExamplesScript`, `SchemaScript`, `ComponentsScript`, `ReleaseScript`), the tool-job coverage audit (about 6,000 lines), the interactivity audit (about 4,900 lines), the Wave 4 plugin consistency policy (about 10,700 lines), the stdio ledger, the router, the `policy` lint export and the CLI entry. 59 test and tooling modules imported names from it, and it imported 36 `*SelfTests` functions from test modules, so the dependency ran both ways.

Now `📜️script.ts` has 166 lines and keeps what AGENTS.md assigns to a script: CLI parse and dispatch. That is the router (`const router = new ScriptRouter(...).register(...)`, unchanged apart from `RootVerifyScript`), the `policy` lint export that `runPolicyScript` loads, the self-test lane registry (section 4) and the entry block. All command logic lives in 56 plain `🟦️.ts` modules; no other script file exists and no module is a re-export barrel. Nothing imports `📜️script.ts` any more (`rss-tools/importers.ts` finds 0 importers in 1,357 files that mention it).

## 2. Baseline (before the first edit)

- CLI: `bun ./📜️script.ts --help` has no help. It exits 1 with `unknown command "--help"; usage: bun ./📜️script.ts <repo|os|components|semio|examples|setup|agents|dev|generate|scale-fixture|new|schema|lint|verify|format|test|bench|stdio|build|release|cpp|publish|clean|micro-commit|commit> [args…]`; that line, the per-verb usage errors and the read-only verbs are the surface that must not change (`rss-cli.before.cli-list-fast.log`).
- Type-check: `tsc -p rss-tools/tsconfig.rss.json` (root script plus the 59 importers and their closure, 31 s): 38 lines of existing errors, all in importer tests (`workspace-contract`, `path-emoji-statutes`, `outcome-law-gate`, `tool-job-latest-wins`), none in the root script (`rss-tsc.before.log`).
- bun tests that import the root script (8 files): 44/0, 0/1, 25/0, 10/5, 0/4, 3/1, 6/0 and `workspace-contract` 619 pass / 137 fail / 83 skip (`rss-tests.before.log`). Tests that read the root script text (17 files): `rss-tests2.before.log`.
- Existing breakage the split must not be blamed for: `bun ./📜️script.ts policy` fails with `[taxonomy] no declared Rust package entry exists below ✏️s/🧑‍💻dev/🌊️flow/📦️packages/🦀️rust`, `stdio ledger` with `ENOENT ... ✏️s/🔌️plugins/🗄️stdio/📇️registry/🔣️.json`, `verify layering` with `Unexpected build argument before --`.

## 3. Taxonomy tree

Command classes follow the existing `<domain>/🎮️command` precedent (`🧼️workspace-cleanup`, `🤖️agent-clients`, `🏗️authoring`) under the repo library `L`. Audit data and policy rules go to the existing `L/⚖️laws` (statutes; sibling of `indexed-generated-output`). Helpers with an existing owner sit beside it: taxonomy filenames under `🧹️normalization/🔣️taxonomy`, Rust source evidence under `🔍️discovery/📖️source-access`, the checkout root under `🗂️workspaces`. Every new folder is `<emoji>️<kebab>` with U+FE0F and a unique emoji among its siblings; the longest repo-relative path is 149 UTF-8 bytes (limit 240).

| Module (`L/` relative) | Lines | Holds |
|---|---:|---|
| `L/🎛️commands/🔧️setup` | 141 | `setup` command family: native bootstrap, repo-local git settings, devcontainer lifecycle and postinstall. |
| `L/🎛️commands/🛠️dev` | 131 | `dev` command family: storybook, storybook-static and the MCP launchers. |
| `L/🎛️commands/🏭️generate` | 109 | `generate` command family: derived config, taxonomy census and plugin-glue inventory. |
| `L/🎛️commands/🧹️code-style` | 27 | `lint` and `format` commands. |
| `L/🎛️commands/🧪️test` | 202 | `test` and `bench` commands: the thin test router and the bench pointer. |
| `L/🎛️commands/✅️verify` | 1539 | `verify` command family: the gate dispatcher over every verification lane. |
| `L/🎛️commands/✅️verify/🚦️gates` | 245 | Census gates behind `verify production-placeholders/docstrings/agnostic/mutation-caps/debug-tags/interface-owners`. |
| `L/🎛️commands/✅️verify/🧟️generated-corruption` | 165 | Generated-corruption census behind `verify generated-corruption`. |
| `L/🎛️commands/✅️verify/🧷️composed-child-refs` | 94 | Composed-child reference asset gate behind `verify composed-child-refs`. |
| `L/🎛️commands/✅️verify/🚨️policy-breach-gates` | 26 | Single-rule stdio-artifact policy gates behind `verify policy-breach <rule>`. |
| `L/🎛️commands/✅️verify/🦀️rust-warnings` | 33 | Plugin-crate and target scope for `verify rust-warnings`. |
| `L/🎛️commands/🗄️stdio` | 47 | `stdio` command family: ledger and structural gates. |
| `L/🗄️stdio-ledger` | 550 | Stdio artifact ledger: definition catalog, support ledger, structural assertions. |
| `L/🎛️commands/🚀️release` | 66 | `publish`, `components` and `release` commands. |
| `L/🎛️commands/📝️commit` | 23 | `micro-commit` and `commit` commands. |
| `L/🎛️commands/🧩️os` | 76 | `os` and `semio` commands: headless OS studio run and the `.semio` file processor. |
| `L/🎛️commands/📚️examples` | 137 | `examples` command: lists and verifies plugin example units. |
| `L/🎛️commands/🧬️schema` | 297 | `schema` command family: generate, check, compile, docs, entries, oracle, test, verify and audit. |
| `L/🧹️normalization/🔣️taxonomy/🏷️filenames` | 41 | Taxonomy filename and option helpers shared by the generate, test and policy lanes. |
| `L/🔍️discovery/📖️source-access/🦀️rust-evidence` | 211 | Rust source evidence readers: production/test source pairs, store roots and comment stripping. |
| `L/⚖️laws/🎯️tool-job-coverage/📋️inventory` | 537 | Tool-job coverage inventory: reserved ids, static rows, publication contracts, app-owned rows, dispositions and proofs. |
| `L/⚖️laws/🎯️tool-job-coverage/🏭️factory` | 359 | Tool-job coverage: concrete factory resolution, microsecond budget laws, proof catalog and reserved routes. |
| `L/⚖️laws/🎯️tool-job-coverage/📤️publication` | 374 | Tool-job coverage: import preparation, dispatch, publication freshness and bounded publication laws. |
| `L/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store` | 1500 | Tool-job coverage: artifact store, envelope and retained-ownership laws per app. |
| `L/⚖️laws/🎯️tool-job-coverage/🛣️routes` | 169 | Tool-job coverage: segmented drain, reserved routes and bounded queues. |
| `L/⚖️laws/🎯️tool-job-coverage/🔒️lifecycle` | 645 | Tool-job coverage: importer inventory, cancellation, close and disposal laws. |
| `L/⚖️laws/🎯️tool-job-coverage/🔁️live-replay` | 1156 | Tool-job coverage: live fixed replay, mounted sessions, retained ownership and the FEM numerical microcursor. |
| `L/⚖️laws/🎯️tool-job-coverage/⚙️fixed-operation` | 419 | Tool-job coverage: fixed-operation registry fixtures and the shared framework action fixture. |
| `L/⚖️laws/🎯️tool-job-coverage/🏗️fem-visual` | 485 | Tool-job coverage: FEM live visual publication law. |
| `L/⚖️laws/🎯️tool-job-coverage/🏁️run` | 442 | Tool-job coverage: retained-command checkpoint laws and the census run behind `verify interactivity tool-jobs`. |
| `L/⚖️laws/🔬️interactivity-audit/🗂️files` | 191 | Interactivity audit: audited file constants, severity, patterns and the allowlist. |
| `L/⚖️laws/🔬️interactivity-audit/🧭️all-app` | 350 | Interactivity audit: metadata-derived all-app discovery ledger. |
| `L/⚖️laws/🔬️interactivity-audit/🔎️scan` | 108 | Interactivity audit: reachability predicates, comment masking and the file scan. |
| `L/⚖️laws/🔬️interactivity-audit/🏁️run` | 172 | Interactivity audit: the run that reconciles findings against the allowlist. |
| `L/⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history` | 460 | Interactivity audit: puzzle-fill tool-run laws, tool-run policy and history closure. |
| `L/⚖️laws/🔬️interactivity-audit/🖥️mounted` | 1803 | Interactivity audit: mounted reconcile, layout, frame transaction, surface lifetime, prepared render and surface lane sources. |
| `L/⚖️laws/🔬️interactivity-audit/🗄️database` | 1836 | Interactivity audit: prepared raster, shard, store sync, database I/O, catalog, compaction, sync-hello, history, CLI and MCP sources. |
| `L/⚖️laws/🧩️plugin-consistency/🏛️foundation` | 397 | Wave 4 plugin consistency policy: crate discovery, region and function parsing. |
| `L/⚖️laws/🧩️plugin-consistency/📋️allowlists` | 257 | Wave 4 plugin consistency policy: shrink-only allowlists and exemptions. |
| `L/⚖️laws/🧩️plugin-consistency/🏗️crate-shape` | 310 | Wave 4 plugin consistency policy: region format, manifest region, struct naming, mod layout and SDK mechanism rules. |
| `L/⚖️laws/🧩️plugin-consistency/🧱️app-coupling` | 222 | Wave 4 plugin consistency policy: cargo artifacts, app coupling, JSON fixtures and ops grammar rules. |
| `L/⚖️laws/🧩️plugin-consistency/🧮️completeness` | 443 | Wave 4 plugin consistency policy: DSL, pack, command envelope, diff, grammar file, protocol file and protocol migration completeness. |
| `L/⚖️laws/🧩️plugin-consistency/🚧️authority-bans` | 532 | Wave 4 plugin consistency policy: server-only db, OS state authority, pack files, raw spawn, null budget and MCP config bans. |
| `L/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape` | 1135 | Wave 4 plugin consistency policy: structural rules over the Shape V2 taxonomy tree. |
| `L/⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture` | 950 | Wave 4 plugin consistency policy: artifacts-only plugin architecture rules and the APA ratchet. |
| `L/⚖️laws/🧩️plugin-consistency/🪆️subset-conformance` | 510 | Wave 4 plugin consistency policy: subset conformance and artifact viewer and editor rules. |
| `L/⚖️laws/🧩️plugin-consistency/✍️handcrafted-mutation` | 886 | Wave 4 plugin consistency policy: handcrafted spec scanners and direct mutation policies. |
| `L/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome` | 1027 | Wave 4 plugin consistency policy: mutation outcome, merge policy and diff-only laws. |
| `L/⚖️laws/🧩️plugin-consistency/🚪️artifact-io` | 1490 | Wave 4 plugin consistency policy: stdio artifact catalog, standards subsets and artifact I/O rules. |
| `L/⚖️laws/🧩️plugin-consistency/🧬️schema-overhaul-s2` | 1040 | Wave 4 plugin consistency policy: sniff reality and the schema overhaul S2 rules. |
| `L/⚖️laws/🧩️plugin-consistency/📐️schema-overhaul-pc` | 519 | Wave 4 plugin consistency policy: schema overhaul phase 2 pilot-closer rules. |
| `L/⚖️laws/🧩️plugin-consistency/🧊️dissolved-kernels` | 303 | Wave 4 plugin consistency policy: dissolved kernels and composition rules. |
| `L/⚖️laws/🧩️plugin-consistency/🧼️clean-mechanism` | 671 | Wave 4 plugin consistency policy: clean artifact standard subset mechanism rules and derived config. |
| `L/⚖️laws/🧩️plugin-consistency/🧭️breaches` | 117 | Wave 4 plugin consistency policy: the full breach run over every rule. |
| `L/⚖️laws/🧪️self-test-lanes` | 47 | `SelfTestLanes`, the port type for the 38 self-test lanes (section 4) |
| `L/🗂️workspaces/📍️checkout-root` | 8 | `WORKSPACE_ROOT`, resolved from the module's own location with `findWorkspaceRoot`, for the 12 tests that used the root script's constant |
| `L/🧪️tests/📎️root-script-sources` | 9 | test support: `rootScriptSources`, the text of the root script and of every module it owns (used by the source-text readers in section 6) |

Largest module: `🔬️interactivity-audit/🗄️database`, 1,836 lines; every module is below 2,000 lines (the brief allows about 2,500). Module sizes are measured with `table.ts`; they include the header and the imports.

## 4. Wrong-direction imports

- **Root imports tests.** The 36 imported `*SelfTests` functions plus 2 lazily loaded ones are hostile-mutation lanes that `verify` runs. No gate imports a test module any more. `SelfTestLanes` (`L/⚖️laws/🧪️self-test-lanes`) is the port; `VerifyScript` is abstract with `protected abstract readonly lanes: SelfTestLanes`; `toolJobCoverageRun`, `interactivityAuditRun` and `interactivityAuditScan` take `lanes` as a parameter; the composition root `📜️script.ts` builds `selfTestLanes` and binds it with `class RootVerifyScript extends VerifyScript`. The two lanes that were lazy (`toolJobCoverageSelfTests`, `toolJobLatestWinsSelfTests`) stay lazy. The router still registers the string `"verify"` first, so Nx root command inference (`rootCommandTargets`) returns the same four targets before and after. This is the pattern `runDependencyVerification` already used for its two ports.
- **Tests import the root.** All 59 importers (58 test modules and `✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts`) now import from the owning module. Names and owners: `F/🗑️generated/rss-tools/importnames.tsv`. `WORKSPACE_ROOT` (12 importers) comes from `L/🗂️workspaces/📍️checkout-root`; `policyReadFileSafe` from its origin.

## 5. Method

1. Parsed `📜️script.ts` with the TypeScript compiler into one unit per top-level statement, with a checker-backed reference graph (`units.ts`, `deps.ts`; 1,466 edges), grouped the units into 54 modules by line range, and proved the module graph acyclic (Tarjan, `mod-graph.ts`).
2. `gen.ts` writes every module from the live text: verbatim unit text, `export` only where another module, `📜️script.ts` or an importer needs the name, relative paths in `import(...)` and `typeof import(...)` re-relativised, imports rebuilt from the identifiers a module really uses, a header docstring with an emoji unique in the file, `//#region` markers kept. The plan pins the SHA-1 of the source and the tool refuses to apply when `📜️script.ts` changed (it did once: a peer re-pointed one `🎠️kernel` path; I re-pinned and regenerated).
3. Checked the generated tree in memory before applying (TypeScript program with the generated files laid over the real tree, `check.ts`: 0 new errors), then applied with `gen.ts APPLY` and re-ran the real `tsc`.
4. Everything after the apply is hand-made or scripted edits listed in sections 6 and 7; `gen.ts` no longer reproduces the tree.

## 6. Source-text readers repaired, and what the split exposed

Five tests read the root script's text or import it, so they could not follow the code:

| Test (under `L/🧪️tests` unless noted) | Change |
|---|---|
| `⚙️root-script-compiler` | reads `rootScriptSources`; the bun/esbuild transpile check accepts a type-only module (both compilers must agree on empty output) |
| `🧱️owned-script-routes` | scans every root script module for imports and `dispatchOwnedScriptRoute` call sites; finds `VerifyScript` in its module |
| `🎚️vitest-configuration-ownership` | visits every root script module for `runVitest` calls |
| `🥾️cross-platform-bootstrap` | imports `SetupScript` and `REPO_LOCAL_GIT_CONFIG` from `🎛️commands/🔧️setup` |
| `⚡️caching/🧪️tests/⚡️cache-contracts` | reads `SetupScript` from its module |
| fixtures `🧫️fixtures/🧱️root-{artifact-schema-law,clean-scaffold,inference-law,schema-field,surface-abstraction-law,taxonomy-workflow}-source/🔣️.json` | the consumer entry `📜️script.ts` is replaced by the modules that now import each owner (`fix-fixtures.ts`). Seven of the eight owners in `root-schema-field-source` had been dead imports of the root script and are dropped from the consumer list |

Moving code into `🧰️framework` also put it under scans that never saw the root script. Found with a differential over all policy rules and with direct runs; fixed:

- `verify history-closure` and the `history-closure` policy matched their own rule definitions (7 new high-priority findings): `historyClosureSources` now skips the rule module.
- The `no-severity-hint` rules (`policySeverityInfoBreaches`, `policyMutationOutcomeMergePolicyBreaches`) matched their own patterns (5 rows each): `POLICY_MUTATION_LAW_SELF_PATHS = {"📜️script.ts"}` became `policyIsRootScriptSource(relPath)`, true for the root script and every module it owns.
- `policyRawSpawnBreaches` and `policyBudgetNullBreaches` scan `📜️script.ts` files only, so they would have lost all moved code: `policyDiscoverScriptTsFiles` now also lists `policyRootScriptModulePaths`.
- The artifact-name census (`git grep` over `🧰️framework`, no hit now because the files are untracked) would have counted 120 new hits: `⚖️laws` joins `ARTIFACT_NAME_SCOPE_EXCLUDED` in `🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts` (laws name what they police). The folder `🪣️puzzle-and-history` was renamed `🪣️tool-run-and-history` so that its own path stays clean. One hit stays: the verify route `puzzle-fill-policy-self-tests`.

## 7. Verification

All comparisons are old versus new. Logs are `F/🗑️generated/rss-*`.

| Check | How | Result |
|---|---|---|
| Type-check | `tsc -p rss-tools/tsconfig.rss.json` (the root script plus the closure of its 59 importers), before and after | before 38 lines, after 37. Against the baseline: 2 lines gone (a `tool-job-latest-wins` error that a peer fixed with `as const`) and 1 new error, `Property 'declaration' does not exist` inside the peer's in-flight test `✏️s/.../✉️base/📥️source/🧪️tests/🟦️.ts`. That peer changed `semioMemberStoreProducerSourceSelfTests` to `(check, sources, domainSchema)` while I worked, so `SelfTestLanes` now has the matching structural port (`MemberProducerSourcesPort`) and `toolJobCoverageRun` passes the store's own `🧬️schema/🔣️.json` and adapts `toolJobMemberStoreProducerExact`. No error in `📜️script.ts` or in any new module. `tsc` on the five edited tests and the new helper shows no error in any touched line (`cache-contracts` keeps errors in lines I did not touch) |
| Module load | `rss-tools/load-all.ts` imports all 56 new modules | 56 loaded, 0 failed (no TDZ, no cycle at load time) |
| Module graph | Tarjan over the unit-level edges | 0 cycles among the 54 split modules |
| CLI surface, fast set | `dual-cli.sh`: 31 commands run through the original script copy and the new script back to back (no args, `--help`, unknown verbs, every `<verb> bogus`, `generate taxonomy`, `os run`, `components`, `release x`, `verify interactivity bogus`, `verify docstrings bogus`, `verify policy-breach bogus`, `test canvas bogus`, ...), output normalised for stack frames, source excerpts and timings | 31 of 31 identical output and identical exit code |
| CLI surface, slow set | the same harness for `examples list`, `stdio ledger`, `stdio quick`, `generate config --check`, `verify history-closure`, `verify interactivity` with `apps`, `p5d`, `p3mn`, `p1q-b1-b6`, `tool-jobs` and bare, `verify generated-corruption` and `verify composed-child-refs` | identical output and exit code for `examples list`, `stdio ledger`, `stdio quick`, `generate config --check`, `verify composed-child-refs`, and `verify interactivity` with `apps`, `p5d`, `p3mn`, `p1q-b1-b6`, `tool-jobs` and bare; `verify generated-corruption` passes in both and differs only in the scanned-file count (390,955 against 390,975: peers added 20 files between the two runs); `verify history-closure`: the original code run on the new tree reports 8 findings, 7 of them inside the new rule module (it scans the whole tree), the new code reports 1, the same single finding as before the split. Earlier old-versus-new pairs that straddled a peer's `🎠️kernel` move (module not found in one of the two runs) were repeated and are not counted |
| Lane wiring | `verify interactivity apps` prints `selfTests=45` in old and new; `verify history-closure` prints `self-test passed (34 cases)` in old and new | identical. The other interactivity lanes stop on repo defects that exist in old and new alike (for example `[policy rust evidence] conditional external module original_outcome_tests_host is not canonical`, `copy differs from its production factory contract`, `Original Repo build caller required`), so their lane calls are covered by the type-check only |
| Policy rules | `policy-dual.ts`: for every exported `(repoRoot) => BreachRecord[]` rule, run the original function (copy of the original script) and the new function on the same tree, compare the sorted JSON rows | 82 rules compared on the same tree: 79 identical row sets (41,546 rows); `policyEmojiPrefixBreaches` differed by 2 rows in the first pass (tree churn over a 17 minute run) and is identical when run back to back (4,178 against 4,178); `policyRawSpawnBreaches` and `policyBudgetNullBreaches` raise the same environment error in both versions (`.cache/.../📜️script.ts is symlink`); `policyMutationArtifactEngineBreaches` stops on a peer's half-written ticket directory in both versions; `policyCleanArtifactStandardSubsetMechanismBreaches` PLACEHOLDER_CLEAN. Rows that pointed at the new modules (self-matches): history closure 7, severity hint 5 and 5; all fixed (now 0 rows about new files; `policyMutationOutcomeMergePolicyBreaches` 709 to 704 rows, history closure 8 to 1 row) |
| bun tests, group A (the 8 root importers that are real tests) | `run-tests.sh` before and after | `diff-only-law-gate` 44/0, `diff-only-law-gate/lanes` 0/1, `outcome-law-gate` 25/0, `canonical-error-progress` 10/5, `owned-child-dispatch/admission` 0/4, `schema-registry` 3/1, `rust-policy-source-evidence` 6/0: identical names before and after. `workspace-contract` 619 pass / 137 fail before and 621 / 135 after; every test failing after failed before |
| bun tests, group B (17 tests that read the root script) | `run-tests2.sh` before and after the repairs of section 6 | every test that passed before passes after, with two exceptions that are not code defects: `owned-script-routes` "every real contribution is reachable..." fails when `git ls-files -co` lists a file a peer is moving at that moment (it passed in the earlier run, and fails again only while `🎠️kernel/🫧️transient/📜️script.ts` is deleted but indexed). Four failures disappeared in the graph tests because of peer changes |
| History closure | `verify history-closure` | the rule definitions now live under a scanned root and matched themselves (7 new high-priority findings). Fixed by excluding the rule module in `historyClosureSources`. Now 1 finding (`footprint-hand`, a Rust file), as before the split |
| Artifact-name census | `runArtifactNameCensus` and `artifactNameHitsOfText` over the new modules | 120 hits would appear once the files are tracked (the census uses `git grep`). 119 are in `⚖️laws`, whose audits must name what they police, so `ARTIFACT_NAME_SCOPE_EXCLUDED` now excludes `⚖️laws` (one-line change in `🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts`). 1 stays: the `verify` route name `puzzle-fill-policy-self-tests` in `🎛️commands/✅️verify` |
| Raw-spawn and null-budget rules | read `📜️script.ts` files only | they would have lost the moved code, so `policyDiscoverScriptTsFiles` now also scans `policyRootScriptModulePaths`. Both rules fail on this tree for a reason that predates the split (`.cache/.../📜️script.ts is symlink`), so they could not be compared |
| Docstring emoji | `verify docstrings emoji-unique --files-from` over the new files; `header-check.ts` | every module header opens with an emoji that is unique in its file (two headers were re-picked after the first run missed `*`-style docstrings). The gate still reports 238 repeats in 26 files; they are docstrings carried verbatim from the original file |
| Path emoji statutes | `pathEmojiStatuteFindings` over every new folder and its siblings | 0 findings for a new folder (every finding concerns an older folder) |
| Launch and MCP JSON | parse `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `.vscode/mcp.json`, `.mcp.json`, `.cursor/mcp.json`, `.devcontainer/devcontainer.json` | all parse. No command was added or renamed, so no launch entry changes |
| `bun ./📜️script.ts policy` | run on the new script | same error as before the split: `[taxonomy] no declared Rust package entry exists below ✏️s/🧑‍💻dev/🌊️flow/📦️packages/🦀️rust` |
| `verify layering`, `lint dependency-direction` | both need the Nx caller | fail identically before and after (`Unexpected build argument before --: lint-rust-source-direction`, `Original parent Script process invocation required`), so layering of the new modules was not machine-checked; the only upward edge pattern (repo library importing `💻️os` product modules) already exists in `L/🟦️.ts`, `🎮️playground` and `⚡️caching/🦀️cargo` |


## 8. Losslessness

Method: `rss-tools/lossless2.ts` (line multiset) and `itemset.ts` (declaration set), original text = snapshot `rss-tools/orig-script.ts.txt` (HEAD commit `1bae6115bc5`, blob `4b5338229b2`, plus the one line a peer changed at 20875, the `🎠️kernel` merge-outcome path; the snapshot is deleted at the end, see section 9).

- **Items.** 941 top-level non-import declarations before, 947 after. Lost: `BUN` (a `process.execPath` alias that nothing used), `toolJobCoverageSelfTestsModule` (replaced by the `toolJobCoverageSelfTests` lane) and `POLICY_MUTATION_LAW_SELF_PATHS` (replaced by `policyIsRootScriptSource`). Gained: `selfTestLanes`, `RootVerifyScript`, `SelfTestLanes`, `policyBreaches`, `HISTORY_CLOSURE_RULE_SOURCE`, `POLICY_ROOT_SCRIPT_OWNER_DIRECTORIES`, `policyRootScriptModulePaths`, `policyIsRootScriptSource` and `WORKSPACE_ROOT` in the checkout-root module (the root script keeps its own `WORKSPACE_ROOT`). `policy` stays in `📜️script.ts` as a one-line wrapper over `policyBreaches`. The 54 unused imported names of the original (for example `getMapCacheDir`, `inspectRustModuleGraph`, `renameSync`) are not carried over.
- **Lines.** 25,740 original lines against 26,253 lines in `📜️script.ts` plus the 56 new modules. Only in the original: 282 lines = 59 import lines + 163 lines of the trailing `export { ... }` list (no importer needs it any more) + 60 edited lines (`BUN`, the `NATIVE_BOOTSTRAP_DIR` join, the dropped `export { Script }`, `class VerifyScript` header, the lane call sites, the `policy` head and tail, the two lane signatures, `.register("verify", ...)`). Only in the new files: 795 lines = 390 import lines + 50 blank lines + 355 others (56 header docstrings of 3 lines, the lane registry, `SelfTestLanes`, the edited-in counterparts of the edited lines above, 6 synthetic region markers, the discovery and exclusion code of section 6). Everything else is byte-identical text, moved verbatim.
- **Edits inside moved code** (all listed here): `lanes.`/`this.lanes.` prefixes on 50 self-test references in `toolJobCoverageRun`, `interactivityAuditRun`, `interactivityAuditScan` and `VerifyScript`; the `lanes` parameter on those three functions; `VerifyScript` is `abstract` with `protected abstract readonly lanes`; `NATIVE_BOOTSTRAP_DIR` is relative and joined with `this.root`; `policy` body became `policyBreaches()`; `historyClosureSources` skips its own rule module; `policyDiscoverScriptTsFiles` also lists the root script's modules; `POLICY_MUTATION_LAW_SELF_PATHS` became `policyIsRootScriptSource`; `export` was added where another module, `📜️script.ts` or an importer needs a name (456 `export` lines in the new files against 111 in the original).
- **Region markers.** The original has 203 marker lines; all 203 are in the new files (0 missing) next to 6 synthetic ones (209). Three umbrella regions (`ToolJobCoverage`, `InteractivityAudit`, `Policy`) straddle modules, so each module that holds only one end gets the opposite marker (`rss-out/synthetic-regions.tsv`). The three regions that were unbalanced in the original (`PolicyRuleHandcraftedSpecP3`, `DirectMutationPolicies`, `PolicyRuleMutationArtifactEngines`) stay unbalanced.

## 9. Not done, risks, hand-offs

- **Not machine-checked:** dependency direction of the new modules (`verify layering` and `lint dependency-direction` cannot run in this environment, before or after); every `verify interactivity` lane except `apps` and `history-closure` (they stop on existing repo defects in old and new); the raw-spawn and null-budget rules (they fail on `.cache/.../📜️script.ts is symlink` in both).
- **Taxonomy registration not done.** The new folders are in the same unresolved class as their siblings (`directory-kind-unresolved` already counts 764 findings under `L`, including `⚖️laws/indexed-generated-output`). Names to register when the library tree is swept: `🎛️commands/{🔧️setup,🛠️dev,🏭️generate,🧹️code-style,🧪️test,✅️verify,🗄️stdio,🚀️release,📝️commit,🧩️os,📚️examples,🧬️schema}`, `✅️verify/{🚦️gates,🧟️generated-corruption,🧷️composed-child-refs,🚨️policy-breach-gates,🦀️rust-warnings}`, `⚖️laws/{🎯️tool-job-coverage,🔬️interactivity-audit,🧩️plugin-consistency,🧪️self-test-lanes}` with their children, `🗄️stdio-ledger`, `🧹️normalization/🔣️taxonomy/🏷️filenames`, `🔍️discovery/📖️source-access/🦀️rust-evidence`, `🗂️workspaces/📍️checkout-root`, `🧪️tests/📎️root-script-sources`.
- **Edits outside my set (one line each, recorded):** `🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts` (`⚖️laws` joins `ARTIFACT_NAME_SCOPE_EXCLUDED`); the five tests and six fixtures of section 6; the import blocks of the 59 importers; the four tests that import `tool-run-and-history` (`interactivity-puzzle-fill-{p4e,run-job,trace}`, `history-closure-policy`).
- **Peer API drift absorbed.** While I worked a peer changed the signature of `semioMemberStoreProducerSourceSelfTests` (new third parameter, structural `check`); the call site had moved into `🎯️tool-job-coverage/🏁️run`, so I followed it there (third argument: the store's own schema JSON). That peer's test still has one type error of its own.
- **Foreign changes seen while working:** a peer is moving `🎠️kernel` modules (`🫧️transient`, `🔄️merge-outcome`, the `🟦️.ts` barrel). It changed one line of `📜️script.ts` (re-pinned and regenerated), broke module loading for a few minutes (`Cannot find module ../../🔨️modules/🎠️kernel/🫧️transient/🟦️.ts`), and left a tracked-but-deleted `📜️script.ts` that makes `owned-script-routes` flaky. The file table overflowed twice (`ENFILE`), which killed my first background baselines; the runs were repeated with fewer processes.
- **Dead imports.** The original imported 54 names it never used; `root-schema-field-source` listed seven of them as "consumers", so the fixture now lists only real consumers.
- **Follow-ups (not done).** `VerifyScript.run` is still a 630-line `if` chain in a 1,539-line class; `🔬️interactivity-audit/{🖥️mounted,🗄️database}` and `🧩️plugin-consistency/🚪️artifact-io` are 1,500 to 1,840 lines each and could split along their `//#region`s; `📜️script.ts` still lists the 36 test imports it injects (one line each).
- **Cleanup.** `rss-out/tree` (the dry-run copy of the repo paths) and the source copies `rss-orig/*` and `rss-tools/orig-script.ts.txt` are deleted when this file is final, because AGENTS.md forbids copying checked-in source into a ticket folder and the deep copy exceeded the path limit. `rss-artifacts` (test outputs) is deleted for the same reason (paths up to 412 bytes). The original is `git show 1bae6115bc5:📜️script.ts` plus the one peer line.

## 10. Tools

`F/🗑️generated/rss-tools/`: `units.ts`, `deps.ts` (analysis), `mod-graph.ts`, `plan.ts` and `plan2.ts` (module plan, trivia moves, lane edits), `gen.ts` (generator: `bun gen.ts <dir>` dry run, `bun gen.ts APPLY`), `check.ts` (overlay type-check), `lossless.ts`, `lossless2.ts`, `itemset.ts`, `header-check.ts`, `fix-tests.py`, `fix-fixtures.ts`, `fix-headers.ts`, `dual-cli.sh`, `policy-dual.ts`, `rule-diff.ts`, `run-tests.sh`, `run-tests2.sh`, `load-all.ts`, `mk-orig*.ts`, `assemble.py` (this file). Logs: `rss-tsc.*`, `rss-tests*.log`, `rss-dual.*`, `rss-policydual*`, `rss-cli.before.*`.
