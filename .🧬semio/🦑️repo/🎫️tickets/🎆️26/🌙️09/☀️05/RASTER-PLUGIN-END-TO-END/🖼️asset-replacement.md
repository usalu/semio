# Image Replacement at Asset Capacity

Source inspection found editPixels and editMask add the new encoded image before removing an exclusively referenced prior image. This exceeds the 64-entry owned-map limit even when the final document still uses 64 assets. Simply moving removal first would break inverse application: undo would point the layer at the old image before restoring that image asset.

A capacity-safe plan must detach the selected reference, remove its exclusively owned old image, add the new image, then attach the new reference within one atomic command. Reversing these events also keeps capacity and reference validity. Shared source images must stay. If there is no free slot and no exclusively owned source to release, reject publication without edits. Existing new assets require no allocation.

New schema and neutral vectors cover capacity, headroom, shared sources and existing destinations. Rust/TypeScript planning and actual retained pixel/mask publication tests are required. The independent TypeScript oracle uses existing fast-json-patch to apply and invert the neutral steps while checking capacity and reference validity. No runtime library is added.

## Implementation and Verification

The shared internal planner now has Rust and TypeScript implementations. Both editPixels and editMask use it; only a full pool requiring a new asset and having an exclusively referenced old asset takes the detach/remove/add/attach path. With headroom, existing add/attach/remove ordering remains valid in both directions. A full pool with shared source data returns a capacity fault before emitting mutations.

TDD evidence: TypeScript red run 61622 passed 127 existing tests and failed for the absent planner module. Green run 3538 passed **134 tests** (raster-asset-replacement-ts-green-1.log), including all seven neutral plans and fast-json-patch forward/inverse capacity and reference checks. No native capacity result is claimed yet.

Native coverage authored before publication changes expands every mask coverage fixture across full pools for pixel/group masks and shared sources. A pixel regression verifies full-pool publication and exact inverse. The planner has native neutral-vector coverage, and a loaded-document test dispatches real retained pixel and mask edits at capacity, then checks one undo/redo restores exact snapshots. These are pending the full Raster suite. New expected native census is 317; inspect the actual executable census because source was edited while dependencies were building.

Current native run 1 (18490) executed 321 tests, with the capacity planner, native pixel/mask replacement and retained full-capacity publication/undo/redo tests all passing. Four unrelated archive/export/retirement-fixture failures keep the whole suite red. Capacity-safe replacement is now verified in native retained history as well as the TypeScript oracle; live UI acceptance remains open.
