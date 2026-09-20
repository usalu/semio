# Sol Raster Table Integration

## Scope

This packet closes the final audit's real-table coverage gap without changing production code. It extends the existing native `RasterTableGpuHarness` in `UI/🧪️tests/🖼️raster-residency/🦀️.rs` to retain its real WGPU queue and adds one bounded integration law through `RasterTextureTable::ensure_raster_step` with `RasterUploadPixels::Scene`.

## Contract linkage

The test includes the existing schema-backed engine fixture `scene-raster-ownership/🔣️.json` directly. It pins:

- GPU upload ownership ending only at committed ACK or terminal partial retirement;
- GPU resident ownership ending at texture retirement;
- `committedAckRequired = true`;
- the renderer-reachable `reference-image-map-v1` profile.

The existing TypeScript source/fixture oracle was run through Bun and Nx and passed **11/11** on 2026-09-20 at 11:07 CEST.

## Real GPU path

The native law creates the existing headless WGPU adapter/device/table harness and a private `SceneRasterPool` with:

- 2 CPU slots;
- 32 bytes of pool credit;
- 8-byte 2×1 RGBA references;
- 48 bytes of GPU resident credit.

It publishes six distinct references. Each reference:

1. reaches a writer only after bounded CPU-slot retirement when necessary;
2. seals a real `SceneRasterLease`;
3. uploads through the production `ensure_raster_step` row path and the harness queue;
4. enters presentation;
5. commits through `commit_presented_step`;
6. releases previous ownership;
7. proves the GPU ACK released its CPU lease and installed the exact pool GPU witness.

The keep set is cumulative, so the table owns six live `wgpu::Texture` objects and six 8-byte witnesses while the two-slot CPU pool has evicted the first identity.

The next ownership-only frame omits the first key. The test then drives real `retire_unowned_step` to completion and requires both the texture lookup and pool GPU witness to disappear. Re-publishing the same descriptor and pixels must take the CPU writer path again, re-upload through the table, commit, and restore the texture at its natural 2×1 dimensions. Closing the table must reduce GPU witness bytes from 48 to zero and leave no WGPU validation error.

Every upload, commit, CPU retirement, unowned-table retirement, and close loop has an explicit ceiling. No loop is open-ended. The adapter-missing path emits an explicit `[DEBUG] ... skipped` receipt; the executed path emits:

`[DEBUG] raster-table-scene-integration executed: textures=6 cpu_slots=2 resident_bytes=48 recovered=2x1 closed_bytes=0`

This makes a hardware-skipped run distinguishable from an executed GPU receipt.

## Validation boundary

- Rust source parse via `rustfmt --edition 2021 --emit stdout`: passed.
- Existing schema/source/neutral TypeScript oracle: 1 file, 11 tests passed.
- Native headless WGPU execution: pending the root-owned native lane; no GPU pass claim is made here.

Exact native filter:

`real_gpu_table_commits_six_scene_leases_retires_the_first_and_recovers_its_natural_dimensions`

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️raster-residency/🦀️.rs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/📓️astra-sol-raster-table-integration.md`

