# P8yo Layout Resumable Exports Final Repair

## Verdict

Status is intentionally split:

- **Layout exporter source: COMPLETE** for the P8yn source-visible blockers. It has persistent bounded JSON cursors, row-segmented PNG work, incrementally destructed chunk storage, chunk-owned terminal output, and fixed-size authority-qualified checkpoints.
- **Picker-capable browser drain: SOURCE/FOCUSED TEST COMPLETE**, but not a real browser/Wasm end-to-end runtime claim.
- **Browser without `showSaveFilePicker`: RED.** The route fails closed; the required zero-dependency streaming fallback does not yet exist.
- **Native/Wasm/runtime/watchdog acceptance: NOT RUN** because Cargo was prohibited under disk pressure.

No interactive terminal path flattens or reserves the legal 32 MiB output. The only whole snapshot clone, whole JSON parse, and output flatten are explicitly confined to the named headless batch adapter.

## P8yn Blocker Closure

### 1. Whole-value package serialization and entity clones

- `JsonValidationCursor` persists byte, token, container, node, string escape, and depth state and examines at most 2,048 source bytes per advance.
- `TypedJsonCursor` emits the complete `LayoutSnapshot` schema field by field. Static/scalar/string leaves emit at most 1,024 bytes per advance. Collection nodes retain indices, and drawing nodes retain a bounded path.
- Package document, manifest, missing-link report, generated preflight, and supplied preflight all use durable cursors. Supplied preflight is copied byte-exactly in at most 4,096-byte slices.
- Package entity records are borrowed from the immutable `Arc<LayoutSnapshot>`; they are not cloned and sent through Serde.
- A nonempty manifest link hash now uses `StringSource::LinkHash` directly. Only the fixed-size derived SHA-256 text is owned when a hash must be synthesized.
- Production source contains no `json_fragment`, `append_json_item`, `serde_json::Value`, `serde_json::to_vec`, or `serde_json::to_writer` path. Serde whole-document operations remaining in this file are in tests and the explicitly named headless batch adapter.

Relevant source: Layout export component lines 43-47, 276-610, 669-1161, 1182-1965, and 2867-2942.

### 2. Whole-fragment JSON validation

- Data-fields JSON and preflight JSON are scanned incrementally by the owned lexer/schema cursor.
- The cursor caps the legal fragment at 64 KiB, decoded strings at 8 KiB, structural nodes at 2,048, depth at 64, and input examined per advance at 2,048 bytes.
- Typed package emission is independent of validation and is likewise cursor-bounded; no legal 64 KiB value is parsed into a whole tree or traversed in one unit.

Relevant source: Layout export component lines 22-48, 276-610, 1101-1161, and 2385-2550.

### 3. Terminal 32 MiB allocation/flatten

- `ChunkRope` pre-admits all 8,192 outer `VecDeque` slots once and owns output as at-most-4,096-byte chunks. Appending across former geometric growth boundaries never reallocates or copies prior chunk descriptors.
- Base64 reads through `take_prefix`, which removes each fully consumed raw descriptor in bounded work. Package commit `pop_front`s one final descriptor per advance. Thus both raw and encoded ropes have zero live descriptors before terminal job drop; completion does not hide an 8,192-descriptor destructor walk.
- `PackageCommit` moves one owned chunk per advance into `ArtifactOutputChunks`, credits that exact chunk, seals once, and returns an empty `CommitCandidate.output`.
- The current shared `ArtifactOutputChunks` uses an exact-reserved `ArtifactFixedQueue<Vec<u8>>`: producer pushes cannot grow/copy the queue, FIFO draining reduces live length to zero, and its source test uses 65 former-growth slots plus a `DropSentinel` to prove fully drained teardown does not revisit capacity.
- Tool downloads and structured Media clone only the small shared queue handle; the interactive path never concatenates output.
- Structured Media credits each payload chunk plus the exact schema length. Framework terminal validation compares `schema.len() + chunks.bytes()` with the credited total and the contract maximum.
- Direct Layout Wasm exposes `takeResultChunk`; it does not expose or rebuild a whole result buffer.
- Whole flattening remains only in `run_layout_export_headless_batch` / `LayoutExportCommit::from_chunks`, whose names and scope identify the batch-only boundary.

Relevant source: Layout export component lines 138-195, 2110-2205, 2308, 3075-3110, and 3418-3479; Layout Wasm component lines 263-377; framework plugin component lines 11355-11470 and 11620-11710.

### 4. Max-row PNG checksum/DEFLATE/CRC

- PNG rows persist across `Initialize`, `Fill`, `Header`, and `Data` states.
- Initialization grows at most 4,096 logical bytes per advance; rectangle fill touches at most 256 pixels; DEFLATE header is five bytes; Adler, IDAT CRC, and row data each scan/append at most 4,096 bytes per advance.
- A legal maximum row may remain in memory, as P8yn allows, but it is not checksummed, framed, CRC-scanned, or appended as a single worker unit.

Relevant source: Layout export component lines 2040-2088 and 2774-2848.

## Fixed Binary Checkpoint

Checkpoint and restore no longer clone scanner state or invoke Serde. The `LXC2` codec is exactly 634 bytes:

- 4-byte magic
- 1-byte export kind
- 1-byte page-presence flag
- two 258-byte fixed authority slots (2-byte length plus 256-byte storage)
- 64-byte canonical base revision
- six little-endian `u64` fields: operation, base revision, generation, completed units, output bytes, and output digest

`restore()` validates exact length, zero padding, kind, page, parent-document authority, canonical revision, operation, base revision, and generation. It then reconstructs all operational cursors by bounded deterministic replay and verifies the checkpoint's progress, output byte count, and rolling digest before resuming. `checkpoint()` is fixed work and does not copy input JSON, snapshot entities, lexer stacks, or output chunks.

Relevant source: Layout export component lines 40, 49, 2325-2346, 3250-3361, and 3784-3822.

## Interactive Construction and Lifetime

- Wasm export methods take their owned page/preflight strings by value and move them into the request.
- The interactive snapshot handoff is an `Arc` increment, not a whole snapshot clone.
- Output naming copies at most 128 characters.
- The request validates bounded page/parent authority and a canonical 64-byte hexadecimal revision before admission.
- Cancellation is checked before and after bounded advances. Dropping `LayoutExportOperation` cancels unfinished work.
- Tool and Media completion guards reject duplicate completion, while operation/generation checks reject stale work.
- The only `Arc::new(snapshot.clone())` occurs in `headless_batch_export`.

Relevant source: Layout export component lines 2262-2346, 3100-3160, 3363-3416, and 3438-3450; Layout Wasm component lines 244-292 and 313-377.

## Required Host/Wasm Segmented Drain

The browser boundary is required end to end:

`PluginWasmHandle.takeSegmentedDownloadChunk(instanceId: number, operationId: bigint): Promise<Uint8Array | undefined>`

- WIT authority is `u64`; marker data is parsed as canonical positive decimal directly into `bigint`, bounded to `u64::MAX`. It is never coerced through `number`.
- Kernel, renderer handle, shard request, structured-clone transport, generated worker, generated jco bridge, and fake handles use the required method. No optional fallback and no numeric-ID ordinary download path exists.
- Worker and client validate the authority and reject zero/overflow.
- Every RPC returns exactly one `Some` chunk of 1..=4,096 bytes or the one `None` terminator. Rust/WIT errors stay errors; the generated bridge does not synthesize `None`.
- Shard disposal and worker failure reject pending reads.
- ShellHost recognizes only the exact versioned marker family, binds the active plugin instance, awaits one producer RPC at a time, preserves order, and aborts the sink on cancellation/error/unmount.
- The sink requires `showSaveFilePicker().createWritable()`. Where that real streaming authority is absent, the route fails closed before producer draining; it never assembles a Blob on the UI continuation.
- Base64 decoding retains only a sub-block tail and writes bounded decoded chunks.

Shared Rust/WIT seam observed in current source: `take-segmented-download-chunk(u32, u64) -> result<option<list<u8>>, plugin-error>`, canonical fault mapping, and authority removal only after observed `None`. These shared framework changes were owned by the framework repair, not this Layout repair.

## Files Changed by This Repair

Layout-owned:

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`

Authorized TypeScript bridge/drain:

- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/SegmentedDownload/🟦️component.ts` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🧪️component.test.ts` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`

One intentionally retained shared seam change:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11408`: `pub fn new(maximum: usize) -> Self` for `ArtifactOutputChunks`.

No other shared Rust framework source was edited by this repair after ownership was reassigned.

## Gates Run

Passed:

1. `rustfmt --edition 2021 --check` on all three modified Layout Rust sources.
2. `git diff --check` over all Layout and authorized TypeScript files.
3. Forbidden-path static scan: no production `json_fragment`, `append_json_item`, `validate_bounded_json`, `serde_json::Value`, whole checkpoint Serde, terminal `reserve_exact`, `takeResultData`, terminal `CommitCandidate.output` payload, growing `Vec<Vec<u8>>`, indexed descriptor tombstones, or Blob fallback.
4. `bun x vitest run …/ShellHelpers/🧪️component.test.ts --environment jsdom`: 1 file, 7 tests passed. Coverage includes exact marker parsing, canonical/max/overflow u64, no-picker fail-closed behavior, sequential ordering, last-Some then None, cross-boundary base64, cancellation, unknown-operation error, empty/oversized item rejection, and total cap.
5. `bun nx run @semio-tech/framework-actor:test --skip-nx-cache`: 3 files, 52 tests passed. Segmented cases cover order, exact authority, unknown operation, empty/oversized response, actor-dispose cancellation, real `structuredClone` preservation of `2^53+1` and `u64::MAX`, zero, and overflow.
6. `bun nx run @semio-tech/framework-kernel:test --skip-nx-cache`: 1 file, 40 tests passed.
7. `bun nx run @semio-tech/framework-renderer-react:lint --skip-nx-cache`: passed.
8. Standalone `bun x tsc --noEmit --skipLibCheck --target es2022 --module esnext --moduleResolution bundler --lib es2022,dom …/SegmentedDownload/🟦️component.ts`: passed.
9. Bun evaluation of `shardWorkerSource()` and `pluginComponentBridgeSource()`: all four assertions passed for bigint validation, `u64::MAX` bound, exact worker forwarding, and exact `jobs.takeSegmentedDownloadChunk(instanceId, operationId)` forwarding.
10. Current shared queue static reinspection: exact `try_reserve_exact` admission, no-growth push cap, FIFO length-to-zero terminal drain, 65-slot former-growth test, and generic `DropSentinel` test are present.

Source tests added but not executed because Cargo was prohibited:

- Exact 32 MiB output fills every pre-admitted slot, rejects max+1, drains all 8,192 descriptors, and asserts zero descriptors/front cursor before drop.
- A forced 257-chunk sequence crosses former power-of-two growth boundaries while asserting stable backing storage.
- Shared `ArtifactFixedQueue` fills/drains 65 exact slots and proves queue drop does not revisit already dropped sentinels.

Not green:

- `bun nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache` remains red with broad pre-existing renderer debt. Examples include missing `TutorialUiSnapshot.interactionSelection`, missing UI exports, Three.js value/type failures, and existing ShellHelpers/PluginRuntime imports. A focused diagnostic filter found no error in the new `SegmentedDownload` module and no `takeSegmentedDownloadChunk`, segmented-download, bigint, or operation-ID diagnostic. The standalone new-module typecheck, actor transport suite, kernel suite, and renderer contract lint are green.
- Whole-file `rustfmt --edition 2021 --check …/plugin/🦀️component.rs` is red because that shared aggregate and its included modules have broad pre-existing/import-order formatting drift. The three Layout-owned Rust files pass rustfmt check. The shared queue seam was source-inspected and `git diff --check` passed, but no claim is made that the aggregate shared file is rustfmt-clean.

## Residual Acceptance Work

- No Cargo command was run. Rust compile/test, Wasm compile, native debug/release, real component regeneration, and real browser/Wasm runtime execution remain unverified in this repair.
- P8yn's serialized native/Wasm conformance, real 1/2/4/default pool dispatch, first-poll pending, stale/duplicate/cancel/drop isolation, byte determinism, max/max+1, checkpoint/replay, and <=8 ms watchdog measurements still require a disk-safe Cargo window.
- Cross-browser segmented download completion remains **RED** where File System Access is unavailable. The current route deliberately rejects with `segmented-download-streaming-sink-unavailable`; the final plan still requires a zero-dependency Worker/ServiceWorker + `ReadableStream` download transport with bounded UI continuations.
- Shared framework cancellation/worker-close cleanup under load remains an external close blocker owned by the framework repair. Terminal fully drained queue teardown is fixed, but cancellation/close cleanup acceptance is not claimed here.
- Whole parse/clone/flatten behavior intentionally remains in named headless batch adapters and tests only; it is not an interactive route.
- The repository was highly concurrent and dirty. This repair preserved unrelated edits and used no modifying Git command.
- No ticket API operation was performed, per instruction.
