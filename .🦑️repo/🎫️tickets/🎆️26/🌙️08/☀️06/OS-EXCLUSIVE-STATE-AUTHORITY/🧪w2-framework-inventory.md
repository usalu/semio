# W2 Framework Host Inventory

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`  
**Date:** 2026-08-06  
**Scope:** `🧰️framework/🔨️modules/**`, `🧰️framework/📦️packages/🦀️rust/**` (excludes `🛍️products/💻️os/**`)

## Host / Session / Store scan (`struct .*Host|struct .*Session|seq: u32|HashMap`)

| Symbol | Path | Maps / minting | Policy (`SEMIO_OS_STATE_AUTHORITY=1`) | Wave 2 disposition |
|--------|------|----------------|----------------------------------------|-------------------|
| `EditorHost` | `✍️editor/🦀️component.rs` | Vec fields only | clean | Session mirror documented; full projection-op funnel pending OS renderer |
| `EditorSession` | same | `RefCell` wasm bridge | clean | WASM façade only |
| `MapHost` | `🗺️tiled-map/🦀️component.rs` | was 5× `BTreeMap` on host | **fixed** → `MapFeatureTables` + `MapTileLedger` | Feature tables = projection snapshot; tiles = GPU fetch ledger |
| `MapSession` | same | wraps `MapHost` | clean | |
| `RasterHost` | `🎨️paint/🦀️component.rs` | was `HashMap` paint/mask | **fixed** → `RasterLayerBuffers` | Layer bytes = edit-session scratch until raster pack ops |
| `RasterSession` | same | wasm + GPU | clean | |
| `GraphHost` | `🕸️node-graph/🦀️component.rs` | delegates to OS `DagHost` | clean | Payload-diff retained cache; documented |
| `BoardSession` | `🎲️board-2d/🦀️component.rs` | OS `BoardHost` inside | clean | No parallel document registry |
| `TerrainSessionCore` | `🏔️terrain/🦀️component.rs` | was `TerrainSessionState` + `HashMap` | **fixed** → `TerrainElevationTiles` | DEM upload ledger; meshes derived on read |
| `ActionBus` | `🎯️action-bus/🦀️component.rs` | `HashMap` handlers | clean (name not in authority regex) | Documented ephemeral chrome dispatch |
| `Platform` | `🖥️platform/🦀️component.rs` | was `AtomicU64` counters | **fixed** → `u64` bumps | Single-threaded shell generation |
| `DomainStore` | `wfc/🌐️domain/🦀️component.rs` | `Vec<Domain>` only | clean | Algorithm scratch; documented |
| `MeshGpuStore` → `MeshGpuTable` | `ui/…/wgpu/🦀️draw.rs` | was map on `*Store` | **fixed** | GPU resource table (render cache) |
| `RasterTextureStore` → `RasterTextureTable` | same | was map on `*Store` | **fixed** | |
| `LayoutEngine` | `wgpu/🦀️flex.rs` | was map on `*Engine` | **fixed** → `TaffyNodeMapping` | Layout pass scratch |
| `TreeDragState` | `wgpu/🦀️input.rs` | was inline `HashMap` | **fixed** → `TreeDragPayload` alias | Transient DnD |
| `TuiHost` / `WasmHost` | `ui/⌨️tui/🦀️component.rs` | wasm shell | not in plan slice | |
| `DefaultMindmapExtension` | `✏️s/…/💭️mindmap/🧩️extension/🦀️component.rs` | `BTreeMap` topics | clean (name) | Documented mirror; `topics` field kept for reasoning plugin API |

## Remaining framework policy breaches (not plan targets)

| Kind | Path | Note |
|------|------|------|
| `item-scope-global` | `📚️compiler/🦀️component.rs:23` | static mut — compiler crate |
| `authority-struct-map` | `🧩️ui/🦀️component.rs:2781` | UI schema registry |
| `authority-struct-map` | `🧬️schema/🦀️component.rs` | JSON schema index |
| `authority-struct-map` | `math/🎯️sampling/🦀️component.rs:4995` | sampler cache |
| `authority-struct-map` | `wfc/🚫️nogood/🦀️component.rs:79` | search nogood table |

## s-modules (mindmap only in this wave)

- No `*Host`/`*Session` structs under `💭️mindmap` besides extension trait types.
- `lang` / `imperative` live under `✏️s/🔌️plugins/**` (other agent).
