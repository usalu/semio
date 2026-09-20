# Astra Sol GPU Raster Residency

## Scope

This packet repairs the retained WGPU raster-table leak confirmed in `📓️astra-terra-host-materials.md`. `RasterTextureTable::commit_presented_step` previously moved each distinct staged key into `live` and retired only a same-key replacement, so more than 256 sequential reference URLs exhausted the fixed 256-item table even after World had retired its CPU pixels.

The repair covers every raster key reachable from a prepared frame: World reference and paint textures in `ScenePass3d::textured_draws`, normal UI raster instances, inline overlay raster instances, and the prepared packet's separate top-overlay draw. It does not change the fixed 256-item or 256 MiB budgets, add a compatibility model, or alter decoder, material, Tree-transfer, or shadow behavior.

## Schema and neutral fixture

The language-neutral contract is:

- schema: `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🖼️raster-residency/🔣️.json`
- fixture: `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🖼️raster-residency/🔣️.json`

It fixes the shared limits and the six required lifetimes: 300 distinct replacements, a shared key across two windows, final-owner close, aborted B while committed A remains paintable, UI/World/overlay ownership, and budget recovery through one-unit retirement steps.

## Production ownership contract

`PreparedRenderPacket::raster_keep_step` progressively scans the main draw and optional top-overlay draw. `DrawList::raster_keep_step` yields one key or one cursor transition per call across textured World draws, normal raster instances, and overlay raster instances. The presenter completes this scan and seals the candidate ownership set before any engine or prepared raster upload can request capacity.

`RasterResidencyLedger` retains three exact fixed-capacity sets:

- `committed` protects the currently paintable packet;
- `candidate` protects the complete sealed packet being prepared or presented;
- `previous` protects the displaced committed packet until `PreparedRenderReplacement.previous` finishes its existing progressive retirement.

A successful GPU commit moves staged textures to `live`, promotes candidate ownership, and transfers the prior committed set to `previous`. Only after the previous prepared packet reaches terminal retirement does the presenter release that ownership and scan the live table for unowned entries. An abort progressively retires its staged resources, clears only its candidate ownership, keeps the committed set intact, and then removes any now-unowned transient live entry. Failure before ownership publication has an idempotent empty abort path.

Both prepared raster upload variants and the EngineCanvas allocation path call `prepare_admission_step` before reserving GPU resources. When item or byte capacity is insufficient, the table advances the existing granular GPU-resource retirement owner across at most one scan or resource-cleanup unit per call. It never evicts a key held by committed, candidate, or previous ownership, and it refuses admission when the remaining capacity is owned by retained frames.

## Laws and oracle

The Rust laws in `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️raster-residency/🦀️.rs` exercise the production ownership ledger and the same fixed registry selector used by `RasterTextureTable`. They cover the six neutral scenarios, including the 300-key bound and a full 256-slot registry that protects its live owner while releasing historical entries. The prepared-render unit law drives the production packet cursor across main, inline-overlay, and separate top-overlay raster owners.

The Vitest law at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖼️wgpu-raster-residency/🟦️.ts` uses an independent JavaScript `Set` ownership oracle for the neutral fixture, then checks the production source seams that publish ownership before raster allocation and release previous ownership only after packet retirement.

Focused command actually run:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache -- '🧪️tests/🖼️wgpu-raster-residency/🟦️.ts' -t 'wgpu prepared raster residency'
```

Observed result:

```text
Test Files  1 passed | 8 skipped (9)
Tests       3 passed | 93 skipped (96)
NX Successfully ran target test-browser-worker and 4 dependency tasks
```

The uncached retained log is `🗑️generated/astra-gpu-raster-residency/vitest-raster-residency.log`.

Additional checks actually run:

- `rustfmt --edition 2021 --emit stdout` parsed the new raster-residency law, draw-list cursor and table, prepared packet, GPU context, EngineCanvas, and prepared-render unit laws without writing shared files;
- `git diff --check` passed for the packet paths.
- Root's integrated generic UI renderer suite passed all 134 tests.

Per root ownership, this lane did not launch Cargo, native, wasm, shader, or browser-runtime builds. The new Rust laws and integrated GPU behavior require the root-owned combined build and runtime verdict; no unrun result is claimed here.

## Files owned by this packet

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🖼️raster-residency/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🖼️raster-residency/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️raster-residency/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🐕️wgpu-present-stall-watch/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖼️wgpu-raster-residency/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🖼️raster-witness-lifecycle/🔣️.json`
