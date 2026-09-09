# Native PDF Cache Acceptance

The coordinator owns this gate after handoff from the Nx executor. It exercises `bun x nx run @semio-tech/stdio-pdf-rs:build --output-style=stream --verbose`, using a private ticket Nx cache/workspace-data directory and the shared ticket Cargo target. Cargo uses two jobs, no incremental compilation, and offline package resolution. No shared cache is reset or removed.

## Required Evidence

1. Finish the first ordinary Nx build and record its exact terminal status, task hash and complete staged deliverable inventory with SHA-256 digests.
2. Repeat unchanged and require a local cache hit.
3. Remove only the owner-marked PDF `dist/build` directory, repeat and require exact inventory/hash restoration from the local cache.
4. Add a unique harmless comment to PDF's production taxonomy Rust root, repeat and require a different task hash and cache miss. Remove only this task's exact marker, preserving any concurrent edits.
5. Add a unique harmless comment to JPG's production taxonomy Rust root, remove only PDF's staged output, repeat and require the baseline PDF hash, local cache hit and exact output restoration. Remove only this task's marker.

The original static transitive-pattern probe used a JPG test file. This runtime proof deliberately uses its production `🦀️.rs` root, which avoids ambiguity from excluded test inputs. Its match against the current normalized transitive source inputs must be checked before mutation.

## Current State

Initial baseline session 1951 failed with exit 1 before staging PDF outputs: 21 shared framework-plugin diagnostics from concurrent window configuration work. Raw output is `🗑️generated/pdf-native-cache-1-baseline.txt`. Nx workspace data is `🗑️generated/nx-root-pdf-native-proof`, cache is `🗑️generated/nx-cache-root-pdf-native-proof`, and the only output tree eligible for removal is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/dist/build`, after verifying its `.nx-artifact.json` owner. No source markers have been applied.

## Production Input Oracle

The independent minimatch check passed on the current baseline Nx graph (exit 0). Its conservative traversal includes all internal project dependencies, explicit input project references and all named target prerequisites, then examines every positive named/target input pattern. Across 53 projects and 2,527 unique positive patterns, the actual PDF production root matched three input occurrences and the actual JPG production root matched zero. Negative exclusions are deliberately ignored, so this proves that no collected positive pattern admits JPG. Receipt: `pdf-native-production-isolation.json` and `pdf-native-production-isolation.txt`. This is structural evidence; the actual native cache mutation/restoration gates remain pending.

The initial baseline reached native Cargo after all three generator commands ran successfully. Its provisional task hash was `308001354016428375`, read from the private Nx SQLite database in read-only mode. This is a pending hash, not a successful cache receipt. Final task-history records and output inventory will be recorded only after completion.

## Restored Library Consumer

`🧪️native-pdf-consumer/🔣️.json` defines a language-neutral metadata expectation. The adjacent Rust consumer links the restored PDF library and staged serde_json dependency, constructs the standalone definition, and compares artifact/factory metadata with that fixture. It must be compiled with rustc using only PDF `dist/build` and its `deps` subdirectory, then executed from ticket-generated output. It must not use the Cargo target directory as a library search path. Rustfmt passed; linking and runtime assertions remain pending the cache restoration gate. The two consumer input files are retained after completion.

## Baseline Stability Note

The initial PDF build failed while shared window configuration work was incomplete. The Nx executor is also reviewing native test-level routing in the common package helper. Those files are declared PDF build inputs, so the first hash may be superseded before its Cargo compilation begins. Treat the first completed build as cache population; establish a stable current-source baseline with a subsequent invocation before claiming reuse or applying a PDF/JPG marker. Do not interpret a new hash caused by these concurrent prerequisite repairs as a failed isolation test.

## Current Baseline Retry

Current-source retry session 42472 uses the same private cache and output ownership. Its log is `pdf-native-cache-2-current-baseline.txt`. Read-only inspection confirmed all eleven ConfigView initializers carry window state, WindowConfig lane matches are complete, fault conversion is imported and local boxed config futures no longer require Send. These repairs were concurrent external work; the coordinator did not modify their implementation. No PDF or JPG markers or output removal has occurred.

## Successful Compile, Failed Staging

Retry42472 finished Cargo dev compilation successfully in 8m29s, including the current shared framework-plugin. The enclosing Nx task then exited1 during output staging: the Cargo-reported `debug/deps/libsemio_framework_os_kernel.rlib` was absent when copied. No PDF deliverable tree was published and no cache receipt was accepted. Kernel emits both rlib and cdylib with unhashed filenames; a successor compilation was rebuilding those shared outputs after the Cargo lock was released. This is a race hypothesis being investigated by the Nx executor, not an established conclusion. Missing required link dependencies must not be silently omitted. Receipt: `pdf-native-cache-2-current-baseline.txt`; task hash `13893530662113076342`.

## Consumer Link Inputs

The current PDF public kind is `stdio.pdf`, contribution identity is `pdf`, and its single native codec factory is `stdio.native.pdf.v1`; these match the retained neutral fixture. The workspace uses Cargo’s separate Rust metadata mode (`unstable.no-embed-metadata = true`), so the restored consumer must supply matching rlib and rmeta inputs where required, mirroring actual Cargo compiler invocations. Its dependency search path is restricted to the restored `deps` tree. The current dev profile does not override panic strategy. This is invocation preparation; no linked-consumer pass is claimed.

## Eager Capture Regression

The Nx executor reproduced dependency replacement after compiler-artifact events with a schema-first fake-Cargo test: the old helper staged successor bytes. The corrected helper captures selected files immediately into a private directory, cancels and awaits Cargo on capture error, stages only captured bytes, and always removes its capture. The focused regression exits0 (`artifact-cargo-eager-capture-final-green.txt`), following the retained red receipt. The coordinator reviewed capture, cancellation, atomic publication and cleanup paths. Native retry3 (`pdf-native-cache-3-eager-capture.txt`) now tests the actual PDF build; the previous two failures remain classified separately. No PDF/JPG markers or staged-output removals have occurred.

The final eager-capture regression also covers a truly absent dependency while fake Cargo stays alive. The helper rejects with ENOENT, cancels and awaits that process within five seconds, preserves both previously published files byte-for-byte and leaves no capture directory. Receipt: `artifact-cargo-eager-capture-final-green-2.txt`, exact exit0. The active real PDF retry is session6068.

The common artifact test router was factored into `caching/📦️artifacts/🦀️rust/📜️script.ts` by concurrent support-boundary work. Its budgeted Nextest call now has the existing progress timer and finally cleanup. The combined observed-progress, fake-Nextest lifecycle, eager-capture failure handling and Cargo/jsonschema oracle passed with normal process termination (`artifact-native-progress-capture-final-green.txt`). This input changed while PDF retry3 was queued, so retry3 remains cache population rather than a stable baseline. The owner has confirmed helper/router edits are settled before the forthcoming unchanged-source proof.

## First Successful Native Publication

Retry3 completed with exact exit 0. Cargo finished in 38m11s including queue wait; ordinary Nx finished all four tasks in 38m14s with 0/4 cache hits. Task hash `3618721459512557588` has terminal success/code 0 in the private task database. Eager capture published 199 deliverables (573,381,895 bytes including the ownership marker). The complete owner-checked inventory and SHA-256 digests are in `pdf-native-population-inventory.json`. This is cache population: stable baseline, exact cache restoration and consumer acceptance remain pending. Read-only SQLite required immutable mode after task completion; the expected latest success row was present in the checkpointed database.

## Published-Output Consumer Pass

The retained consumer compiled with rustc using only `dist/build` and `dist/build/deps` search paths, paired PDF rlib/rmeta extern inputs, and the staged `serde_json-29b1d187bb150e8e` pair. Compilation and execution both exited 0. The executable constructed the standalone PDF definition and matched the neutral serde_json artifact/factory metadata. macOS `otool -L` showed only `/usr/lib/libSystem.B.dylib`, proving no Cargo target runtime dependency. Receipts: `pdf-population-consumer-build.txt` and `pdf-population-consumer-run.txt`. This proves the published closure is consumable; the same check must run again after cache restoration.

## Stable Generator Baseline

The Nx executor corrected observed unconditional generated-file rewrites. Shared writer and graph/schema/UI/styling source inputs are now settled. Its real generator probe processed 34 output files with zero content or mtime changes after generation, following a retained red/green neutral helper test. The fourth native PDF invocation, `pdf-native-cache-4-stable-writers.txt`, now establishes the baseline against these final build inputs.

The artifact executor subsequently removed measured component-only dependencies from default Puzzle/FEM package graphs and refreshed Cargo.lock. Invocation4 was already queued, so its hash may precede that legitimate lock refresh. Continue it normally, then require an unchanged current-source cache hit before treating any hash as stable.

## Restoration Ownership Procedure

`stageArtifacts` coordinates publication with a sibling `dist/build.lease` file opened exclusively (`wx`), containing owner and pid. Before the restoration probe removes PDF outputs, acquire that same lease exclusively, verify the exact owner marker plus complete baseline SHA-256 inventory, and atomically move only the verified `dist/build` tree into a unique ticket-generated backup. Release only this process’s lease before invoking Nx. This makes the disappearance reversible and prevents overlap with cooperating native publishers. Require exact inventory restoration and a local PDF cache hit; retain the backup until the probe passes. No source marker or output move has been performed yet.

## Refreshed Native Publication Pass

Invocation4 completed with exact exit0: Cargo dev41m00s and ordinary Nx41m04s including queue wait, all four tasks passed with zero cache hits. The task hash is `17171063120336117952`. The owner-checked output inventory contains 199 deliverables plus its marker, totaling 574,051,165 bytes. Complete SHA-256 inventory: `pdf-native-baseline-4-inventory.json`. Invocation5 repeats unchanged against current sources in the same private cache/workspace directories (`pdf-native-cache-5-unchanged-current.txt`). No PDF/JPG marker or staged-output move has occurred.

The private cache also persists an exact completed `run.json` receipt with each task hash, numeric status and `cacheStatus`; invocation4 was copied to `pdf-native-cache-4-run.json` after verifying its PDF hash and status. Subsequent proof stages will prefer these completed run receipts over active SQLite/WAL state. Invocation5 reached Cargo with a cache miss after the recorded Cargo.lock refresh; it remains a baseline refresh and no isolation verdict is inferred.

Invocation5 also retains its Nx file-map hash census (`pdf-native-baseline-5-workspace-file-hashes.json`) so a subsequent baseline miss can be checked for concurrent source changes. This census is diagnostic evidence; it does not broaden PDF source ownership or replace the explicit PDF/JPG task-hash and restored-output checks.

The current full GIS runtime replay exposed a shared Store retirement cycle between `tail_undo_cache` and displaced snapshots. The registry executor is repairing that actual runtime defect under the existing strict disposal tests. Store is a declared PDF dependency, so invocation5 may again precede a necessary shared-source change; require a subsequent unchanged hash/cache hit after the repair settles. Preserve the current invocation and its input census, and do not misclassify such changes as unrelated-JPG invalidation.

The required-nullable GIS repair also changed first-party value-derive input before the next PDF baseline. Store and value-derive source owners have now reported their scoped repairs settled. Invocation5 remains a publication refresh; establish the unchanged cache baseline after it finishes and these declared inputs are included in the new task hash.

The current effective native target graph also omits the actual local artifact router and its imported common Rust artifact router. The coordinator recorded an independent positive-pattern audit in `📓️native-local-router-input-audit.md`; the Nx executor owns the regression and correction. Finish invocation5 normally, then establish the final native PDF baseline only after this input repair settles.

The native local-router input repair has now passed ordinary current Gismap and PDF project graph checks. Each build/check/test target includes its actual local router and imported common Rust router; sibling router exclusion remains valid. The helper revision is settled. Finish active invocation5, then rebaseline on this corrected input contract and refresh the literal production-root isolation oracle.

## Interrupted Publication Refresh And Owned Recovery

Invocation5 returned exact exit 137 without a diagnostic identifying the termination cause. Its log stops at a build progress line, so neither successful publication nor a completed Nx cache receipt is accepted. Read-only inspection found orphan Bun 44067 (parent 1, exact PDF package working directory) and Cargo 44069 (child of that Bun, exact PDF manifest). Both were still queued. The coordinator sent SIGTERM only to those verified owned PIDs and confirmed both disappeared; no shared server or other native command was touched. Receipts: `pdf-native-cache-5-process-audit.json` and `pdf-native-cache-5-orphan-cancellation.json`.

The owner-checked 200-file published inventory remains byte-identical to invocation4 (`pdf-native-after-interruption-5-inventory.json`). No PDF/JPG marker or output move occurred. Fresh invocation6, session90282, uses the same private cache/workspace data and the final router-input contract. Its receipt is `pdf-native-cache-6-final-router-inputs.txt`; it is active and must supply exact terminal evidence before the sequential cache proof proceeds.

## Final Router-Epoch Production Isolation

The coordinator refreshed the full production-root oracle against the ordinary post-router graph. The conservative closure covers 31 projects, 34 target invocations and 2,322 unique positive input patterns, with zero unresolved groups or targets. Independent minimatch accepts the actual PDF production root and rejects the actual JPG production root; both the local PDF router and shared Rust artifact router are included. All candidate paths were checked to exist. Graph SHA-256: `816ac778bc18db7d6f116c7026ebb23408c00365ff989060e03b2302d4e4c372`. Receipts: `pdf-native-production-isolation-post-router.json` and `pdf-native-production-isolation-post-router.txt`, exact exit 0. This remains structural evidence; cache-hit restoration is pending.

## Disk-Full Population 6 And Retry 7

Invocation6 terminated with exact exit1 during the disk-full event. Native fingerprint writes, Nx SQLite and the private `run.json` write reported filesystem exhaustion; no current success/cache receipt is accepted. A fresh full filename/size/SHA-256 inventory proved the published PDF output remains exactly equal to invocation4 (200 files). The process audit found no surviving PDF-owned Cargo/Bun build; unrelated broad clippy commands mention PDF among their package selections and were preserved.

Invocation7 uses the same private cache/workspace and warmed Cargo target, with jobs2 and incremental compilation disabled. Raw output is `pdf-native-cache-7-resource-recovery.txt`. No PDF or JPG source marker and no output move has been performed. The acceptance sequence remains successful current publication, unchanged local hit, exact restoration, own-source invalidation, unrelated-JPG reuse and restored-output-only consumer execution.

## Population 7 Recovered Terminal Evidence

After the usage interruption, every prior process handle was missing and a fresh full process inventory showed no surviving native build. The completed PDF raw log reports successful native compilation and staging of 199 deliverables; its private completed `run.json` confirms all four tasks status0. PDF task hash is `7195935323700449431`, cache-miss; ordinary Nx elapsed40m18s including queue. The captured current owner-checked inventory has 200 files including marker, totaling575,334,356 bytes. Receipts: `pdf-native-cache-7-run.json`, `pdf-native-baseline-7-inventory.json`.

Invocation8 repeats the ordinary target on current sources in the same private cache/workspace. Because shared source changes continued during population7, it must establish current reuse before source-mutation tests. No source marker/output move has yet occurred.

## Population 8 And Shared Helper Baseline

Invocation8 completed with exact exit0, Cargo5m36 and ordinary Nx5m40, including queue. All four targets passed with zero cache hits. PDF task hash3875210145897375986 published199 deliverables plus its marker (200files,575,666,898bytes). Receipts: `pdf-native-cache-8-run.json`, `pdf-native-baseline-8-inventory.json`, `pdf-native-cache-8-current-reuse.txt`. A declared common test-helper signature changed between task hashing and compilation while repairing real Flow/Norm fixture failures. Invocation9 therefore refreshes that legitimate input baseline; it is active with a PDF cache miss. No PDF/JPG source markers or output moves have occurred.

## Invocation 9 Shared IO Compile Failure

Invocation9 terminated with exact exit1 after19m00s including queue. It failed in shared IO route ranking: seven E0277 diagnostics at IO lines2164,2209,2256 pass synchronous u8 fidelity ranks to `resolve_ready<F: Future>`. The GIS executor owns current-source inspection and removal of stale wrappers where applicable; no PDF package diagnostic occurred. The completed failed task receipt is `pdf-native-cache-9-run.json`, full diagnostics `pdf-native-cache-9-settled-helper-inputs.txt`. No source marker/output move occurred. Current publication remains the previous successful baseline unless a fresh owner/inventory check proves otherwise.

The GIS executor verified the imported fidelity/confidence rank methods are synchronous and removed only the three stale Future-wrapper expressions (three changed lines, all seven diagnostics). Invocation10 repeats ordinary PDF Nx build after this repair settled, in the same warmed Cargo target and private cache/workspace. No PDF/JPG source marker/output move has occurred.

## Population 10 and Graph Recovery

Invocation 10 passed all four ordinary Nx tasks with no cache hits and PDF task hash `15383820945192969713`. Its published closure contains 199 deliverables plus the owner marker (200 files), totaling 576,026,115 bytes. Native work took 23m19s; Nx took 23m27s including queue time. Invocation 11 failed during graph construction before native work, because a relocated JSON fixture import was stale. It did not move or alter published output. The import was subsequently corrected by another agent. Invocation 12 uses the supported `NX_NO_CLOUD=true` flag with the same private local cache to avoid remote onboarding waits; its result is pending.

Invocation 12 passed with PDF hash `2796437571782912483`, a cache miss, 200 staged files, and 575,985,277 bytes. Its task interval was 08:44:15–08:51:27 UTC (7m12s). The graph now succeeds. Sequential unchanged-source repeats 13/14 are underway; they perform an ownership-verified output backup and local restoration only after observing a real local hit. The independent PDF/JPG source-isolation script has been prepared but has not yet mutated a source or moved an output.

## Comparable Input Census

The invocation13 private Nx file-map snapshot contains 83672 files; 23802 entries differ from invocation10 in the same Nx-native hash format. Critical shared changes include `Cargo.lock`, `Cargo.toml`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`. A separate direct SHA-256 workspace census used a different inventory/hash algorithm and is retained only as a separate diagnostic; it must not be compared directly with the Nx-native decimal hashes. The incompatible draft comparison was replaced with the actual Nx file-map comparison. This evidence classifies shared input drift, not a completed cache-isolation pass.
