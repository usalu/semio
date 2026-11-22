# Result E68 (tests-caching-C)
Runs: `repo test-file` one file at a time (load 34-42). Paths relative to `C` (⚡️caching) unless noted. Claims in `claims-tests.md`, landing in `T/landings.md`, open product issues in `findings-E68.md`.

| file | before | after | cause / fix |
|---|---|---|---|
| 🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/🟦️.ts | 3/3 | 4/2 | (1) static owner command test: the native-caller schema now requires `codeRoot`; the test capabilities lacked it (added `codeRoot:root`, test fix). (2)+(3) dispatcher and nextest tests run a real `repo run` and need the graph service; it cannot start ("Closed compiler builtin-read import coverage refused" in `repo service host status`, plus 150 s whole-graph reads that exhaust the Original Script deadline) = environment/product, not a test defect. Still red. |
| 🧪️tests/🧾️cargo-provenance/🟦️.ts | 4/2 | 6/0 | scratch is inside the checkout, so Cargo picked up the repo `.cargo/config.toml` (`rustc` = compiler ingress). Both Cargo spawns now set `CARGO_BUILD_RUSTC=rustc`, `CARGO_BUILD_RUSTC_WRAPPER=""`. |
| 🧹️pruning/🏪️budget/🧪️tests/🟦️.ts | 11/1 | 12/0 | fixture target names lacked the `cache-` prefix (`cache-retire-inactive[-preview]`); extracted-function harness stubs unknown names, so `machinePathV1` is now a real port. |
| 🗂️suites/🧪️tests/🟦️.ts | 3/2 | 5/0 | route table: new test `🩹️patches/🧪️tests/🔭️loader` was unrouted (new suite `patch-loader-processes`, plus target `test-patch-loader-processes` in `📋️project.json`); `ledger/🙈️ignored` and the process-spawning files of `tools-authorizers`, `tools`, `bootstrap-dependencies` moved to `<suite>-processes` suites (300 s budget class, same platforms). |
| 🦀️cargo/📦️units/🧪️tests/📥️source/🟦️.ts | 0/1 | 1/0 | converted from the checkout Original (`receiveScriptProcessInvocation(process.env)`) to `repository.invoke(...,{maximumElapsedMilliseconds:110000})`; the real rustc dep-info oracle is unchanged. |
| 🚀️bootstrap/🛠️tools/🔁️probe/🧪️tests/🟦️.ts | 2/1 | 3/0 | product adds optional `SEMIO_NX_AUTHORIZER_ROOT` (newest authorizer bundle); test strips it and asserts it is absent or absolute. |
| 🚀️bootstrap/🔌️nx-plugin/🧪️tests/🦀️native-dependency-closures/🟦️.ts | 0/1 | 1/0 | extracted function now calls `readCargoManifestRoleV1`; supplied as plain-TOML port; the Git-HEAD oracle comparison is unchanged. |
| 🧪️tests/📋️values/🧪️tests/🟦️.ts | 3/1 | 4/0 | `library/🟨️.d.mts`: `projectBuildTargetForNxV1(target,name?)` (runtime has the second parameter). |
| 🦀️cargo/🗃️unit-cache/🧹️gc/🧱️units/🧪️tests | 13 pass | 13 pass | already green when I ran it. |
| 🧪️tests/🗿️artifact-source/🟦️.ts | 0/1 | 0/1 | ticket env no longer required (uses `SEMIO_TEST_ARTIFACT_DIR`); next failure "Exact Original workspace required" = Original/Nx-namespace cluster (E57). |
| 🦀️cargo/📦️units/🏗️compiler/🔏️code/🧪️tests/📖️reader | 1/2 | unchanged | product: closure 408 files vs schema const 384; I did not raise the pin (findings-E68). |
| 🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🧭️closure-law | 3/1 | unchanged | product: undeclared computed import sites in kernel closures (findings-E68). Test takes 3+ minutes. |

## E49's arguments file, analysed
Static owner command `toBe`: the first red was actually the missing `codeRoot` in the test's native-caller capabilities (fixed). Dispatcher "capacity sized at 46%": not reproduced; the capacity line now reads "admitted run"; the failure is the graph service not answering. Nextest filter: same cause, and when the service does run, the whole-graph read exceeds the Original Script deadline (155 s measured).

## NOT verified
- No full pass of the 111 red files; only the files above were run. Not touched (claimed by E57/E59/E69 or not reached): Original-required, Nx-namespace, harness-drift ratchet, `🧹️pruning/🏪️budget/📜️history`, `🔏️authorizers/🧪️tests/📥️scope`, `🧪️tests/🎮️playground-preparation`, `🗂️suites` aggregate targets for `repository-exact` and `bootstrap-original-invocation` (their project targets were missing when I looked, they now pass in the suites test so a peer added them).
- Moved suites were only checked by the route-table test, not run through the real aggregate `repo test`.
- The suites edit moved files out of 15 s suites into 300 s `-processes` suites; Windows is kept where the origin suite had it.

# Round 2 (16:00-16:45)
| item | before | after | what |
|---|---|---|---|
| `📖️reader` | 1/2 | 2/0 | not accidental: 390 of 393 files come through the one import of `🏪️store` (`withRepositoryCacheStoreV1`); re-pinned 384 -> 416 in schema (const + description with justification) and corpus. |
| `🔏️authorizers/🧪️tests/📥️scope` | 2/2 | 3/0 | law pinned `withBuildTestInvocationV1` once per fixture test; one fixture now uses `repository.invoke`; the law accepts either. |
| `🎮️playground-preparation` | 3/1 | 4/0 | the Original envelope (30 s) is exhausted by the whole-repo Cargo walk at load 50; 120 s envelope, 150 s test timeout. |
| `📜️history` | 1/2 | 1/2 | next failure: ENOENT scandir of `.cache/b/t/m/rc/<digest>` in the private workspace tools partition (the code reader's receipt directory is not prepared by the private repository). Not fixed. |
| `📤️arguments` e2e | 4/2 | 4/2 | service starts now; first blocker was the resident `PlanRequest.arguments` schema (`minLength 1`) refusing the literal empty argument that the arguments schema admits: fixed (minLength removed). Remaining: dispatcher test timed out at 180 s, nextest test hits "Original Script process deadline exhausted" (whole-graph read 150 s+ under load 40-50). Needs an idle machine or a smaller graph slice; not a defect found. |
| `🧭️closure-law` | 3/1 | 3/1 | 12 of 14 undeclared owners resolved. Product changes: `project-records` + `entry-lane` import the lazy modules with literal specifiers (typed by new `🧬️source-analyses/🟨️.d.mts` and `scriptProviderIdsV1` in the script-inputs d.mts); `yieldingV1` moved to `🛡️resilience/⏳️yielding` (journal no longer pulls the replica self-check, resilience re-exports it); 10 rows declared in `ComputedImportSites` (2 native-addon, 3 package bases, `bun:jsc` x3 sites as E57's computed specifier = scheme with roots [], farm worker with its service as root, capture-scheme module loader). Still red: (a) `🤝️shared-worktree/🛡️resilience` is still reached via `🩺️attest` <- `🗜️compaction` <- `🌡️host-capacity/🧹️disk-recovery/🔌️real` <- `host-capacity` <- project farm: `attest` needs `importClosureV1`, `moduleSourcesV1`, `restartJitterMillisecondsV1`, `StoreStalledV1` from resilience; split those four out like `yieldingV1`, or declare the resilience site. (b) `🚨️policy/🏃️runner` (`import(href)` of every policy script) via `🎛️commands/🔧️setup` <- `🧭️entrypoints`: needs the 10 policy scripts as roots or a separate non-kernel entry; owner decision, not guessed. |

NOT verified: the moved/yielding module under the replica supervisor after restart; closure-law after the two remaining owners; `🛡️resilience` yield tick test is load-flaky (`ticks > 20` in 300 ms at load 50).
