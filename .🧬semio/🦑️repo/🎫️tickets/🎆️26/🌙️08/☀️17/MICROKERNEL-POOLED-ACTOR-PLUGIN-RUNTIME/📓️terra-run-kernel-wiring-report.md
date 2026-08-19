# 🎠️ terra-run-kernel-wiring — report

## Mission
Rewire `🏃️run` (and `💻️os/🖥️host`) off the direct-construct/ad-hoc-actor-id bypass and onto the real
microkernel: `kernel.activate(plugin_id) -> ActorId`, turns dispatched by the kernel, one activation
facade shared with the wgpu target's pattern.

## What I measured FIRST (baseline, before any edit)

```
$ CARGO_TARGET_DIR=<scratchpad>/target-run-kernel-wiring cargo check -p semio-framework-os-run --lib --all-targets
error: could not compile `semio-framework-number` (lib) due to 620 previous errors
```

`semio-framework-number` (not in this packet's `path_scope`) is mid an unrelated, live async-conversion
sweep — every arithmetic method (`Integer::mul`, `Rational::from_integer`, `Natural::from_str_radix`,
…) was made `async fn` but call sites inside that SAME crate were not updated with `.await`, so the
crate does not compile on its own. Then, independently:

```
$ cargo check -p semio-framework-actor --lib      → 266 errors (same missing-.await pattern:
                                                      Kernel::new/activate, ActorKind::tag, ThreadTransport::
                                                      new_pair/send/kill, lane_defaults::budget_for, … all
                                                      `pub async fn` on disk, called without `.await` inside
                                                      the same crate's OWN Default impls / Display impls /
                                                      even its own #[test] bodies)
$ cargo check -p semio-framework-os-kernel --lib  → exit 0, 57 warnings (green ALONE)
$ cargo check -p semio-framework-os --lib         → 6 errors in 🗣️dsl/📖️grammar/🦀️component.rs
                                                      (E0733 recursive-async-fn-needs-Box::pin), a
                                                      DIFFERENT failure than os-kernel shows alone —
                                                      feature-unification trap (📌️important.md's own
                                                      "Run what the CONSUMER runs" lesson), not something
                                                      I introduced
```

Full logs: `terra-run-kernel-wiring-cargo-check-os-host.txt` (13.5k lines, the `-p semio-framework-os
--lib` run after my edits), `terra-run-kernel-wiring-blocker-evidence.txt` (tails of the three isolated
checks above). **None of the three failing crates (`semio-framework-number`, `semio-framework-actor`,
`semio-framework-os-kernel`'s dsl/grammar module) are inside this packet's owned paths** (`🏃️run/**`,
`🖥️host/**`). I grepped the full 13.5k-line log for `🎠️activation.rs` and for error-tagged lines under
`🖥️host/🦀️component.rs` — **zero hits**. `semio-framework-os` (my facade's crate) and
`semio-framework-os-run` are never even reached by rustc in this build — the graph fails upstream of
both, every time, regardless of anything I changed.

**Conclusion: full compile/test verification is UNRUN, blocked by pre-existing, out-of-path_scope
breakage in a live, unrelated async-conversion sweep.** This matches the brief's own prediction ("The
SDK is not green yet, so full verification may be blocked") — the actual blocker turned out to be one
level lower (`semio-framework-actor`/`semio-framework-number`) rather than the guest SDK specifically,
but the shape is the same: I cannot get a green `cargo check` on my own files no matter what I write,
because the crates underneath them don't compile yet, for reasons entirely outside `path_scope`.

## What I changed

### 1. New shared kernel-activation facade — `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation.rs`
`pub struct NativeKernelRuntime` — owns one `semio_framework_actor::Kernel` + K real `ShardExecutor`
OS threads (K=1 for `run`'s own sequential use). `activate()` mints the `ActorId` via `Kernel::activate`,
instantiates via `GuestRuntime::instantiate`, and hands the `GuestInstance` to its pinned shard's
`ShardExecutor::register` — genuinely dispatched-by-the-kernel from here on: `submit` → `Kernel::submit`,
`tick_and_dispatch` → `Kernel::tick` + per-shard `ShardFrame::Grant`, `complete` → `to_actor_turn_result`
+ `Kernel::complete`, `wait_for_outcomes`/`try_recv_outcomes` drain the shard-outcome channel. Also
exposes `kernel_mut()` (raw `Kernel` access, no shard handoff — see below) and a `pub async fn
actor_budget_from_turn_budget` bridge helper.

**Why this lives in `semio-framework-os` and not in `🎯️targets/🧊️wgpu`'s own `ParallelRuntime`**
(`🎠️runtime.rs`, the pattern this mirrors almost line-for-line, per the brief): that crate's dependency
stack (wgpu/vello/winit/image/resvg/rfd) is wildly inappropriate for a headless CLI or this host crate.
`ParallelRuntime`'s own file is untouched — this is a parallel implementation of the identical proven
pattern, not an edit to it. **Named, not hidden, gap**: the clean long-term home for one literal shared
type is `semio-framework-plugin-host` (`🔌️plugin/🖥️host/**` — every type both copies build on already
lives there), which is a *different* "host" directory than this packet's owned `💻️os/🖥️host/**`
(exactly the naming hazard `📌️important.md` already warns about) and outside `path_scope`. Emitting as
a `lease-request` below rather than attempted here.

Mounted from `📦️glue.rs` as `#[cfg(not(target_arch = "wasm32"))] pub mod activation;`. Added
`semio-framework-actor`, `semio-framework-plugin-host`, `semio-framework-async` as native-only
dependencies to `🖥️host/📦️packages/🦀️rust/Cargo.toml` — **this crate depended on none of them before**,
which is the literal gap the brief names ("💻️os/🖥️host does not depend on plugin-host at all").

### 2. `🏃️run/🦀️component.rs` — `WasmtimeNodeHost` now drives real turns through the kernel
- `AppChannelHost::open` is now `async fn` (was sync) — propagated through `SpaceRunner::open_node`
  (now `async fn`) and its one call site in `compute_node` (already `async`).
- `WasmtimeNodeHost` fields: `runtime: Arc<GuestRuntimes>` → `guest_runtime: Arc<GuestRuntimes>` +
  `kernel: NativeKernelRuntime`; `next_actor_ordinal: u64` (the ad hoc counter) → `plugin_ordinals:
  HashMap<String, u16>` (one ordinal per DISTINCT plugin, matching `Kernel::activate`'s own contract —
  same convention the wgpu target's `kernel_runtime::plugin_ordinal` already uses); added
  `instance_actors: HashMap<u32, ActorId>`, `now_ms`, `next_turn_seq`.
- `load_runtime_recursive`'s actor id now comes from `self.kernel.kernel_mut().activate(...)` — the
  **literal fix for "🏃️run … mints its own RuntimeActorIds"**. Deliberately the RAW `Kernel::activate`
  (via `kernel_mut()`), not the full facade `activate()`: this instance's `GuestInstance` stays owned by
  `PluginInstanceHandle`, which calls `GuestRuntime::execute_turn` on it directly for post-turn job
  dispatch (`io_router`/`inference_router`) — routing THAT through `ShardExecutor` ownership (what the
  full facade does) would take the instance away from `PluginInstanceHandle` entirely, which is
  `🔌️plugin/🖥️host`-internal and out of `path_scope`. Also made this fn (and `manifest_for`) `async fn`;
  `load_runtime_recursive`'s self-recursion over `manifest.dependencies` is now `Box::pin(...).await`
  (R10's residue shape 3 — a recursive async fn needs boxing).
- `open()` now has a REAL body (was `unreachable!()` — that comment was stale: `manifest_for` stopped
  "always erroring" once R1-native-manifest landed committed descriptors, a fact this file's own doc
  hadn't caught up to): resolves the manifest, mints the app-instance `ActorId` via the FULL
  `kernel.activate(...)` (this one DOES hand the `GuestInstance` to a real `ShardExecutor` thread — a
  genuine per-app-instance actor, not the plugin-service one above), submits `Event::InstanceOpen`
  through the new `run_turn` helper.
- `exchange()`'s passthrough branch now has a REAL body too (was a hard "not built in this packet"
  error): encodes each passthrough `AppCommand` as one `Event::AppCommandEvent`, submits the batch via
  `run_turn`, and decodes `TurnResult.effects`' `Effect::Respond{req, result}` entries back into
  `AppFrame`s via `protocol::decode_app_frame` — **exactly the mechanism this file's own pre-existing
  doc comment already specified** (I verified `Effect::Respond`/`RequestOutcome` are real types in
  `🎠️kernel/🦀️component.rs`, not stale — the doc was accurate, just unimplemented).
- New `run_turn` (private): `Kernel::submit` → loop `tick_and_dispatch`/`wait_for_outcomes`/`complete`
  until nothing left to grant → returns the driven actor's `TurnResult.effects`. Mirrors
  `🎯️targets/🧊️wgpu/📦️glue.rs`'s own `KernelThreadState::run_turn` almost line for line, minus
  UI-surface reconciliation (which `run` has no use for).
- New `app_command_seq` helper: an exhaustive match over all 27 `AppCommand` variants extracting their
  `seq: u64` field, reused directly as the kernel envelope's own `Event::AppCommandEvent.seq` (sound
  because one `exchange` batch's commands already carry distinct seqs from `SpaceRunner::compute_node`'s
  own local counter, so `Effect::Respond{req}` correlates directly with no separate translation table).
- `NODE_TURN_BUDGET` constant factored out of `load_runtime_recursive`'s old inline `let budget = ...`
  and now shared with `run_turn` too (same values, one definition).
- Two `#[cfg(test)]` `AppChannelHost` impls (`FakeHost`, `RecorderHost`) updated `fn open` → `async fn
  open` to match the trait. One real test, `note_plugin_manifest_loads_from_its_committed_descriptor`
  (the actual "native smoke" test the brief's own header refers to), updated to drive the now-`async`
  `WasmtimeNodeHost::new`/`manifest_for` with `futures_lite::future::block_on` — tagged as a sanctioned
  R4-clause-5 test bridge, same convention every other test in this module already uses.

### 3. `🏃️run/📦️bin.rs`
`fn run` split into a thin `semio_framework_async::block_on(run_async(args))` wrapper + `async fn
run_async` carrying the whole old body — needed because `WasmtimeNodeHost::new` is `async fn` now too
(it builds the `NativeKernelRuntime`). Switched from `futures_lite::future::block_on` to
`semio_framework_async::block_on` — the canonical E5 bridge — per the brief's explicit instruction;
tagged `// 🚫️async: E5 executor bridge`, one per crate (R2). `SpaceRunner::run`'s own await moved inline
(no longer needs its own nested `block_on`, since `run_async` is already async).

### 4. `🏃️run/📦️packages/🦀️rust/Cargo.toml`
`futures-lite` moved from `[dependencies]` to `[dev-dependencies]` — grepped the whole crate after the
above changes and confirmed every remaining `futures_lite::` call site is inside `#[cfg(test)] mod
tests` (11 call sites, all pre-existing plus the one new one in the native-manifest test); production
code's one bridge is `semio_framework_async::block_on` now.

## What I did NOT change (explicitly out of scope, named rather than silently skipped)
- `PluginInstanceHandle`'s direct `GuestRuntime::execute_turn` calls (`io_router`/`inference_router`
  post-turn job dispatch) — lives in `🔌️plugin/🖥️host/**`, not owned.
- `artifact_ref` binding into `instance_directory` at `open()` time — needs the manifest's own app
  entry's `io.document_schema`, a separate, still-unwired concern per this struct's pre-existing doc.
- `run_transaction`/`undo_transaction_group`'s `exec`/`plan` closures — already documented elsewhere as
  needing a full post-turn effect-dispatch loop; untouched.
- Relocating `ParallelRuntime` itself into `semio-framework-plugin-host` so `run`/this host/wgpu share
  ONE literal type instead of two parallel implementations of the same pattern.

## `lease-request`
```lease-request
path: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/**
reason: relocate 🎯️targets/🧊️wgpu/🎠️runtime.rs's ParallelRuntime into
  semio-framework-plugin-host (e.g. a new pub mod parallel_runtime, re-exported) so
  run's NativeKernelRuntime (🖥️host/🎠️activation.rs, this packet) and the wgpu target's
  ParallelRuntime become ONE literal type instead of two hand-synced copies of the same
  ~230-line pattern. Not attempted here: outside run-kernel-wiring's owned paths
  (💻️os/🖥️host/** is a different "host" directory than 🔌️plugin/🖥️host/**), and the
  wgpu target file itself would need its own import switched over too.
```

## Regressions
None observed — nothing that compiled before my edits still fails to compile *because of my edits*;
the whole dependency graph was already red before I started (see baseline above), for reasons entirely
outside `path_scope`.

## Files touched
- `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation.rs` (new)
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml`
- Ticket-folder evidence: `terra-run-kernel-wiring-cargo-check-os-host.txt`,
  `terra-run-kernel-wiring-blocker-evidence.txt`

## For the coordinator / a sibling packet
- **Cross-packet finding, lifted here per W4 item 8**: `semio-framework-actor` (266 errors) and
  `semio-framework-number` (620 errors) are BOTH currently red from the same defect class (async
  conversion landed on function signatures without inserting the matching `.await`s at call sites
  inside their own crates) — neither is this packet's to fix, but whichever packet(s) own them should
  know two independent crates are down with the identical symptom right now.
- `semio-framework-os-kernel --lib` alone is green; `semio-framework-os --lib` (which depends on it) is
  not — a live instance of the "run what the CONSUMER runs" feature-unification trap
  `📌️important.md` already documents. The 6 errors are `E0733` (recursive async fn needs `Box::pin`) in
  `🗣️dsl/📖️grammar/🦀️component.rs`'s `print_symbol`/`print_alternatives`/`print_prim` — not touched by
  this packet, not in `path_scope`.
- Once the two crates above are green, re-run `cargo check -p semio-framework-os --lib` and `cargo check
  -p semio-framework-os-run --lib --all-targets` (both with `CARGO_TARGET_DIR` in the scratchpad, per
  rule 24) — that is the actual acceptance gate for this packet's own code, still UNRUN.
