# Wave B17 — the `plugin.reactor-close-authority` trap

Implementation pass, 2026-09-11. Target: the guest actor trapping with
`plugin.reactor-close-authority: "native close terminal unavailable"`, reported as the fleet's
highest-priority defect by `📓️2026-09-11-wave-B13-export-history-locale.md` §5.

Every command tail and every browser reading quoted below is real output from this pass or from the
archived probe logs named inline.

**Headline.** The trap is real and is now fixed at three levels, but **B13's diagnosis of it was
wrong on both the trigger and the timeline**, and several waves have been scoped off that wrong
reading. The trap is not a 30-second idle timer, it is not in any battery, and "Agent disconnected"
is not a symptom of it.

---

## 1. When it appeared — per build

`reactor-close-authority` **does not appear in a single battery**. Measured over every battery
artefact in `🗑️generated/` (`hot-swap` / `activation.revoked` / `trapped` occurrence counts):

| artefact | build | hot-swap | revoked | trapped |
| --- | --- | --- | --- | --- |
| `probe-2026-09-11T12-10-48.md` | #44-pre | 0 | 0 | 0 |
| `probe-2026-09-11T12-33-00.md` | #44-pre | 0 | 0 | 0 |
| `probe-2026-09-11T13-15-23.md` | #44 | 0 | 0 | 0 |
| `probe-2026-09-11T14-48-04.md` | #45b | 0 | 0 | 0 |
| `probe-2026-09-11T15-13-33.md` | #46 | 0 | 0 | 0 |
| `battery-2026-09-11-{44pre,44,45,45b,46}-6013.txt` | all | 0 | 0 | 0 |

It appears in exactly **three** short `--only=` lane probes:
`probe-2026-09-11T13-14-30.md`, `probe-2026-09-11T13-51-51.md`, `probe-2026-09-11T15-13-01.md` —
i.e. #44-era, #44-era and #46-era. So it is **not** a #45 or #46 regression and it did not arrive
with B0/B2/B3/B8/B9 or with any host-live change (B4/B6/B14). It is orthogonal to the build.

What all three have in common is the console line immediately **before** the trap and the line
immediately **after** it. From `probe-2026-09-11T13-14-30.md`'s console tail, in order:

```
log:   [DEBUG] hot-swap puzzle {pluginId: puzzle, version: 0.1.0, addedApps: Array(0), removedApps: Array(0)}
error: [DEBUG] PluginRuntime: turn failed for actor puzzle#1 Error: actor-activation.revoked
log:   THREE.WebGLRenderer: Context Lost.
error: [DEBUG] action failed noteShellCommand {…} Error: [DEBUG] program puzzle: no actor for instance 1 (createApp not called, or already destroyed)
error: [DEBUG] shard 0 worker fault [handler/turn] actor=puzzle#1 module=/🔌️plugin-modules/🧩️puzzle/🌉️bridge.js?v=1789132471849: {"tag":"fault","val":{"0":123,"1":34,"2":99,…
error: [DEBUG] PluginRuntime: actor puzzle#1 trapped: shard 0 worker fault [handler/turn] actor=puzzle#1 …
warning: [DEBUG] hot-swap rolled back for puzzle AggregateError: plugin-handle.retirement-failed
error: [DEBUG] typed-operation completion subscription failed Error: plugin-handle.closed
error: [DEBUG] history snapshot failed Error: plugin-handle.closed
error: [DEBUG] render failed [unknown] no-code Error: plugin-handle.closed
error: [DEBUG] readConflicts failed [unknown] no-code origin=unknown Error: plugin-handle.closed
```

Decoding the byte map (`{"tag":"fault","val":{"0":123,"1":34,"2":99,…}}` → ASCII) confirms the code:

```
probe-2026-09-11T13-14-30.md  →  {"code":"plugin.reactor-close-      (truncated by the log's own line cap)
```

`probe-2026-09-11T15-13-01.md:264-299` is the same sandwich, and `probe-2026-09-11T13-51-51.md:1271-1317`
is the variant where the trap does not fire but the rollback still happens — there the OLD actor
(`puzzle#1`) and the NEWLY established one (`puzzle#2`) are **both** `actor-activation.revoked`
afterwards.

**So: the trigger is a plugin HOT-SWAP**, i.e. `reloadPlugin` — vite re-delivering
`🌉️bridge.js?v=…` whenever a peer deploys a wasm build into a live `:6013` page (or a dev pressing
the reload control in the plugins panel). During a fleet that deploys every ~20 minutes this looks
like "the actor dies a while after boot", which is how it was read.

### 1b. Two corrections B13's §5 needs

1. **"pre-existing in #45b and in the peer's #46 battery"** — it is in neither. Both are
   `FAULTS=0`, and `probe-2026-09-11T14-48-04.md`'s console **tail** (the last lines of the run, at
   t≈1089 s) shows a fully live guest still publishing:
   ```
   debug: [DEBUG] puzzle3d.brushPreview.lane utility= preview=0 vortices=2
   warning: [DEBUG] history snapshot refresh {"instanceId":1,"cursor":2,"upserts":2,"canUndo":false}
   warning: [DEBUG] settle puzzle#1 empty-required stop continuation=1 status=more-work drain=true
   ```
2. **"`notices:["Agent disconnected"]` from roughly the third step onward"** — that notice is the
   collaboration/presence chip, not an actor obituary. It sits next to `Remote: detached` /
   `No one else is here` in the very same body dump, it is present in the #44-pre battery from
   `[86.7s]`, and in #45b at `[349.6s]` the same run pastes 121 tree items through the guest:
   ```
   [349.6s] clipboard after: {"example":"Concrete Forest","treeItems":121,"notices":["Agent disconnected"], …}
   ```
   **"Any battery step after ≈t+40 s is measuring a dead guest" is not true** and any wave descoped
   on that basis should be re-opened.

---

## 2. The lifetime / close path, with file:line

The guest half of the ladder, from the host's close request down to the fault:

| step | file:line |
| --- | --- |
| host drives `Event::InstanceClose` | `⚛️reactor/🔄️turn/🦀️.rs:409` `owner.request_close(*request, runtime)` |
| admission reserves + activates the four participants and takes the lease | `⚛️reactor/🚪️lifetime/🦀️.rs:302-336` `advance_admission` → `lease.begin_close` |
| every turn asks the focused lifetime for its receipt | `⚛️reactor/🔄️turn/🦀️.rs:1207` → `NativeLifecycleRegistry::prepare_turn` (`🚪️lifetime/🦀️.rs:468`) |
| `prepare_turn` mints `Retired` once the witness is empty | `🚪️lifetime/🦀️.rs:164` `prepare_retired` |
| …then walks the 4-rung release ladder, **re-reading the witness on every rung** | `🚪️lifetime/🦀️.rs:196` `release_owner_step` |
| the witness itself | `🚪️lifetime/🦀️.rs:352` `NativeLifetimeOwner::terminal_is_empty` |
| its first clause, the app/worker lease | `🔌️plugin/🚪️lifetime/🦀️.rs:51` `PluginInstanceCloseLease::is_retired` |
| the fault constructor | `⚛️reactor/🦀️.rs:977` `reactor_close_fault` → `plugin.reactor-close-authority` |

`"native close terminal unavailable"` existed at exactly one site in the repo —
`⚛️reactor/🚪️lifetime/🦀️.rs:346` (pre-fix) — so the string identifies the rung unambiguously.

The host half:

| step | file:line |
| --- | --- |
| hot-swap entry | `🏛️ShellHost/🟦️.tsx:3403` `reloadPlugin` |
| destroys the live instances under the OLD handle, errors swallowed | `:3446-3453` `destroyApp(…).catch(() => {})` |
| **commit point** — store now names the new handle | `:3476-3478` `UPSERT_LOADED_PLUGIN` + status/supervisor |
| new session established on the new handle | `:3480` `establishPrimarySession(newHandle)` |
| predecessor retirement, which is what throws | `:3486` `await current.handle.dispose()` |
| "rollback" | `:3487-3492` (pre-fix) `console.warn("hot-swap rolled back")` + `await newHandle?.dispose()` |

---

## 3. Root cause

Three defects stacked, each of which alone is enough to kill the session.

### 3a. A terminal witness that can be RETRACTED by lock contention (the trap itself)

`PluginInstanceCloseLease::is_retired` (`🔌️plugin/🚪️lifetime/🦀️.rs`) answers `Ok(false)` for a
`TryLockError::WouldBlock` on either the close cell or the close **worker pump** — a real
`std::sync::Mutex` held by a real worker thread:

```rust
let pump = match state.pump.try_lock() {
    Ok(pump) => pump,
    Err(std::sync::TryLockError::WouldBlock) => return Ok(false),
```

`GuestLifecycleCell::release_owner_step` then re-read that witness on **each of the four release
rungs**, and treated a `false` as a hard error even though the phase was already `Retired`:

```rust
if !self.owner.…?.terminal_is_empty()? {
    return Err("terminal receipt still owns native descendants");
}
```

So: mint the receipt on an empty witness → the close worker takes its own pump lock for one step →
the next release rung reads "not empty" → `plugin.reactor-close-authority` → the turn fails → the
actor traps. "Momentarily contended" and "still live" were the same answer, and a receipt that had
already been minted could be un-minted.

### 3b. The fault was erased, which is why nobody could diagnose it

`terminal_is_empty` discarded a fully-formed `semio_framework::Fault` and replaced it with a
`&'static str` that names nothing:

```rust
if !self.lease.is_retired().map_err(|_| "native close terminal unavailable")? { … }
if !self.released[0] && !super::reactor_close_complete(self.key).map_err(|_| "reactor close terminal unavailable")? { … }
```

`is_retired` can fail four structurally different ways — a `RuntimeCloseStatus::Fault(cause)` carrying
the worker's own cause and elapsed-µs, a poisoned close cell, a poisoned pump, and
"reported terminal while retaining an owner" — and all four arrived at the browser as one opaque
sentence. The `GuestLifetimeOwner` trait's `&'static str` error type made the erasure mandatory.

### 3c. A "rollback" that runs past the point of no return (what actually kills the session)

`reloadPlugin`'s `catch` fires for `await current.handle.dispose()` — a failure to retire the
**predecessor**. By then the store already names the new handle and `establishPrimarySession` has
already run on it. The catch then did `await newHandle?.dispose()`, destroying the handle the shell
had just adopted. That is the exact origin of the `actor-activation.revoked` storm, the
`plugin-handle.closed` on every later call, and the `Plugin Recovery / This program crashed.` card.
Note the direction: 3a/3b make `dispose()` throw, 3c turns that into a dead app.

---

## 4. Fix

**F1 — fault provenance (`⚛️reactor/🚪️lifetime/🦀️.rs`).** `GuestLifetimeOwner::terminal_is_empty` and
`release_terminal` now carry `semio_framework::Fault`, as do `GuestLifecycleCell::prepare_retired`,
`release_owner_step` and `NativeLifecycleRegistry::prepare_turn`; the three literal-string sites wrap
with `reactor_close_fault` at the point they are raised. `self.lease.is_retired()?` and
`reactor_close_complete(self.key)?` now propagate verbatim. One call site outside the module changed —
`⚛️reactor/🔄️turn/🦀️.rs:1207` drops its now-redundant `.map_err(reactor_close_fault)`.

**F2 — idempotent retired lease (`🔌️plugin/🚪️lifetime/🦀️.rs`).** `PluginInstanceCloseLease` gains a
`terminal: Cell<bool>` latch. Emptiness is a fact about an allocation that has already been handed
over, so once observed it is never re-derived; a contended read after the receipt was minted can no
longer retract it. `release_owner_step` additionally treats a non-empty witness at `Retired` as a
**wait** (`Ok(())`), the same shape the existing law
`pending_close_authority_waits_without_consuming_structural_close_credit` already pins for the close
job — only a structurally impossible release is still a fault.

**F3 — the hot-swap never un-commits (`🏛️ShellHost/🟦️.tsx:3403-3502`).** `reloadPlugin` tracks a
`committed` flag set at the `UPSERT_LOADED_PLUGIN` dispatch. Past the commit point a failure is
reported and the committed handle is retained (supervisor stays `running`/`loaded`); the destructive
`newHandle?.dispose()` + `supervisor: "crashed"` arm now runs only for a pre-commit failure, which is
what "rollback" was always supposed to mean.

Files changed:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` (one line)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🚪️lifetime/🦀️.rs` (law)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs` (law)
- `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧫️fixtures/🔣️.json` (fixture case `contendedWitnessStaysTerminal`)

---

## 5. Laws — both proven RED at HEAD, GREEN after

### 5a. `a_retired_lifetime_waits_out_a_retracted_terminal_witness_instead_of_faulting`

`⚛️reactor/🚪️lifetime/🧪️tests/🚪️lifetime/🦀️.rs:144`. Drives a real `GuestLifecycleCell` to `Retired`,
retracts the witness mid-release ladder (the `WouldBlock` shape), and requires a wait.

**Before the fix** (F2's `release_owner_step` clause reverted to `Err(…)`, everything else in place —
this is the production fault, verbatim, with its production code):

```
running 1 test
test …::a_retired_lifetime_waits_out_a_retracted_terminal_witness_instead_of_faulting ...
thread '…' panicked at …/🧪️tests/🚪️lifetime/🦀️.rs:178:38:
a retracted witness is a wait, never a fault: Fault { origin: Framework,
  code: FaultCode("plugin.reactor-close-authority"), severity: Error,
  message: "terminal receipt still owns native descendants", … retryable: false }
…
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
error: test failed … (signal: 6, SIGABRT: process abort signal)
```

(The SIGABRT is `GuestLifecycleCell::drop`'s own assertion — a killed close can never release its
cell, which is exactly why the browser session is unrecoverable afterwards.)

**After:**

```
running 1 test
test component::reactor::instance_lifetime::tests::a_retired_lifetime_waits_out_a_retracted_terminal_witness_instead_of_faulting ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 658 filtered out; finished in 0.00s
```

### 5b. `instance_lifetime_close_witness_survives_quarantine_removal_and_reused_id` (extended)

`🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:286-292`. After the existing law drives a
REAL `PluginInstanceCloseLease` to terminal emptiness, the law now takes the close worker's own
`pump` mutex and asserts the witness holds — the production contention, not a simulation.

**Before the fix** (F2's latch removed):

```
thread '…::instance_lifetime_close_witness_survives_quarantine_removal_and_reused_id' panicked at
  …/📨️dispatch/🦀️.rs:290:9:
assertion `left == right` failed: a terminal witness must not be retracted by a contended close worker pump
  left: false
 right: true
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 658 filtered out; finished in 1.51s
```

**After:**

```
test component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_witness_survives_quarantine_removal_and_reused_id ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 658 filtered out; finished in 2.07s
```

---

## 6. Verification runs (all foreground)

| gate | result |
| --- | --- |
| `RUST_MIN_STACK=… cargo test -p semio-framework-plugin --lib -- --test-threads=1 a_retired_lifetime_waits_out_a_retracted instance_lifetime_close_witness runtime_close reactor_close guest_instance_lifecycle` | `running 15 tests` … `test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 644 filtered out; finished in 2.03s` — includes all six `runtime_close_budget_tests`, `reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps`, and every `guest_instance_lifecycle_*` |
| B2/B8 laws: `… -- --test-threads=1 a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot every_admitted_typed_operation_slot_is_released_by_the_host_continuation_that_reports_it_runnable a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford a_nakagin_scale_mixed_surface_turn_retires_every_ladder a_nakagin_scale_world_publication_reconciles_and_retires` | `running 5 tests` … `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 654 filtered out; finished in 2.71s` |
| `cargo check -p semio-framework-plugin --lib` | `warning: \`semio-framework-plugin\` (lib) generated 6 warnings` / `Finished \`dev\` profile … in 1.99s` — **0 errors, and 6 warnings is the pre-edit baseline** (my first cut added 3 `unnecessary qualification` warnings; fixed) |
| `RUST_MIN_STACK=… cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: … generated 88 warnings` / `Finished \`dev\` profile … in 0.49s` — **0 errors** |
| `RUST_MIN_STACK=… cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Checking semio-s-plugin-puzzle v0.1.0 …` / `Finished \`dev\` profile … in 24.51s` — **0 errors** |
| `bun nx run @semio-tech/framework-renderer-react:typecheck` | 4 errors reported in ShellHost-adjacent files, all pre-existing: `🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts(86,5)`, `ShellHost/🟦️.tsx(1905,52)`, `(7800,35)`, `(7801,116)` — B13 reported the same set at `1905\|7774\|7775` before peer line drift. **No error at 3403-3502.** |
| renderer-react vitest, `SEMIO_TEST_LEVEL=long bun x vitest run --config 🧰️framework/…/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts` | `Test Files 3 failed \| 21 passed (24)` / `Tests 9 failed \| 875 passed (884)` — the **same 9** B4, B10 and B13 all baselined (B13 read `9 failed \| 874 passed (883)`; the extra pass is a peer's new test). **No new failure.** |

The wasm component build was deliberately NOT run — the coordinator builds #47.

---

## 7. Browser evidence

The trap is quoted verbatim in §1 from `🗑️generated/probe-2026-09-11T13-14-30.md`,
`probe-2026-09-11T13-51-51.md` and `probe-2026-09-11T15-13-01.md`, including the `hot-swap` line
that precedes it, the `[DEBUG] typed-operation slots`-era faults around it, and the
`hot-swap rolled back … plugin-handle.retirement-failed` that follows. That is the full recipe and it
is deterministic: **drive `reloadPlugin`** (the plugins-panel reload control,
`📌️ChromePanels/🟦️.tsx:1280` `host.reloadPlugin(entry.pluginId)`, or any peer deploy into an open
page) on a live session.

`:6013` was wedged for most of the verification window (18 `curl` attempts over ~12 min, all
`%{http_code} 000`, vite `pid 6648 … --port 6013 --strictPort` alive but answering nothing;
`pgrep -f "browser-probe|lane-probe"` empty throughout, nothing killed). It recovered at the end of
the pass and **F3 was proven live**, host-live on a reload with no wasm rebuild.

### F3, measured live on `:6013` — AFTER

Booted the page with a console hook on `/hot-swap|revoked|trapped|plugin-handle|retirement-failed|reactor-close|createApp not called/`,
then let a real hot-swap land. The captured sequence:

```
log:   [DEBUG] hot-swap puzzle
error: [DEBUG] typed-operation completion subscription failed [DEBUG] program puzzle: no channel for instance 1 (createApp not called, or already destroyed)
error: [DEBUG] local interaction observation failed        [DEBUG] program puzzle: no channel for instance 1 (createApp not called, or already destroyed)
warn:  [DEBUG] hot-swap puzzle retained its committed handle; predecessor retirement failed plugin-handle.retirement-failed
```

and the state 15 s later:

```
{"recovery": false, "canvases": 1, "treeItems": 65, "dead": 0}
```

`dead: 0` counts `actor-activation.revoked` + `trapped` + `plugin-handle.closed` — **none**, where the
archived probes had a storm of all three from this exact line onward. The successor is not merely
mounted, it is dispatching:

```
[warn] [DEBUG] performInvocation settled {"invocationKind":"action","instanceId":2,"actionId":"setActiveExample","frames":2,…}
[warn] [DEBUG] command ingress settled status=command-complete observed=command-pending,command-complete
```

`instanceId: 2` is the post-hot-swap actor. The predecessor still refuses to retire
(`plugin-handle.retirement-failed` — that is the Rust half, F1/F2, which rides #47), but it is now a
reported leak instead of a killed session, which is exactly the contract F3 buys.

---

## 8. For the probe's `FAULT_RE` — read this before editing it

The probe is B15's; not edited here. **Adding `reactor-close-authority` alone will NOT catch this**:
the fault crosses the console as a **byte map**, never as text —
`{"tag":"fault","val":{"0":123,"1":34,"2":99,…}}` — which is why a scan of every artefact in
`🗑️generated/` finds the literal string `reactor-close-authority` **zero** times while three probes
clearly carry the fault. Extend `FAULT_RE` with the plain-text markers that DO appear, plus the
decoded form for completeness:

```
/actor(?:\s+\S+)?\s+trapped:|actor-activation\.revoked|plugin-handle\.(?:closed|retirement-failed)|hot-swap rolled back|plugin\.reactor-close-authority/
```

The minimal single fragment the coordinator asked for is `plugin\.reactor-close-authority`; the
load-bearing ones in practice are `actor-activation\.revoked` and `plugin-handle\.retirement-failed`.
A run that matches any of them is measuring a dead guest from that line onward — unlike
`"Agent disconnected"`, which means nothing of the sort (§1b).

---

## 9. Handover

- **Re-open anything descoped on B13 §5's "dead guest after t+40 s".** The batteries were healthy;
  `context-menu-opens FAIL rows=0`, the export primary route and the catalogue steps in #45b/#46 were
  **not** blocked by a dead actor and need a real owner.
- The typed-operation fault that co-occurs in `probe-2026-09-11T13-14-30.md`
  (`action failed setGridSpacing … fixed typed-operation and segmented-output authorities did not
  pre-admit the exact operation slot`) is B16's lane, not this one; it is a separate hard fault in
  the same run.
- `destroyApp(…).catch(() => {})` at `🏛️ShellHost/🟦️.tsx:3446-3453` still swallows the
  instance-destroy failures silently. Left as is (three call sites, and the close is legitimately
  best-effort there), but a hot-swap that cannot destroy its own instances should at least say so.
