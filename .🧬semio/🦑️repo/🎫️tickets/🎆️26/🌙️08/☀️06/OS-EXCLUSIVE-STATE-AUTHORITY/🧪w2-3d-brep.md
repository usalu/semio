# Wave 2 — 3D BrepkitKernel → OS EngineCache

## Summary

Migrated `BrepkitKernel` from a plugin-owned `HashMap<String, Entry>` registry (with optional local blake3 minting) to host-style **OS engine derive**:

- `BrepEntityEngine` (`ENGINE_ID = "s.3d.brep.entity"`) — identity pack round-trip in `EngineCache`
- `BrepMeshCacheEngine` (`ENGINE_ID = "s.3d.brep.mesh-cache"`) — engine-key tessellation cache (replaces `(SolidId, f64)` HashMap keys)
- `live: HashSet<String>` — liveness (DrawingStore pattern)
- `entity_lut: HashMap<EngineKey, Entry>` — process-local topo/curve lookup (not identity minting; identity = `EngineCache::derive`)

`GeometryHandle` is now **64-char hex** of `EngineKey` (same as `DrawingHandle`), not `solid-{16hex}`.

## Package

- **Crate:** `semio-s-3d`
- **Check:** `cargo check -p semio-s-3d --lib` — **green** (2026-08-06)
- **Tests:** `cargo test -p semio-s-3d --lib` blocked on this machine (Xcode license / linker); logic covered by new `geometry_handles_are_engine_derived_hex` test in kernel `mod tests`.

## Dependency

`semio-framework-os-kernel` already optional under `brep` feature in `✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml` — **no root Cargo.toml change**.

## Files touched

| File | Change |
|------|--------|
| `✏️s/🔨️modules/🧊️3d/📐️brep/🧰️kernel/🦀️component.rs` | Engine region, `BrepkitKernel` cache/live/lut, mesh `EngineKey` cache, dispose/retain, tests |
| `✏️s/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️component.rs` | `GeometryHandle` = hex engine key only |
| `✏️s/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️component.rs` | Host test expects 64-char hex handle |

## Alignment

- Reference: `✏️s/🔨️modules/◻2d/🗄️store/🦀️component.rs` (`DrawingStore` + `DrawingEngine`)
- Host: `BrepEngineHost` unchanged structurally; kernel now owns its own `EngineCache` (document-op cache on host remains separate)

## Follow-ups

- Wire `BrepEngineHost` to share one `EngineCache` with `BrepkitKernel::with_engine_cache` when Wave 1b unifies host injection.
- Enrich `entity_pack` for free curves/surfaces (material still coarse for `Entity::Curve`).
