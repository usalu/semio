# Registered Full Snapshot Slot Feasibility

Read-only current-source plan, 2026-10-13. Parent reports actual Root Native29513 three selected laws passed and Source30690 four tests/154 expectations passed; no execution by this audit.

## Wrapper-Free Import Is Feasible With Pre-Admitted Moves

Use registered specialized `P` function pointer accepting caller's actual mutable Any loan of `Option<P>`. Before any native/SQL effects, downcast to exact Option<P> and require None. Wrong type/nonempty destination must return static literal cause and preserve destination. TypeId<P> provider identity/schema/dialect checks also precede admission. A mutable loan prevents concurrent destination replacement during synchronous operation.

Before invoking shared typed synchronous import, price all additional output transitions with original encode owner and SQL Port.work: moving P into caller Option<P>, constructing returned IoOutcome<()> / IoResult<()> shape and moving actual Vec<Diagnostic> header. These are item/copy/depth payments, not fictitious heap allocations. Charge precise actual moved headers/values; do not allocate a wrapper, Any box, Arc or new receiving authority. Prepay before constructing P; if prepayment fails destination stays None and original owner retains any already-required receiving frame.

On successful typed synchronous import, destructure IoOutcome<P>, perform only infallible P and diagnostic-vector header moves into destination/result. No checkpoint, grant check, SQL settlement or fallible callback may follow P publication into external slot. All owned diagnostic backing is already handled by typed producer; moving its Vec header does not allocate/copy contents. Return IoOutcome<()> with original diagnostics. On typed failure keep destination None and return exact original error/diagnostics; original producer already keeps rejected P in original native receiving frame when publication/close fails. Do not extract P from recipient to fabricate an output.

This prepay approach fits actual typed import: it prices own result headers/validation/snapshot moves before constructing its IoOutcome<P>, finishes body settlement, closes scratch and checks publication under original owner before returning. Additional caller-slot moves are distinct work and must be paid up front, but require no second enclosing snapshot owner. The public destination becomes readable only after typed success. Test body grants close to transfer demand to distinguish insufficient prepaid destination work from producer failure.

## Minimal Current Registry Changes

Current `🏪️store/🚪️io/🦀️.rs::ArtifactSqliteSnapshotCodec` has raw export/import pointers and optional snapshot_type. Add separate full file export/import-slot pointers, retaining raw bridges as distinct raw-domain capability. Generic sqlite_codec constructor specializes both to actual Self. Export uses borrowed existing ErasedSnapshotRead.get<P>(); that method requires P:Send+Sync+'static, so include appropriate actual constructor bounds (or a narrower host full capability constructor) rather than assuming ArtifactSqliteSnapshot alone implies Sync.

`identical_to` must compare added pointers/presence as well as schema/TypeId/raw pointers. Typed native_identity validator must bind same TypeId; guest TypeId None cannot acquire these host-native thunks. Actual registration rejects mismatched provider identities; borrowed registry lookup should preserve this and avoid cloning Codec/schema or formatting causes.

Factor actual async typed file bodies (no await) into synchronous typed implementations; current public async API simply calls the same implementation. Registered pointers call it synchronously, avoiding boxed Future and future-allocation custody. Do not implement duplicate P/R/physical pipelines.

## Public Store Export Owner Law

Actual `ArtifactStore::snapshot_read()` (`🏪️store/🦀️.rs:4956`) captures snapshot owner, publishes current generation/revision authority and issues original fixed registry lease. Call `.into_erased()` on returned typed read: that moves the exact original Arc/lease with no payload clone/allocation. Record borrowed get<P>() pointer before/after registered full export, and verify exact legal full fields. Store snapshot_owner makes Arc aliases internally; export must not clone that root again.

After export, `erased.try_into_typed::<P>()` returns same original typed read (mismatch returns original); use `try_return_to_registry_witness` for refusal-preserving original return. Store `take_returned_snapshot_read_retirement(grant)` (`:5584`) requires the actual installed initial snapshot retirement factory and handles returned lease maintenance. Keep returned witness until appropriate typed retirement; pump same Store-owned cleanup and close. Do not replace it with standalone erased read/type-specific synthetic lease.

## Required Neutral Laws

Actual registered legal TXT with arbitrary schema, embedded separators/NUL/Unicode, empty list versus one empty line and ending combinations must roundtrip exact full fields through caller slot. Independent SQLite inspects every row/metadata and physical file. Export original root pointer unchanged. Wrong Option type and Some destination refuse before heap/counters; wrong provider TypeId/schema/coordinate refuses likewise. Prepay all five grant axes, actual SQL/native cancellation, cumulative prior receipts, rejected P retained, zero/funded original cleanup and accepted destination with only infallible final moves. Source binds specialized full thunks and excludes raw native carrier conversions/clone/BoxFuture. This plan changes capability registration, not raw format fidelity semantics.
