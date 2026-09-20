# Sol Raster Capacity Follow-up

## Result

The raster table now recognizes immutable content before allocating a staged replacement. A second full 256-key frame whose pixels are unchanged reuses all committed textures with zero new table entries. Changed pixels under the same key remain a separate staged generation; when every retained credit is protected, admission applies explicit backpressure until peak headroom exists.

EngineCanvas now publishes the single texture it rendered into the raster table. It no longer allocates or retains an unused second texture/view outside raster accounting.

## Content identity

`RasterContentIdentity` includes width, height, byte length, and two independent 64-bit digest lanes. `PreparedRasterProducer` updates the digest while advancing its existing page cursor. Each call examines at most `PREPARED_RASTER_PAGE_BYTES` (16 KiB), so large images add no unbounded presenter callback.

EngineCanvas uses its immutable surface token, document generation, scene revision, metrics generation, primary metrics generation, and dimensions as the corresponding revision identity.

The table carries identity through reservation, allocation claim, staged entry, live entry, cancellation, and retirement. Reuse requires the same key and identity. A second different identity for one staged key in the same candidate is rejected.

## Capacity and frame semantics

- Live or current-candidate staged content with the same key and identity completes without reservation, allocation, upload, or witness mutation.
- Changed same-key content retains committed A while staging B. Presentation reads B; abort retires B and keeps A; commit replaces A only in the existing bounded commit retirement.
- A changed upload at 256 protected entries does not evict committed, candidate, or previous imagery. It reports retained-frame backpressure.
- Previous ownership ordering is unchanged: the previous prepared packet retires before its raster ownership is released.
- The candidate scan still includes World textured draws, UI/paint rasters, inline overlay rasters, and top-overlay rasters.

## EngineCanvas accounting

The `ReplacementTexture` and `ReplacementView` phases, `EngineGpuSurface`, and its private retirement lane were removed. A changed surface now renders one target, stages that exact texture/view, retires the Vello renderer in its own step, then publishes. An unchanged surface advances directly from identity reuse to freshness-checked publication.

Closing a surface clears only the presenter slot identity. The raster table remains the sole GPU image owner and releases the texture through packet ownership.

## Neutral law and tests

Added:

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🖼️raster-capacity-followup/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🖼️raster-capacity-followup/🔣️.json`
- Production-helper Rust laws for repeated full-capacity reuse, changed same-key staging/conflict, full-capacity backpressure, changed replacement abort, and previous retention.
- Prepared-producer Rust law proving identical pixels have identical identities and a one-byte change differs.
- EngineCanvas source law proving one target is created and no presenter-owned replacement target remains.
- Independent Vitest `Map`/`Set` oracle for full-frame reuse, changed-content backpressure, same-key abort, and source wiring.

## Verification performed

- Focused Nx/Vitest: `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache -- '🧪️tests/🖼️wgpu-raster-residency/🟦️.ts'` — PASS, 9 files and 98 tests; 4 Nx dependency tasks passed. Log: `🗑️generated/astra-raster-capacity-followup/focused-vitest.log`.
- Bun parsed the neutral schema and fixture JSON successfully.
- `rustfmt --edition 2021 --emit stdout` parsed prepared, draw, GPU, and EngineCanvas production sources successfully.
- Scoped `git diff --check` passed.

Cargo, native WGPU, and WASM builds were not launched. Root owns that acceptance lane.
