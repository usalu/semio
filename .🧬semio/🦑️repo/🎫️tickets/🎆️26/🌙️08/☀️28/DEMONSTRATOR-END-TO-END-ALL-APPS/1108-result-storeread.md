# storeread
- step1: store-read lib native W-storeread-4 + wasm W-storeread-5 rc0 (Sync bounds in cursor-ledger-and-vcs and envelope-genesis; replay lane did the rest).
- step2 dedupe: one `ArcRootRetirement` (Owned or Snapshot issuer) in value factory crate `arc-custody/🏭️factory/🌳️root` (also box_demands/box_depth/box_close_step; snapshot-read re-exports them under artifact_retirement_box_*). Removed store-read ArcRootRetirement, publication ArtifactStorePostRootRetirement (now ArcRootRetirement::new_snapshot), replayop 🌅️next NextRootRetirement. Owned issuer path always converts to SharedOriginalStorage then FactorySharedRetirement.
- native W-storeread-7 rc0 (factory, snapshot-read, store-read, publication, replay-operation, paged-one-item). wasm + batch tests next.
STATUS: BLOCKED in-progress (store-read FIXED-verified W-storeread-4/5)

## Final
- wasm W-storeread-8 rc0 (factory, snapshot-read, store-read, publication, replay-operation, paged-one-item, ephemeral).
- `--tests -p semio-os-store-one-item-batch` : W-storeread-9 queued behind the cpu-picker (position 7 of 7); result file `🗑️generated/1105b/W-storeread-9.rc` appears when done. Earlier W-storeread-6 failed only on a transient mid-edit import (ephemeral displaced-owned-root), since fixed.
- Rerun: chk.sh W-storeread-N Cargo.toml native --tests -p semio-os-store-one-item-batch
- Debt: Owned-issuer root path now always goes root -> SharedOriginalStorage -> FactorySharedRetirement (store-read's old direct FactoryOwnedRetirement path is gone); unused imports left in publication prepared/♻️retirement (warnings only).
STATUS: FIXED-verified W-storeread-4 (native store-read), W-storeread-5 (wasm), W-storeread-7/8 (dedupe native+wasm); batch --tests pending W-storeread-9

## Addendum
- W-storeread-9 (`--tests -p semio-os-store-one-item-batch`, root manifest) rc 1: 13 errors, all in one-item-batch `📨️emission/🧳️receiving/🦀️.rs` (RetainedCloneSourceTake import, NativeReceivingFailure field access, publication debt #4). No store-read errors. Area claimed by gen3d/mutapply; not touched.
- DisplacedOwnedRootRetirement (store-ephemeral) kept: genuinely different (a lazily-admitting holder that owns root+factory unadmitted and admits the ArcRootRetirement frame as its active child on the first granted turn; the factory type is the already-admitted frame). It already uses factory ArcRootRetirement::admit_original; no duplicate logic remains.
STATUS: FIXED-verified W-storeread-4/5 (store-read), W-storeread-7/8 (dedupe); batch tests BLOCKED by one-item-batch emission receiving (W-storeread-9)
