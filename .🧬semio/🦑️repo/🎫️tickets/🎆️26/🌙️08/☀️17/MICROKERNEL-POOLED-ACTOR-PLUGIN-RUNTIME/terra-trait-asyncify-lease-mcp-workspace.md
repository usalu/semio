# lease-request: 🌉️mcp/🏠️workspace/🦀️component.rs (two `execute_turn` call sites)

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️component.rs`

Reason: my packet (terra-trait-asyncify, W6) changed `GuestRuntime::execute_turn` (and
`start_job`/`step_job`/`cancel_job`/`checkpoint`/`restore`) to return
`semio_framework_async::HostFuture<Result<T, _>>` instead of a bare `Result<T, _>` — see that
trait's own doc comment in `🖥️host/🦀️component.rs` (~L490) for the full rationale (a genuinely-
suspending backend needs to share the ONE trait every `Arc<dyn GuestRuntime>` holder already uses,
and `async fn` in a dyn trait is not an option). This file is in a completely different product
module (`💻️os/🔨️modules/🌉️mcp`, not `🔌️plugin/🖥️host`) — outside every path my packet owns — so I
could not apply this myself without violating rule 3. **This crate does not compile until this
lands** (it has two real, live `execute_turn` call sites, not just doc-comment mentions).

## The fix (mechanical, two lines, one new use)

`🖥️host/🦀️component.rs`'s new `poll_ready<T>(future: HostFuture<T>) -> T` helper is `pub` (not
`pub(crate)`) specifically so external `Arc<dyn GuestRuntime>` holders like this file can reuse it
instead of writing a second copy — see `poll_ready`'s own doc comment for why. It is already
re-exported at the plugin-host crate root (`glue.rs`'s `pub use component::*;`), so:

### 1. New import (near this file's existing `use semio_framework_plugin_host::GuestRuntime;`, ~L17)
```rust
use semio_framework_plugin_host::poll_ready;
```

### 2. `activate_plugin_instance` (~L327)
CURRENT:
```rust
    let turn = runtime.execute_turn(&mut instance, &[open_event], budget).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("`{}` InstanceOpen turn faulted: {error}", entry.plugin_id)))?;
```
REPLACEMENT:
```rust
    let turn = poll_ready(runtime.execute_turn(&mut instance, &[open_event], budget)).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("`{}` InstanceOpen turn faulted: {error}", entry.plugin_id)))?;
```

### 3. `PluginArtifactChannel::exchange_one_real` (~L399)
CURRENT:
```rust
        let turn = self.runtime.execute_turn(guest, &[event], budget).map_err(|error| Self::not_wired("execute_turn", error))?;
```
REPLACEMENT:
```rust
        let turn = poll_ready(self.runtime.execute_turn(guest, &[event], budget)).map_err(|error| Self::not_wired("execute_turn", error))?;
```

Both call sites run on a plain synchronous call stack (no executor around either function) — the
exact same "eagerly-ready by construction" position `🧵️shard/🦀️component.rs`'s `ShardLoop::pump`
and `🖥️host/🦀️component.rs`'s own `PluginInstanceHandle::run_job_to_completion` are in (see the
sibling lease-request `terra-trait-asyncify-lease-post-turn-relay.md` for that one). Behavior is
byte-for-byte unchanged: `poll_ready` panics only if a future is not ready on its first poll, and
every `GuestRuntime` impl in the tree today (`WasmtimeRuntime`, `MockGuestRuntime`) is eagerly
ready by construction.

No other line in this file needs to change — I grepped it for `.start_job(`/`.step_job(`/
`.cancel_job(`/`.checkpoint(`/`.restore(` and found none; `execute_turn` is the only
`GuestRuntime` method this file calls.
