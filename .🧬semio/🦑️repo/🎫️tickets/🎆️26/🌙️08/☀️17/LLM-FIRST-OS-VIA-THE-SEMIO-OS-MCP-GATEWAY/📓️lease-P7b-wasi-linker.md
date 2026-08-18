# 📓️ lease-request — P7-headless-workspace → WASI must be wired into `semio-framework-plugin-host`'s `WasmtimeRuntime`

**From**: terra, packet `P7-headless-workspace` (packet P7b: real headless instantiation)
**To**: whoever owns `🔌️plugin/🖥️host/**` in the peer ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`
(packet B1/B1b per `📌️important.md`'s own collision table — this file is explicitly listed as NOT
ours to edit: `"🔌️plugin/🖥️host/**, 💻️os/🖥️host/🦀️component.rs, 🧩️extension/🦀️component.rs,
🏃️run/🦀️component.rs | B1 | headless workspace after their G1"`).
**File**: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`

## What's broken

Driving the real gateway end to end against `🗒️note`'s real committed descriptor + real
`wasm32-wasip2` build (`target/wasm32-wasip2/debug/semio_s_plugin_note.wasm`) fails at
`WasmtimeRuntime::instantiate`:

```
{"code":"INTERNAL","message":"Internal: instantiating `note`: wasmtime: component imports instance
`wasi:io/poll@0.2.9`, but a matching implementation was not found in the linker","retryable":false}
```

## Root cause — verified, not assumed

`world actor`'s own WIT (`🔌️plugin/🧬️schema/📜️component.wit:816`) declares exactly one import:
```wit
world actor {
  import pure;
  export reactor;
  export jobs;
  export checkpoint;
  export describe;
}
```
No WASI import anywhere in OUR world. The `wasi:io/poll@0.2.9` (and by extension the rest of WASI
Preview 2 — clocks, filesystem, random, cli stdio, sockets) is pulled in **transitively** by the Rust
`wasm32-wasip2` target's own runtime/libc shim — every real component built for that target needs a
full WASI Preview 2 linker regardless of what its own WIT world imports. This is a well-known
wasmtime/wasm32-wasip2 characteristic, not a bug in the plugin's WIT or in this packet's descriptor
reading.

`wasmtime-wasi = "22.0.1"` is **already a declared dependency** of `semio-framework-plugin-host`
(`📦️packages/🦀️rust/Cargo.toml`) — confirmed by reading it — but repo-wide grep for
`wasmtime_wasi::|WasiCtx|WasiView|add_to_linker` across the ENTIRE `🧰️framework` tree returns **zero
matches outside that one Cargo.toml line and the identical unused declaration in
`🔌️plugin/📇️describe/📦️packages/🦀️rust/Cargo.toml`**. Nobody in this codebase has ever actually
wired it. `ActorHostState` (`🔌️plugin/🖥️host/🦀️component.rs:692`) has no `WasiCtx`/`ResourceTable`
fields and no `WasiView` impl; `WasmtimeRuntime::new`'s linker only calls
`actor_bindings::semio::framework::pure::add_to_linker`.

## What we determined instead of assuming (per sol's explicit ask)

1. **Is there an existing plugin-host entry point that already wires WASI, that we should call
   instead of assembling our own `Linker`?** No — confirmed by the grep above. `🏠️workspace`
   correctly calls the ONE shared entry point that exists (`semio_framework_plugin_host::
   WasmtimeRuntime::new`/`instantiate`/`execute_turn`) rather than constructing a second `Linker`
   anywhere — that part of the architecture is already right; there is nothing to "reuse" that isn't
   already being reused, because the WASI wiring itself has never been written by anyone.
2. Since `WasmtimeRuntime`'s `linker: Linker<ActorHostState>` field and `ActorHostState` struct are
   both **private** to `🔌️plugin/🖥️host/🦀️component.rs`, there is no way to inject additional linker
   imports from outside that file — the fix cannot be made from within `🏠️workspace/**` (or any of
   this packet's other owned paths) no matter how it's structured. It genuinely has to land in the
   owning file.

## The exact fix (verified against the pinned `wasmtime-wasi = "22.0.1"` source, not guessed)

```rust
// top of file, alongside the existing wasmtime imports:
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime::component::ResourceTable; // already re-exported at wasmtime_wasi::ResourceTable too

// ActorHostState gains two fields:
struct ActorHostState {
    // ...existing fields unchanged...
    wasi_ctx: WasiCtx,
    resource_table: ResourceTable,
}

impl WasiView for ActorHostState {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.wasi_ctx }
    fn table(&mut self) -> &mut ResourceTable { &mut self.resource_table }
}

// in WasmtimeRuntime::new, alongside the existing
// `actor_bindings::semio::framework::pure::add_to_linker(&mut linker, ...)` call:
wasmtime_wasi::add_to_linker_sync(&mut linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;

// in instantiate()'s `ActorHostState { ... }` construction, add:
wasi_ctx: WasiCtxBuilder::new().build(),  // sandboxed default: no inherited stdio/fs/network/env —
                                            // matches this crate's own "capability-gated imports"
                                            // security stance (module doc, line 1); tighten further
                                            // per BrokerCapabilityGrant if/when a capability needs
                                            // real WASI access (fs/clock/random) — none of `pure`'s
                                            // own three functions (log/now-ms/trace-span) do.
resource_table: ResourceTable::new(),
```

`add_to_linker_sync::<T: WasiView>(linker: &mut wasmtime::component::Linker<T>) -> anyhow::Result<()>`
signature and the `WasiView { fn ctx(&mut self) -> &mut WasiCtx; fn table(&mut self) -> &mut
ResourceTable; }` shape verified by reading `~/.cargo/registry/src/…/wasmtime-wasi-22.0.1/src/{lib,ctx}.rs`
directly (the exact pinned version this workspace already resolves) — not from general wasmtime-wasi
documentation, which drifts across versions.

## Status

Pending as of this report. Blocks the ENTIRE headless-instantiation path (not just this packet —
any real `wasm32-wasip2` component instantiated through `WasmtimeRuntime` hits this identically,
including `🏃️run`'s own `WasmtimeNodeHost` once its separate manifest-decode gap is cleared, and
`H3-wgpu-native`'s `create_app` the first time it actually instantiates rather than only compiling).
