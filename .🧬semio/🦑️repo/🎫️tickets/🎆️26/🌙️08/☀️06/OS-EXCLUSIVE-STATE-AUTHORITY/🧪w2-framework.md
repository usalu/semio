# W2 Framework Surface Migration

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`  
**Date:** 2026-08-06

## Goal

Peel authoritative `HashMap`/`BTreeMap` fields off framework `*Host`/`*Session`/`*Store`/`*Cache` types and align WASM surface hosts with projection mirrors + GPU ledgers (OS `DocumentStore` / `EngineCache` remain authoritative).

## Delivered

### Surface hosts (`semio-framework-surface`)

- **`MapHost`:** `MapFeatureTables` (positions/routes/regions) + `MapTileLedger` (raster/vector tile bytes). Public GIS tables via `host.features`.
- **`RasterHost`:** `RasterLayerBuffers` for paint/mask scratch.
- **`TerrainSessionCore`:** dropped `TerrainSessionState`; elevation keyed in `TerrainElevationTiles.by_key`.

### UI wgpu retained state (`semio-framework-ui`)

- **`MeshGpuTable`**, **`RasterTextureTable`** (renamed from `*Store` to satisfy authority-struct policy).
- **`LayoutEngine`:** `TaffyNodeMapping.by_ui` holds taffy id map.
- **`TreeDragState`:** `TreeDragPayload` type alias for drag MIME map.

### Core (`semio-framework-core`)

- **`Platform`:** removed `AtomicU64` generation counters; `notify` / `notify_chrome` use plain `u64` (`&mut self` shell).

### Docs / algebra

- Module docs on `EditorHost`, `GraphHost`, `ActionBus`, `DomainStore`, `DefaultMindmapExtension` (s-module).

## Policy probe (targets)

With `SEMIO_OS_STATE_AUTHORITY=1`, plan-target paths (`tiled-map`, `paint`, `terrain`, `platform`, `wgpu draw/flex/input`) report **0** `authority-struct-map` / `id-minting` breaches.

Framework non-OS total down to **6** unrelated high-priority items (compiler static, ui/schema/sampling/wfc nogood) — see `🧪w2-framework-inventory.md`.

## Verification

| Gate | Result |
|------|--------|
| `cargo check -p semio-framework-core` | **pass** |
| `cargo check -p semio-framework-ui` | **pass** |
| `cargo check -p semio-framework-surface` | **blocked** — transitive `semio-s-3d` compile error (`GeometryHandle::content_addressed` missing; other agent wave) |
| `cargo check -p semio-framework-editor` | not run in isolation (depends on surface chain) |

Log: `🧪w2-framework-cargo-check.log`

## Hard cases / follow-ups

1. **`EditorHost` text + LSP vectors** — still mutated in-process for WASM play; needs `DocumentApp` projection ops + host `sync_from_pack` (OS renderer integrator).
2. **`MapFeatureTables` / `RasterLayerBuffers`** — still hold document-shaped data locally until tiled-map / paint plugins emit store ops instead of JSON fixture sync.
3. **`GraphHost` / `BoardSession`** — retained `DagHost` / `BoardHost` from OS infinite board; delete duplicate paths when EngineCanvas always uses `DocumentSession.engines`.
4. **`semio-framework-surface` check** — unblock when `semio-s-3d` brep handle migration lands.
5. **Framework TS chrome** (`localStorage`, `ShellScope`, `Tree`) — still listed in `🧪inventory-core.md`; out of Rust wave scope.
6. **Remaining 6 Rust policy breaches** in compiler / ui registry / schema / sampling / wfc nogood — separate framework hygiene ticket or Wave 3 integrator batch.
