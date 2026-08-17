# W2 2D Drawing Engine Migration

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`  
**Date:** 2026-08-06  
**Owner:** Wave 2 — ◻2d DrawingStore

## Goal

Remove plugin-owned `DrawingStore` registry (`HashMap` + local handle minting). Scene nodes are derived through the OS `Engine` / `EngineCache` contract (`ENGINE_ID = "s.2d.drawing"`).

## Delivered

### `DrawingEngine`

- Implements `semio_framework_os_kernel::Engine` via path-included `os_engine` module in `semio-s-2d` glue (same source as `🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs`).
- `ENGINE_ID = "s.2d.drawing"`.
- `compute` validates/normalizes JSON-encoded `StoredNode` packs (serde round-trip).

### `DrawingStore`

- **Removed:** `registry: HashMap<String, StoredNode>` and `content_handle` / `drawing-*` minting.
- **Added:** process-local `EngineCache` (documented stand-in until host `EngineHost` wiring), `live: HashSet<String>` for dispose/retain only (not node storage).
- `derive_node` → `cache.derive(DrawingEngine::ENGINE_ID, pack)`; `DrawingHandle` is hex-encoded `EngineKey` (64 chars).
- `with_engine_cache`, `engine_cache_mut` for host injection.
- `DrawingKernel` trait unchanged; mutations use `with_mutated` → new derive.

### `DrawingHandle`

- Opaque hex engine key; removed `DrawingHandle::new(kind, id)`.

### Exports

- `semio-s-2d` re-exports `DrawingEngine`, `DrawingStore`, and OS engine types (`Engine`, `EngineCache`, `EngineKey`, …) from glue.

### Tests (extended in `🗄️store/🦀️component.rs`)

- `drawing_engine_id_matches_os_contract`
- `derive_twice_same_node_is_same_handle`
- `handles_are_hex_engine_keys`
- Updated `bool_op_many_single_handle_is_content_addressed` for content-addressed semantics.

## Follow-ups

- Durable scene graph should move to **document projection ops** (not engine cache).
- Flow draw `Mutex<DrawingStore>` global should receive host `EngineHost` when Wave 1b guest authority lands.
- Prefer `semio_framework_os_kernel::os_engine::*` dependency once integrator resolves `cdylib` / Xcode link path for dev tests.

## Verification

| Gate | Result |
|---|---|
| `cargo check -p semio-s-2d --lib` | **pass** (`Finished` in ~0.8s) |
| `cargo check -p semio-s-2d --lib --tests` | **pass** |
| `bun ./📜️script.ts test` / `cargo test -p semio-s-2d` | **blocked** — linker `cc` exit 69 (Xcode license) when building `semio-framework-os-kernel` `cdylib` via `semio-framework-core` dependency chain |
