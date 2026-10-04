# Native Build Isolation

At21:19UTC the fleet native jobs showed Cargo/nextest heartbeats for10–15minutes but no live rustc process anywhere. The root contract Cargo child held its private target lock and the configured shared intermediate build lock plus145per-unit lock files. No compiler child was active. The shared repository uses unstable fine-grain-locking; this is evidence of stalled shared coordination, not proof of a specific Cargo defect.

Only this fleet’s own waiting jobs were interrupted; foreign processes and cache files were left intact. Root Source layout red1 ended130 without reaching a test verdict. A single isolated intermediate root is now used serially by our native checks: `🗑️generated/stdio-native-build-oct03`, passed through CARGO_BUILD_BUILD_DIR; each gate retains its own CARGO_TARGET_DIR. Root Source layout red2 runs first. No permanent build configuration was changed and no earlier blocked log counts as a pass.

The isolated rerun immediately advanced through dependency compilation. At21:22UTC two rustc workers were active in the isolated root (wasmparser/serde_derive), confirming it had escaped the stalled shared-unit state. An unrelated shared compile resumed after the fleet released its own waiters; no claim is made about its outcome.

## Current Shared Schema Extraction Coherence

The isolated native compile escaped the stalled shared build directory and reached real compiler diagnostics after 3m17s (`source-draft-layout-red-2.log`). The new canonical `semio-framework-dsl-record` imports were missing from the OS kernel manifest. The next run exposed its missing derive dependency and removed `os_dsl` schema/notation/control paths. Narrow repairs added the two existing first-party dependencies and used the canonical record/value owners in Pack CLI, the OS grammar and Store controls. Run 4 compiled past OS kernel and then found the same missing direct record dependency in the tool-run manifest; that declaration was repaired too. These are compile-coherence repairs needed by the editor test, not a new compatibility facade.

Runs 2–4 failed before the Source layout test body. They must not be counted as assertion-level red tests or editor validation. Run 5 is the next attempt. All generated logs remain under the ticket generated folder.

Run 5 was stopped by a newly added `SnapshotRetirementStep::Blocked` variant at the retained Text envelope match. Another concurrent worker had already added that arm by inspection, so root made no edit there. Run 6 passed OS kernel and tool-run, then exposed the plugin SDK missing Record/derive dependencies and references to now-private Store/Protocol facade types. Root repaired the exact compiler-reported SDK files to their canonical Value/Record owners, preserving each operation. Run 7 is the next compile attempt. No Source layout assertion has executed yet.

Run 7 passed OS kernel, tool-run and plugin SDK; it reached Stdio contract references to the old private kernel Value facade. The contract now declares Record directly, names canonical Value types, and its exported macros reference explicit Value/Diagnostic reexports. Run 8 compiled and reached the genuine Source layout assertion (`grow=false`, expected `true`). After the two-line layout repair, the entire 96-test contract library is running.

The Media execution repair reran the complete contract library as `source-contract-green-4.log`: **96 passed, 0 skipped**, Nx success, cache disabled. Root independently inspected the stored receipt. Source layout and the duplicate-member diagnostic law are now green at this current-source checkpoint. Browser acceptance remains pending the new component build.

## October 4 — Artifact Manifest Follow-through

A source/manifest audit found direct canonical Record references in12additional Stdio artifact packages without the manifest edge, plus a Semio derive reference whose generated Record code needs the same owner. The exact13manifests now declare the existing first-party Record dependency;10also declare their directly referenced Record derive. This is a source-coherence repair for the catalog linking gate, not a new runtime library or a compatibility facade. Packages: EPW, MP4, SVG, MP3, IFC, BCF, JPEG, AVI, WAV, STL, DXF, BMP, Semio. Native catalog validation is still pending; the changes do not establish these formats’ runtime acceptance.


## October 4 Preview Integration

Preview 36 stopped on a stale generated Cargo lock. Registered lock refresh passed, and preview 37 reached four XML editor private Value imports; the ownership lane corrected those canonical owners. Preview 38 then compiled past XML and stopped on a duplicate artifact attribute on PlaybookSpec. Root removed only that identical duplicate attribute; the missing generated constants were downstream of the failed derive. Preview 39 is building with the separate stdio-wasm-build-oct04 intermediate directory. No current-component browser success is claimed.

## October 4 Preview 39–40 Integration

Preview 39 stopped at ZIP private former OS imports and missing direct Record/derive package dependencies, plus a STEP dependency concurrently repaired by another worker. Root repaired ZIP imports and eleven remaining artifact manifest omissions. A narrow follow-up removed identical concurrent duplicate dependency lines only. The full Stdio inference/test qualified Value scan is recorded in `../report/canonical-stdio-inference-imports-oct04.md`.

Preview 40 stopped before current component compilation: concurrent dependency edits made the root lock stale and a duplicate Record dependency in the shared Stdio contract broke Cargo/Nx parsing. PDF attempts 4 and 5 also stopped in graph creation for this same duplicate. The exact duplicate is removed; registered lock refresh 8 passed. No native or browser acceptance is inferred from these preparation gates.

Disk briefly fell to 249 MiB free, so the fleet held new native work. Inspection found only 3.2 GiB in this ticket's generated outputs, mostly active build prerequisites. Free space recovered externally to 15 GiB; root deleted no files. Existing inputs, reports, and active compiler directories remain intact.

Preview 41 stopped at a missing pixels-to-2D direct dependency; its owning worker had already repaired the manifest when root inspected it. Preview 42 then exposed a JSON TryInto inference error, likewise repaired concurrently before root edited it. Root made no change to either of those fixes. Preview 43 stopped at a newly stale hub lock; the next registered lock refresh is active. Five additional grouped imports in Stdio sources/tests were canonicalized, listed in the import report. These failed builds do not establish browser behavior.
