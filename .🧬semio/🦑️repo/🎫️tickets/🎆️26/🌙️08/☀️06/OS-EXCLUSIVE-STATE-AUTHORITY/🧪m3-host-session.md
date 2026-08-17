# M3 Host Session — Wave 1b

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`  
**Date:** 2026-08-06  
**Owner:** Wave 1b host (plugin-host only — guest SDK owned elsewhere)

## Delivered

### `DocumentSession` (host)
`🔌️plugin/🖥️host/🦀️component.rs`

```rust
pub struct DocumentSession {
    pub generation: u64,
    pub command_log_len: u64,
    pub engines: store::EngineCache, // re-export from semio-framework-os-kernel / os_engine
}
```

- Default engine cache budget: `64 MiB` (`DEFAULT_ENGINE_CACHE_BUDGET_BYTES`).
- Constructed in `WasmPluginRuntime::host_state` as `HostState.session`.

### WIT (`host` interface)
`🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit` (already present / confirmed):

```
engine-derive: func(engine-id: string, input: list<u8>) -> result<list<u8>, list<u8>>;
engine-read: func(engine-id: string, key: list<u8>) -> result<list<u8>, list<u8>>;
```

Matches existing host style (`list<u8>` fault bytes, not `plugin-error`).  
`engine-derive` returns the 32-byte `EngineKey`; `engine-read` takes `engine-id` + those key bytes.

### Host stubs
- `engine_derive` → capability `ArtifactKind::Engine` + `Rights::Invoke` → `EngineCache::derive` → key bytes.
- `engine_read` → capability `ArtifactKind::Engine` + `Rights::Read` → rebuild `EngineHandle` → `EngineCache::read`.
- `WasmPluginRuntime::register_engine` wires kernels into the session cache.

### Bindgen path
Corrected host `bindgen!` path from missing `../../../⚡️implementations/🦀️rust/📜️wit` to  
`../../../📦️packages/🦀️rust/📜️wit` (relative to host package `Cargo.toml`).

## Out of scope (this wave slice)
- Guest SDK `🔌️plugin/🦀️component.rs` (other agent).
- Full DocumentStore / draft / command_log move into `DocumentSession` (later host wave).
- Root / plan / integrator files.

## Verification
See `🧪m3-host-cargo-check.log` from `cargo check -p semio-framework-plugin-host --lib`.

## Verification result

| Gate | Result |
|---|---|
| `cargo check -p semio-framework-plugin-host --lib` | **pass** (`Finished … in 7.07s`) with `DEVELOPER_DIR=/Library/Developer/CommandLineTools` (Xcode.app license blocks `cc` otherwise) |
| Log | `🧪m3-host-cargo-check.log` |

Host crate warning only: unused `extern crate … as vcs` in host glue (pre-existing / unrelated).
