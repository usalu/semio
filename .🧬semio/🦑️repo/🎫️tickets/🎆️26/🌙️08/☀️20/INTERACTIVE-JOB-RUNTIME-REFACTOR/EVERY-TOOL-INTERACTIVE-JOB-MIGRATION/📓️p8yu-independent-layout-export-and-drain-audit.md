# P8yu Independent Layout Export And Drain Audit

Date: 2026-08-22  
Scope: current Layout exporter and picker-capable segmented browser drain, source/static seam only  
Verdict: **PASS for this narrow source/static seam. Phase 8 remains RED.**

## Method And Boundaries

Read the repository instructions, attached master plan, P8yj/P8yk/P8yp/P8yr framework reports, P8yl/P8yn audits, and P8yo repair report. Re-attacked the current source rather than trusting those reports. No Cargo command, native/Wasm compilation, browser/Wasm execution, watchdog timing measurement, cancellation/close-under-load test, ticket lifecycle call, or modifying Git command was used.

The verdict is deliberately limited. It accepts the inspected static source seam and the non-Cargo gates below; it does not claim the unavailable-picker route, shared close cleanup, or any runtime acceptance gate is green.

## A. Layout Job And Codec — Accepted Statically

- The four registered Layout commands use the exact Layout-owned factory and `LayoutExportRequest` keeps the immutable `Arc<LayoutSnapshot>` rather than cloning the snapshot. The Wasm operation likewise increments the `Arc`, owns page/preflight input, returns a `WorkerJobSession`, exposes progress/cancel/one-chunk take, and cancels on unfinished drop. The direct `layout:out` reducer fails closed while the registered media factory constructs the same job.
- `Validate`, `Plan`, `Encode`, `Base64`, `PackageCommit`, and `Complete` are persistent stages. Validation has bounded collection/item/string/dimension caps; dynamic JSON/preflight uses the owned byte/token/container cursor (2,048 input bytes per advance), and typed JSON writes fields/strings in bounded credits rather than production `serde_json::Value`/whole-fragment serialization. Supplied package preflight is validated and copied byte-exactly in 4,096-byte units.
- PNG state retains row/fill/header/data cursors: initialization, checksum/CRC, output, and rectangle work are segmented. PDF emits catalog/page/object/xref/trailer units. ZIP emits local/data-descriptor/central-directory/EOCD units. Base64 consumes at most 3,072 raw bytes, retains only its small tail, and moves output incrementally.
- Each `InteractiveJob::step` validates operation/generation, checks cancellation before and after each unit, consumes fuel, yields, emits checkpoints every 64 units and previews every 16, and returns an empty terminal `CommitCandidate.output`. Factory contract is one work unit, 2 ms, 2 MiB raw input, 131,072 decoded items, and 32 MiB output. Current source tests express deterministic 1/2/4/default worker schedules, stale/cancel behavior, caps, checkpoint replay, and descriptor drainage, but Rust tests were not run.
- Checkpoint is a fixed 634-byte `LXC2` binary record: magic, kind, page flag, two length-prefixed zero-padded 256-byte authority slots, canonical revision, and six `u64`s. Decode requires exact length/padding/kind/page/document/revision/operation/base/generation equality. Restore replays bounded units and verifies completed-unit/output-length/digest before exposing a preview. No production whole checkpoint Serde/state clone was found.

## B. Chunk Authority And Terminal Protocol — Accepted After Re-Audit

- Layout's raw and encoded `ChunkRope`s preallocate 8,192 descriptor slots, emit at most 4,096-byte chunks, enforce the exact 32 MiB cap, remove consumed descriptors incrementally, and do not flatten on the interactive terminal path. The only flatten (`LayoutExportCommit::from_chunks`) is in named headless batch helpers/tests.
- Shared `ArtifactOutputChunks` uses an operation-identity `Arc`, exact `try_reserve_exact` descriptor admission, nonempty 1..=4,096-byte FIFO items, checked total cap, and `pop_front` length-to-zero drainage. Structural consumers require exact `Arc::ptr_eq`, sealing, and exact credited bytes. Its current tests include the former-growth 65-slot and drained `DropSentinel` cases; those Rust tests were not run.
- A real race was found during this audit: `push` held the queue lock but the then-current `seal` only performed an atomic CAS, allowing a producer that had passed its last sealed check to append after terminal sealing/`None`. The live repair was re-read before this verdict: `seal` now obtains the same nonblocking state lock before CAS and byte snapshot. Thus a concurrent producer/sealer fails closed with `interactive-job.segmented-output-busy`, and no `take_chunk` can observe terminal `None` before an admitted producer completes. The new `segmented_output_seal_is_linearly_ordered_with_push` source test directly holds producer authority, observes the sealer's busy fault, then confirms the successful push/seal byte count. `bytes` is monotonic total credit and `remaining` is advisory queue length; queue mutation itself remains protected by that same lock.
- The Rust/WIT route is exact: `take-segmented-download-chunk(u32, u64) -> result<option<list<u8>>, plugin-error>`. Guest error mapping preserves faults; the app removes segmented authority only after a returned `None`, so last `Some`, terminal `None`, then unknown-operation fault are distinct.

## C. Browser Bridge — Accepted Statically

- Only the exact marker `semio-segmented-handle-v1:identity|base64` enters the segmented route. The effect data is canonical positive decimal, parsed directly to `bigint`, rejects zero/noncanonical/overflow values, and is never coerced through `number`.
- Kernel, renderer handle, shard message, structured-clone client, generated worker, and generated JCO bridge require `takeSegmentedDownloadChunk(instanceId, operationId)`. Client and worker preserve `bigint`, reject invalid authority, move one `Uint8Array` of 1..=4,096 bytes per response, and propagate worker/WIT faults as rejection rather than `undefined`.
- ShellHost selects the active plugin and base-session instance, awaits exactly one read per loop iteration, preserves FIFO order, and sends the supplied abort signal. The picker sink writes each received bounded chunk directly with `showSaveFilePicker().createWritable()`; it has no Blob fallback. Absent picker throws `segmented-download-streaming-sink-unavailable` before reading producer chunks. Base64 handling retains a bounded tail and writes decoded subblocks.

## Non-Cargo Gates Reproduced

| Gate | Result |
| --- | --- |
| Layout three-file `rustfmt --edition 2021 --check` | PASS |
| Dedicated Layout Bun/Nx unit target | Not available: the Layout source fixture has no project target; its focused exporter tests are Rust/Cargo tests and were not run under the no-Cargo constraint |
| ShellHelpers focused Vitest | PASS, 1 file / 7 tests (rerun after queue repair) |
| `@semio-tech/framework-actor:test --skip-nx-cache` | PASS, 3 files / 52 tests |
| `@semio-tech/framework-kernel:test --skip-nx-cache` | PASS, 1 file / 40 tests |
| `@semio-tech/framework-renderer-react:lint --skip-nx-cache` | PASS |
| standalone SegmentedDownload TypeScript check | PASS |
| generated worker/JCO bridge bigint/forwarding assertions | PASS |
| `verify interactivity tool-jobs --self-test` | PASS, 46 clean |
| `verify interactivity` | PASS, DENY clean; one documented test-only blocking bridge |
| current seal/export forbidden-pattern assertions and scoped `git diff --check` | PASS |

The focused browser test covers marker and u64 parsing, picker absence, serial draining/terminator, base64 boundary, cancellation/error, item limit, and total cap. Actor tests cover transport authority and `bigint` structured clone. These tests do not prove a real component/Wasm/browser execution.

## Still Red / Not Claimed

1. Browsers without `showSaveFilePicker` deliberately fail closed; the required zero-dependency streaming fallback remains RED.
2. Shared cancellation and app-close cleanup remain externally RED: this audit does not accept collection teardown/close-under-load behavior.
3. Native Rust tests/build, Wasm/component generation/runtime, real browser picker drain, renderer typecheck, worker watchdog timing, real cancellation/supersession/drop isolation, and full max-envelope conformance were not run and are not claimed.
4. Phase 8 remains rejected by its broader full-operation/reserved/importer/global-payload requirements irrespective of this narrow pass.
