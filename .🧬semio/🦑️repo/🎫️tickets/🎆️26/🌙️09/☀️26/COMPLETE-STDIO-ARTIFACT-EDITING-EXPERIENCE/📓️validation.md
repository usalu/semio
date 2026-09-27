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

## Current Continuation Evidence

The native catalog and optimized shipping component both stopped on `No space left on device`; no all-editor runtime or browser result was produced. The existing `repo:cache-report -- --json` completed and reported disposable cache units. The coordinator did not run bulk cleanup: the clean skill kills other development processes, which conflicts with the user's explicit concurrent-work instruction. At the next continuation, `df -h .` reports 141 GiB available, so verification can resume without deleting further files. The completed cache-report handle is absent.

The new language-neutral typed-path fixture has 14 accepted cases covering nested fields, array insertion/removal, escaped/empty map keys, enum representations, transparent wrappers, flattening, custom codecs and root replacement. Existing fast-json-patch and Ajv independently validate these semantics. The TypeScript shared editing suite passed **50 tests / 102 assertions**, including strict source type checking. Native direct-path derive coverage is still being implemented.

PNG's focused large-raster retained-store test passed after correcting the test document dimensions to 1024×512 RGBA (2,097,152 bytes). The original 3×3 dimensions did not match the supplied pixel buffer and correctly failed artifact admission. MP4's new compact metadata test stopped in an in-flight shared value-codec syntax error and has not yet executed. The source was repaired subsequently; a green rerun is still required.

## Compact Patch Implementation

The compact patch TypeScript gate first failed because the new implementation module did not exist, then exposed a NodeNext import extension error after implementation. After correction, strict type checking and **64 tests / 150 assertions passed**. These include independent JSON Patch results for set, append, remove, move, cross-parent moves and escaped-key rename; exact inverse restoration; compound-edit atomicity; and a 2 MiB payload retained by reference while metadata changes. Forward and inverse metadata patches remain under 1 KiB in the neutral fixture.

Matching native helpers and schema-first carriers are authored under shared editing `🩹️patch`, together with a native oracle/codec/large-metadata test module. Native direct-path access now has a stable implementation checkpoint for records, enums, containers, shape metadata and object-key lookup. The focused derive suite, the shared contract suite, the complete default editor catalog and the optimized shipping component are running. Native compact patch results are still unverified. The React source-conflict regression passed **17/17**, as reported by the execution owner in `react-explicit-conflict-green.log`; WGPU runtime verification remains outstanding.

## Current Shared Build and Shipping Boundary

The stable-core catalog attempt exited before Cargo because the concurrently restored shipping feature closure contains only seven format packages (nine editor subsets). The optimized shipping component and MP4 large-publication third attempt both failed in generated shared ToValue code, not linker or editor assertions. The value worker owns the repair and the WGPU task is coordinated to preserve it.

The separate END-TO-END-OS-HUB-COLLABORATION-MCP ticket records a real wasm-dev million-function linker refusal and a bounded nine-editor component restored by its LB worker. That shared change is preserved. The native 88-editor fixture gate now explicitly checks the full-app-catalog feature, while a separately registered shipping gate retains the 36-format/88-playground requirement and is currently expected to fail. The component gate checks shipping coverage before building, so a successful nine-editor link cannot be mistaken for completion. Native acceptance must explicitly enable full-app-catalog. Optimized full-fleet size/link behavior is still unproven, and the task still needs a viable complete shipping arrangement.

## Compact Publication Integration Checkpoint

The read-only compact/native audit identified two active P1s: WGPU accessibility Value followed by Blur could republish a stale draft, and the MP4 payload stripping helper could accept a payload edit then restore the old bytes. Root authored neutral byte-array edit vectors and native regression tests for MP4, both JPEG editors, and both TIFF editors before replacing their helpers. The MP4 regression run is queued; no red runtime assertion has completed yet. The WGPU worker owns its retained single-publication repair.

The shared retained reducer now prepares and validates a native path patch instead of projecting the complete document for schema validation. It encodes each actual wrapped forward mutation and exact inverse against the store's one-item maximum before publication, advances the base sequentially for multi-mutation emits, and refuses an invalid native diff. The 16 MiB wire/work admission budget is distinct from the 1 MiB persisted publication-item maximum. This integration is not compiled or runtime-proven yet.

The component checker now has an explicit `--full-catalog <absolute output>` diagnostic that selects the complete native feature and isolates Cargo intermediates and uplifted outputs beneath the caller-owned root. It cannot overwrite the concurrently used nine-editor component. It checks the native fixture, links the existing optimized profile, and subjects extracted bytes to the same independent WebAssembly validation and first-party count/structure limit. This diagnostic is not a declaration that the full catalog ships; the separate shipping coverage gate remains authoritative and failing.

The focused native direct-path suite completed successfully: 7 tests passed, zero skipped, Nextest run `27607ff8-c41b-4a6a-a547-950d9b2c04f8` (20m37s aggregate including the shared Cargo wait; 35ms test time). It proves the authored derived path fixture and its OS dependency feature selection, not the full stdio catalog or component. The native catalog source fixture also passed (88 editors, six actions). The separate shipping source fixture remains an expected failure at seven of 36 format packages.

The schema fragment AJV oracle passed with 18 assertions, covering the 2 MiB sibling payload, numeric bounds, enum, required/minItems/dependency and discriminated-union frontier. The five pilot payload-strip helpers have now been replaced by direct native path preparation plus authoritative schema validation. Their native regressions and full retained integration remain pending.

## Compound Move Schema Paths

A new neutral vector moves the first scalar of an array into a child list of the object that shifts from index two to index one after removal. Compact validation previously resolved every operation path against the original native snapshot, so it could misclassify that destination parent. Validation now collects each operation's schema path while advancing a detached native snapshot, then validates the final document context. Native regression is pending.

The expanded TypeScript suite passed 69 tests before the new shifted-parent vector. The first 70-case run failed only in fast-json-patch's move validation: that library checks the destination against the pre-removal document and rejects the shifted parent; our result already matched the expected fixture. The independent oracle vector now spells out the equivalent remove/add operations, each validated by the library. This oracle correction is not recorded as a first-party runtime failure.

The corrected shifted-parent independent oracle rerun passed all 70 TypeScript tests with 169 assertions. This includes the five new payload byte edits and structural move/inverse fixture. The native shared contract, five media regressions, full native catalog, and optimized full catalog component remain running, so this is TypeScript evidence only.

The CSV browser launch through `bun nx` failed before startup because the repo bootstrap tried to start a stale/unavailable Nx daemon despite `NX_DAEMON=false`. The retry uses `bun x --no-install nx` with the ticket-owned workspace data and the same existing Nx target; it has progressed through dependency generation and remains running. No browser behavior is claimed.

Fresh WGPU WASM validation exposed an additional empty-enum derive exhaustiveness error in NoConfigMutation/NoPresenceMutation/NoTransientMutation. The owning worker repaired zero-variant root/path/shape/key/read/write expansions with exhaustive `match *self {}` and added tagged/untagged compile laws. Its standalone Rust reference check passes; the earlier 7/7 Nx run predates this new regression law. Full feature selections still need fresh validation.

## Continued Native Validation and Exact Publication

The isolated optimized full-catalog component run `full-catalog-component-current.log` failed in the shared plugin crate with six E0004 empty-enum derive errors. The fix was authored after that run compiled its macro. Its failed owned Cargo subprocess was terminated once the diagnostic was recorded, and the same isolated output was reused in `full-catalog-component-current-2.log`. No full component link or browser acceptance result exists yet. The native all-editor catalog, shared contract, MP4 payload regression, and CSV development component remain active; they are not passing results.

The central retained edit reducer now compares its emitted native mutation result with the exact schema-validated requested snapshot before returning an emit. This rejects accepted no-ops and unrequested native normalization, including the media strip-and-restore defect covered by the authored payload fixtures. Native verification is pending.

The schema fragment AJV-only oracle passed one test with 26 assertions in `schema-fragment-ajv-oracle-2.log`, including length boundaries on a 2,097,152-item byte array. The task router now also runs the framework native schema tests; that expanded target has not yet passed.

The exact retained publication admission now has six language-neutral cases under the compact patch fixture: forward/inverse at and above the exact 1 MiB limit, accepted no-op refusal, and unrelated payload-loss refusal. The authored native test compares encoded lengths with serde_json and expected edits with json_patch, then invokes the production publication admission helper. These new native cases are pending execution in the shared contract run.

The concurrent renderer owner reports high host memory pressure while shared and isolated Rust graphs compile together. Our component retry and CSV preview continue with two Cargo jobs; no additional heavyweight build is being launched while active proofs run. Queue time and source progress are not test results.

## Audit Corrections: All Dispatch Routes and Exact Undo

The Terra audit identified two additional real gaps. Publication admission now replays each exact inverse batch against its forward result and refuses any failure to restore the native pre-state; a seventh neutral case covers an encoded but ineffective inverse. Shared schema and publication validation moved into the default `SnapshotEditingEditor::snapshot_edit_emit`. All 88 implementation hooks are now named `snapshot_edit_mutations`; the three CSV/JSON direct handlers that bypassed that trait are explicitly routed through its validated entry. The all-editor catalog law now invokes the actual direct editor reducer for the accepted edit and schema-identity change/removal rejection cases, using a framework-owned fixture helper. Native results remain pending.

The media worker removed the remaining WAV and PNG detach/restore fallbacks and authored a WAV Raw-to-Pcm8 discriminator preservation regression. Five compact native PatchSnapshot leaves are authored but not mounted. Details schema capabilities and discoverable optional fields are authored by the text worker and also await native execution.

The updated neutral all-editor catalogue fixture passed through Nx in `native-catalog-fixture-current-2.log`: 88 editors and six action types, including its new direct-command schema-identity rejection inputs. This is AJV/catalogue fixture validation only; the all-editor Rust execution remains pending.

The second isolated optimized component diagnostic reached its 20-minute build deadline before linking (`full-catalog-component-current-2.log`, exit 1). This is a timeout, not a confirmed component-size failure. The new owned-child-tree cancellation left no isolated compiler process running. Its completed build artifacts remain available for reuse. A later retry needs an explicit longer build budget; no immediate heavyweight retry is launched while the shared native lane and CSV preview are under memory pressure.

A no-op law was added to the all-editor native catalogue using each neutral meaningful-edit path and its original serde_json value. The shared validated emit now succeeds without an artifact event when the exact requested snapshot is unchanged; the seven compact pilot admission paths accept that successful no-op. This prevents unchanged field submissions from adding undo history. Native execution is pending.

Lock ownership was checked using lsof plus active compiler descendants. Shared contract Cargo 24706 waits on `cache/cargo/target/debug/.cargo-lock`; active native compilation is the UI WGPU-engine test Cargo 732. CSV preview Cargo 35633 uses a distinct temporary target under the stdio package dist directory, so cancelling it would not free the contract target lock. No unrelated or preview process was interrupted.

## Bounded Admission Consolidation

Seven pilot media editors still executed the complete native reducer, cloned snapshots, and encoded forward/inverse mutations during admission. Those overrides and the other 81 identical admission overrides are now replaced by one shared trait default using the bounded incoming-event and addressed-path checks. Authoritative schema validation and exact wrapped publication/inverse validation remain in the execution reducer before publication. The MP4 regression now distinguishes successful bounded admission from refusal of an oversized inverse during execution and verifies that the original bytes remain intact. The neutral large-sibling admission fixture already exercises the shared helper; the changed native pilot/catalogue checks remain queued and have not passed.

The latest read-only Details audit also found production external-reference/allOf schemas, unbounded missing-field scans, invalid template candidates, minProperties removal, and derived-path limits. The Details owner is implementing those corrections; the source findings are in `🔍️research/🪟️details-capability-audit-2026-09-27.md`. No browser behavior or full shipping-component success is inferred from these source changes.

The CSV React preview build ended after 57m36s without starting port 6212. Its cached contract had compiled before the admission method gained a shared default, while its later Markdown editor compilation read the removed override; Cargo reported E0046 for that mixed source checkpoint. The source trait now contains the default. This requires a fresh build and is not runtime acceptance. The native UI lock owner also exited; the queued native catalogue, contract and payload jobs remain pending. Only source edits continue until the pilot mounts are coherent, avoiding another build against a changing partial interface.

## Compact CSV and JSON Pilot Mounts

The CSV and JSON base PatchSnapshot leaves are now mounted in native aggregates, text/binary codecs, schema unions, and editor dispatch. Existing specialized cell/node mutations remain available. New neutral 2 MiB field fixtures drive native forward/inverse and codec laws and TypeScript independent JSON Patch/Ajv comparisons. The TypeScript run first failed two new tests because a relative shared schema reference resolved under the leaf URL. The shared patch schema now has an absolute canonical identifier, and all seven pilot references use it; CSV's optional discriminator also matches the actual camelCase aggregate value. The corrected Nx run passed strict typing and **72 tests / 179 assertions** (`compact-native-pilot-schema-green.log`). Native pilot compilation and runtime remain pending.

The previously missing CSV/JSON leaf files were found under the typo `🗟️artifacts` directory, alongside an earlier PNG duplicate; no concurrent cleanup is established. Their 15 authored inputs were preserved under `📥️inputs/🩹️misplaced-leaves`, and only those misplaced duplicates were removed after verifying every canonical counterpart exists. The canonical text codecs use byte-wise hex decoding to reject malformed Unicode without slicing panics.

With the CSV build stopped and host memory reporting 51% available, the isolated optimized full-catalog diagnostic was resumed with one Cargo job and an explicit two-hour build budget (`full-catalog-component-current-3.log`). It reuses its prior output directory, remains independent of the shipping component, and has no link or browser result yet.

## Guarded Archive Primary Editing

Both ZIP dialects now share required canonical nodeId/value/revision parsing and prefilled localized comment/name drafts with Apply, Discard, cancellation and failure labels. Entry targets derive from the current name, so reordering cannot silently retarget a rename; stale, duplicate and colliding names fault. Explicit revision tokens reject a changed saved comment. The native RenameEntry inverse remains safe because the UI refuses ambiguous names and name collisions. Empty comments remain valid; unchanged edits publish no event. These are source changes, not runtime confirmation.

A language-neutral archive fixture and native serde_json comparison cover reordered edits, exact inverse, malformed action arguments, stale targets, duplicate/destination collisions and comments. Window tests now cover the explicit draft structure and requested archive slice. Native execution classification for the new primary action remains under integration; no browser acceptance or archive native test pass is claimed.

The PNG pixel-region focused TypeScript Nx retry completed successfully after its owned stalled daemon process was stopped. The execution agent reports the exact run in `png-pixel-region-ts.log`; native PNG control/retained-job tests remain pending. Five media PatchSnapshot pilots are now mounted, and their shared neutral schema/oracle cases are authored; the expanded oracle gate has not yet been reported.

## Media Pilot Aggregate and Primary-Control Validation

The focused PNG primary pixel-region command completed through Nx after the owned stalled daemon tree was replaced with the no-daemon invocation: `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun x --no-install nx run @semio-tech/stdio-png:test -- --run …/patch-pixel-region/🧪️tests/🟦️.test.ts`. It exited zero in 44.1 seconds; the retained log is `🗑️generated/png-pixel-region-ts.log`.

The expanded compact-patch neutral oracle initially passed PNG/JPEG/TIFF and failed MP4/WAV because their newly authored leaf descriptors declared the text surface while leaving `textOpcode` null. Both descriptors now identify the real `patch-snapshot` opcode. The corrected `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun x --no-install nx run @semio-tech/stdio-snapshot-editing-js:test` run passed **77 tests / 232 assertions**, zero failures, and exited zero. The seven native-pilot rows cover CSV, JSON, PNG, JPEG, TIFF, MP4, and WAV with Ajv aggregate/leaf validation, `fast-json-patch` result comparison, absolute shared-schema identity, discriminator/tagging shape, descriptor opcode, binary protocol tag, two MiB unrelated payload preservation, exact inverse, and sub-one-MiB patch size. The retained log is `🗑️generated/media-pilot-patch-ts.log`.

The source inventory has 54 assigned media/spatial snapshot-editing roots. Five roots use the compact native patch fallback; **49 remain** on a full-snapshot fallback: 47 direct `snapshot_edit_set_snapshot` routes plus JPEG baseline and TIFF baseline semantic dispatchers whose unmatched paths still emit `SetSnapshot`. The implementation and bounded large-structure plan are recorded in `📓️media-compact-patch-rollout.md`.

Native pilot compilation/runtime is still pending. The existing media regression session **69896** remains pending and must not be represented as a pass. The queued MP4 Cargo process observed as PID 25866 had not reached test execution when this handoff was recorded. No additional aggregate was mounted before this native pilot gate.

## Fixture Runner Correction

Inspection of the ZIP test output exposed an important distinction: the generic artifact TypeScript `test` router builds declarations and checks module exports; it ignores the requested `--run` fixture path. Therefore the earlier PNG focused target success (`png-pixel-region-ts.log`) and first ZIP target success (`zip-primary-text-ts.log`) establish package smoke checks only, not execution of their newly authored feature tests. The shared snapshot editing 77-test / 232-assertion result uses its own real Bun test runner and remains valid.

The PNG and ZIP package `📜️script.ts` routers now explicitly invoke their feature tests after the package smoke checks, using the existing owned-process lifecycle. Those feature tests use Bun's test API. Corrected Nx runs are pending in `png-pixel-region-fixture.log` and `zip-primary-text-fixture.log`; no corrected feature pass is claimed yet.

ZIP's two native editor roots now declare set-node in their tool rosters and use the new shared BoundedNativeEditingEditor factory/proof integration. This source is uncompiled and still needs retained runtime/cancellation/history verification.

The corrected feature runners now executed successfully through Nx: ZIP **3 tests / 23 assertions** (`zip-primary-text-fixture-2.log`, exit 0) and PNG **2 tests / 6 assertions** (`png-pixel-region-fixture-2.log`, exit 0), in addition to their package checks. The first corrected runner attempt failed due to a misplaced workspace-helper import; the imports were verified on disk and corrected before these successful runs. ZIP covers Ajv command shape, independent JSON Patch expectations, reordered/stale/duplicate targets, Unicode byte limits and comment conflict tokens. PNG compares authored region outputs with pngjs and native PNG reopen in JavaScript. Native Rust retained execution, rendered controls and browser behavior remain pending.

Two new ZIP native retained laws are authored for base and ISO dialects: register the exact factory, load the neutral archive, dispatch a guarded rename, settle publication, undo/redo, and reopen saved Pack bytes. They have not run.

## Native Compile Feedback and Current Gates

The shared contract native run ended after 111 minutes with 12 test-compilation errors: six fixture include paths, four usize/u32 viewport arguments, and two ambiguous DslValue parses. Full rendered diagnostics were recovered from Cargo's fingerprint into `contract-native-current-2-rendered-errors.log`. The Details owner repaired all 12, verified each include path, and started one corrected replacement (`schema-constraint-contract-current-3.log`, session 63260). No contract native tests have passed yet.

The third optimized full-catalog run compiled the shared contract and reached ZIP, then failed on four new archive UI type/import errors. ZIP now uses the framework UI contract Label and raw tree builder explicitly. Its next attempt rejected a relative output directory before building; the corrected fifth run uses the same absolute ticket output and remains active (`full-catalog-component-current-5.log`, session 82092, one Cargo job). This is still no completed full-component/link-size result.

Adding strict checks to the new ZIP/PNG editor implementations exposed an existing ZIP TypeScript parser returning undefined for required normalized defaults. The parser now matches Rust's empty entries/comment/data defaults. The next ZIP target passed strict typing and its three current feature tests with 24 assertions (`zip-primary-text-fixture-4.log`); a fourth neutral default-parity test was added afterward and remains unrun. PNG strict typing plus its two feature tests passed (`png-pixel-region-fixture-3.log`).

ZIP's text surface identities no longer include editable name hashes, so a remote name change can reach the existing draft-conflict logic instead of remounting the text control. Action targets remain name-derived and revisions remain checked. These UI behaviors still require native/browser confirmation. The retained native ZIP tests are authored but unrun.

## Native MP4 and ZIP Test Cache Correction

MP4 session 69896 completed after 114m31s including queue/compile time. The one selected native payload law ran and failed with `snapshot-edit.schema-unregistered` for document discriminator `s.stdio.mp4` at `$.schema`; zero passed, one failed, 56 skipped. The media owner is repairing registration/discriminator correctness.

ZIP TypeScript run 5 was a cache hit replaying run 4 (3 tests/24 assertions), not execution of the newly added fourth default-value law. Its replay must not count as current validation. ZIP and PNG package targets now include their canonical artifact taxonomy and shared implementation sources in Nx input declarations, excluding package output directories. The package wrappers reject unsupported test selectors rather than silently ignoring them. A fresh noncached test run is required.

Draw coordination inspection: Drawing Cargo.toml already declares semio-s-artifact-stdio-xml. The current IO wrapper still calls unresolved `export::svg::v1_1::any` instead of the canonical `crate::standards::v1::subsets::any::io::export::serializers::artifacts::svg::v1_1::any`. No Draw source was changed. Reply via the app thread tool failed with “direct app-server input is not allowed for multi-agent v2 sub-agents”; no message was delivered.

ZIP fresh TypeScript run 6 passed 4 tests/25 assertions; run 7 passed 5 tests/37 assertions including UTF-8 byte boundaries, both with `--skip-nx-cache` and strict editor typing. An explicit unsupported `--run missing.test.ts` now exits nonzero before the package smoke suite, verified through Nx.

The first archive-order oracle run (run 8) failed because the TypeScript diff parser omitted its newly declared order field. The failing JSON Patch comparison exposed that omission; parser repaired and replacement run 9 queued. ZIP native primary session 51878 is running; no native assertion pass claimed. The native 88-editor catalogue session 14795 returned compilation failure on the earlier ZIP UI imports (already repaired in current source); its fixture-only 88-editor validation is not a native catalogue pass.

Optimized full catalogue component run 5 ended at 15m20s with two PNG compile errors: InteractiveJobCloseStep imported from the wrong crate, and patch_pixel_region mounted privately. The media owner is repairing both. Full shipping catalogue and browser acceptance remain unverified.

A native execution audit found that the current bounded native helper still invokes whole reducers and inverse/diff/apply atomically. Shared extension hooks for real ArtifactCommandWork and store preparation cursors are in progress; factory registration by itself is not evidence of bounded preparation or responsive cancellation.

## Archive Ordering Checkpoint

ZIP TypeScript run 9 and run 10 passed 6 tests/43 assertions with cache bypassed. Run 10 also strictly typechecked the real diff implementation. The schema, Rust and TypeScript sparse diff models now carry optional exact entry order; diff application no longer sorts names as a side effect. The independent JSON Patch oracle checks reversing entries, restoring their order, and refusal of missing/repeated/unknown members. Native tests for exact inverse/composed rename and both mounted viewport body keys are authored but have not executed.

The ZIP codec also sorted on decode and encode. Both now preserve authored member order; its rich codec fixture expectation was updated to the actual physical order. A native independent `zip` crate reader law checks exported member ordering and first-party reopen, using the existing version 6 test dependency only. This law has not executed. ZIP native removal inverse still appends the restored member and needs an indexed insertion mutation to restore its original position; that outstanding issue must not be hidden by unordered comparison. Other ZIP header metadata remains a writer policy and is not yet completely editable.

Full component run 6 stopped after 2m17s on a new shared contract compile error: `window_kit_snapshot_revision<S: dsl::ToValue>` uses an unresolved root module. Routed to the table owner for repair. No full-catalog native or browser success is claimed.

ZIP run 11 passed 7 tests/47 assertions plus strict editor/diff typing. Neutral insertion vectors now prove restoration at the original position against JSON Patch. The native base/ISO add mutations accept an optional `before` member anchor (omission means append); removal inverse captures the following member rather than appending. The independent base archive oracle now preserves insertion/removal positions too. Native execution remains pending; these source changes do not count as an executed undo law.

Native ZIP run 1 failed after 13m21s before assertions on the earlier shared contract `dsl::ToValue` alias. One warm replacement run 2 is active (session 6528) after that repair and the ZIP test-only `zip` crate declaration. Seven owned ZIP Rust files passed rustfmt parsing only.

Media worker recorded fresh uncached Nx passes in `📓️media-pilot-repairs-2026-09-27.md`: MP4 1/13, WAV 1/9, PNG 2/6. Native checks remain separate. Full component run 7 (session 7970) is running after both PNG import/visibility and shared contract alias repairs. Terra read-only audit slot is active again.

Full component run 7 failed at 8m49s on malformed DOCX strict text-codec escaping. The office worker replaced ad-hoc text escaping in all six DOCX/PPTX facets with the existing structured JSON codec and repaired PPTX newline literals, then parsed/checked all sixteen office editor roots. Full component run 8 is active (session to be recorded from orchestration). Native retained-route test run 27219 failed compilation; current Details test type/lifetime fixes are being checked in the existing contract lane, not counted as passing.

ZIP now mounts a retained target-resolution cursor in both dialect factories. It examines one bounded UTF-8 member name per turn, detects duplicate/stale/colliding targets, and retains scalar cursor state without copying member payloads. A native 2 MiB unchanged-owner law is authored; syntax parsing passed, compilation/execution pending. The store preparation still clones/inverts/applies atomically and remains an explicit unresolved performance issue.

ZIP run 12 passed 8 tests/55 assertions and strict editor/diff typechecking, with Nx cache bypassed. The TypeScript mirror now uses the same one-name-per-turn target cursor as native Rust; the neutral 2 MiB fixture checks yields, unchanged payload ownership, exact final mutation, and terminal refusal. Native mounted cursor compilation/execution remains pending. Root component run 8 is session 48510.

ISO stored/deflated insertion payloads and their reference oracle now also accept the following-member anchor; deletion inverse restores the original position, with a native first-member deletion law authored. These newly touched ISO/codec tests must be included in the next native ZIP verification beyond the primary editor filter. Other archive header metadata remains fixed writer policy and incomplete for the user’s full-detail requirement.

Native ZIP run 2 ended after 9m10s with one compile error in the new cursor: ArtifactOwnedToolJobContext is exported under `semio_framework_plugin::app`, not the crate root. Corrected the exact type path. The replacement is the whole ZIP native package with component-app-assembly (session 61632, `zip-native-all-current-3.log`), covering new codec/ISO/order changes in addition to primary editor laws. No native ZIP assertions have passed yet.

Shared Details test E0283/E0515 diagnostics reported by the other application-law chat were repaired by the owner (explicit DslValue and owned UiText). The single native contract replacement is session 73383, `schema-constraint-contract-current-5.log`; no pass assumed while pending.
