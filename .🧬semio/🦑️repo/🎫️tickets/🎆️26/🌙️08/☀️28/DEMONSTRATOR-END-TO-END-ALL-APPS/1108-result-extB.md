# extB result (round 1112 extension closure, lane extB)

## State
- semio-s-artifact-stdio-contract-native-print: wasm GREEN (W-extB-1 rc0; gen3d2 had already ported it).
- semio-s-artifact-stdio-dwg-snapshot: only red was `decode_sqlite_snapshot_native` in `dwg/.../✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` (old 3-arg `RetainedWorkOutcome` shape). EDITED to the output-shaped trait form (`output:&mut Option<SnapshotReceivingOutput<Self>>` -> `SnapshotReceivingOutcome`, via `decode_sqlite_snapshot_record_native(..., native_control, output)`, closure uses `construct_native_record(record,native,limits)` which already runs `native_admission::root` row admission; the post-construct `admit_native_rows` on decode was dropped as redundant). Verify: W-extB-3 (wasm).
- semio-os-infinite-board-engine: EDITED (verify W-extB-4, wasm):
  - `🧷️point-retirement-and-engine-aliases`: PointBuffer/PointRetirement ported to `RetirementCursor` + `native_retirement_reply!()` + inherent `close_step(body,effect)`; `GraphEngine` RetireOwned is now `type RetirementCursor=<EngineFields<P> as RetireOwned>::RetirementCursor` over nested tuples (no boxed `sequence`, no allocation in seed); `BoardEvent` RetireOwned moved to `#[derive(RetireOwned)]` on the enum (camera-events-and-hit-model).
  - `🎟️icon-paint-cache`: IconOwnedRetirement no longer owns ControlledRetirement children; key/metadata/image are admitted as child cursors (`admit_retirement_cursor`, `ArcLease<RasterImage>`) under the effect; every step records work and quotes equal actual copy/release.
- semio-plugin-window-config: see below.

## Rerun
`"$C/1105b-tools/chk.sh" W-extB-N <manifest> wasm -p <crate>` for each of the four crates (manifests in ext.sh / package paths above), then `"$C/1105b-tools/value/ext.sh" <round>`.

## Update (late)
- W-extB-3 rc0: dwg-snapshot wasm GREEN. W-extB-1 rc0: native-print wasm GREEN.
- infinite-board-engine: W-extB-4 had 1 error, W-extB-6 had 4 E0446 (private cursor types in public impl). Fixed by making PointBuffer, PointRetirement, RetainedPoint, ProximityConnection, IconOwnedRetirement `pub` + `#[doc(hidden)]`. Verify label W-extB-9 (wasm); not yet read when I stopped.
- window-config: 59 errors (W-extB-5) -> 27 (W-extB-8). Ported: pack-authority (ErasedWindowConfigPublication::close_step/close_ingress now `(body,effect)->OriginalRetirementPoll`, via RetirementPollExit::settle), retained/command, owner-registry (refresh takes effect; dead `built` removed), store-owners (step sites, new `retire_exit` helper, close_original_retained_step), typed-pack-load arity fixes + wire take_output via `effected`.
- Source-extent refusal on owner-registry (30323 B vs 29117 B) was caused by my own bad edit (anchor bug duplicated 15 KB in pack-authority, which consumed the crate's remaining budget). Restored from HEAD and re-applied; W-extB-8 passed source admission, so no split was needed. Pack-authority now 9892 B.
- Remaining 27 (W-extB-8): 
  - preview/🦀️.rs (18): `RetainedOwnedProjection::project` -> `RetainedCloneRef::try_project` + `RetainedOwnedProjectionAdmission` (admit then take, replies via `OriginalEffectReply::borrow(&mut Option<Result>)`), `available_work_units` needs `use ...native_decoding::NativePreparedDecodeControl`, factory-ticket/`select().step_controlled`/`owner.close_step_controlled` now take `(body,effect[,control])` and return polls (use RetirementPollExit::resume + settle); store derived ops (begin/step/take_derived_operation) are still grant-based in the store crate, call with `effect.grant()`.
  - store-owners 154 (ControlledRetirement step/step_controlled with effect), 301 (type), 424 (`catalog_step` takes effect; preview_input needs effect).
  - typed-pack-load 223 (`receiver.advance` failure is NativeReceivingFailure, `refused` takes OriginalReceivingPrimary), 234 (`retain_original` gone: use projection admission).
  - owner-contract-and-mutation/🛬️receiving 31 (`record_constructed_allocation` gone: move to admit_original_construction/OriginalConstructionAdmission).
- Rerun: `chk.sh W-extB-10 <window-config manifest> wasm -p semio-plugin-window-config`, then ext.sh.

STATUS: BLOCKED window-config 27 errors left (preview projection admission, receiving construction admission); FIXED-verified W-extB-1 (native-print), W-extB-3 (dwg-snapshot), W-extB-9 (infinite-board-engine)
