# HostAsyncRuntime R1 Impl Ripple

Packet: os-ripple / db-dedyn follow-up from macros-blockon (R1 trait shape).

## Trait change (R1)

In `⏳️async/🦀️component.rs`, `HostAsyncRuntime`:

- `async fn sleep_until(&self, deadline_ms: u64);` — was `-> HostFuture<()>`
- `async fn cancel_scope(...) -> ScopeDrainReport;` — was `-> HostFuture<ScopeDrainReport>`

Reference impl: `testkit::ManualRuntime` in the same file.

## Impl status

| Crate | Type | Status |
|-------|------|--------|
| `semio-framework-os-kernel-db` (`db_storage` `InlineRuntime`) | Already R1-shaped (`async fn`, plain returns) | No change needed |
| `semio-framework-os-services` (`TokioHostRuntime`) | `sleep_until` / `cancel_scope` updated | Done |
| `semio-hub` (`HubDbRuntime`) | `sleep_until` / `cancel_scope` updated | Done |

## Verification

```bash
export CARGO_TARGET_DIR=/tmp/semio-cargo-scratch-r1-hostasync
cargo check -p semio-framework-os-kernel-db --features fs
cargo check -p semio-framework-os-services
cargo check -p semio-hub
```

As of this edit, workspace dependency crates (`semio-framework-actor`, `semio-framework-pack`, `semio-framework-os-kernel`, …) still fail for unrelated async-migration errors, so these three `-p` checks do not reach a clean pass yet. No `HostAsyncRuntime` / `sleep_until` / `cancel_scope` mismatch was reported for the edited impls once earlier blockers are cleared.

## Out of scope (same ticket, other packets)

- `TokioHostRuntime` / `HubDbRuntime`: remaining trait methods still `fn` not `async fn` (pre-R1 drift; separate os-ripple pass).
- `TimerWheel::spawn_driver` and other call sites still treating `sleep_until` as `-> HostFuture<()>` (lines ~502–504 in services).
- `ScopeTable::open_scope` / `CancelToken::root` `.await` wiring.
- Hub `connect_db` `open_scope(...)` missing `.await`.
