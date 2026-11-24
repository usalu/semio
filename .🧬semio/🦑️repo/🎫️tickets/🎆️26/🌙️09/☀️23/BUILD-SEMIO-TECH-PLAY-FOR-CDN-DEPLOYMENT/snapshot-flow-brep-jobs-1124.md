# Snapshot fix: semio-s-plugin-flow-brep-original-jobs (wasm32-wasip2, release)

Nothing was compiled; verification is by reading only.

## Causes
1. Four E0277 (`dyn ErasedSnapshotRetirement` not Sync): `#[derive(RetireOwned)]` without `#[value(send_only)]` emits a shared-original path with `where Self: Sync`. `GeometryOutputData`, `deconstruct::Metadata`, `deconstruct::DeconstructOutput` and `topology_output::TopologyOutput` hold `Option<BudgetedOwnedEntry>` (-> `RetainedCloneClose` -> `Box<dyn ErasedSnapshotRetirement>`, Send only), so they cannot be Sync. The derive (`derive/🧬️retained-clone/🦀️.rs` `original_allocation_retirement`) skips the shared path when `declared_send_only` is set.
2. E0271 at `🦀️.rs:1980`: closure `read = |rank| ...` inferred `rank` from `rank + 1` as i32 (the literal fallback); `entry_at_rank` takes usize. Fixed by annotating `rank: usize`.

## Model
Siblings already using `#[value(send_only)]` under `#[derive(RetireOwned)]`: `🥽️mesh/🧮️operator-core/🦀️.rs` (MeshSourcePreparation) and the flow editor retained jobs. Live repo already had this exact fix; ported byte-identically (the crate dir now has no diff against live).

## Files changed (all under /Users/ueli/.sinc/play/✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/〰️analytic-curves/)
- 🦀️.rs (send_only on GeometryOutputData; `rank: usize`)
- 🧊️deconstruct/🦀️.rs (send_only on DeconstructOutput and Metadata)
- 🧾️topology-output/🦀️.rs (send_only on TopologyOutput)

## Live already had the fix: yes.

## Unresolved / risks
- Not compiled. `send_only` removes the shared-original admission for these types; any code requiring them to be shared-original would error, none was reported in the log. Other errors could surface after these five (rustc stops at type-check phase).
