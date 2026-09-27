# Editing Validation

## Baseline

- `bun nx run @semio-tech/stdio-artifact-contract-rs:test -- --lib`: passed on 2026-09-26. Native runner executed 2 tests, both passed. This establishes the existing shared contract baseline only; it does not validate the new editing implementation.
- `bun nx run @semio-tech/stdio-csv-rs:test -- --lib editor` exited successfully but ran **zero tests**. Artifact editor modules are gated by `component-app-assembly`; this invocation is not a passing editor test. The launch gate now enables that feature and excludes the separate shared contract package.
- `bun nx run @semio-tech/stdio-csv-rs:test -- --features component-app-assembly --lib editor`: passed, 13 native editor tests run, 44 nonmatching tests skipped. Build plus run took 10m28s with concurrent compilation. This run predates the complete Details-window integration and must be repeated after that work lands.
- The initial CSV React launch failed because the Nx daemon did not start. A retry uses the installed Nx CLI via Bun with `NX_DAEMON=false`, preserving other agents' daemon sessions. Browser validation has not yet been performed.
- The browser retry reached the full stdio component compilation and exposed unfinished PNG `SetSnapshot` trait/codec integration. The owning agent is repairing that implementation. The outer Nx development task reported success despite its child build failing; the inner compiler diagnostics are the authoritative result. Browser verification remains pending.

## Launch Registration

Both `.vscode/launch.json` and its `.vscode/🧩️launch.seed.jsonc` source now include the shared contract test and the all-artifact editor test target in the existing stdio gate group. The commands invoke existing Nx targets and the package script routers. The all-artifact command limits native task concurrency to two while implementation agents remain parallel.

## Acceptance Evidence

Pending implementation and independent audit. Native command tests, schema fixtures, third-party oracle comparisons, browser console evidence, and any limitations must be recorded here or linked before completion.

## Full Editor Catalog Gate

Added a neutral acceptance fixture and JSON Schema, plus 88 statically typed editor tests and one coverage law. Each editor must declare the Details window and six retained edit actions, parse typed/Unicode arguments, preserve the action ID through binary replay, and preserve arguments against an independent serde_json oracle. This gate checks declarations and command replay; runtime edit/undo and browser acceptance remain separate obligations.

First run started with `bun nx run @semio-tech/stdio-plugin:test -- --features full-app-catalog --test editor_catalog`. Result pending. The gate is registered in both launch configuration sources.

## Long Text And Document Paging

The coordinator added a neutral text-buffer carrier schema and Unicode/empty/large-source fixture, Rust split/merge tests, a TypeScript reassembly implementation and tests, and React/WGPU carrier routing. TextWindowKit now publishes its buffer through existing paged scene carriers. DocumentWindowKit now exposes a windowed page tree whose opened pages use complete read-only text scenes. Long source values no longer need to fit a 512-byte label or a 32-KiB scene header. These are source changes awaiting test execution, not runtime acceptance claims.

Nx graph construction repeatedly restarted under concurrent edits. The first two waiting coordinator commands were stopped by their verified process IDs only, and rerun through the installed Nx runner with NX_DAEMON=false. Those runs subsequently reached their test scripts. Later runs use a ticket-local NX_WORKSPACE_DATA_DIRECTORY to avoid shared graph-lock contention; shared daemons and other agents' processes were preserved.

The TypeScript text-buffer carrier suite passed **4/4 tests**, including JSON Schema validation with Ajv and complete empty, Unicode, and large-source reassembly. The successful run used the workspace `bun nx` bootstrap with `NX_DAEMON=false`, `NX_PLUGIN_NO_TIMEOUTS=true`, and ticket-local graph data. A direct Nx attempt before it failed loading the cold Python plugin; no code failure was claimed from that infrastructure attempt.

## Long Text and WindowKit Checks

The native long-text lane test passed: 1 executed, 144 filtered; TypeScript previously passed all 4 lane tests. Full catalog integration stopped on Details lifetime and missing UI contract imports; assigned to its owner. The framework WindowKit runner stopped in its prerequisite completion oracle because the completion schema still asserted defaultProofs=0 and lacked operationCompletions, while the current neutral fixture and native completion tests require defaultProofs=2 and operationCompletions=1. Updated that schema to the existing native authority, preserving the oracle rather than bypassing it. WindowKit verification remains pending.

## Complete Table Transport

Added a schema-first neutral 4096-row Unicode table fixture, Rust and TypeScript tests, then paged columns/rows transport in TableScene and both renderer hosts. TableWindowKit previously encoded all cell values into the fixed SurfaceProps document and could reject ordinary large tables. Updated its existing small-table test to reconstruct both carriers and added a large-table assembly test. The initial test process was queued by Nx graph contention and had not reached execution before implementation; no observed red result is claimed. New checks remain pending.

The combined TypeScript lane suite executed after implementation and passed all 6 tests (17 assertions). Its log retains the original `table-lanes-red.log` name, but the result is green; the queue timing is documented above. The optimized full editor component build has now reached Cargo after fixing zero-valued default build budgets and deferring catalog-only imports to the catalog command.

The independent editor-catalog fixture check passed through Nx: Ajv validates the shared schema; 88 native catalog test registrations and 6 unique edit operations match the fixture. This is a fixture check, not runtime proof.

## Catalog Runtime Acceptance Expansion

The 88-root suite now requires one non-no-op typed detail edit per root (family defaults and exact-app overrides in the neutral fixture). It compares the entire edited snapshot with an independent serde_json pointer update, replays native binary/text mutations, checks complete inverse restoration, and drives retained publication plus undo/redo through the actual registered app. The format workers are supplying valid initial-snapshot paths. This expansion is not yet compiled or passing.

The first full optimized browser build stopped at an in-flight TIFF parameter-name error; the first direct CSV preview stopped at BMP SetSnapshot leaf derivation. Both were repaired by the format owner, and both commands are running again. Neither result establishes a browser size limit or runtime success.

## Shared Window Verification

`bun nx run @semio-tech/framework-plugin:test -- window_kits_tests` completed successfully: 21 tests passed, 823 skipped. The native suite includes the 4096-row table carrier and 500-page Unicode document cases. The task took 23m48s including shared compilation/lock time. The pending direct-cell API changes require subsequent focused validation.

## Lossless Source Correction

Advanced Details Source now uses complete typed snapshot JSON, because native ArtifactDsl output can be binary hex and can normalize away represented fields. Shared native helpers print through the first-party JSON writer, reject duplicate keys (including escaped/nested keys), validate typed conversion, and preserve full-width unsigned integers. Matching TypeScript helpers use the same neutral source fixtures. The TypeScript test oracle now uses the existing fast-json-patch test dependency instead of calling the implementation under test as its own oracle. An actual red Nx run failed because snapshotEditSource did not yet exist; implementation followed. Green verification is pending. Native transport verification completed: 2 tests passed, 144 skipped, for full Unicode text/table lanes.

## Lossless Source Integer Validation

`bun nx run @semio-tech/stdio-snapshot-editing-js:test` completed successfully with 28 tests and 51 assertions. The independent-oracle cases include exact unsigned/signed 64-bit source integers, duplicate object keys, escaped keys, Unicode, and metadata omitted by native format serialization. Rust source validation is being rerun separately; this result does not verify browser behavior.

## Shared Editing and Typing Validation

The TypeScript package test now includes strict `tsc --noEmit` before its tests. Both typing and all 29 tests (53 assertions) passed in `source-typecheck-final.log`, including a red-then-green rejection test for codecs that discard an unrelated optional field. The Rust contract run completed with 16/16 tests passing (`lossless-source-rust-final.log`), including complete Source helpers. This native run began before the later canonical-argument and empty-value admission changes; those changes require a new run.

## Canonical Action Admission And Reopening

The latest Rust contract run (`canonical-empty-input-contract.log`) passed all 16 tests, including host-admitted explicit null, empty text, root pointer, empty object key, and JSON-encoded exact wide integers. The native catalog run stopped on the already-fixed Binary job dependency; a new run is active. Catalog acceptance now also checks Pack save/reopen and lossless editable-source reopen after retained undo/redo for every editor; those new checks have not executed yet. This is artifact persistence validation, not a claim that every native-format export has been verified.

## Unified Shipping Fleet And Launch Coverage

The new shipping-default gate failed with `shipped component exposes editors for 7/36 formats`, then passed after removing the separate library-only full-app-catalog feature and the nine-editor runtime branch. The complete 88 identities are now explicit in the neutral editor fixture and checked against the compiled manifest. Seventy-nine additional authored playground rows preserve the original nine ports and use collision-checked React 6400–6478/WGPU 6500–6578 ports. Registry/launcher generation and the expanded coverage gate are running. The first coverage-gate attempt revealed an undefined assertion helper in the new test itself; it was fixed, and is not recorded as a valid behavior regression. The browser build started before the feature change and failed against its stale seven-format dependency selection, so it must be rerun after a stable component check.

## Numeric Inputs And Renderer Handoff

The shared TypeScript source suite passed 36 tests with 60 assertions, including strict type checking and neutral integer-literal inputs for floating fields. Rust tests were authored before the corresponding exact-safe-integer comparison change; their queued run had not executed before the implementation was applied, so no red outcome is claimed. Wide integers remain exact and cannot silently normalize into a floating field. The text worker changed Details numeric controls to exact JSON text input for both renderers. The core worker reported React quick 16/16; its pending WGPU process session could not be read from the coordinator (`Unknown process id`), and no compile success is claimed.

The complete playground gate passed. Registry generation exposed a real launch-name collision between DWG standards; the prefix resolver now distinguishes multiple standards and handles complete keycap/ZWJ emoji clusters. New neutral fixtures use existing emoji-regex and Ajv as independent test oracles. Launcher generation and the launch suite remain running.

## Launcher And Native Compile Results

Registry generation completed for all 88 stdio editor playgrounds; the generated launch file contains one React and one WGPU development entry per editor. The launch suite executed 11 tests: 10 passed, including the new independent emoji/prefix test and all-playground launcher coverage. The separate existing WASI profile-policy test failed because another task added `profile.wasm-dev.package.semio-framework-hash.opt-level = 0` to Cargo without updating its neutral policy fixture. This ticket preserves that concurrent change and does not claim the whole launch suite passes.

The focused native numeric-source run reached compilation after 21m56s and failed in newly added shared test code: a raw JSON string containing `"#/$defs/` needed a longer Rust raw-string delimiter, and `FaultCode` has no `as_str` method. The shared owner repaired both errors. The numeric native regression has not executed yet; its TypeScript counterpart remains green.

The media worker reports the 2,097,152-sample WAV retained-store regression passed (1/1), including cancellation, metadata preservation, publication, undo and redo. PNG large-pixel publication is pending. The follow-up independent read-only audit found a WGPU table focus trap after an accepted accessibility edit, a React source-draft conflict latch, and a silent Details pointer limit; the core and Details owners are repairing them. See `🔍️research/🧬editor-integration-followup-audit-2026-09-27.md`.
