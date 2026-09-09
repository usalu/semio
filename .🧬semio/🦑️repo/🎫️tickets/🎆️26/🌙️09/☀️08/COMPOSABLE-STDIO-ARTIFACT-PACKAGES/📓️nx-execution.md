# Nx Artifact Package Execution

## Scope

This execution owns the reusable Nx/Bun package boundary, all stdio artifact package declarations, the 24 single-artifact plugin extractions, the Norm and multi-artifact Nx declarations, the three Block TypeScript declarations, launch integration, and package-contract/cache evidence.

## Implemented

- Added a shared byte-aware generated-file writer and routed the normal graph, schema, UI-axis, and styling prerequisites through it, so byte-identical generation preserves source output mtimes while changed bytes are still published.
- Added schema-first stdio package fixtures and a repository-wide language-neutral artifact package contract backed by Ajv and Cargo/Nx metadata.
- Registered 36 stdio Rust and 36 stdio TypeScript artifact projects. TypeScript declarations export only built `dist` JavaScript and declaration files.
- Added the neutral Rust and TypeScript package routers to the existing Cargo `📜️script.ts`. Expensive native work supports progress and process-tree cancellation; native time limits use `SEMIO_BUILD_BUDGET_MS` and default to unlimited.
- Extracted and declared 24 single-artifact plugin Rust packages with direct taxonomy-root `lib.path = "../../🦀️.rs"` ownership and private composition mounts.
- Added Nx declarations/routers for Norm contract plus 15 Norm artifacts and 17 multi-artifact packages.
- Added declaration-only TypeScript packages for Block 2D, 3D, and 5D with emitted declarations and real package-name resolution tests. The dynamic catalog now also includes the independently extracted Sequence browser artifact as the 40th TypeScript package.
- Standardized the seven framework artifact Nx names under `@semio-tech/framework-*` and added Space/Collection Nx declarations.
- Added authoritative artifact contract/selector launch configurations to `.vscode/🧩️launch.seed.jsonc`; generation produced matching entries in `.vscode/launch.json`.
- Replaced Lowpoly's optional CAD plugin dependency with the direct CAD artifact package.
- Remounted Draw FSM runtime and proc-macro crates directly from their taxonomy roots and removed package-local Rust implementation files. Updated the live taxonomy descendant contract to require declaration-only package roots.
- Memoized native dependency-root closures in the Nx plugin. Direct normalization improved from 34.5 s to 24.8 s for 446 projects after warm repository file discovery.
- Rewrote all 36 stdio TypeScript package routers to use the neutral artifact builder directly. Artifact TypeScript targets now hash their exact transitive source imports and exact command implementation rather than the repository-wide library tree.
- The neutral TypeScript builder copies JSON assets referenced by emitted declarations. Its package consumer typecheck disables `skipLibCheck`, isolates ambient type roots, and asserts the actual exported definition shape when present.
- Generalized Nx native target normalization so local artifact wrappers use `nativeSources`/`nativeTestSources` and their actual Cargo dependency owners.
- Native dependency inputs conservatively include declared optional normal/build dependencies (and dev dependencies only for test inputs). Default builds can therefore invalidate on an inactive optional dependency; feature-aware narrowing requires explicit, verified target-feature metadata. The PDF sibling-isolation proof remains precise because PDF's declared closure is narrow.
- Removed Writer's obsolete public artifact host re-export and duplicate package-level example mounts.
- Retargeted 12 Flow child identity, edit, and retained-command native laws to `semio-s-artifact-flow-flow`. The one factory-composition law remains on `semio-s-plugin-flow`, where its `plugin()` and `FlowApps` inputs are owned. A source-mount audit confirms each named law is compiled by its selected owner.
- Retargeted the scoped Lowpoly `wasm-dev` optimization override from the composition package to `semio-s-artifact-lowpoly-lowpoly`; the render/app implementation is owned by that taxonomy-root leaf.
- Repaired the shared cache-contract, runner-self-test, browser-bundle, fresh-component, spatial-kernel geometry, and brepjs TypeScript sources after concurrent `import.source.*` rewrites left invalid source-context expressions or moved-path imports. Direct module-import smoke checks now pass and the repository has no remaining `import.source`/`source.directoryname` spellings outside generated dependencies.
- Closed the artifact-router argument boundary: build/check/test now reject caller-supplied workspace, package, manifest, configuration, and undeclared-target selectors; test levels are stripped before validation. Cargo build capture now emits the same periodic progress receipts as check/test while waiting for a shared build lock.
- Native target normalization now hashes the selected target's local `📜️script.ts` and its literal executable import closure in addition to the shared Cargo/toolchain contract. This covers package-local test environment behavior such as Gismap's test-only stack size without adding sibling artifact routers.
- Aligned the Block 2D, 3D, and 5D editor config JSON Schema, GraphQL, Protobuf, and TypeScript surfaces with their current Rust records after selection moved to the shared `ViewModel`; presence and document selection state remain unchanged. Block 3D retains its eight active representation, filter, window, brush, preview, and camera fields and drops the additional retired hover field.
- Corrected all 24 assigned composition parent test routers so they await the native test process and forward the resolved test level's remaining Cargo selectors. The Flow and CAD routers already satisfied this contract; the other 22 now match it.
- Corrected both nested Draw FSM test routers to resolve test levels, forward remaining Cargo selectors, and await native completion. Their parse and source contracts pass (`🗑️generated/draw-fsm-router-syntax.txt`, `🗑️generated/draw-fsm-router-contract.txt`).

## Passing evidence

- `cargo metadata --offline --format-version 1` refreshed the workspace lock successfully after the Lowpoly dependency change.
- Block 3D TypeScript package test passes with progress reporting, JavaScript build, `.d.ts` emission, package-name `tsc` resolution, and runtime module resolution (`🗑️generated/block-3d-typescript-test-async.txt`). Earlier direct Block 2D/3D/5D package tests also pass.
- The stdio home-I/O source/fixture gates pass, including direct/shared/full feature closure and third-party Ajv/TOML oracles.
- Direct stdio PDF TypeScript build/test passes with runtime package-name resolution and emitted declaration consumption.
- Launch seed/output validation finds one contract configuration, one selector configuration, and both required inputs.
- Authoritative isolated `bun x nx show project @semio-tech/stdio-pdf-rs --json` passes with `NX_ISOLATE_PLUGINS=false`. The normalized project hashes PDF taxonomy sources through `production -> default`; `nativeSources` also enumerates the mounted PDF source graph.
- Repository-wide package contract passes on the final current registry and stdio parent mount after its package-root wrapper removal: `AJV=2/3 packages=99 rust=99 typescript=40 dag=clean`. It covers schema, declaration, source-owner, Nx ownership, resolved runtime Cargo closure, parent duplicate-mount/public-compatibility rejection, and retired composition-namespace checks. The legacy scan deliberately retains the independent `semio_s_plugin_stdio_test_oracle::artifacts::*` third-party oracle namespace (`🗑️generated/all-artifact-package-contract-final-current-2.txt`).
- PDF cache restoration and source isolation pass with exact inputs. Baseline task hash `2961489084152580833`; deleting only PDF `dist` produced a 100% local hit and restored JS (`28f47dcb40e97669e54c02a5cddc600d294e8298cf6b2cc6827110e4cbc90dd3`) plus `.d.ts` (`4a51329398088b7239d0e916c6c01177000713c6dbf5264a7f62b95a639b3e89`). A temporary PDF source marker caused a cache miss at hash `6201406382484965991`. After exact PDF restoration, a temporary JPG-only source marker plus PDF `dist` deletion returned a 100% local hit at the original hash and restored identical outputs. All probe markers were removed.
- The declaration-aware PDF test passes through Nx and emits three required outputs. A fresh build cache followed by deleting the entire `dist` restored JS, `.d.ts`, and `🧬️schema/📜️artifact-definition.json` from a 100% local cache hit with exact hashes `28f47d…`, `4a5132…`, and `5ca2eb…`.
- Launch regeneration passed through the normal `repo:generator-inputs` prerequisite and `@semio-tech/plugin-registry:generate`. Seed/output contain the canonical stdio contract project and no stale Cargo-name target.
- Final launch regeneration after the Sequence extraction passed in 1m45s with both tasks executed. The generated JSONC contains the repository contract, generic artifact selector, and `@semio-tech/sequence-tests:test` harness exactly once; it contains no stale `semio-s-artifact-stdio-contract:*` command.
- The Nx aggregate completed all 40 artifact TypeScript package tests with status 0. Each package built its distribution, emitted declarations, typechecked a package-name consumer, and resolved the runtime package. Sequence emitted and consumed its handwritten declaration sidecar.
- Sequence normalized inputs explicitly include its root TypeScript facade, browser JavaScript, host JavaScript dependency, and adjacent handwritten browser `.d.ts` sidecar.
- Sequence sidecar hashing and restoration pass. Baseline task hash `2708385174624081084`; a temporary adjacent browser `.d.ts` marker forced a cache miss at `10778854061449214357`. The source was restored to SHA-256 `81985d256abea2c8b944f709a050b4dbd3cbdc4fd22772732f7ce928e4acb400`. After deleting only Sequence `dist`, Nx produced a 100% local hit at the baseline hash and restored the sidecar (`81985d…`), entry declaration (`450e1a…`), and JavaScript (`a6e877…`).
- The schema-first native-preparation fixture, third-party `jsonschema` oracle, Cargo metadata oracle, artifact-router selector regression, and queued-build progress regression pass. The unsafe-router test was recorded red before the implementation and green afterward.
- The artifact Rust `test` route now delegates to the repository's budgeted Cargo/Nextest runner after exactly one `resolveTestLevel` pass, preserving level profiles, cumulative `--skip` filters, assertion budgets, runtime filters, and libtest arguments. Schema-first fake-Cargo lifecycle tests were recorded red against both the former raw `cargo test` route and a double-normalization bug. The final `test quick long --lib matching_case -- --nocapture` probe observes Nextest list/run, retains `long` and `matching_case` as runtime filters, retains `quick::`, skips `long::` and `exhaustive::`, and terminates normally (`🗑️generated/artifact-test-level-runner-red.txt`, `🗑️generated/artifact-test-level-looking-filter-red.txt`, `🗑️generated/artifact-test-level-runner-final-green.txt`).
- A fresh focused native-preparation run after the final router changes passes the independent Cargo metadata and `jsonschema` dependency/input oracle (`🗑️generated/native-preparation-direct-final.txt`).
- The ticket-close ledger's dedicated plain-path section now has 564 unique repo-relative paths after the Mathematical graph-window repair. Of these, 560 currently exist and four are intentionally absent historical/remount paths: two Writer ownership moves and the two removed package-local Draw FSM implementations. All current actionable paths pass the focused `git diff --check` receipts (`🗑️generated/nx-owned-file-ledger-path-check-current.txt`, `🗑️generated/nx-owned-file-diff-check-current.txt`).
- The focused UI built-tree retirement source oracle is green after replacing its obsolete individual-assertion spelling with exact checks for the current nine typed depth witnesses and the exhaustive guarded `while` loop (`🗑️generated/ui-built-tree-retirement-oracle-final.txt`). The original failure is preserved in `🗑️generated/flow-source-fixture-final.txt`.
- The complete Flow source-fixture suite is green (`🗑️generated/flow-source-fixture-final-11.txt`). Its sequential red receipts identified stale expectations after the framework Flow extraction: host-wire ordinals now match the 36-row command table at 33/34, the parameter intent fixture/schema resolve from the framework artifact and compile their two schema `$defs`, the 14-entry canonical variant schema agrees with its enum and Rust variant oracle, source witnesses point to the mounted snapshot/test modules, Ajv keywords are registered on Ajv instances before compilation, and editor/viewer owner assertions use the leaf-local retirement module. All hostile/rejection checks remain active.
- The retained `addWidget` language-neutral oracle passes independently of Cargo: two accepted requests and 24 distinct hostile/cancellation/ownership cases match the schema and production-source witnesses (`🗑️generated/flow-add-widget-retained-oracle-final.txt`). Its two native retained laws remain part of the 13-law runtime gate.
- Flow's 13 exact native laws now have verified source ownership: 12 are mounted by `semio-s-artifact-flow-flow`; the single `flow_actual_surface_factories_close_all_owners_under_neutral_grants` law stays with `semio-s-plugin-flow` because it calls the parent-owned `plugin()` assembly and `FlowApps`. The three script routes use these exact package groups, and every law name has one source witness in its expected owner (`🗑️generated/flow-exact-law-source-ownership.json`).
- The parent duplicate-mount contract was recorded red on Mathematical's package wrapper. All 22 remaining composition-level example mounts, Mathematical's duplicate CAS/polynomial mounts, and Imperative's public artifact extension re-export were removed. The repository-wide package-wrapper scan now reports zero direct taxonomy implementation mounts and zero root-level artifact public re-exports. Canonical example access remains on the artifact leaves; Raster now owns its previously parent-only demo-session mount and test.
- Current normalized Rust PDF inputs contain 835 positive PDF-owned source paths and no JPG taxonomy source or `stdio-jpg` owner (`🗑️generated/nx-pdf-rust-current-input-isolation.json`). The normalized build replaces `^production` with an explicit 29-project `nativeSources` closure plus the three generator prerequisites. A conservative traversal of every Nx project dependency reachable from those inputs and prerequisites covers 31 internal projects including PDF; none is the JPG project and no configured named-input or target-input string contains JPG. A third-party `minimatch` probe evaluated all 8,914 path patterns across that whole over-approximated closure against an actual JPG Rust source and found zero matches (`🗑️generated/nx-pdf-rust-full-transitive-isolation.json`, `🗑️generated/nx-pdf-rust-full-transitive-glob-proof.json`). The native task cache mutation/restoration receipt remains pending the shared Cargo queue.
- A normal `@semio-tech/forms-forms-rs:check` invocation completed its four ordinary prerequisites, including a successful `framework-graph:generate`. The earlier print-latex/report-actor preview-target routing error did not recur. Its leaf Cargo process had no compiler child and was cancelled with status 130 after 890 seconds queued on the shared build lock so it can be restarted with `CARGO_BUILD_JOBS=2` after the active compiler finishes. The 24-leaf loop's first Writer process was likewise cancelled with status 130 after 890 seconds queued, with no compiler child; neither cancellation discarded compilation work.
- The restarted ordinary `@semio-tech/forms-forms-rs:check` gate completed successfully with all four normal prerequisites (5/5 tasks) in 90m44s using `CARGO_BUILD_JOBS=2` and `CARGO_INCREMENTAL=0`; its leaf compiled in 89m27s. The old print-latex/report-actor preview routing failure did not recur (`🗑️generated/forms-normal-prerequisite-nx-check-jobs2-final.txt`).
- The empty Block 2D/5D config contract passes a third-party Ajv oracle and the generated TypeScript parsers; `{}` is accepted and the retired config-local `selectedIds` shape is rejected by both JSON schemas. Direct neutral package tests pass for both TypeScript artifact packages (`🗑️generated/block-empty-config-contract.txt`, `🗑️generated/block-2d-typescript-empty-config-test.txt`, `🗑️generated/block-5d-typescript-empty-config-test.txt`).
- The Block 3D config contract passes strict Ajv with the repository keyword/formats registered: its JSON Schema fields exactly match the eight current Rust fields, a current minimal record validates, and the retired `selectedIds`/`hoveredVortexFullId` keys are refused. Its direct neutral TypeScript package test passes (`🗑️generated/block-3d-config-contract.txt`, `🗑️generated/block-3d-typescript-config-test.txt`).
- The 24-parent router source contract confirms every test command resolves the test level, forwards `rest`, and awaits `runCargoTestBudgeted` (`🗑️generated/owned-24-parent-router-contract.txt`).
- The local native-router input contract was recorded red before normalization inspected the selected command. Its schema-validated fixture is green against a third-party esbuild metafile: the exact three-file local/imported router closure is present and an unrelated sibling router is absent. Ordinary current Nx project normalization independently confirms that Gismap and PDF build/check/test targets each hash their local router and the shared artifact Rust router while excluding the respective Gisterrain/JPG sibling router (`🗑️generated/native-local-router-input-red.txt`, `🗑️generated/native-local-router-input-green.txt`, `🗑️generated/gismap-project-native-router-current.json`, `🗑️generated/gismap-native-router-input-proof.txt`, `🗑️generated/pdf-project-native-router-current.json`, `🗑️generated/pdf-native-router-input-proof.txt`).

- The schema-backed generated-write fixture was recorded red before the shared writer existed and green afterward. All four real prerequisite freshness checks passed, then their generate commands processed 34 files (graph 21, schema 3, styling 8, UI 2) with zero SHA-256 or nanosecond-mtime changes (`🗑️generated/generated-write-mtime-red.txt`, `🗑️generated/generated-write-mtime-final-green.txt`, `🗑️generated/generator-noop-mtime-proof.json`).
- A final repository source scan found no `semio_s_plugin_<owner>::artifacts::...` imports. The retained `semio_s_plugin_stdio_test_oracle::artifacts::...` paths belong to the independent third-party-style test oracle and are deliberately excluded (`🗑️generated/legacy-plugin-artifact-import-final.txt`).
- The post-Puzzle/FEM-manifest repository contract rerun exits 0 on the current workspace: `AJV=2/3 packages=99 rust=99 typescript=40 dag=clean` (`🗑️generated/all-artifact-package-contract-post-manifest-final.txt`).
- A repository-wide package-purity scan finds zero Rust implementation files and zero non-generated TypeScript/JavaScript implementation files beneath artifact `📦️packages` roots. The one declaration sidecar under Sequence's `dist` is generated output and is excluded explicitly (`🗑️generated/artifact-package-declaration-purity-final.txt`).
- The package-native execution surface contains no remaining hardcoded 120-second, 180-second, or one-hour build timeout. Flow's exact laws now inherit the repository build budget, while all expensive runners retain progress and owned-process cancellation (`🗑️generated/native-hardcoded-budget-final.txt`).
- The same repository contract rerun after the local-router input repair also exits 0 with the unchanged authority result `AJV=2/3 packages=99 rust=99 typescript=40 dag=clean` (`🗑️generated/all-artifact-package-contract-post-router-final.txt`).
- The current standalone Writer leaf retry reaches `semio-s-artifact-writer-writer` on the repaired Trinity/OS graph and completes successfully in 7m57s. The enclosing two-entry command exits 1 only because its earlier Mathematical entry encountered ENOSPC before Writer acquired the lock (`🗑️generated/owned-focused-default-retries-current.txt`).
- The current Flow composition parent check reaches both the extracted `semio-s-artifact-flow-flow` leaf and `semio-s-plugin-flow`, then completes successfully in 18m44s. The 24-parent matrix continues sequentially with VCS (`🗑️generated/owned-24-parent-composition-current.txt`).
- Animate's standalone default check completes successfully in 29m45s on the current graph, independently of its already-green `preview-window` feature check. The default matrix continues sequentially with Shooting (`🗑️generated/owned-23-standalone-default-current.txt`).
- The coordinator's independent conservative production-root audit of the repaired full PDF graph passes on 31 projects, 34 targets, 2,322 positive patterns, and zero unresolved inputs: PDF production and both its local/shared router inputs are admitted, while JPG production remains excluded. This is source-only evidence; the coordinator owns the remaining native cache restoration run.
- Five newly mounted Writer window mutations now own one descriptor and one JSON Schema payload each. A focused third-party Ajv pass validates all five descriptor shapes and compiles all five Draft-07 payload schemas (`🗑️generated/writer-window-mutation-authority-ajv.txt`).
- The repository graph generator also accepts those Writer mutation authorities and completes normally, writing its nine current manifests through the content-preserving generated-file writer (`🗑️generated/writer-window-mutation-taxonomy-generate.txt`).

## Current failures and pending gates

- A prior disk-exhaustion interval invalidated the first Lowpoly `cad-fixtures`, Draw FSM/runtime, required-nullable derive, Mathematical default, Flow/VCS default, and Mathematical parent attempts with explicit `ENOSPC` diagnostics. These are environmental red receipts rather than source verdicts. Disk later recovered, and focused/current sequential retries retain the shared target with `CARGO_BUILD_JOBS=2` and `CARGO_INCREMENTAL=0`; no cache or active build directory was reset.
- Draw FSM remount reached rustc and exposed only proc-macro entry integration errors. The duplicate import/public helper errors are fixed; its retry remains pending behind active native work.
- The first 24-leaf standalone-default sweep reached Writer and exposed a real root namespace collision (`schema`/`dsl`) before the shell hit zsh's readonly `status` variable and stopped. Writer now follows the established Forms boundary: the external schema crate is `framework_schema`, the artifact text facet is `document_dsl`, and duplicate root snapshot/example exports are removed. Its focused retry is queued on the shared native build lock; the full loop will restart with `result_code` after that leaf resolves.
- The 13 Flow exact-law runtime receipt remains queued behind active native Cargo writers; package identity and module mounting are already verified statically.
- The standalone-default Writer retry reached the extracted leaf and identified three facets that still relied on the former root `schema` name. Each now imports the artifact-local `crate::schema` explicitly. The next run exposed four command facets missing `WriterConfigMutation` and five newly mounted window mutations without mandatory descriptor/payload authorities. A concurrent ownership rewrite then moved those commands to the current `NoConfigMutation` surface, superseding this lane's four import edits; they remain historical touches only. The ten language-neutral authority files remain current and validated. Writer retry session `58648` and remaining-23-leaf matrix session `44605` are attached behind the shared two-job Cargo queue (`🗑️generated/writer-standalone-default-current-2.txt`, `🗑️generated/writer-standalone-default-current-3.txt`, `🗑️generated/writer-standalone-default-current-4.txt`, `🗑️generated/owned-23-standalone-default-current.txt`). Enabled component/app composition and parent imports remain separate acceptance passes.
- The Writer retry reached current rustc but stopped in the shared OS kernel before compiling Writer: four channel codec branches reference missing `AppCommand::{LoadWindowConfig,ReadWindowConfigs}` variants (`🗑️generated/writer-standalone-default-current.txt`). This is retained as an upstream integration failure; Writer must be retried after the shared WindowConfig channel repair.
- A schema-first fake-Cargo regression reproduced the PDF staging race: Cargo emitted dependency and primary paths, then replaced a dependency before process exit; the old delayed staging published `successor-output` instead of the compiler-event bytes (`🗑️generated/artifact-cargo-eager-capture-red.txt`). `buildCargoArtifacts` now copies each selected compiler artifact into an owned private directory as its event arrives, uses `SEMIO_TEST_ARTIFACT_DIR` when present, cancels and awaits Cargo if capture fails, stages only captured bytes after a successful exit, and always removes its capture directory. The focused fixture now stages the original dependency and primary bytes. A second failure case emits an absent dependency and keeps Cargo alive: it is cancelled and awaited in under five seconds, the prior staged tree remains byte-identical, and the capture directory is removed. The Cargo metadata oracle still agrees (`🗑️generated/artifact-cargo-eager-capture-final-green-2.txt`).
- Artifact package tests now keep the budgeted Nextest build/list queue visible with the same ten-second native progress timer and stop it in `finally`; the common runner retains process-tree cancellation and test-level budgets. The focused fake-Nextest lifecycle terminates normally and records an observed progress line (`🗑️generated/artifact-native-progress-capture-final-green.txt`).
- The exact 24-leaf feature inventory confirms 22 leaves have no selectable feature, Animate additionally requires a `preview-window` pass, and Lowpoly additionally requires a `cad-fixtures` pass. The parent inventory confirms Process, CAD, and Sourcing default-enable `plugin-entry`; the other parent composition crates have no default feature. Default leaf checks, the two explicit feature checks, and a separate parent matrix are therefore recorded independently (`🗑️generated/owned-24-leaf-feature-inventory.json`, `🗑️generated/owned-24-parent-feature-inventory.json`).
- The GIS Map direct-config red receipt exposed three distinct contract mismatches. Required-nullable mutation fields now use the first-party `#[value(required)]` presence control, with a serde-oracle integration regression proving missing-key rejection and explicit-null acceptance; a focused Ajv pass agrees for both payload schemas (`🗑️generated/gis-config-required-nullable-ajv.txt`). The inverse fixture spells typed `f64` values as `1.0`. The nonfinite law retains the repository JSON writer's established nonfinite-to-null contract while proving the mutation is Fatal, state is unchanged, and the fallible `Gis2dConfigDiff` persistence serializer rejects the value. The GIS artifact router sets `RUST_MIN_STACK=268435456` only for its `test` command so the 241-law retained replay does not overflow the default test-thread stack. The focused derive Nx regression and coordinator-owned GIS component rerun are active; no duplicate GIS Cargo process was started by this lane.
- The Mathematical default leaf red receipt found a genuine stale taxonomy mount: the artifact root still referenced the removed top-level editor config, while the canonical implementation is the graph-window config. The root now mounts that exact source/schema and the artifact's Rust editor/commands/tests use the canonical `EquationGraphWindowConfig` and `EquationGraphWindowConfigMutation` names. Focused Mathematical and Writer retries are attached in session `69875`; the original red remains in `🗑️generated/owned-23-standalone-default-current.txt`.
- Active attached native receipts after continuation: remaining 23 defaults `44605`, two leaf features `5870`, 24 parent compositions `40933`, Draw FSM tail `9446`, fresh Flow 13-law routes `2849`, focused required-nullable derive `93255`, and focused Mathematical/Writer retry `69875`. All use the shared ticket target with two Cargo jobs and incremental compilation disabled. The preceding Flow session `71554` was cancelled cleanly with status 130 while it had no compiler child: its three exact-law routes still passed a hard one-hour build budget and had spent 52 minutes queued. Those explicit budgets are removed, so the replacement honors the shared `SEMIO_BUILD_BUDGET_MS=0` cooperative queue contract.

## Relevant generated evidence

- `🗑️generated/nx-normalizer-profile-tracked.txt`
- `🗑️generated/nx-normalizer-profile-memoized.txt`
- `🗑️generated/nx-pdf-normalized-project.json`
- `🗑️generated/nx-pdf-normalized-inputs.txt`
- `🗑️generated/all-artifact-package-contract-current.txt`
- `🗑️generated/launch-generate.txt`
- `🗑️generated/launch-validation.txt`
- `🗑️generated/block-3d-typescript-test-async.txt`
- `🗑️generated/draw-fsm-package-remount-check.txt`
- `🗑️generated/nx-pdf-cache-precise-restore.txt`
- `🗑️generated/nx-pdf-cache-relevant-mutation.txt`
- `🗑️generated/nx-pdf-cache-exact-populate.txt`
- `🗑️generated/nx-pdf-cache-exact-restore.txt`
- `🗑️generated/nx-pdf-cache-exact-relevant.txt`
- `🗑️generated/nx-pdf-cache-exact-unrelated-jpg.txt`
- `🗑️generated/nx-pdf-cache-exact-*-run.json`
- `🗑️generated/nx-pdf-cache-exact-*-hashes.txt`
- `🗑️generated/nx-pdf-declaration-assets-test.txt`
- `🗑️generated/nx-pdf-assets-cache-populate.txt`
- `🗑️generated/nx-pdf-assets-cache-restore.txt`
- `🗑️generated/launch-generate-final.txt`
- `🗑️generated/launch-validation-final.txt`
- `🗑️generated/artifact-typescript-40-nx-summary.json`
- `🗑️generated/sequence-typescript-normalized-inputs.txt`
- `🗑️generated/sequence-sidecar-cache-baseline.txt`
- `🗑️generated/sequence-sidecar-cache-mutation.txt`
- `🗑️generated/sequence-sidecar-cache-mutation-run.json`
- `🗑️generated/sequence-sidecar-cache-restore.txt`
- `🗑️generated/sequence-sidecar-cache-restore-run.json`
- `🗑️generated/sequence-sidecar-cache-proof.txt`
- `🗑️generated/launch-generate-sequence-final-4.txt`
- `🗑️generated/launch-validation-sequence-final.txt`
- `🗑️generated/all-artifact-package-contract-final-6.txt`
- `🗑️generated/all-artifact-package-contract-final-current-2.txt`
- `🗑️generated/artifact-router-arguments-red.txt`
- `🗑️generated/artifact-build-progress-red.txt`
- `🗑️generated/artifact-router-progress-green.txt`
- `🗑️generated/artifact-router-existing-contract.txt`
- `🗑️generated/ui-built-tree-retirement-oracle-final.txt`
- `🗑️generated/flow-source-fixture-final.txt`
- `🗑️generated/flow-source-fixture-final-2.txt`
- `🗑️generated/flow-source-fixture-final-3.txt`
- `🗑️generated/flow-source-fixture-final-4.txt`
- `🗑️generated/flow-source-fixture-final-5.txt`
- `🗑️generated/flow-source-fixture-final-6.txt`
- `🗑️generated/flow-source-fixture-final-7.txt`
- `🗑️generated/flow-source-fixture-final-8.txt`
- `🗑️generated/flow-source-fixture-final-9.txt`
- `🗑️generated/flow-source-fixture-final-10.txt`
- `🗑️generated/flow-source-fixture-final-11.txt`
- `🗑️generated/parent-artifact-mount-contract-red.txt`
- `🗑️generated/all-parent-artifact-mounts-after-owned.txt`
- `🗑️generated/all-parent-artifact-compatibility-after.txt`
- `🗑️generated/nx-pdf-rust-current-input-isolation.json`
- `🗑️generated/nx-pdf-rust-final-project.json`
- `🗑️generated/nx-pdf-rust-transitive-input-isolation.json`
- `🗑️generated/nx-pdf-rust-prerequisite-inputs.json`
- `🗑️generated/nx-pdf-rust-full-transitive-isolation.json`
- `🗑️generated/nx-pdf-rust-full-transitive-glob-proof.json`
- `🗑️generated/forms-normal-prerequisite-nx-check-2.txt`
- `🗑️generated/native-preparation-direct-final.txt`
- `🗑️generated/artifact-cargo-eager-capture-red.txt`
- `🗑️generated/artifact-cargo-eager-capture-final-green.txt`
- `🗑️generated/artifact-cargo-eager-capture-final-green-2.txt`
- `🗑️generated/artifact-native-progress-capture-final-green.txt`
