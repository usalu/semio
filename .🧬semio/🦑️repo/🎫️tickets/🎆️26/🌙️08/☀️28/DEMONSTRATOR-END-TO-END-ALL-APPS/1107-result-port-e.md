STATUS: GREEN on wasm32-wasip2 for 6 geometry crates (port-e-1), 2d-regions/booleans/2d (port-e-5), pixels-codecs + math-rank (port-e-9), puzzle-board-scene (port-e-12; logs in 🗑️generated/1105b/). UNVERIFIED: board engine + dag host (port-e-3: blocked upstream by pack-json/io-control), all crate tests.

## Done
- 2d-trace, 2d-path-flatten, 2d-path-stroke: ledger `reserve_vec`/`slot` take the allocation marker, threaded as `alloc:&mut Option<usize>` through `step`/`advance`; trace gets `BitmapTraceError::Refused(ValueError)` so no numeric-cursor refusal is dropped; trace retirement uses `retirement_seed`, `close_step(effect)->OriginalRetirementPoll`.
- 2d/🧹️retire `physical_work_retirement!`: `close_step(effect)->OriginalRetirementPoll` (all macro users now must pass an effect).
- 3d-brep-curve (+ surface, mesh-io): `retire_geometry_variants!` macro deleted, enums use `derive(RetireOwned)`.
- math-dense, 3d-mesh-contract: `retirement_seed` via array / `retirement_fields_*`.
- value pack lib: additive re-export `PagedListReleaseOutcome/Failure`.
- board engine `close_backing_page(maximum_bytes, effect)`; `GraphEngineBackingRetirementStep::Refused(ValueError)` (no more `expect`).
- dag host `DagInputApplication::step/advance` take `effect` (insert cursors).
- 2d-booleans (+paths): alloc marker threaded, `BooleanError::Refused(ValueError)`, Heap on `retirement_seed`.
- 2d-regions: sync close via `AuthoringWallet` (stock = existing STOCK, work 1e9); `with_standalone_effect` removed (runtime decision).
- math-rank: `ScalarRankCursor::advance/close_step(effect)`; refusals converted, pure steps record `performed(1)`. Energy `prepared` caller must pass an effect.
- pixels-codecs: RasterLease/PngEncodeJob/PngInflater on `retirement_seed`; `PngEncodeWork::close_step(effect)`; `PngDecodeJob::advance(effect)`; `ImageDecodeJob::advance(source,effect)`; blocking driver uses a wallet sized from the decode limits (stock sizing unproven at runtime).
- puzzle-board-scene: `Projection::take_retirement(effect)` returns the admission outcome, restores owners when not consumed.
## Not done
- Rust/TS tests of these crates still call the grant-based APIs (trace/path/booleans work tests, rank, png, scene); downstream callers (draw, flow trace, raster, energy prepared) need the new `alloc`/`effect` args.

## pack-record-* (coordinator 09:30, exclusive) -- NOT PROVEN
- Fresh run port-e-15 (rc 1): `pack-record-encode` 14 errors, `pack-record-retained` 21 errors; `-encode-cursor` and `-preflight` are not reached (they depend on encode). preflight's schema cursor was already migrated by lane dslowned (claim 09:36).
- Edited, uncompiled, direction = pass the caller's `effect` next to `control`: encode `🧮️expression` (`next`/`encode` take `effect`, `advance/close(&mut effect.nested(grant)).result?`), encode `🧭️symbols` (`push`, `note`, `retire_one(effect)` with typed reserve/release faults, `record(effect)` for pure pops).
- Open decision (needs runtime/coordinator): the controlled encoders (`encode_record_body_controlled`, `_into`, `encode_projected_record_body_into`, `encode_value_record_body_*`, `encode_document_controlled`, `intrinsic_document`) have no funded action; each needs an `effect` parameter, and all callers (replication codec, snapshot hosts) must pass the one from their `original.effect(..)` turn. Not started: borrowed `🧬️static` (order_push, retire_one), borrowed cursor/wire/document cursor (encode-cursor 59), retained receiving (`step` uses removed `native.receive_retained_work`; needs the NativeReceiver/`NativeLocalAuthority` slot shape as in pack-json receiving), retained `📦️document` (catalog/value `close_step(items,effect)`, `reserve_allocation(effect)`), record-body-cursor symbol reserves.

## pack-record-* -- BLOCKED on an effect provider for controlled encoders (10:30)
Threading `effect` through the 8 controlled encoder entries would break ~25 callers that hold only `control` (sqlite snapshot hosts, store codecs, intrinsic, print mutations, generated `__dsl` hosts); none has a funded action to take one from (they call `to_value_controlled(control)`-style closures, hosts lane owns the output-slot shape).
Needed from runtime2: `NativeEncodeControl::with_effect(grant, maximum_work, |effect| ..) -> Result<Option<R>, ValueError>` (the control already carries the work recipient/action), so entries keep their signatures and only Expr printing (`RetainedExpressionCursor` is effect-only) and scratch release (`retire_one(effect)` from close turns) use it. Page reservations under an already charged control can use `PagedList::reserve_one_cold(bytes)`.
Left as is (uncompiled): expression `next/encode(.., effect)`, symbols `push/note/retire_one(effect)`. No caller touched yet; nothing broken beyond the pre-existing red.

## pack-record-* -- PROVEN on wasm32-wasip2 (11:50, supersedes BLOCKED above)
- `NativeEncodeControl::with_effect` (value encode/🎟️effect; port-e-16 rc 0) and the one-shot mirror `NativeDecodeControl::with_effect` (decode/🏭️construction/📦️effect). Same OriginalWork sequence as the job provider / AuthoringWallet::run; None = unadmitted, nothing ran. NOT done: re-expressing `effect_allocated*` on it (they are the resumable settlement variants), and the cross-driver corpus law runruntime2 asked for.
- pack-record-encode (port-e-17), -encode-cursor + -preflight (port-e-18), -retained (port-e-21), and callers pack-intrinsic, pack-record, pack (port-e-24) all rc 0.
- Controlled encoders keep their signatures (expression printing uses `control.with_effect`; scratch pages use `reserve_one_cold` under the already charged control; scratch/inline/order release `retire_one(effect)`), so no sqlite host / store codec was edited.
- Retained: `RetainedRecordReceiving::step(source,native,effect)` (owed work recorded once via `effect.performed`, capacity charged on `native`), `close_step(effect)->OriginalRetirementPoll`, `RetainedValueDriver::{new,one,close_step}`, body cursor `reserve_allocation/close_step`, source `step/close_step`, storage admit/retire take `effect`. Intrinsic `RetainedIntrinsic*::{new,advance,advance_original_step,close_original_step,close_step}` take `effect`.
- Downstream NOT updated (will break, owners need `effect`): node-graph scene (surface), plugin reactor manifest metadata (uses RetainedIntrinsicBody), fem 2d/3d snapshot hosts (RetainedRecordReceiving::admit/step/close), crate tests.

- 12:10 sequence consolidated into `run_original_effect` (port-e-27 rc 0); still open: re-express resumable effect_allocated*/NativeSnapshotBodyWallet::effect_encoded, and the 4-driver corpus law (fixture beside original-effect-provider).

- 12:25 regression fixed (spent out-param); authoring law 6/6 (port-e-28), wasm port-e-29. Recorded as DEBT: effect_allocated* re-expression, 4-driver corpus law.
- 12:55 with_allocated_effect added (port-e-31 rc 0); allocating corpus case still part of the open law debt.

- OPEN REQUEST (reviewfix, not started, outside assigned scope): brep `TessellationJob::into_mesh` and `Body::compact` need fallible `try_into_mesh`/`try_compact` with stocks sized from entity counts (no admit-all, no expect/panic); bound `boolean_sync` usize::MAX budget. Needs coordinator assignment.
- 14:10 runtime2 repro denied_and_unperformed_effects_do_not_post_work: reproduced (port-e-36), fix edited (reconcile work claim at completion), proof run port-e-37 pending -- UNVERIFIED. Zero-performed case not yet in a corpus law.
