# Sol Raster Lifecycle Laws

Source-stable test-only follow-up on 2026-09-20 for the three coverage gaps in `📓️astra-terra-raster-capacity-audit.md`. No production API or runtime source changed.

## Executable native laws

`🖱️ui/🧪️tests/🖼️raster-residency/🦀️.rs` now mounts a headless WGPU harness around the real `RasterTextureTable`. It creates the production-compatible bind-group layout and real textures, views, samplers, buffers, and bind groups.

- `production_table_reoffers_a_full_committed_frame_and_backpressures_a_changed_protected_key` fills and commits all 256 table slots through production admission and presentation cursors. It then re-offers the identical sealed ownership set without staging or peak credit, and proves a changed protected key advances the bounded unowned scan before returning retained-frame backpressure.
- `production_table_paints_changed_same_key_only_while_presenting_and_abort_restores_committed_content` observes committed A before presentation, staged B during presentation, A after the production abort cursor, and B after the production commit cursor. It also asserts previous ownership remains until explicit release.
- Both laws close the real table to terminal-empty and inspect WGPU's validation error scope.

`EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs` now executes `EngineGpuSlot::publish_candidate` for a resource-free reuse candidate, rejects a changed scene tuple into `ClosingAdmission`, and drives `EngineCanvasPresenter` primary-metrics invalidation across its fixed scan while observing the candidate transition. The test clears its test-owned slot before presenter drop.

## Independent oracle

`wgpu-raster-residency/🟦️.ts` now gives the independent `CapacityOracle` an explicit presentation state. Its same-key scenario proves A before presentation, B during presentation, A after abort, and B after a second presentation commits.

## Verification

- `rustfmt --edition 2021 --check <raster-residency law> <EngineCanvas standalone law>`: passed.
- Focused Vitest was invoked through Nx with `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec -- bun x vitest run 🧪️tests/🖼️wgpu-raster-residency/🟦️.ts --config 🧪️tests/🎚️config/🟦️.ts`. Nx stopped before Vitest because the current shared project graph reports the existing `framework-os-kernel ↔ value-derive-rs` circular dependency.
- The new native laws are mounted but were not run locally. Root owns the active Cargo/native/WASM lane and received the exact law names at the source-stable checkpoint.

No prior passing count is attributed to this follow-up.
