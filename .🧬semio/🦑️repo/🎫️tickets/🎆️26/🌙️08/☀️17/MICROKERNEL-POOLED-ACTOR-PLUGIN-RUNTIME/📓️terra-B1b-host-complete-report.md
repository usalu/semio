# 📓️ terra — B1b-host-complete report

Packet `B1b-host-complete`, finishing packet `B1-host-native`. Executor "terra" (Sonnet 5). Working-tree
baseline throughout (never `HEAD`). Target dir `🎯️target-b1b` used for every cargo command.

## 0. Session summary

Started by reading `📓️terra-B1-host-native-report.md`. B1 had landed `GuestRuntime`/`MockGuestRuntime`/a
real `WasmtimeRuntime` but was blocked on an upstream WIT keyword bug and had done zero deletion work.
By the time this packet started, the guest side was green (confirmed: `cargo check -p
semio-framework-plugin --target wasm32-wasip2 --features component-guest` → `Finished`, per the
dispatch brief) and the WIT had moved to one consolidated file, world `actor`.

First real build attempt surfaced **112 errors**, almost all self-inflicted by B1's own
`additional_derives: [Clone, Debug]` on the `bindgen!` call — wasmtime-wit-bindgen 22.0.1 always writes
a hand-rolled `impl Debug` for every WIT `record`/`variant`, so requesting `Debug` again in
`additional_derives` collides with it (`E0119`, ~91 of the 112). Fixing that one line, plus a
`jobs`/`reactor` import-path bug (they are `export`s of `world actor`, so their bindings live under
`actor_bindings::exports::...`, not the top level), dropped the count to the 24 errors B1's report
predicted: `WasmPluginRuntime`/`ExtensionRuntime` referencing dead `PluginWorld`/`ExtensionWorld` types,
plus 2 pre-existing, out-of-scope `DefaultApp` errors.

## 1. Files touched, SHA-256 (final)

| file | status | sha256 | lines |
|---|---|---|---|
| `🔌️plugin/🖥️host/🦀️component.rs` | edited | `ef3a7e1b869aab02f8ffdb13c8a0c7a873e6be761394f3b3d68447316d9dbc1f` | 4061 |
| `🔌️plugin/🖥️host/🧵️shard/🦀️component.rs` | **new** | `d1e74302f684a7f3094e853ea5240ed1023f040ba89e474c3e5210c8d813abf7` | 281 |
| `💻️os/🔨️modules/🏃️run/🦀️component.rs` | edited | `595bf33683d12a0aa17bd0918d46bbe7b2b23d88a1091900c9d04d3b100538b0` | 2025 |
| `💻️os/🖥️host/🦀️component.rs` | **not touched** | unchanged | — |
| `💻️os/🔨️modules/🧩️extension/🦀️component.rs` | **not touched** (verified: no `WasmPluginRuntime`/`ExtensionRuntime` reference, unaffected by the deletion) | unchanged | — |

`git diff --stat` for the two edited files: `🏃️run/🦀️component.rs` +320/-... (net −~90 lines), `🔌️plugin/🖥️host/🦀️component.rs`
1898 lines touched (+564/−1654 net, i.e. a large net *shrink* — the deletions in §3 are bigger than the
additions in §1/§2).

## 2. Status of each required-result item

### `ShardLoop` — **DONE**
New file `🔌️plugin/🖥️host/🧵️shard/🦀️component.rs`, wired in via `#[path = "🧵️shard/🦀️component.rs"]
pub mod shard;` at the top of the host `component.rs` (mirrors the `⚛️reactor` module's own
`#[path]`-submodule idiom). `ShardLoop::pump` drains every currently-buffered `Envelope` off a
`ShardTransport` (real `semio_framework_actor::ShardTransport` trait, packet A1 — I did **not**
redefine it, per the design doc's own note that it already exists there), groups `Payload::Event`s by
destination actor, runs exactly one `execute_turn` per actor with an envelope this pump, and separately
drives `Payload::JobStep{job}` through `step_job`; every outcome (`ShardOutcome::Turn`/`Job`/`Fault`) is
sent back over the same transport as JSON bytes. `Suspend`/`Resume`/`Cancel` payloads are real
`semio_framework_actor::Payload` variants with no `GuestRuntime` counterpart yet (no live `Kernel`
scheduler exists to decide *when* to checkpoint/restore) — surfaced as a `ShardOutcome::Fault` naming
the reason, not a silent drop. 3 tests, all real (a `LoopbackTransport` implementing the real
`ShardTransport` trait, not a fake API): one full turn round-trip, one "unregistered actor is a
reported Fault, not a vanished envelope," one unregister/actor-count.

Left the process-transport seam alone as instructed — no `ProcessTransport`, no `semio-shard` `[[bin]]`.

Also added `GuestRuntime::start_job` (design-runtime.md §2's trait listing has `step_job` but not
`start_job`, even though `jobs.wit` declares three functions, not one — a job cannot be stepped before
it exists). Implemented for both `WasmtimeRuntime` (calls the real WIT `start-job` export) and
`MockGuestRuntime` (trivial: `start-job` has no interesting `Ok` payload to script).

### Post-turn router relay — **DONE for the mechanism that has a real job kind; explicitly NOT for the one that doesn't**
New `PluginInstanceHandle` (`//#region 🔀️PostTurnRelay`, same file) replaces `Arc<WasmPluginRuntime>` as
the thing `IoRouter`/`ArtifactInferenceRouter` hold per registered plugin. It wraps `{actor,
runtime: Arc<dyn GuestRuntime>, instance: Mutex<GuestInstance>}` and drives one cold job
(`start-job` → `step-job` loop until `Done`/`Failed`) to completion — safe because every caller runs
POST-TURN, against a *different* actor's instance than whichever turn's effect triggered it (never
re-entrant into an in-flight turn's own `Store`, the exact deadlock `IoRouter::run_io`'s own doc already
guards against one layer up at route-resolution time).

- `io_run`/`io_sniff` dispatch via `semio.io-run`/`semio.io-sniff` — both **real, documented job kinds**
  (`jobs.wit`'s own doc comment names them). `IoRouter::run_io`/`identify`'s pure route-resolution
  algorithm (≤3 hops, cycle-free, ranking, self-owned-hop reentrancy guard) is **byte-for-byte
  unchanged** — only the dispatch call at the end of each hop moved from `WasmPluginRuntime::io_run`
  to `PluginInstanceHandle::io_run`.
- `infer` dispatches via `semio.infer` (also a documented job kind, `types.wit`'s own comment).
- `IoRouter::compose` (the OLD `IoKey`-keyed mechanism) still resolves ownership correctly (pure,
  unchanged), but its dispatch now returns a clear `PluginHostError` instead of guessing a wire format:
  `artifact-compose` is not an export of `world actor` at all, and no job kind for the old mechanism is
  named anywhere in the WIT (only `semio.io-run`/`semio.io-sniff`/`semio.infer` are). I judged inventing
  one unilaterally, with no real component to validate the guess against, worse than a documented gap —
  same policy B1 already applied to `Effect::IoRun` (still blocked on A3, see §5).
- `IoRouter`/`ArtifactInferenceRouter::register_plugin` no longer take an `Arc<WasmPluginRuntime>` and
  make no wasm call at all: `world actor`'s `describe()` interface says plainly that
  `list-artifact-dialects`/`list-io-entries`/`list-artifact-inferences` are now static data inside the
  build-time `PackageDescriptor` (packet E1), not runtime exports — so registration now takes the
  ALREADY-DECODED roster as parameters. `AppRouter::register_plugin`'s wrapper (which only ever read
  `runtime.manifest`, no wasm call) was deleted outright; every caller now calls the pure
  `register_manifest(plugin_id, &manifest)` it already delegated to.
- New tests (`IoRouterPostTurnRelay` region): `WasmtimeRuntime` still correctly compiles real
  `stdio.wasm`/`cad.wasm` and correctly *rejects* instantiating them (neither exports `world actor`
  yet — same honest-negative convention `wasmtime_runtime_tests` already used); a `PluginInstanceHandle`
  driving a scripted `Running` → `Done` job to completion through a real `start_job`/`step_job` loop; a
  full `IoRouter::run_io` 2-hop cross-plugin route through TWO separate `PluginInstanceHandle`s backed
  by TWO separate `MockGuestRuntime`s (proves the chain really crosses instance boundaries, not just
  calls the same handle twice); `IoRouter::compose` resolving ownership correctly then reporting
  "not yet wired" rather than a resolution failure.

### Deletions — **DONE**
`WasmPluginRuntime`, `ExtensionRuntime`, `PLUGIN_FUEL_BUDGET` (all three on `📌️important.md`'s
"must not exist at exit" list) are gone from `🔌️plugin/🖥️host/🦀️component.rs`. Also deleted the OLD
`HostState` (the pre-actor synchronous `impl semio::framework::host::Host for HostState` block) — not
explicitly named on the list, but it only ever existed to back `WasmPluginRuntime`'s Store, implemented
a WIT `host` interface that no longer exists (`world actor` imports only `pure`), and every reference to
it outside its own region was already dead. `grep`-verified after deletion: zero remaining references
to any of the four outside test code (which I rewrote — next paragraph) and doc comments.

**`ProgramSupervisorState` — only ONE of the two definitions deleted.** The one inside the deleted
`WasmPluginRuntime` region is gone. The second, in `💻️os/🖥️host/🦀️component.rs`'s `PluginHost` struct
(`supervisor: HashMap<String, ProgramSupervisorState>`, actively written by `load_plugin`/
`hot_swap_plugin`/`hot_swap_failed`, read by `recovery_ui`), is **untouched** — see §5.

The ~510-line real-wasm test block (`mod tests` at the bottom of the host `component.rs`) was rewritten,
not deleted:
- `//#region 🔖️IoRouterW1d`'s 6 tests (route determinism/ranking/max-hops/reentrancy-guard/conflict-
  rejection) are **byte-for-byte unchanged** — they were already pure, no `WasmPluginRuntime` dependency,
  never needed touching.
- `wasm_plugin_runtime_api_exists`/`extension_runtime_constructs_engine_and_linker`/
  `wasm_plugin_runtime_loads_real_plugin_component_if_present` — deleted. They tested types that no
  longer exist, and the third was redundant with `wasmtime_runtime_tests`' own real-`stdio.wasm`
  coverage (which correctly asserts the OPPOSITE: instantiate now *rejects* pre-migration wasm).
- `io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins` (the packet brief's
  own "hard part") — **could not be preserved as a real end-to-end wasm test**, for two independent
  reasons, both already true before I started and neither fixable from inside my `path_scope`: (1) no
  `.wasm` in this repo exports `world actor` yet (confirmed, same finding B1 already made — stdio/cad
  predate the ABI migration; W3 hasn't happened), and (2) the mechanism it tested — `IoKey`-keyed
  `compose` — has no `world actor` guest entry point at all (see the post-turn-relay section above).
  Replaced with: (a) the real-wasm compile/instantiate-rejects test (§ above), which is the honest
  successor to what this test's FIRST two lines did, and (b) `MockGuestRuntime`-backed coverage that
  proves the exact same "two loaded plugins, one shared router, a call that can only be answered by
  crossing into the OTHER plugin's instance" shape this test proved, narrowed to the mechanism
  (`run_io`) that actually has a reachable dispatch path today. This is genuinely *new*, real coverage
  of the post-turn relay itself (which had zero coverage before this packet, since it didn't exist) —
  not a downgrade in what it tests, a change in which mechanism it tests, forced by the WIT.
- `plugin_dependency_infrastructure_wires_real_loaded_plugins_and_one_real_extension` (the
  `W2aPluginDependencyE2e` region) — deleted. Its `PluginGraph`/`InstanceDirectory` assertions are
  redundant with `plugin_graph_tests`/`instance_directory_tests` (separate, already-existing, unaffected
  modules elsewhere in this file, both still pure-fixture and still green); its
  `ArtifactMutationRouter`/`ArtifactInferenceRouter`/`ExtensionRuntime` portions called
  `list_artifact_mutations()`/`list_artifact_inferences()`/`ExtensionRuntime::new()` — all three gone,
  no replacement reachable without E1's descriptor decode.
- `merge_policy_gates_a_real_dispatch_and_laissez_faire_still_surfaces_its_message`
  (`MergePolicyE2e`) — deleted. This test was never about `IoRouter`/cross-plugin routing at all — it
  exercised the OLD per-verb `exchange`/`create_app`/`set_merge_policy` ABI end to end against a real
  `block.wasm`. That whole mechanism has no `world actor` equivalent (see item 4 below); preserving it
  would require solving the SAME "real per-command reply correlation over `execute_turn`'s effects"
  problem `WasmtimeNodeHost::exchange` hits, which belongs with the kernel/scheduler packets (H1-H4/T1),
  not `IoRouter`.

`cargo test -p semio-framework-plugin-host` output is pasted in §6 — it currently cannot run at all
(blocked by the pre-existing `DefaultApp` bug, §5), so none of the above could be verified by actually
running `cargo test` this session. Every one of the surviving/rewritten tests DOES compile (confirmed:
`cargo check -p semio-framework-plugin-host --all-targets` has zero errors outside `DefaultApp`) and I
read each one back after writing it to check the assertions are real, but "compiles" is not "passes" —
flagging this explicitly per the mandatory-honesty rule.

### `🏃️run`'s `WasmtimeNodeHost` onto `GuestRuntime` — **PARTIAL, and the honest reason why**
`semio-framework-os-run --all-targets` now compiles with **zero errors, zero warnings from this crate's
own code** (full output in §6). Getting there required three genuinely separate things:

1. **Channel v12 fallout** (not mine to have caused — A4-channel landed mid-session and retired
   `AppCommand::Hello`/`AppFrame::Welcome`/`UiSection`/`Effects`/`Events`, replacing the last two with
   `UiPatch`/`UiSnapshotEnd`; its own doc comment: "lifecycle now arrives through the reactor ABI's
   `Event::InstanceOpen`/`InstanceClose`"). Fixed `frame_in_reply_to`'s match, dropped the `Hello`
   command from `SpaceRunner::compute_node`'s frame script (now starts with `SetMergePolicy`), and
   removed `FakeHost::exchange`'s now-impossible `Hello`/`Welcome` arm. These are correct, verified
   fixes independent of the ABI migration below.
2. **`register_plugin` call-site signature updates** — mechanical, following straight from the
   post-turn-relay section's signature changes.
3. **`WasmtimeNodeHost` itself onto `GuestRuntime`** — the deep part. `WasmPluginRuntime::exchange`
   (a single synchronous per-command-batch RPC returning `AppFrame`s directly) has **no `world actor`
   equivalent at all** — the new ABI's only guest entry points are `reactor::poll(events, budget) ->
   turn-result` and `jobs::{start-job,step-job,cancel-job}`. Channel v12's own doc confirms the
   INTENDED replacement for the lifecycle half (`Event::InstanceOpen`/`InstanceClose`), and
   `Effect::Respond{req, result}` (already marshaled by `EffectEventMarshal`) looks like the intended
   per-command-reply half — but turning "submit `AppCommandEvent`s, call `execute_turn`, correlate
   `Effect::Respond`s by `req.0 == seq`, translate into `AppFrame`s" into real code needs a genuine
   design decision about the WIRE CONVENTION between `req`/`seq` and the `Respond` payload shape that
   is not specified anywhere in this ticket, and — separately — `run_transaction`'s contributed-mutation
   planning (`artifact_mutation_plan`) and `document_session` have no `world actor` equivalent whatsoever.
   Both are real post-turn dispatch loops over `execute_turn`'s effects that belong with the
   kernel/scheduler packets (H1-H4/T1), not `IoRouter`/`GuestRuntime` — building them here, blind, with
   no real `world actor` component to validate against, risked shipping confidently-wrong protocol that
   a future packet would have to un-invent.
   
   There is also a SECOND, independent blocker discovered while doing this: `PluginManifest` itself has
   no `world actor` source any more either. `WasmPluginRuntime::load` used to get it via a real wasm
   call (`plugin.manifest()`, the OLD `plugin` interface — gone). `describe()`'s doc comment says this
   is now static data inside a build-time-only packed `PackageDescriptor` (packet E1, concurrently in
   flight, not decoded anywhere I have access to). So even the STRUCTURAL parts of `WasmtimeNodeHost`
   (compile a component, instantiate it, walk its manifest's dependencies) are blocked one layer
   upstream of the exchange problem.

   **What I actually did**: restructured `WasmtimeNodeHost` onto real `GuestRuntime`/`CompiledHandle`
   types (`runtime: Arc<dyn GuestRuntime>`, `compiled_for_plugin` cache, `guest_instances` map — all
   correctly typed, none populated yet), made `load_runtime_recursive` compile a plugin's real bytes
   (this part DOES work — `WasmtimeRuntime::compile` succeeds on real `.wasm`) then fail loudly and
   explicitly at the manifest-read step with a message naming exactly what's missing and why, and made
   `open`/`exchange`/`run_transaction`/`undo_transaction_group` propagate that same class of clear,
   named error instead of guessing at unspecified wire behavior. Every one of these methods is real,
   correctly-typed code reachable the moment E1's descriptor decoder and a real `execute_turn`-based
   effect-dispatch loop exist — not a blanket `todo!()`. `resolve_open_artifact`/`set_default_app`/
   `clear_default_app` (pure host-level ops, no wasm dependency) are **fully untouched and still work**.

## 3. `PluginHost.supervisor` (💻️os/🖥️host/🦀️component.rs) — **NOT STARTED**
Design-runtime.md §"FailurePolicy": "`PluginHost.supervisor` becomes a read view over `KernelMetrics`."
I did not touch this file this session. Honest reason: `KernelMetrics` (packet A1's
`semio_framework_actor::KernelMetrics`) is a real type, but nothing in this codebase yet wires up a
live `Kernel` instance for `PluginHost` to read metrics FROM — `Kernel`/`ShardTable`/`Scheduler` exist as
types in `semio_framework_actor` (A1, confirmed present) but no packet in this ticket instantiates a
live one anywhere yet (that's H1-H4/T1's job). Turning `PluginHost.supervisor: HashMap<String,
ProgramSupervisorState>` (actively written by `load_plugin`/`hot_swap_plugin`/`hot_swap_failed`, read by
`recovery_ui`) into "a read view over kernel metrics" with no live kernel to view is the same shape of
premature-invention risk I declined to take for `WasmtimeNodeHost::exchange` above, compounded by zero
time budget left after the rest of this packet. `ProgramSupervisorState`'s second definition therefore
still exists, and is still live/written/read — flagging honestly as **not done**, not attempted.

## 4. `## peer-coexistence`

`CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`'s `IoRouter` region (route resolution: `resolve_io_route`,
`walk_io_routes`, `io_route_rank`, `io_entries_conflict`, `route_reenters_calling_plugin`) — **every
one of these functions is untouched, verbatim**, confirmed by re-reading them against B1's own report's
line-range record before editing anything nearby. Only `IoRouter`'s STRUCTURAL fields
(`runtimes: HashMap<String, Arc<PluginInstanceHandle>>`, was `Arc<WasmPluginRuntime>`) and the
DISPATCH call sites at the end of `run_io`/`identify`/`compose` changed — exactly the "call-boundary
moves, algorithm doesn't" instruction `📌️important.md` gives for this exact region. All 6 of the
peer's own `IoRouterW1d` tests still pass unmodified (well, unrun — see `cargo test`'s blocked status,
§5 — but they compile unchanged and I did not alter their assertions).

A4-channel landed mid-session (`CHANNEL_VERSION = 12`, `Hello`/`Welcome`/`UiSection`/`Effects`/`Events`
retired) and broke `🏃️run/🦀️component.rs` in the way its own packet brief predicted ("will make the
tree red ... the renderer packets can pick them up"). I picked it up as part of getting `os-run` green
— documented in §2 item 4, not silently absorbed.

## 5. `## blocked-on`

1. **`🎚️config/🧬️schema/🧬️mutations/🦀️component.rs:75,92`, `DefaultApp` not in scope** — same bug B1's
   report already flagged, confirmed still present, confirmed still NOT from any packet in this ticket
   (`git log --date=iso` on that file: last touched 2026-08-17 13:55 by commit `506c4f39d5`, part of a
   `🚩️52X`-numbered commit sequence unrelated to this ticket's naming). The fix is a one-line import
   (`use crate::opening_config::DefaultApp;` inside that file's own test module — the compiler's own
   suggestion), but the file is outside every packet's `path_scope` in this ticket. **This is the ONLY
   thing standing between `semio-framework-plugin-host` and a fully green `cargo check --all-targets`
   AND the only thing blocking `cargo test` from running at all.** Filing as a
   ```lease-request
   file: 🎚️config/🧬️schema/🧬️mutations/🦀️component.rs
   change: add `use crate::opening_config::DefaultApp;` to the test module containing the two
    `DefaultApp {...}` literals at lines ~75 and ~92 (or fully-qualify each call site).
   reason: blocks `cargo test -p semio-framework-plugin-host` and `--all-targets` from ever
    reaching a clean run for any packet in this ticket; a one-line, zero-risk import fix.
   ```
2. **`Effect::IoRun` still doesn't exist in `semio_framework::kernel`** — confirmed unchanged from B1's
   own finding (`🎠️kernel/🦀️component.rs`'s `Effect` enum has `RegistryQuery`/`IoCompose` but no
   `IoRun` variant). `🎠️kernel` is out of every packet's `path_scope` in this ticket except its owner.
   `EffectEventMarshal`'s `E::IoRun(_inner) => return Err(...)` (B1's own code, untouched by me) and the
   `io_run_effect_is_a_reported_error_not_a_silent_mismap` test that pins this behavior are both still
   exactly as B1 left them.
3. **Manifest decoding (`PluginManifest` from a real component) and the `exchange`/transaction
   effect-dispatch loop** — both new findings this session, both explained in full in §2 item 4. Neither
   is a bug I introduced; both are consequences of the ABI migration this ticket is executing, and both
   belong to E1 (manifest/descriptor decode) and H1-H4/T1 (kernel/scheduler) respectively, not B1b.
4. **`PluginHost.supervisor`** — not started, §3.

## 6. `## acceptance`

All three commands, run in the **foreground**, in this session, in this order, with
`CARGO_TARGET_DIR=".../MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target-b1b"`:

```
$ cargo check -p semio-framework-plugin-host --all-targets
    Checking semio-framework-plugin-host v0.1.0 (.../🔌️plugin/🖥️host/📦️packages/🦀️rust)
error[E0422]: cannot find struct, variant or union type `DefaultApp` in this scope
  --> .../🎚️config/🧬️schema/🧬️mutations/🦀️component.rs:75:45
error[E0422]: cannot find struct, variant or union type `DefaultApp` in this scope
  --> .../🎚️config/🧬️schema/🧬️mutations/🦀️component.rs:92:56
error: could not compile `semio-framework-plugin-host` (lib test) due to 2 previous errors
```
Exit code: **101**. Both errors are the pre-existing, out-of-scope `DefaultApp` bug (§5.1) — the
`(lib)` target itself (not `(lib test)`) compiles with **zero errors and zero warnings** (confirmed by
re-running with just `--lib`, and by the full `--all-targets` log showing no `semio-framework-plugin-host
(lib) generated N warnings` line at all).

```
$ cargo test -p semio-framework-plugin-host
error[E0422]: cannot find struct, variant or union type `DefaultApp` in this scope  (×2, same as above)
error: could not compile `semio-framework-plugin-host` (lib test) due to 2 previous errors
```
Exit code: **101**. Cannot run — blocked upstream by the same bug. **Not claiming any test passed** —
this command never reached the point of running a single test this session.

```
$ cargo check -p semio-framework-os-run --all-targets
    Checking semio-framework-os-run v0.1.0 (.../🏃️run/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 27.77s
```
Exit code: **0**. Zero errors, zero warnings.

## 7. What I'd do next, in order, once the blockers clear

1. Land the one-line `DefaultApp` import fix (§5.1) — unblocks `cargo test`, letting every rewritten
   test in this packet actually run for the first time.
2. Once E1's `PackageDescriptor` decoder exists: wire `load_runtime_recursive`'s marked continuation
   point (the block comment in `🏃️run/🦀️component.rs` spells out the exact sequence) — decode the
   manifest, recurse over dependencies, instantiate a `GuestInstance`, wrap it in a
   `PluginInstanceHandle`, register with every router using the NEW pre-decoded-roster signatures this
   packet already built.
3. Once H1-H4/T1 land a real effect-dispatch loop over `execute_turn`: wire `WasmtimeNodeHost::open`
   (submit `Event::InstanceOpen`) and `exchange` (submit `Event::AppCommandEvent`s, correlate
   `Effect::Respond` by `req.0 == seq`) — the design is sketched in both methods' own doc comments.
4. Once A3 (or whoever owns `🎠️kernel`) adds `Effect::IoRun`: `EffectEventMarshal`'s one blocked arm
   unblocks itself; `IoRouter::run_io`'s dispatch is already real and waiting.
5. `PluginHost.supervisor` → `KernelMetrics` read view, once a live `Kernel` exists anywhere to read
   from (§3).

## 8. Temporary files

All scratch is `.txt`/`.md` inside this ticket folder: `terra-b1b-check1.txt` through
`terra-b1b-check10.txt`, `terra-b1b-checkFinal.txt`, `terra-b1b-test1.txt`, `terra-b1b-testFinal.txt`,
`terra-b1b-osrun1.txt` through `terra-b1b-osrun4.txt` (raw `cargo check`/`cargo test` output, kept as
evidence per this ticket's own rule against deleting ticket-folder scratch). No `.log` files. No
`[DEBUG]` markers were added (none were needed — every diagnostic in this pass came from real compiler
output, pasted above, not print-debugging).
