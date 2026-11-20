# 🧱️ WFC grid3d editor

## Rust verification

- The shared-store workspace run `[DEBUG] grid3d-full-14.log` recorded 263 passed, 2 failed and 2 ignored. Every non-history test passed, including the solver and preview laws, one-fuel cancellation, and the multi-collapse fill burst.
- `history_edits_end_to_end` and `conflict_history_edits_end_to_end` both stop during terminal close with `hydration retains its complete unreceived original failure`. Grid2d reports the same framework failure.
- Grid3d fill stages its initial preview once and advances preparation within available fuel and deadline. The shared test driver pays receiver setup separately from each requested job quantum. See the [fundamental fixes](fundamental-fixes.md) for the related changes.

## Source changes

All under `🗿️artifacts/🧱️grid3d/`:

- `🧬️schema/🧬️mutations/🦀️.rs`: `dsl::original_pack_value_operation_receiving!(Grid3dMutation)`.
- `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`: host decode and encode use the output-slot signature; its `🧪️tests` go through `decode_native`, `encode_native` and `settled`, which retires owned values on refusal.
- `🚪️io/💾️binary/📸️snapshot`: removed `decode_into` and its retirement test.
- `✏️editor/🫧️transient/🦀️.rs`: `#[derive(SnapshotProjection)]` replaces the hand-written impl.
- `🔨️modules/🏠️host/💡️inferences/🦀️.rs`: output and payload close through `retirement_demands` and `close_original_effect_step`; chunk and row closes use `close_original_retained_step`; publication close maps `Complete` to `Pending`.
- Fill tool `🛠️tools/🪣️fill/🦀️.rs`: explicit error type on granted close outcomes; publication close mapped like above.

## Wiring review

- Playground `grid3d` (`s.wfc.grid3d@1/*#editor`, ports react 6044 / wgpu 6144) is in the generated catalog; composition crate is `🌎️hub/🧩️compositions/🀄️wfc/🧩️component`.
- The wfc bridge registers `Grid3dMutation` and the `s.wfc.grid3d` standard.
- Editor exposes `resizeGrid`, `changeCellSizes`, `changePeriodicity`, `createRule`, `deleteRule`, `pinCell`, `maskCell`, a `solve` command, a preview window and a fill tool.
- The solve job reports progress and handles cancellation.
- Unverified: the wasm component build, preview dispatch at runtime and the GIS quartet test.
