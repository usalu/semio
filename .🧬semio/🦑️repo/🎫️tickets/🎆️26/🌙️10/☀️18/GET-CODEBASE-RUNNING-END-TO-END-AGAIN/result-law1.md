# Result law1: framework-module source-text law suites (first half of `result-tst2-suites.md`)

Executor `law1` (Sonnet 5.5), foreground only, no sub-agents, no cargo, no modifying git command, no `package.json` / `bun.lock` edit. Scope = the 90 red `bun:test` suites of the framework modules (`🧰️framework/🔨️modules/**` and the two top-level `🧰️framework` suites) in table order; the named domains (job, worker, actor, async, value, pack, process, store-free framework, kernel, time-travel, tool-run, 2d, transient) were worked, the rows `replication, pixels, graph, ui, dsl, io, 3d, schema` (55-79) were only baselined (code `X`, left to the second executor).

Abbreviations in paths: `M/` = `🧰️framework/🔨️modules/`, `F/` = `🧰️framework/`.

## Result

| | before (tst2 final) | after (law1, full file, finite envelope) |
|---|---|---|
| suites green of the 90 | 0 | 30 |
| suites worked (rows 1-54 + 80-90 = 65) | 0 green | 30 green, 35 still red |
| suites not in the named domains (rows 55-79) | 25 red | 25 red (untouched) |

Of the 35 still red worked suites (31 as in the table notes plus rows 1, 3, 12, 14 reverted to red by the ruling; row 49 stays red): 4 missing/wrong packages (F: rows 5, 9, 35, 89), 7 harness/environment (E: 17, 19, 31, 32, 42, 43, 45; 31/32/42 are green per test), 5 cargo/native (H: 20, 21, 36, 37, 38), 10 open owner/model questions (G: 22, 24-27, 34, 41, 46, 80, 87), 4 partially fixed with one residual each (6 unverified rewrite, 39, 40, 49), 1 not examined (48).

## Harness (found, not source defects)

* One `bun test` process per test is the designed execution: `receiveScriptProcessInvocation` may be called once per process (`Original process birth already received`), and an expected failure inside an issued operation poisons the whole Original (`settle` rethrows owner failures, later children get `Original process admission closed`). Files with several `receiveScriptProcessInvocation` tests are therefore red as a file but green per test. Migrating them to `withBuildTestInvocationV1` was tried and reverted (the cancel/refusal laws rely on catching the rejection outside the Original). `law1-tools/iso.py` runs every red test of a file in its own process (`-t "^name$"`); column `iso` = of the tests red in the full file, how many pass/fail alone.
* The process envelope must be finite: `law1-tools/envwire.ts` mints it from `LAW1_MAX_MS` (default 0 = unbounded, as tst2 did) and `LAW1_CAPS` (JSON capabilities). `LAW1_MAX_MS=3600000` makes `process/routing/.../📤️child` green (it asserts `original.deadlineEpochMilliseconds != null`). Capabilities `native` / `buildMode` of the real launcher are not reproducible (`H`).
* `--tsconfig-override <hoist>/tsconfig.json` with the emulated root hoist as in tst2 (`LAW1_HOIST` selects another farm; `hoist-lru` = lru-cache 6.0.0 + yallist 4.0.0 for the two lru suites). Env: `SEMIO_TEST_ARTIFACT_DIR` (short scratch), `SEMIO_TICKET_DIR`, `SEMIO_BUILD_MODE=prototype`, `SEMIO_REPO_ROOT`, `--timeout 120000`.

## Decision classes

A moved/split -> re-pointed (`readModuleTreeSource` / `readSourceFamilySource` / new owner file) | B renamed -> needle updated | C intentional design change -> law rewritten to the new invariant (evidence in the row) | D accidentally lost -> production restored | E harness/environment | F missing/wrong package | G open: owner decision or law ahead of production | H needs cargo / native / launcher | X not in scope.

## Production defects restored (class D)

1. `.vscode/launch.json` (1722 entries) and `.vscode/🧩️launch.seed.jsonc` (1146 entries): every generated per-crate `🦀️ <crate> 🏗️build/🔎️check/🧪️test` entry (groups `4_build/4_gate/4_test`, present already at HEAD) lacked `env` `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` and `NODE_DISABLE_COMPILE_CACHE=1` (law `process/build/🎛️environment`). Inserted after `cwd`, entries with a later `env` block untouched, parse and duplicate-key checked (13802 / 10349 launch entries, 0 bad). The producer of these entries (the monolith-split launch writers) must emit the env, otherwise the next sync regresses it.
2. 20 `repo:script` targets in 14 carved crates (`io/codec`, `pack/source`, `replication/error`, `replication/crypto`, `ui/contract/{identity,budget,.,text}`, `schema/artifact-reference/borrow`, `schema/composition/admission`, `job/worker/authority`, `job/outcome/publication` (+fault), `job/preview/drive`) lost `options.cwd`: `bun ./📜️script.ts test-*` ran at the workspace root, i.e. the wrong script (397 of 417 sibling targets carry the cwd). Restored (cwd = project directory); law `process/routing/command/boundary` first test is green.
3. `M/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`: `runCargoTestsV1` / `runCargoLintV1` dereferenced `admission.control` before the `admission?.control` guard (TypeError instead of `Original Cargo test admission required`); early guard restored in both entries.

## Cross-cutting design decisions applied (class C, please confirm)

* **Reverted per coordinator ruling**: the five `🛍️products/🦑️repo` exemptions (script-boundary, tool-run/ownership, actor context-boundary, actor owner-boundary, routing/command/boundary) were removed again; those laws are red on purpose until the framework scripts delegate through `bun repo ...` / a framework abstraction. Exact list of 75 scripts + 121 other files: `followup-law1-script-product-imports.md` (not implemented: needs cargo to verify). Of the routing/command/boundary edits only the asset-reference stub (`?raw/?url/?inline`) and the `cwd` restoration remain.
* **Hardened process control**: a finite `createIssuedResourceControlV1` request derives its own deadline-bound signal (identity with `original.control.signal` only for 0 ms), `withScriptProcessEnvelope` captures frozen copies of policy and capabilities, forged `ScriptInvocation` objects are refused. Laws `routing/exports`, `routing/finite`, `cleanup/operation` re-stated accordingly.
* Wrapper/Body split of the persistent opener (`openPersistentNativeImageV1` -> `openPersistentNativeImageBodyV1(input, incomingControl, parent, openLifetime)`, same for `openNativeFixedPlatformV1`, `recheckPhysical` -> `checkPhysical+mappingRead+recheckPhysical`): extraction laws now extract the Body functions and bind the new free variables.

## Born-red laws (never passed at any reachable revision)

`pack/value/decode/source` (reads `../🦀️.rs` that never existed; draft-07 Ajv for a 2020-12 schema), `job/session-return` (asserts `release_retirement_slot` never calls `drop(retirement)`: it does in every revision back to 03c902f; compact-formatting anchors).

## Open items for owners

* **job** (`job/work/grant`, 2 laws; `job/budget/corpus-oracles`): the retained-work wiring was never completed at HEAD or in c44e964: `ToolExecutionContract.work_grant` + `with_retained_work` exist (action-bus), `StepContext.retained_work` exists but is always built from `NO_RETAINED_WORK`, `StepBudget`/`BatchDriveConfig` have no `work_grant`, `retained_work_can_enter` / `retained_work_charge` exist nowhere, `retained-command-job` calls `work.step` without quoting `work.work_demands` (~20 literal sites, needs cargo). Separately the plugin no longer derives the worker step budget from `contract.max_step_micros` (only validated; the retained route admits with `fuel_per_step: cx.fuel_remaining(), step_budget_us: 0`): restore or restate the microsecond binding (`toolJobMicrosecondWorkerExact` + mutation table).
* **deps**: `lru-cache` must be 6.0.0 (legacy `new Lru({max,length})` API; working-tree `2d/🧮️compute/package.json` pins 11.5.3 -> 11 red; root lock has 6.0.0); `immer` unresolved (callers law); root hoist lacks `bun-types, undici-types, fast-uri, json-schema-traverse, require-from-string, jsonc-parser` (pack/json absence).
* **law ahead of production** (`💾️cache/🧹️retirement`, 4): the test (7880cc3) names `requireCompilerWorkspaceProducerCustodyV1`, `producerLayout.cwd`, a funded-child guard that exist in no production file.
* **test imports of specific code** (law `routing/command/boundary` #2 stays red): `M/🧪️test/🔌️adapter/🧪️tests` imports `@semio-tech/repo-test/*`, `M/🖼️canvas/🔌️runtime-assets/🧪️tests/📦️bundle-staging` imports the os plugin browser-bundle distribution.
* **process models to restate**: `native-build/inputs/consumers` (loan-child controls done, modeled failures leave the Original), `cargo/execution` (forged invocation), `storage/platform/contract` (`maintenanceMethods()` expects 3 arrows, production has 2), `witness` (2: asset-entry refusal text `Closed fresh standalone declared asset entry required` exists in no tracked file; publisher options `one`), `receipt/retention` (nested graph-drainage child), `publication/owner` (publisher lifecycle `success` row).
* **cargo / native / launcher** (`H`/`E`): `recipe` production test (needs `NativeStorageReservation` + real cargo), `component-pass` (raw `cargo -Zunstable-options` on stable; runStub already migrated to issued controls), `lease/overlap` (compiler ingress `-vV`), `storage/platform/contract/parent` (capabilities.native), `capacity/{custodian,scope}` (spawned children), `leases/inventory` (30 s deadline), `value/neutral-owner` (rewritten workspace layout, unverified: it runs cargo; layout checked statically).

## Per-law decision table

`before` = tst2 final p/f, `after` = law1 full-file p/f (finite envelope), `iso` = for the tests red in the full file, passes/failures when each is run in its own process (process family only; empty = not run).

| # | suite | before | after | iso | class | evidence |
|---|---|---|---|---|---|---|
| 1 | `F/📏️script-boundary/🧪️tests/🟦️.ts` | 1/2 | 2/1 |  | A (law 1) ; ruling (law 2) | fixture roster of 6 new router commands fixed; router bundle law red on purpose: `typecheckRepositoryProjectV1` import from products/repo must be inverted |
| 2 | `M/⏪️time-travel/🧪️testing/📥️values/🧪️tests/🟦️.ts` | 1/1 | 2/0 |  | A | schema pinned inferenceExports to 15; actual roster 16 named + default = 17 (corpus already 17) |
| 3 | `M/⏯️tool-run/🏛️ownership/🧪️tests/🟦️.ts` | 1/1 | 1/1 |  | ruling | cargo script imports repo cargo admission; must be inverted (exemption reverted) |
| 4 | `M/⏳️async/♻️cancel-return/📬️claim/🧪️tests/🟦️.ts` | 1/1 | 2/0 |  | A | plugin facade carved: needles now in typed-operation-unit + ♻️cancellation; negative needle over source family |
| 5 | `M/◻️2d/🧮️compute/🧪️tests/🟦️.ts` | 34/11 | 34/11 |  | F | legacy `new Lru({max,length})` API; package.json (working tree) pins lru-cache 11.5.3, root lock has 6.0.0 -> 45/0 with 6.0.0 |
| 6 | `M/🌱️value/🧪️tests/🧩️neutral-owner/🟦️.ts` | 16/1 | 16/1 |  | A+C | copy-workspace layout rebuilt for 🧰️framework/Cargo.toml root; UNVERIFIED (runs cargo) |
| 7 | `M/🎒️pack/🌱️value/🛬️decode/🧾️source/🧪️tests/🟦️.ts` | 0/1 | 1/0 |  | B+C | born red (read ../🦀️.rs never existed); Ajv2020 for 2020-12 schema; Rust half re-pointed to successor 📖️source RetainedRecordBodySourceDecode |
| 8 | `M/🎒️pack/🔤️json/📥️decode/🧾️receipt/🧪️tests/🟦️.ts` | 3/2 | 5/0 |  | A | step_source/advance live in 🧮️parser-and-grammar-cursor -> readModuleTreeSource |
| 9 | `M/🎒️pack/🔤️json/🧪️tests/🚮️absence/🟦️.ts` | 0/1 | 0/1 |  | F | root node_modules lacks bun-types, undici-types, fast-uri, json-schema-traverse, require-from-string, jsonc-parser |
| 10 | `M/🎒️pack/🚰️source/🧪️tests/🟦️.ts` | 1/1 | 2/0 |  | A | pack-source consumers after db/spr/cli carve (roster updated) |
| 11 | `M/🎠️kernel/🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts` | 1/1 | 2/0 |  | A | `replication` dependency now owned by the carved kernel crate manifest |
| 12 | `M/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🏛️boundary/🟦️.ts` | 1/1 | 1/1 |  | ruling | script imports repo wasm policy; must be inverted (exemption reverted) |
| 13 | `M/🎭️actor/🎟️retained-turn/🤝️admission/🧪️tests/🟦️.ts` | 1/1 | 2/0 |  | C | kernel admission schema added 0c3e789 for 🕸️kernel fixture; negative law re-stated |
| 14 | `M/🎭️actor/🧪️tests/🏛️owner-boundary/🟦️.ts` | 0/1 | 0/1 |  | ruling | script string refs to repo wasm policy; must be inverted (exemption reverted) |
| 15 | `M/🎭️actor/🧬️ownership/🧪️tests/🟦️.ts` | 3/1 | 4/0 |  | A | symbols declared in #[path]-mounted modules -> readModuleTreeSource |
| 16 | `M/🏃️process/🏗️build/🎛️environment/🧪️tests/🟦️.ts` | 1/1 | 2/0 | 0/0 | D | 1722/1146 generated per-crate launch entries lost env BUN_RUNTIME_TRANSPILER_CACHE_PATH/NODE_DISABLE_COMPILE_CACHE; restored in launch.json + seed |
| 17 | `M/🏃️process/🏗️build/🎟️capacity/🛟️custodian/🧪️tests/🟦️.ts` | 0/1 | 0/1 | 0/1 | E | child process hangs/aborts under the test envelope (needs real launcher) |
| 18 | `M/🏃️process/🏗️build/🎟️capacity/🧪️tests/🔐️inheritance/🟦️.ts` | 5/1 | 6/0 | 0/0 | A | schema-first: launcher policy json folded into 🧬️schema Configuration.const |
| 19 | `M/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/🟦️.ts` | 4/1 | 4/1 | 0/1 | E | spawned child scripts end with Original process admission closed |
| 20 | `M/🏃️process/📦️artifacts/🏗️native-build/🎯️recipe/🧪️tests/🟦️.ts` | 2/3 | 3/2 | 1/1 | H | tsc --ignoreConfig fixed (TS 7); production test needs storage reservation + real cargo |
| 21 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/📥️parent/🟦️.ts` | 23/25 | 23/25 | 10/14 | H | needs launcher capabilities.native + cargo children |
| 22 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/🟦️.ts` | 26/34 | 32/28 | 10/17 | G | maintenanceMethods() expects 3 withExclusiveMaintenance arrows, production has 2 (re-derive model); rest = parent suite |
| 23 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests/🔤️ordering/🟦️.ts` | 0/1 | 1/0 | 0/0 | A | second `enumerate` arrow (hardlink walk); selected by entry guard |
| 24 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests/🟦️.ts` | 10/36 | 38/8 | 5/2 | A+G | 4 extraction drifts fixed (schema bindings, duplicate verify, reporter schema); 2 open: asset-entry error, publisher options |
| 25 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🪟️windows/🧪️tests/🟦️.ts` | 11/16 | 11/16 | 14/1 | G | cascade: 14 of 16 red tests pass per test; 1 open (Windows component schema owners), not analysed |
| 26 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🧪️tests/📦️owner/🟦️.ts` | 3/13 | 3/13 | 11/1 | G | cascade: 11 of 12 pass per test; 1 open (publisher lifecycle memory-port model: a `success` row is refused), not analysed |
| 27 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🧪️tests/🟦️.ts` | 16/61 | 17/60 | 55/4 | A+G | aggregates witness+owner; see witness |
| 28 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧪️tests/🟦️.ts` | 8/2 | 10/0 | 0/0 | B+C | openNativeImageStorageV1 delegates to persistent open; withOperation precedes deadline check |
| 29 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/📥️paths/🟦️.ts` | 0/1 | 1/0 | 0/0 | A | wrapper->Body split (openNativeFixedPlatformBodyV1) |
| 30 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/📥️receipt/🟦️.ts` | 1/1 | 2/0 | 0/0 | A | physical recheck split into checkPhysical+mappingRead+recheckPhysical |
| 31 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🧪️tests/🟦️.ts` | 3/1 | 3/1 | 1/0 | E | multi-receive file; designed per-test process (iso) |
| 32 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧪️tests/🟦️.ts` | 3/7 | 3/7 | 7/0 | E+C | multi-receive file; finite request derives distinct signal |
| 33 | `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️receiver/🧪️tests/📥️release/🟦️.ts` | 0/2 | 2/0 | 0/0 | A | open*Body split with openLifetime; bindings extended |
| 34 | `M/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/📥️consumers/🟦️.ts` | 0/2 | 0/2 | 0/2 | G | control refinement edits done; modeled failures leave the Original callback |
| 35 | `M/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🎯️target/🧪️tests/📋️callers/🟦️.ts` | 0/1 | 0/1 | 0/1 | F | tsc: `immer` unresolved in actor activation test |
| 36 | `M/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🏎️overlap/🟦️.ts` | 1/1 | 1/1 | 0/1 | H | needs launcher capabilities.buildMode + compiler ingress (cargo -vV) |
| 37 | `M/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/📥️inputs/🟦️.ts` | 0/3 | 0/3 | 0/3 | H | raw cargo -Z on stable / nightly |
| 38 | `M/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts` | 9/19 | 9/20 | 0/19 | C+H | runStub migrated to issued controls; rest needs cargo nightly |
| 39 | `M/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧪️tests/🟦️.ts` | 11/4 | 14/1 | 0/2 | A | component-pass driver carved (row{resolve,reject}, sessionOwner discard, pending join); 1 nested child case open |
| 40 | `M/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts` | 28/15 | 30/13 | 12/1 | B+C | owner ordering re-anchored; lsof newline f-marker accepted (4dab415); 1 Generated-fixture ENV |
| 41 | `M/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts` | 53/4 | 53/4 | 0/4 | G | law ahead of production: requireCompilerWorkspaceProducerCustodyV1 / producerLayout absent |
| 42 | `M/🏃️process/📦️artifacts/📤️publication/📥️sources/🧪️tests/🟦️.ts` | 16/5 | 16/5 | 5/0 | E | multi-receive file; green per test |
| 43 | `M/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🟦️.ts` | 3/4 | 3/4 | 2/1 | E | 30 s law deadline under host load |
| 44 | `M/🏃️process/🧪️testing/🦀️cargo/🎟️admission/🧪️tests/🟦️.ts` | 0/1 | 1/0 | 0/0 | D+C | admission null-guard restored in runCargoTestsV1/runCargoLintV1; fabricated control refused earlier |
| 45 | `M/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🍎️image.ts` | 0/1 | 0/1 | 0/1 | E | Original process admission closed (poisoned birth in this model) |
| 46 | `M/🏃️process/🧪️testing/🦀️cargo/🏃️execution/🧪️tests/🟦️.ts` | 15/2 | 15/2 | 0/2 | G | law drives executeCargoRequestV1 with a forged ScriptInvocation; hardening refuses it |
| 47 | `M/🏃️process/🧪️testing/🦀️cargo/🧪️tests/📥️preparation/🟦️.ts` | 0/1 | 1/0 | 0/0 | B | withCargoInputs port: args:compiler.arguments; selection owned by framework runner |
| 48 | `M/🏃️process/🧭️routing/🎛️command/🧪️tests/❄️cold/🟦️.ts` | 0/1 | 0/1 | 0/1 | not examined | spawns a repo:script command with specifics removed |
| 49 | `M/🏃️process/🧭️routing/🎛️command/🧪️tests/🚧️boundary/🟦️.ts` | 0/3 | 1/2 | 0/1 | D+ruling | 20 repo:script targets' cwd restored (law 1 green); bundling law needs the same inversion; 3 foreign test imports remain |
| 50 | `M/🏃️process/🧭️routing/📤️exports/🧪️tests/🟦️.ts` | 0/1 | 1/0 | 0/0 | C | finite request derives its own deadline-bound signal |
| 51 | `M/🏃️process/🧭️routing/📥️invocation/🏃️process/📤️child/🧪️tests/🟦️.ts` | 0/1 | 1/0 | 0/0 | E | needs a finite process envelope |
| 52 | `M/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📊️profile/🟦️.ts` | 0/1 | 1/0 | 0/0 | C | debug profiling removed from graph/rust; stale consumer dropped |
| 53 | `M/🏃️process/🧭️routing/📥️invocation/🧪️tests/⏱️finite/🟦️.ts` | 3/1 | 4/0 | 0/0 | C | withScriptProcessEnvelope captures frozen copies |
| 54 | `M/🏃️process/🧭️routing/🧪️tests/🟦️.ts` | 54/2 | 56/0 | 0/0 | A | aggregate of exports/profile |
| 55 | `M/📡️replication/⚠️error/🧪️tests/🟦️.ts` | 3/1 | 3/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 56 | `M/📡️replication/🎮️mutation/🧾️transaction/🎟️admission/🧪️tests/🟦️.ts` | 1/1 | 1/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 57 | `M/🔲️pixels/🧪️testing/🧭️ownership/🧪️tests/🟦️.ts` | 10/1 | 10/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 58 | `M/🕸️graph/🧪️tests/🧩️suite/🟦️.ts` | 22/1 | 22/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 59 | `M/🖱️ui/🌐️locale/🧪️tests/🟦️.ts` | 55/3 | 55/3 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 60 | `M/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🎟️presented-source/🧪️tests/🟦️.ts` | 0/1 | 0/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 61 | `M/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🧪️tests/🟦️.ts` | 12/2 | 12/2 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 62 | `M/🖱️ui/🧠️runtime/🎭️present/📦️resources/📤️transport/📨️message/🧪️tests/🟦️.ts` | 0/1 | 0/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 63 | `M/🖱️ui/🧪️tests/♻️physical-job-close/🟦️.ts` | 7/2 | 7/2 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 64 | `M/🖱️ui/🧪️tests/📐️dock-axis-geometry/🟦️.ts` | 0/1 | 0/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 65 | `M/🖱️ui/🧪️tests/📦️prepared-close/🟦️.ts` | 3/1 | 3/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 66 | `M/🖱️ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🟦️.ts` | 4/1 | 4/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 67 | `M/🖱️ui/🧪️tests/🧊️feature-ownership/🟦️.ts` | 0/0 | ?/? |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 68 | `M/🖱️ui/🧪️tests/🧭️native-command/🟦️.ts` | 0/1 | 0/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 69 | `M/🖱️ui/🧬️contract/🧪️tests/🔮️corpus-oracles/🟦️.ts` | 3/1 | 3/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 70 | `M/🖱️ui/🧬️contract/🧪️tests/🧱️owners/🟦️.ts` | 4/3 | 4/3 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 71 | `M/🗣️dsl/🧬️schema/✨️derive/🧪️tests/🧱️ownership/🟦️.ts` | 7/1 | 7/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 72 | `M/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🟦️.ts` | 6/1 | 6/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 73 | `M/🚪️io/🪶️sqlite-snapshot/🧪️tests/🤝️interoperability/🟦️.ts` | 0/3 | 0/3 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 74 | `M/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🟦️.ts` | 8/7 | 8/7 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 75 | `M/🧊️3d/🥽️mesh/🧪️tests/📦️targets/🧪️packaging/🟦️.ts` | 0/2 | 0/2 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 76 | `M/🧬️schema/✅️validator/🧪️tests/🧩️neutral-owner/🟦️.ts` | 0/1 | 0/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 77 | `M/🧬️schema/🧩️composition/📥️admission/🧪️tests/🟦️.ts` | 3/1 | 3/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 78 | `M/🧬️schema/🧪️tests/🧱️neutrality/🟦️.ts` | 12/2 | 12/2 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 79 | `M/🧬️schema/🧪️tests/🩹️fragment-validation-oracle/🟦️.ts` | 0/1 | 0/1 |  | X | not in the named first-half domains (replication/ui/io/3d/schema/dsl/graph/pixels): not touched |
| 80 | `M/🧵️job/⏱️budget/🧪️tests/🔮️corpus-oracles/🟦️.ts` | 1/1 | 1/1 |  | G | plugin step budget no longer derived from contract.max_step_micros; owner decision |
| 81 | `M/🧵️job/⏱️context/📸️capture/🧪️tests/🟦️.ts` | 1/1 | 2/0 |  | A+B | typed-operations moved; PluginLifecycleFailure path changed |
| 82 | `M/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts` | 3/5 | 8/0 |  | A+B | job/worker carve; `step_fault=Some(error.into())`; Store close_granted pub; no os test-support import |
| 83 | `M/🧵️job/♻️retirement/👷️authority/🧪️tests/🟦️.ts` | 0/5 | 5/0 |  | A+B | worker authority carve; Arc::into_inner(original) |
| 84 | `M/🧵️job/♻️session-return/🧪️tests/🟦️.ts` | 1/1 | 2/0 |  | A+C | Option handle replaces ManuallyDrop inner; release funding asserted |
| 85 | `M/🧵️job/🎟️context/🛑️cancel/🤝️alias/🧪️tests/🟦️.ts` | 0/1 | 1/0 |  | A | BatchJobSession impl carved |
| 86 | `M/🧵️job/🎟️context/🪙️loan/🧪️tests/🟦️.ts` | 2/1 | 3/0 |  | A | test double callers helper moved to store 🧪️testing |
| 87 | `M/🧵️job/🏃️work/🎟️grant/🧪️tests/🟦️.ts` | 1/2 | 1/2 |  | G | retained-work wiring never completed (see report); needs cargo |
| 88 | `M/🧵️job/🧪️tests/📦️physical-close/🔮️oracle/🟦️.ts` | 1/1 | 2/0 |  | B | copied_bytes:copied |
| 89 | `M/🫧️transient/🧪️tests/🟦️.ts` | 4/2 | 4/2 |  | F | legacy lru-cache API (root lock 6.0.0) |
| 90 | `F/🧪️tests/🧱️ownership/🟦️.ts` | 0/3 | 3/0 |  | A+B+C | ui-contract->ui-declaration; alias-resolving export extractor; product-resident crates by name/role |

## Files edited (64; may include concurrent peers' edits in the same files)

- `F/📏️script-boundary/🧪️tests/🟦️.ts`
- `F/📏️script-boundary/🧪️tests/🧫️fixtures/🔣️.json`
- `F/🧪️tests/🧱️ownership/🟦️.ts`
- `F/🧪️tests/🧱️ownership/🧫️fixtures/🔣️.json`
- `M/⏪️time-travel/🧪️testing/📥️values/🧪️tests/🧬️schema/🔣️.json`
- `M/⏯️tool-run/🏛️ownership/🧪️tests/🟦️.ts`
- `M/⏯️tool-run/🏛️ownership/🧪️tests/🧫️fixtures/🔣️.json`
- `M/⏳️async/♻️cancel-return/📬️claim/🧪️tests/🟦️.ts`
- `M/🌱️value/🧪️tests/🧩️neutral-owner/🟦️.ts`
- `M/🎒️pack/🌱️value/🛬️decode/🧾️source/🧪️tests/🟦️.ts`
- `M/🎒️pack/🔤️json/📥️decode/🧾️receipt/🧪️tests/🟦️.ts`
- `M/🎒️pack/🚰️source/🧪️tests/🔣️.json`
- `M/🎠️kernel/🏛️ownership/📥️command-ingress/🧪️tests/🧫️fixtures/🔣️.json`
- `M/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🏛️boundary/🟦️.ts`
- `M/🎭️actor/🎟️retained-turn/🤝️admission/🧪️tests/🟦️.ts`
- `M/🎭️actor/🧪️tests/🏛️owner-boundary/🟦️.ts`
- `M/🎭️actor/🧪️tests/🏛️owner-boundary/🔣️.json`
- `M/🎭️actor/🧪️tests/🏛️owner-boundary/🧬️.json`
- `M/🎭️actor/🧬️ownership/🧪️tests/🟦️.ts`
- `M/🧵️job/⏱️context/📸️capture/🧪️tests/🟦️.ts`
- `M/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts`
- `M/🧵️job/🎟️context/🪙️loan/🧪️tests/🟦️.ts`
- `M/🧵️job/♻️retirement/👷️authority/🧪️tests/🟦️.ts`
- `M/🧵️job/♻️session-return/🧪️tests/🟦️.ts`
- `M/🧵️job/🎟️context/🛑️cancel/🤝️alias/🧪️tests/🟦️.ts`
- `M/🧵️job/🧪️tests/📦️physical-close/🔮️oracle/🟦️.ts`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `M/🏃️process/🏗️build/🎟️capacity/🧪️tests/🔐️inheritance/🟦️.ts`
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️receiver/🧪️tests/📥️release/🟦️.ts`
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/📥️paths/🟦️.ts`
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/📥️receipt/🟦️.ts`
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧪️tests/🟦️.ts`
- `M/🏃️process/📦️artifacts/🏗️native-build/🎯️recipe/🧪️tests/🟦️.ts`
- `M/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🎯️target/🧪️tests/📋️callers/🟦️.ts`
- `M/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts`
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests/🔤️ordering/🟦️.ts`
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests/🟦️.ts`
- `M/🏃️process/🧭️routing/📤️exports/🧪️tests/🟦️.ts`
- `M/🏃️process/🧭️routing/📥️invocation/🧪️tests/⏱️finite/🟦️.ts`
- `M/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📊️profile/🔣️.json`
- `M/🏃️process/🧭️routing/🎛️command/🧪️tests/🚧️boundary/🟦️.ts`
- `M/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts`
- `M/🏃️process/📦️artifacts/💾️cache/🧪️tests/🔣️.json`
- `M/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧪️tests/🟦️.ts`
- `M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧪️tests/🟦️.ts`
- `M/🏃️process/🧪️testing/🦀️cargo/🎟️admission/🧪️tests/🟦️.ts`
- `M/🏃️process/🧪️testing/🦀️cargo/🟦️.ts`
- `M/🏃️process/🧪️testing/🦀️cargo/🧪️tests/📥️preparation/🟦️.ts`
- `M/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🧪️tests/📥️consumers/🟦️.ts`
- `M/🎒️pack/🚰️source/📦️packages/🦀️rust/📋️project.json`
- `M/📡️replication/⚠️error/📦️packages/🦀️rust/📋️project.json`
- `M/📡️replication/🔐️crypto/📦️packages/🦀️rust/📋️project.json`
- `M/🖱️ui/🧬️contract/🆔️identity/📦️packages/🦀️rust/📋️project.json`
- `M/🖱️ui/🧬️contract/📏️budget/📦️packages/🦀️rust/📋️project.json`
- `M/🖱️ui/🧬️contract/📦️packages/🦀️rust/📋️project.json`
- `M/🖱️ui/🧬️contract/🔤️text/📦️packages/🦀️rust/📋️project.json`
- `M/🚪️io/📦️codec/📦️packages/🦀️rust/📋️project.json`
- `M/🧬️schema/🗿️artifact-reference/🔗️borrow/📦️packages/🦀️rust/📋️project.json`
- `M/🧬️schema/🧩️composition/📥️admission/📦️packages/🦀️rust/📋️project.json`
- `M/🧵️job/👷️worker/🎟️authority/📦️packages/🦀️rust/📋️project.json`
- `M/🧵️job/📬️outcome/📤️publication/⚠️fault/📦️packages/🦀️rust/📋️project.json`
- `M/🧵️job/📬️outcome/📤️publication/📦️packages/🦀️rust/📋️project.json`
- `M/🧵️job/🔭️preview/🎟️drive/📦️packages/🦀️rust/📋️project.json`

## How verified (exact)

* Per suite: `LAW1_MAX_MS=3600000 python3 law1-tools/run1.py <file>` (full file), `.../iso.py <file>` (each red test in its own process), `.../one.py <file> <name-regex>`; all run from the repo root, foreground, with the tst2 environment (envelope + short artifact dir + emulated hoist). Full run of all 90 rows: `law1-tools/final-run.py` (outcomes in the `after` column, row 47 re-run after its fix: 1/0).
* Production edits verified by their law: cargo admission (1/0), process/build/environment (2/0), boundary #1 (cwd), launch entries (static parse: 0 bad of 13802 / 10349).
* Not verified: `value/neutral-owner` rewrite (runs cargo), `component-pass` migration (still red for other reasons), `consumers` partial edits.
* Tools: `law1-tools/` (run1.py, iso.py, one.py, envwire.ts, report.py, final-run.py, errs.py, migrate_receive.py (unused, tried and reverted), notes.tsv = raw per-law notes). Generated output under `🗑️generated/law1-out` deleted.
