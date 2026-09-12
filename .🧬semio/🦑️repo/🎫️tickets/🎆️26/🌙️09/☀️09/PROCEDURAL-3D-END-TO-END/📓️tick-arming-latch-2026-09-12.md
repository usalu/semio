# Tick Arming Latch — one pending `flowEvalTick` per (instance, preview window), and the 5 ms-per-tick registry re-parse

Lane `tick-arming-latch`, 2026-09-12, Opus execution agent. Repo MCP was down all session
(`-32602 invalid initialize params`); bookkeeping is on disk and no ticket was opened, closed or reopened.
Command outputs under `🗑️generated/tick-latch/`.

**Restage required: YES** (guest Rust changed: the flow framework, the shared chain, both generation3d
surfaces). Nothing in here is observable on `:6018` until the coordinator restages the procedural component.
No wasm build and no dev server was started from this lane.

Closes `📓️contributions-push-starvation-2026-09-12.md` §6 **item 2** ("`pending_effects` still polls a
scratch session") at the framework-signature level, and the spin it left behind.

---

## 1. What was wrong

The live 80 s probe (`🗑️generated/probe-restage-2b/console.txt`) showed **298 `flowEvalTick` invocations
against 111 settles**, each settle carrying `effects: 2` or `effects: 4`, with every extension answer
waiting ~16 s behind the queue. Three independent defects produced that, and the first-tick cost had a
fourth cause that is not a queueing artefact at all.

### 1.1 The poll could not see the chain it was duplicating

`Generation3dPlayApp::pending_effects` (and the viewer's) probed a **throwaway** `FlowEvalSession`
(`with_scratch_session` → `FlowEvalSession::new()`): its `tick_scheduled` latch is always clear, so
`session.sync(host)` always answered "pending" and the poll armed **one more tick per preview window per
host refresh**, on top of whatever chain was already running. Every settle drives a `refreshUi`, so the
duplicates compounded. The framework signature was the reason it could not do better —
`ArtifactApp::pending_effects(doc, cfg, view)` is handed no owner, so no app could reach its retained
session from there even in principle.

### 1.2 A tick that parked extension work ALSO re-armed itself

`preview_eval::evaluate_tick` computed its re-arm from `more` **before** it knew whether the tick had
parked an `ExtensionInvocation`:

```rust
let effects = if more && may_rearm(fixture) { vec![rearm(…)] } else { Vec::new() };   // ← before
…
if let Some(pending) = pending_extension_eval { extension_invocations.push(…) }        // ← after
```

An evaluate hop leaves the requesting node in `remaining`, so `more` is `true` — the tick emitted a re-arm
**and** an invocation whose `flowEvalResolve` re-armed again. That is the literal `effects: 2` the console
reported, and it doubles the chain on every extension hop. The second tick then recomputes the identical
pending request and parks a duplicate invocation.

### 1.3 A faulted answer re-armed forever

`resolve_eval` re-armed unconditionally. A faulted `invokeExtension` comes back with an empty `outputJson`,
`seed_node_cache` fails, the node cache keeps nothing — so the next tick parks the identical request, faults
again, and re-arms again, at the host's own cadence, for as long as the app is open. With the sibling lane's
extension currently faulting, this alone is an unbounded spin.

### 1.4 The first tick's own cost: a 5.2 ms JSON re-parse of the whole contribution table, per tick

`flow_extension_invocation_address` — the ONE flow-id → plugin-id translation, called by
`preview_eval::geometry_extension_address()` — went through `installed_flow_extensions()`, which
**`from_json_str`-parses every contributed manifest on every call**. Measured natively with `[DEBUG]`
timing (`🗑️generated/tick-latch/instrumented-2.txt`):

```
[DEBUG] installed-flow-extensions us=8709 count=2 manifest-bytes=106109     (cold)
[DEBUG] installed-flow-extensions us=5288 count=2 manifest-bytes=106109
[DEBUG] installed-flow-extensions us=5215 count=2 manifest-bytes=106109
```

**5.2 ms per call for 106 kB of manifests, in a native debug build** — ~49 ns/byte. It is on the tick's hot
path (the tessellate producer) **and** on the preview status render path (`preview_progress_status_json`),
so a settled app paid it several times per refresh. The served playground's closure is **248 635 chars**,
2.3× larger, in a wasm debug build. That is the measurable, non-queueing half of the browser's ~17 s first
tick; the rest is §1.1–§1.3's queue depth.

Per-tick breakdown, same run, before the fix:

```
[DEBUG] tick-cost us host-build=465 session-tick=1965 may-rearm=0 retire-cold=300 invocations=5525 publication=2 total=8257
[DEBUG] tick-cost us host-build=473 session-tick=1300 may-rearm=0 retire-cold=309 invocations=5588 publication=1 total=7599
```

`invocations` — the tessellate producer, i.e. `geometry_extension_address()` — was **67–73 % of the whole
tick**. `may_rearm`/`unserved_flow_operator_kinds` measured **3–27 µs** and is not a hot spot; the
catalogue (`flow_neuron_kind_info_map`, `flow_app_catalogue_json_shared`) was already cached per registry
generation and is not one either.

---

## 2. The latch design

### 2.1 Who owns it

The **retained `FlowEvalSession`** — the one object that already survives every turn, already knows what the
evaluation owes, and is already reachable from every hop of the chain. Per CLAUDE.md's taxonomy the latch is
**ephemeral local-only** state: it is not a document, not a config, not a transient publication, never
serialized, never replicated, never event-sourced. It is bookkeeping about work in flight *in this process*,
exactly like `tick_scheduled` and `pending_tessellate_by_hash` beside it. Nothing a peer or a reload needs to
agree with, so nothing to source events for.

### 2.2 The shape

`🧰️framework/…/🌊️flow/🖥️host/🦀️.rs`:

```rust
struct FlowEvalWindowTickLatch { armed: bool, in_flight: u32, owed: bool, unfinished: bool }
fn flow_eval_window_key(window_id: &str) -> u64            // hashed, never retained
window_tick_latches: BTreeMap<u64, FlowEvalWindowTickLatch>   // on FlowEvalSessionState
```

Per **WINDOW**, not per session: two preview windows on one instance each own their retained evaluation
publication, and generate-mode evaluates a *patched* fixture of its own, so one window's debt is never
another's. Keys are hashes of plain `Copy` rows, so the close ladder retires the whole table with one
`clear()` (the `tessellate_progress_by_hash` precedent) and `terminal_is_empty` asserts it.

### 2.3 The API, and who calls it

| method | meaning | called by |
|---|---|---|
| `arm_window_tick(w) -> bool` | **the one gate.** Refuses when a tick is already pending; when an extension answer is outstanding it marks `owed` and refuses, handing the arm to the last answer | every gesture: `setContributions`, `setActiveExample`, the generation commands, every viewer view command, and the chain's own continuations |
| `window_tick_owed(w) -> bool` | a window that never ticked owes its first; otherwise `unfinished && !armed && in_flight == 0` | the refresh poll |
| `arm_owed_window_tick(w)` | the two above, as one question | `pending_effects` |
| `begin_window_tick(w)` | the armed tick is now RUNNING; the latch is free for what this tick decides | `evaluate_tick`, and the editor's empty generate-tick early return |
| `note_window_tick_outcome(w, unfinished)` | the tick's own `more` | `evaluate_tick` |
| `note_window_extensions_in_flight(w, n)` | the tick parked `n` invocations; they own the continuation | `evaluate_tick` |
| `settle_window_extension(w) -> bool` | one answer landed; answers whether it discharged a sibling's `owed` arm | `resolve_eval`, `resolve_tessellate` |
| `abandon_window_tick(w)` | this chain gave up — a fault nothing in this process can clear | a fold that seeds nothing; a tick whose graph `may_rearm` refuses |
| `retain_window_tick_latches(&[ids])` | drop latches of windows that left the roster | the refresh poll |
| `clear_window_tick_latches()` | a registry replacement abandons every chain | `invalidate_for_flow_extension_registry`, `begin_close` |

**The law, stated once:** at most ONE pending `flowEvalTick` per `(app instance, preview window)` exists at
any time, whoever asked for it. A tick with an in-flight `ExtensionInvocation` never re-arms. A fan-out of
answers re-arms once — at the last one, not once each. A fault settles with no continuation. Only a gesture
resumes an abandoned chain.

### 2.4 The framework signature

`ArtifactApp::pending_effects` gains `owner: &ArtifactInstanceOperationOwnerHandle` as its first argument,
mirroring `render_with_request_context` exactly (`ArtifactApp`/`ArtifactEditor`/`ArtifactViewer`/`EditorApp`/
`ViewerApp` + the viewer-forwarder macro). `VcsArtifactApp::pending_effects` supplies
`self.instance_operation_owner` — the same handle the runtime already lends to render and to every retained
command job. The object-safe `PluginApp::pending_effects(&mut self, view)` is unchanged, so both host call
sites (`refreshUi` at `🔌️plugin/🦀️.rs:32571` and the post-mutation drain at `:34119`) are untouched.
No scratch-session probing remains anywhere: the viewer's `with_scratch_session` is deleted outright, and the
editor keeps its only for the marks-free `render`/`handle` fallbacks the framework still offers.

`ArtifactInstanceOperationOwnerHandle::new` is now `pub` — it wraps an owner the CALLER built and confers no
authority of its own (`with_mut` downcasts to the caller's own type), so an app's laws can build the handle
their retained work expects instead of reaching into a live instance.

### 2.5 The registry cache

`FlowExtensionRegistryState` gains `installed: Option<Arc<Vec<FlowExtensionInfo>>>`, built lazily under the
same lock that owns `contributed` and dropped by `FlowRegistryReplacement::publish` — the ONE thing that can
change the table (verified: every `contributed` mutation in the file is followed by `publish`).
`installed_flow_extensions_shared()` is the new hot-path reader; `installed_flow_extensions()` clones from it;
`flow_extension_invocation_address` reads the `Arc` and allocates only the returned plugin id.

---

## 3. Measured, before and after

Same instrumentation, same law, same warm binary
(`🗑️generated/tick-latch/instrumented-2.txt` → `instrumented-3.txt`):

| phase, per tick | before | after |
|---|---|---|
| `invocations` (tessellate producer → `flow_extension_invocation_address`) | **5 525 / 5 588 µs** | **176 / 222 µs** |
| whole tick, tessellate hop | **8 257 / 7 599 µs** | **2 606 / 2 339 µs** |
| `installed_flow_extensions` per call, 106 109 manifest bytes | **5.2 ms** | not rebuilt (one `Arc` clone) |
| `may_rearm` / `unserved_flow_operator_kinds` | 3–27 µs | unchanged (not a hot spot) |
| `host-build` (`fixture.clone()` + dag rebuild) | 433–778 µs | unchanged |

`a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted` + its viewer twin, wall
clock, warm binary: **0.86 s → 0.58 s**, both still `re-armed 5 ticks and painted meshes=3`.

The refresh poll additionally no longer builds a `FlowHost` or evaluates anything at all — it reads the
latch. That removes a `fixture.clone()` + dag rebuild + status-json build (~1.5–3 ms natively) from **every
host refresh**, per surface.

---

## 4. What an 80 s browser probe should show after a restage

`http://127.0.0.1:6018/?plugin=generation3d`, same probe as `🗑️generated/probe-restage-2b/`:

| line | before (measured) | after (expected) |
|---|---|---|
| `setContributions deferred effects` | 1, with **2** deferred `flowEvalTick` (one per attached preview window) | **unchanged: 1 line, exactly 2 re-arms** |
| `flowEvalTick` invocations, whole 80 s | **298** | **≤ 16** with a working extension; **≤ 6** with the extension still faulting |
| `flowEvalTick` settles | 111 | **equal to the invocations, 1:1** |
| `effects:` on a tick settle | `2` or `4` | **`0` or `1`, never 2 or 4** |
| ticks per host refresh once settled | 1 per preview window | **0** |
| first settled tick after the install | 25.5 s (17 s after install) | **within ~1–2 s of the install** |

Derivation of the two invocation bounds, from the native chain on the same example: a converging chain is
5 ticks and 6 extension round trips per preview window (1 math evaluate, 2 brep evaluate, 3 brep tessellate)
— with 2 attached preview windows that is 10 ticks, plus the install's 2, plus at most one first-tick per
window, ≈ 13–16. With the extension faulting (the sibling lane's open defect), each window reaches its first
evaluate hop, gets a faulted answer, abandons, and stops: 2–3 ticks per window, ≈ 4–6 total, and then
**nothing at all until the user acts**. Either way the command-ingress queue never fills, so an extension
answer is no longer 16 s behind a queue of ticks.

If a probe after the restage still shows the tick count climbing with wall time, the arming source is
something outside this lane's latch (a host-side re-dispatch loop), not the guest.

---

## 5. Tests

### 5.1 New language-agnostic fixture

`✏️s/…/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🔒️tick-latch.json` — a declared
state machine, at the SUBSET level beside `🪟️tick-addressing.json` (which says *which* window an armed tick
names; this says *whether* one may be armed at all). 13 rows, each a `surface` (`editor` | `viewer`), an
attached roster, a `sequence` of events (`refresh`, `gesture`, `tick{parks,more}`, `resolve{ok,more}`,
`invalidate`, `detach`) and the `armed` window list each event owes.

Rows: install → exactly one tick per attached window · a refresh after the install adds none · two/three
refreshes in a row leave one pending tick · a tick with an in-flight invocation arms nothing (neither from
the poll nor from a gesture) · a resolve with more work re-arms exactly once · a fan-out of three answers
re-arms once, not once each · a faulted answer arms nothing and leaves no standing debt · a settled
evaluation arms nothing on any later refresh · an unfinished tick re-arms itself once · two previews latch
independently · a detached window owes a first tick again when it returns · the viewer's install and its
resolve fan-out.

### 5.2 Rust laws — `⏱️flow-eval-tick/🧪️tests/🔬️unit/🦀️.rs`

- `every_arming_source_passes_through_one_latch_per_preview_window` — replays every fixture row against the
  REAL `FlowEvalSession` latch API, asserts the `armed` lists row by row, and asserts the fixture covers
  both surfaces and that the declared window kinds are the ids the surfaces actually register.
- `a_second_refresh_arms_no_second_tick_for_the_same_preview_window` — the REAL editor, through the REAL
  poll: the gesture arms 1, three refreshes on top arm 0, and after the chain converges the poll still arms 0.

### 5.3 Third-party twin — `…/🔬️unit/contract.ts`

`testGeneration3dTickLatchContract()` is an **independent TypeScript implementation** of the same state
machine (its own `TickLatchTable`), replaying the identical rows and asserting the identical `armed` lists,
plus the invariant itself (no window ever carries more than one pending tick across a row).

### 5.4 Everything run, verbatim

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly \
  --lib -- flow_eval_tick tick_addressing set_contributions --test-threads=1 --nocapture
→ 16 passed; 0 failed
  [STATS] latch: gesture=1 refreshes=[0, 0, 0] ticks=2 settled=0
  [STATS] tick-latch install-arms-exactly-one-tick-per-attached-window: surface=editor armed=[[], ["preview", "generate"]]
  [STATS] tick-latch a-tick-with-an-in-flight-invocation-arms-nothing: surface=editor armed=[["preview"], [], [], []]
  [STATS] tick-latch a-fan-out-of-answers-re-arms-once-not-once-each: surface=editor armed=[["preview"], [], [], [], ["preview"]]
  [STATS] tick-latch a-faulted-answer-arms-nothing-and-leaves-no-standing-debt: surface=editor armed=[["preview"], [], [], [], []]
  [STATS] tick-latch viewer-resolve-fan-out-re-arms-once: surface=viewer armed=[["view-preview"], [], [], ["view-preview"]]
  [STATS] setContributions resumed the chain with 1 re-arm(s) across 31 page(s)

cargo test … --lib -- viewer:: preview_eval:: --test-threads=1        → 52 passed; 0 failed
cargo test … --lib -- example_switch work_capacity --test-threads=1   → 10 passed; 0 failed
cargo test … --lib -- a_late_contributions_install --test-threads=1 --nocapture
  → 2 passed; 0 failed — [STATS] late install re-armed 5 ticks and painted meshes=3
                         [STATS] viewer late install re-armed 5 ticks and painted meshes=3

bun …/⏱️flow-eval-tick/🧪️tests/🔬️unit/contract.ts
  → generation3d contribution-gated arming blocked=0 served=1 resume=setContributions budget=512
    generation3d tick-latch rows=13 surfaces=editor+viewer arms=21
bunx tsc --noEmit --strict … 🔬️unit/contract.ts                       → exit 0
```

### 5.5 Regression — the whole gen3d lib

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- --test-threads=1
→ 367 passed; 5 failed        (push lane's baseline: 358 passed; 6 failed)
```

**+9 passing, −1 failing.** `refresh_pending_effects_arms_flow_eval_tick_chain` — one of the baseline 6 —
now PASSES: it was timing out on the 30 s retained-operation deadline, and the latch removed the duplicate
chains that pushed it past it. The remaining 5 are all attributed elsewhere, unchanged:

| failing test | attribution |
|---|---|
| `add_generation_records_an_undoable_generation_operation` | peer undo lane (`🔌️plugin/🦀️.rs:6866`, "undo did not revert to the expected snapshot") |
| `undo_redo_round_trips_flow_graph_edits` | same |
| `two_instances_converge_disjoint_widget_moves` | declared fail-closed `module.vcs` remote-merge class |
| `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | same |
| `generation_preview_is_one_app_transient_shared_by_two_generation_windows` | 30 s retained-operation deadline vs a 29.8 s debug boolean cut (`📓️viewer-eval-chain-2026-09-12.md` §7 item 2) |

One additional test, `modes::edit::windows::preview::tests::switching_active_example_changes_preview_meshes`,
failed in a single full-suite run with `exceeded its exact maintenance grant` and **passes in isolation**
(`🗑️generated/tick-latch/run-switching.txt`) — same wall-clock class, not reproducible.

### 5.6 Other crates

```
cargo test -p semio-framework-os-flow --lib -- --test-threads=1
→ 154 passed; 57 failed   (push lane's baseline: 153 / 57 — same 57, one more passing test in the crate)
cargo test -p semio-framework-os-flow --lib -- registry:: --test-threads=1
→ 7 passed; 0 failed      (all seven registry laws, including the replacement/admission ones, green in isolation)

cargo test -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib
→ 228 passed; 3 failed
  two_instances_converge…, vcs_artifact_app_non_empty_retained… = the declared module.vcs pair
  add_widget_undo_redo_round_trip = the peer undo lane's red, same family as gen3d's two

cargo check --keep-going -p semio-s-plugin-{procedural,flow,fem,energy} -p semio-framework-plugin  → 0 errors
CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-plugin-procedural \
  --target wasm32-wasip2 --profile wasm-dev --keep-going                → 0 errors, Finished in 1m 45s
```

**Not this lane, and not gated by it** (recorded so the next lane does not re-triage them):

- `cargo test -p semio-s-artifact-flow-flow --lib` → **130 passed / 89 failed**, every one asserting
  publication LANES (`left: [Child, Ui, Terminal]`, `right: [Child, Terminal]`). A peer modified
  `…/🌊️flow/…/✏️editor/🦀️.rs` at **02:59 today**, mid-session. This lane changed exactly one line in that
  file (the `pending_effects` signature) and emits no `Ui` lane.
- `cargo test -p semio-framework-plugin --lib -- --test-threads=1` → **522 passed / 145 failed**, headed by
  `app-definition.interactive-job-classification: unclassified interactive command 'main:resize'` across
  `app_builder_tests`, `describe`, `schema_stamping`, `reactor` — a peer's in-flight interactive-job
  classification change. `retained_command::` (6/6) and
  `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` (1/1) pass in isolation; no
  `pending_effects` law is in the failure set.

### 5.7 Warnings

`cargo check -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib` reports
**zero** warnings in any file this lane touched (the viewer's now-dead `with_scratch_session` was deleted
rather than left behind).

---

## 6. Files

**Changed — framework**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `FlowEvalWindowTickLatch`,
  `flow_eval_window_key`, `FlowEvalSessionState::window_tick_latches`, the ten-method latch API
  (§2.3), the close ladder + `terminal_is_empty` rows, `invalidate_for_flow_extension_registry` and
  `begin_close` clearing the table
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` — `FlowExtensionRegistryState::installed`,
  `installed_flow_extensions_shared`, `installed_flow_extensions` off the shared `Arc`,
  `flow_extension_invocation_address` allocation-free on the hit path,
  `FlowRegistryReplacement::publish` dropping the memo
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `pending_effects` gains
  `owner: &ArtifactInstanceOperationOwnerHandle` across `ArtifactApp`/`ArtifactEditor`/`ArtifactViewer`/
  `EditorApp`/`ViewerApp` and the viewer-forwarder macro; `VcsArtifactApp::pending_effects` supplies the
  retained handle; `ArtifactInstanceOperationOwnerHandle::new` is `pub`

**Changed — generation3d**
- `…/✳️any/🧵️preview-eval/🦀️.rs` — `evaluate_tick` begins/records/parks through the latch and re-arms only
  when it parked nothing; `resolve_eval` abandons an unfoldable answer and arms once; `resolve_tessellate`
  arms once per fan-out; `rearm_attached_previews(session, windows)`
- `…/✳️any/✏️editor/🦀️.rs` — `pending_effects(owner, …)` on the RETAINED session with roster pruning;
  `Generation3dPreviewCommandWork` carries the owner handle and re-arms through it
- `…/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs` — the empty generate-mode tick discharges its latch
- `…/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs`, `…/🎨️set-active-example/🦀️.rs`
- `…/✳️any/👁️viewer/🦀️.rs` — `pending_effects(owner, …)`, `Generation3dViewCommandWork` owner handle,
  `with_scratch_session` deleted
- `…/✳️any/👁️viewer/🎮️commands/🧩️set-contributions/🦀️.rs`

**Changed — peer surfaces (signature only)**
- `✏️s/🔌️plugins/🌊️flow/…/✏️editor/🦀️.rs`, `🌀️procedural/…/🌀️generation2d/…/✏️editor/🦀️.rs`,
  `🏗️fem/…/🧊️3d/…/{✏️editor,👁️viewer}/🦀️.rs`, `🏗️fem/…/◻️2d/…/✏️editor/🦀️.rs`,
  `🔋️energy/…/✏️editor/🦀️.rs`

**New**
- `…/✳️any/🧫️fixtures/🔒️tick-latch.json`

**Changed — tests**
- `…/✏️editor/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/{🦀️.rs,contract.ts}` — the two laws and the twin
- `…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` — `instance_operation_owner`, `retire_instance_operation_owner`
- `…/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs` — one owner handle, retired at the end
- `…/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs` — asserts the switch's OWN arm and that the poll adds none
- `…/✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs` — the two chain laws drain from the gesture's receipt
  effects, the way `ShellHost` feeds `requestedEffects` back
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the first-turn law polls through the app instance

---

## 7. Open

1. **The latch assumes the armed effect is delivered.** One pending tick is recorded the moment it is armed
   and discharged only when the tick RUNS, so a host that drops a `requestedEffects` / receipt effect leaves
   that window without a chain until the next gesture. In-process this is the guest's own return value and
   `ShellHost` dispatches both queues, so it is not a transport risk today — but it is the one assumption the
   design makes, and it is what made three existing laws need updating (they polled instead of replaying the
   receipt). A cheap non-heuristic recovery does not exist without a refresh serial on the wire; the current
   levers are a roster change (the poll prunes the latch) and any gesture after an abandon.
2. **`generation2d` still polls a scratch session.** Its `pending_effects` now takes the owner handle but
   ignores it — its `flowEvalTick` is an app-wide `HostOnly` route with no window address, so the per-window
   latch does not apply as-is. It would spin the same way against a slow or faulting extension.
3. **The 248 kB closure is still re-parsed once per registry generation.** That is correct and unavoidable,
   but the first tick after an install pays the whole per-generation cache warm-up (address table, kind-info
   map, catalogue JSON) in one turn. If the restaged probe still shows a slow first tick, that warm-up —
   not the arming — is where to look next.
4. **`resolve_eval` still infers "faulted" from "the fold seeded nothing."** The typed `ok`/`faultCode` the
   host echoes onto the response action is on the sibling lane's extension-completion wire; when that lane
   lands, `FlowEvalResolve` should carry it and `abandon_window_tick` should key off the fault code instead.
5. **Runtime (browser) proof is missing**, as for every guest change in this ticket: the restage is the
   coordinator's. §4 states the exact numbers to check against.
