# 📓️ terra-host-dedyn report

Packet: `host-dedyn`. Crate: `semio-framework-plugin-host` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/**`).

## 0. Live-tree hazard note

Re-read every file from disk immediately before each edit throughout (per binding rules). One
auto-commit (`09c3cf6df6`) landed mid-session while I was editing `🦀️component.rs`; verified by
`git show ":<path>" | wc -l` against the on-disk `wc -l` that it was the auto-commit bot capturing
my own in-progress work, not a reversion or a peer's overwrite (counts matched within the single
edit in flight). No recovery action was needed.

## 1. Work A — `GuestRuntime` → `GuestRuntimes` enum

**Hand-written, not `#[dyn_enum]`/`dyn_enum_close!`.** Two of the three variants
(`Mock`/`Recording`) are `#[cfg(test)]`-gated, and `📓️terra-dyn-enum-macro-report.md`'s own
acceptance suite never exercised cfg-gated variants — exactly the fallback the packet brief
anticipated ("if cfg-gated variants defeat the macro, hand-write the enum ... say so"). This is
useful signal for the ~50 remaining families: **cfg-gated variants are untested macro territory**,
treat them as a hand-write trigger until someone extends the macro's test suite to cover them.

Double-future collapse done first (`fn X(..) -> HostFuture<Result<T, E>>` → `async fn X(..) ->
Result<T, E>`) on the trait, `MockGuestRuntime`, and `WasmtimeRuntime` — all `Box::pin(std::future::
ready(..))` wrapper bodies removed. `compile`/`instantiate`/`drop_instance` stay plain `fn` per the
trait's own documented reasoning (CPU-bound / destructor, no suspension).

```rust
pub enum GuestRuntimes {
    Wasmtime(WasmtimeRuntime),
    #[cfg(test)] Mock(Arc<MockGuestRuntime>),
    #[cfg(test)] Recording(Arc<shard::RecordingRuntime>),
    // AsyncActor(AsyncPluginRuntime) — later packet, ⏳️runtime.rs untouched per brief
}
```

`Mock`/`Recording` wrap `Arc<..>` of the concrete type, not the bare value — deliberate deviation
from `design-dedyn.md` §1.1's illustrative sketch. Reason: many existing tests hold their OWN
`Arc<MockGuestRuntime>`/`Arc<RecordingRuntime>` to call inherent, non-`GuestRuntime` methods
(`script_turn`, `observed_events`, `last_turn_budget`, ...) on the SAME instance a
`PluginInstanceHandle`/`ShardLoop` is concurrently driving. A bare-value variant would force every
one of those ~15 test call sites to construct a second, disconnected instance, silently breaking
the "script it, then observe it through the driven handle" test pattern. `Wasmtime` needed no such
indirection (no test keeps a second handle to the same `WasmtimeRuntime`).

`RecordingRuntime` moved out of `🧵️shard`'s `mod tests` to `shard` module top level, `pub(crate)`,
so `GuestRuntimes::Recording` can name it (`shard::RecordingRuntime`) — matches the design doc's own
reference to it at that path.

15 sites converted `Arc<dyn GuestRuntime>` → `Arc<GuestRuntimes>`: `🦀️component.rs` (`PluginInstanceHandle`
struct + ctor + 9 test-construction sites), `🧵️shard/🦀️component.rs` (`ShardLoop` struct + ctor),
`🧵️shard/🏃️executor.rs` (`spawn` signature). Construction sites wrap
`Arc::new(GuestRuntimes::Mock(mock.clone()))` / `Arc::new(GuestRuntimes::Wasmtime(WasmtimeRuntime::new(..)?))`.

## 2. Work B — `poll_ready` removed, `block_on` in its place

`poll_ready` fn deleted entirely from `🦀️component.rs`. Every call site replaced:

- **`🧵️shard/🦀️component.rs`** (the 7 in-crate call sites, "by design — the poll backend"):
  `ShardLoop::pump`/`pump_primed`/`consume_frame`/`dispatch_envelope`/`send_outcome`/`heartbeat`
  are now `async fn`, `.await`ing `self.runtime.*`/`self.transport.*` directly. **Thread roots**
  (R4 clause 4) wrap the whole loop in ONE `semio_framework_async::block_on`:
  - `🧵️shard/🏃️executor.rs`'s `ShardExecutor::spawn` thread closure — `block_on(async { while ... {
    ... shard.pump_primed(primed).await ... } })`.
  - `🧵️shard/👶️child/🦀️main.rs`'s `fn main` (E3) — `block_on(async { loop { shard.pump().await
    ... } })`.
- **`🦀️component.rs`'s `PluginInstanceHandle::run_job_to_completion` (~:1528,1530, brief's
  ~:1444,1446 — line numbers shifted by earlier edits)**: tagged **E5**. `run_job_to_completion` is
  called from inside a wasmtime host-import call chain (`IoRouter::run_io`/`compose`/`identify` →
  guest wasm import → this method) — a genuinely-sync ABI boundary wasmtime's linker imposes, not
  something threadable through `.await` without mounting `⏳️runtime.rs`'s async runtime (explicitly
  out of scope this packet). `block_on` replaces `poll_ready` at the two exact call sites, tagged
  with a written E5 reason in the code.
- **Tests**: never converted to `#[async_test]` en masse — instead a single `fn pump(shard: &mut
  ShardLoop) -> Result<usize, PluginHostError> { semio_framework_async::block_on(shard.pump()) }`
  helper in `🧵️shard/🦀️component.rs`'s test module, mechanically substituted for all 20
  `shard.pump()` call sites (`#[test] fn` is a sanctioned `block_on` entry point, R4 clause 5).
  Same pattern (inline `block_on(...)` per call, no signature changes) applied to
  `mock_guest_runtime_tests`' 6 direct `poll_ready(runtime.execute_turn(..))`-style calls in
  `🦀️component.rs`, and to `process-transport`'s tests (`ProcessTransport`'s own send/recv/kill).
- **`ProcessTransport`'s `Drop::drop`**: tagged **E5** — `Drop` is E1 (external trait, language-fixed
  sync), cannot `.await`; bridges into the now-async `ShardTransport::kill` via `block_on`. Sound:
  `kill` does no real awaiting (pure `AtomicBool`/`Mutex` work).

`semio_framework_async::block_on` was NOT written by this packet — it already existed, tagged E5,
at `🧰️framework/🔨️modules/⏳️async/🦀️component.rs:434` (a prior packet's work), and this crate
already depended on `semio-framework-async` (`workspace = true`) — **zero `Cargo.toml` edits were
needed** for this packet.

## 3. Work C — `ShardTransport` and `PluginApp`

### `ShardTransport` (4 → `ShardTransports` enum, hand-written)

**Hand-written for a SECOND, independent reason beyond cfg-gating**: `ShardTransport` is declared
OUTSIDE this crate (`semio_framework_actor::ShardTransport`) — this packet's path scope forbids
touching `🔌️plugin/**` outside `🖥️host/`, so `#[dyn_enum]` cannot be applied to the trait
declaration at all. `dyn_enum_close!`'s bare-invocation mechanism (`terra-dyn-enum-macro-report.md`
finding 1) only works when the trait's OWN crate emits the captured delegation macro — a family
whose trait lives in a crate you don't own is **structurally** macro-ineligible, not merely a
cfg-gating inconvenience. Worth flagging for the remaining families: **any family whose trait is
foreign needs either a lease on that trait's owning crate to add `#[dyn_enum]`, or a hand-write.**

```rust
pub enum ShardTransports {
    SharedThread(executor::SharedThreadTransport),
    Process(super::process_transport::ProcessTransport),
    Stdio(super::process_transport::StdioTransport),
    #[cfg(test)] Loopback(LoopbackTransport),
}
```

Closed set confirmed by census: `ThreadTransport` (the actor crate's own impl, wrapped here as
`SharedThreadTransport` to dodge the `E0117` orphan rule — pre-existing pattern, unchanged),
`ProcessTransport`/`StdioTransport` (`🧵️shard/🚚️process-transport/🦀️component.rs`), `LoopbackTransport`
(test-only, moved out of `🧵️shard`'s `mod tests` to module scope, `pub(crate)`, same reasoning as
`RecordingRuntime`). All 4 impls' methods converted `fn` → `async fn` to match the trait (they were
already mismatched with the trait's async signature before this packet — see finding below).
`Box<dyn ShardTransport>` → `ShardTransports` at `ShardLoop`'s struct field, ctor, and all 15
`ShardLoop::new(..)` call sites (14 test sites wrapping `ShardTransports::Loopback(transport)`,
`executor.rs`'s `ShardTransports::SharedThread(..)`, `main.rs`'s `ShardTransports::Stdio(..)`).

**Finding — this crate did not compile before this packet's edits landed.** `ShardTransport`'s 4
trait methods are `async fn` in the live tree (someone's prior async-ification sweep), but all 4
impls (`LoopbackTransport`, `ProcessTransport`, `StdioTransport`, `SharedThreadTransport`) still had
plain sync `fn` bodies — a straight signature mismatch (E0053), and separately `Box<dyn
ShardTransport>` was never object-safe once the trait's methods went `async fn`. Every caller that
did `transport.send(..)`/`.recv()` without `.await` (dropping the resulting future, doing nothing)
was silently broken too — `🧵️shard/🏃️executor.rs`'s own `kernel_side.send(..)` test call sites (×4)
had this exact bug and are fixed alongside the enum work. This is precisely the class of "staged
async damage" `📌️important.md` describes; task C's enum conversion was the natural place to also
close it since fixing the impls' signatures is a precondition for the enum to type-check at all.

### `PluginApp` (brief: 11 dyn in host-side code) — **measured 0, not 11**

Two independent full-text scans across every `.rs` file in `🖥️host/**` (a regex `dyn ([A-Za-z_]\w*)`
scan, and a plain substring scan for `PluginApp`) both found **zero** `dyn PluginApp` anywhere in
this crate. The only `PluginApp` token present is `ActorKind::PluginApp` (`🦀️component.rs:1706,1796`)
— an unrelated enum variant name, not the SDK trait. **Recommend the packet's own dyn census be
re-measured before this count is trusted** — same caveat `terra-dyn-enum-macro-report.md` already
raised about `sol-dyn-families.json` being stale for other families. No `PluginApp` work was done
because there was nothing to do; flagging rather than fabricating a conversion.

### `HostAsyncRuntime` (10, crate-adjacent) — untouched, correctly out of scope

Per `design-dedyn.md` §1.2 this family is explicitly NOT an enum — it goes generic (`Arc<dyn
HostAsyncRuntime>` → `Arc<R> where R: HostAsyncRuntime`), a different, larger-blast-radius
transformation assigned elsewhere. Confirmed all 10 sites are in `⚡️effects/🦀️component.rs`, none
touched.

## 4. Verification

### Grep invariants — zero live `dyn GuestRuntime` / `dyn ShardTransport` in `🖥️host/**`

```
$ python3 -c "... regex 'dyn ([A-Za-z_]\w*)' scan over every .rs under 🖥️host/ ..."
```
Only comment mentions remain (`🦀️component.rs:1049`, `🧵️shard/🦀️component.rs:553` — both inside doc
comments explaining WHY the enum replaced them). Reproduced with a second, differently-implemented
query (`grep -rn "dyn GuestRuntime\|dyn ShardTransport"` piped through a comment filter) — also
zero live hits. `dyn HostAsyncRuntime` (10, out of scope) and 6 other unrelated dyn families
(`ResourceLimiter`, `RouterEffectHandler`, `EffectMetricsRecorder`, `BackboneTransport`,
`CapabilityChecker`, `StorageBackend`, `EnvelopeInjector` — all in `⚡️effects`/`⏳️imports.rs`, all
`HostAsyncRuntime`-adjacent) remain, correctly untouched.

Also swept for leftover `Box::pin(std::future::ready(..))` (zero), bare `poll_ready` (zero live,
comments only), and every `.execute_turn(`/`.start_job(`/`.step_job(`/`.cancel_job(`/`.checkpoint(`/
`.restore(`/`.send(`/`.recv(`/`.heartbeat(`/`.kill(` call site across the crate to confirm each is
either `.await`ed, `block_on`-wrapped, or a genuinely unrelated method (`mpsc::Sender::send`,
`ThreadTransport::recv_deadline`, `BackboneTransport::send(uri, payload)` — different trait,
different arity). All accounted for.

### `cargo check` — **UNRUN, blocked by `semio-framework-os-kernel`** (exactly as this packet's
brief anticipated)

```
$ CARGO_TARGET_DIR=<scratchpad>/target-host cargo check -p semio-framework-plugin-host --lib
...
error: could not compile `semio-framework-os-kernel` (lib) due to 1052 previous errors; 17 warnings emitted
```
Exit: nonzero (build halted on the dependency, `semio-framework-plugin-host` itself was never
reached). Grepped the full output for `plugin-host`/`plugin_host` — **zero matches**, confirming
every error belongs to `semio-framework-os-kernel`, none to this packet's own code.

```
$ CARGO_TARGET_DIR=<scratchpad>/target-host cargo check -p semio-framework-plugin-host --all-targets
...
error: could not compile `semio-framework-os-kernel` (lib) due to 1049 previous errors; 17 warnings emitted
```
Same result (rule 26: ran both `--lib` and `--all-targets`, not just one). The error count differs
between the two runs (1052 vs 1049) because the sibling `kernel-ripple` packet is concurrently
fixing that crate live — expected fluctuation, not a signal about this packet's own changes.

`cargo test -p semio-framework-plugin-host --lib` was **not run** — it would hit the identical
upstream compile failure before reaching this crate at all, so running it would add nothing beyond
what the two `cargo check` runs above already prove.

**This packet's acceptance is therefore UNRUN, blocked by `semio-framework-os-kernel`.** Per the
brief, this is expected and I have not edited that crate. Cheap verification proves: (a) every
targeted `dyn` is gone from this crate's own source, (b) the compile failure that DOES occur
attributes zero errors to this crate, (c) brace-balance and structural review of every edited file
by hand (no compiler available to lean on).

## 5. Cross-packet findings to lift

1. **cfg-gated variants and cross-crate trait ownership are the two real reasons `dyn_enum_close!`
   didn't apply here** — both hand-write triggers, distinct from each other. Worth ratifying as
   explicit guidance for the remaining ~50 families rather than rediscovering per-packet.
2. **`sol-dyn-families.json`'s `PluginApp: 11` for this crate does not match the live tree (0
   found, two independent scans).** Re-measure before the next wave trusts it — same caveat already
   raised for `AuditSink`/`Decider` in `terra-dyn-enum-macro-report.md`.
3. **External consumers of `GuestRuntime`/`ShardLoop` outside this packet's scope will break** once
   this lands, and neither is `🌉️mcp/🏠️workspace` (already flagged "NOT yours" in the brief):
   - `🌉️mcp/🏠️workspace/🦀️component.rs:294,353,366` — `&dyn GuestRuntime` / `Arc<dyn GuestRuntime>`.
   - `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
     and `🎠️runtime.rs` — multiple `Arc<dyn GuestRuntime>` fields/params/tests (a bench harness,
     `KernelThreadState`/`ParallelRuntime`).
   Both need `Arc<GuestRuntimes>` ripple once their packets pick this up — emitting as
   `lease-request`s, not touched here (outside `🖥️host/**`).

```
lease-request:
  file: 🌉️mcp/🏠️workspace/🦀️component.rs
  reason: Arc<dyn GuestRuntime> / &dyn GuestRuntime (3 sites) now fail to compile — GuestRuntime's
    async methods are no longer dyn-compatible (O1/R1). Needs Arc<semio_framework_plugin_host::GuestRuntimes>.

lease-request:
  file: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs
        (+ sibling 🎠️runtime.rs)
  reason: same — multiple Arc<dyn GuestRuntime> fields/params/tests, out of this packet's crate scope.
```

## 6. Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` — `GuestRuntime` trait
  (double-future collapse), `poll_ready` deleted, `MockGuestRuntime`/`WasmtimeRuntime` impls
  converted to `async fn`, new `GuestRuntimes` enum + `From` impls, `PluginInstanceHandle` retyped,
  `run_job_to_completion` → `block_on` (E5), 9 test construction sites rewrapped, 6
  `mock_guest_runtime_tests` call sites → `block_on`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📦️glue.rs` —
  `#![allow(async_fn_in_trait)]` added (R7).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs` — `ShardLoop`
  retyped and its 6 methods made `async fn`; new `ShardTransports` enum + `From` impls;
  `RecordingRuntime`/`LoopbackTransport` moved out of `mod tests` to module scope (`pub(crate)`),
  converted to `async fn`; test module's `LoopbackProbe` (`pub(super)`) + `pump()` helper; 20
  `shard.pump()` call sites, 15 `ShardLoop::new(..)` call sites.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs` —
  `SharedThreadTransport` made `pub(crate)` + `async fn` impl; `spawn`'s signature retyped; thread
  closure wrapped in ONE `block_on`; 4 `Arc::new(GuestRuntimes::Mock(..))` test sites; 4 broken
  (pre-existing, un-awaited) `kernel_side.send(..)` test call sites fixed with `block_on`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️component.rs`
  — `ProcessTransport`/`StdioTransport`'s `ShardTransport` impls → `async fn`; `Drop::drop` →
  `block_on` (E5); 8 test call sites (`send`/`recv`/`kill`/`heartbeat`) → `block_on`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️main.rs` — runtime
  construction wrapped in `GuestRuntimes::Wasmtime`, transport in `ShardTransports::Stdio`, pump
  loop wrapped in one `block_on`.

No file outside `🖥️host/**` was edited. `Cargo.toml` untouched (no new dependency needed).
`⏳️runtime.rs` not mounted, not touched, not read for editing purposes.
