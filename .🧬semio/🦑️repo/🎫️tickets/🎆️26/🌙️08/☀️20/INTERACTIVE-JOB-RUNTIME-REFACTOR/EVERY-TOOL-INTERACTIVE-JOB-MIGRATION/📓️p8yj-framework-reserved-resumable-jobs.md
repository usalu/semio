# Phase 8 Framework Reserved and Resumable Jobs Closeout

Date: 2026-08-22  
Packet: p8yj  
Verdict: **REJECT for Phase 8 acceptance; ready for an independent audit of the fail-closed foundation only.**

## Scope and governing evidence

This packet continued the shared framework foundation described by the attached Phase 8 plan and `p8yh-independent-framework-acceptance-audit.md`. It intentionally does not claim completion from handler-only jobs, envelope-only progress cursors, or Rust-only segmented-output symbols.

No Cargo command was run because the packet explicitly prohibited it under disk pressure. No ticket API or modifying Git operation was used. Concurrent Layout source was inspected but its plugin files were not edited by this packet. The concurrent change making `ArtifactOutputChunks::new` public was preserved.

## Implemented foundation

### Domain-owned exact proof catalogs

The shared plugin host now exposes `ArtifactBoundedFirstStepProof` and `bounded_first_step_tool_proofs!`. Each row carries the owner type/file, controller, document schema, factory, tool id, and explicit five-value execution contract. `ToolOwnerWitness` is compiler-derived from the owner type; the constructor does not accept a caller-forged witness.

The previous central `BOUNDED_FIRST_STEP_PROOFS` table and fixed nine-row verifier condition were removed. Runtime catalog collection is dynamic, requires `N > 0`, rejects duplicate or extra rows, and requires an exact bijection with real `Migrated` declarations. The existing nine handler proofs are now owner-local and file-disjoint:

- Draw: 1 (`canvasPointerDown`)
- Flow: 2 (`duplicateWidget`, `duplicateWidgetStep`)
- Forms: 2 (`setTryValue`, `setTryValueStep`)
- Remodel: 4 (`runReconstruction`, `retryStage`, `runStage`, `advanceReconstruction`)

These nine rows are deliberately **not execution authority**. Production registration count is zero because the shared `dispatch_typed_command_inner` still performs material preparation before `WorkerJobSession` and presence/transient/emit application afterward. The verifier therefore reports all command rows as unproved until the complete prepare/job/commit operation is bounded.

Negative verifier fixtures cover omitted proof, duplicate proof, forged owner, forged schema, forged factory, extra proof, declaration without proof, copied-owner witness loss, and runtime proof identity loss.

### Reserved routes fail closed

The generic framework reserved factory activation was removed. The verifier now rejects the existing wrappers because they cursor raw envelopes and return raw clones while the real history/config/import work remains outside the persistent job. It also rejects:

- whole `decode_op(&self.raw)` in a single configuration stage;
- post-job history/clipboard dispatch;
- whole emit serialization outside the worker;
- envelope-only cursor jobs and post-job monolithic operation fixtures.

Exact fail-closed reserved routes: `undo`, `redo`, `commitCheckpoint`, `createAlternative`, `switchAlternative`, `checkoutCheckpoint`, `revertToCommand`, and `configuration-binary` (8).

### Import preparation fail closed

The verifier rejects `dispatch_import_media` because it performs `serde_json::to_vec(&(port, media))` and clones the raw envelope before constructing the app-owned job. The 35 pending importer owners remain fail closed and must receive owned/`Arc` media authority directly, with any encoding cursor inside the concrete job.

### Close cancellation fail closed

The verifier rejects `Drop for VcsArtifactApp` because it calls `resolve_ready(self.tool_cancellations.cancel_all())`, whose whole-map drain is not bounded in a UI/drop callback. Required repair is O(1) scope-generation or parent-token cancellation plus asynchronous bounded cleanup and saturation/close tests.

### Segmented output seam

The Rust shared seam remains source-complete at its operation-owned boundary:

- `ArtifactOutputChunks` uses exact `Arc` identity and `VecDeque` storage;
- every pushed chunk is capped at 4096 bytes;
- total bytes use checked addition against the operation cap;
- sealing is explicit;
- draining uses O(1) `pop_front`;
- the wire marker is bounded as `semio-segmented-handle-v1:<original-encoding|identity>`;
- batch-only conversion is the only flattening path;
- `plugin_take_segmented_download_chunk(instance_id, operation_id)` exposes one bounded chunk or `None`.

This is **not end-to-end complete**. `ShellHost` still forwards the numeric operation id to `downloadMediaExport`, while `ShellHelpers` only handles its existing base64 behavior. A separate shared host packet must add exact marker recognition and a `takeSegmentedDownloadChunk(instanceId, operationId)` drain loop/sink before browser downloads can be certified. The verifier must not infer end-to-end completion from Rust symbols alone.

## Canonical census

The generated ledgers are byte-identical:

- `p8yj-current-command-ledger.json`
- `p8yj-canonical-diff-check.json`
- SHA-256: `d154d9b75394b827e778d92be7aa2c5e3c66d98397f8f46b1e027a03b5c86a0d`
- bytes: 310,953

Exact current counts:

| Inventory | Count |
|---|---:|
| Macro host files / invocations | 50 / 50 |
| Macro command rows / unique rows | 775 / 773 |
| Literal registrations | 656 |
| Live registered command rows | 884 |
| Admitted complete operations | 0 |
| Owner-local handler proof rows | 9 |
| Production `ToolJobFactory` implementations | 11 |
| Bounded-first-step production activations | 0 |
| Typed dispatch sites / aliases | 3 / 4 |
| Framework reserved residual routes | 8 |
| App-owned importer residual owners | 35 |
| Process-global payload-store candidates | 34 |
| Verifier negative self-tests | 38 |

The complete per-row command ledger, exact importer owner list, exact global-store file/line inventory, factory inventory, and failure array are in `p8yj-current-command-ledger.json`. `p8yj-importer-cohorts.json` retains the exact 36-owner, three-cohort planning inventory (35 pending; Puzzle5d is the sole non-pending owner in that inventory).

## Gates executed

| Gate | Result |
|---|---|
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS, 38 clean |
| `bun ./📜️script.ts verify interactivity` | PASS, DENY clean; one allowlisted blocking bridge finding |
| `bun ./📜️script.ts verify interactivity tool-jobs` | Expected FAIL-CLOSED: 0 admitted, 884 remaining, 8 reserved, 35 importers, 34 globals |
| Two `--format json --output ...` generations plus `cmp -s` | PASS, byte-identical |
| `git diff --check` on the shared script/host and four owner catalog files | PASS |

Native compilation, Rust tests, Wasm compilation/runtime, browser drain behavior, watchdog timing, cancellation saturation, and close-under-load tests were not run and remain mandatory acceptance gates.

## Files changed by this packet

- `/Users/ueli/Documents/semio/📜️script.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- Draw, Flow, Forms, and Remodel editor `component.rs` owner files listed above
- `📊️p8yj-current-command-ledger.json`
- `📊️p8yj-canonical-diff-check.json`
- this report

## Acceptance blockers

1. Move all typed-command preparation and commit-candidate application into a bounded persistent operation state machine.
2. Replace the eight reserved envelope wrappers with concrete jobs that own their real prepare/scan/decode/commit-candidate work.
3. Remove importer whole-media pre-serialization and pass operation-owned media authority into concrete jobs.
4. Replace whole-map Drop cancellation with O(1) scope cancellation and bounded asynchronous cleanup.
5. Complete the ShellHost/ShellHelpers segmented handle drain and verify bounded browser streaming.
6. Classify/replace the 34 global payload stores and migrate or explicitly fail-close all 884 command rows.
7. Run the prohibited-in-this-packet native, Wasm, runtime, saturation, and timing gates.

The packet is suitable for independent review of the scalable proof-catalog design, strict fail-closure, ledger reproducibility, and Rust segmented sink. It is not suitable for Phase 8 acceptance and must remain REJECT until the blockers above are closed.
