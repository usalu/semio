# 🎨️ Example switch — the picker changes the label and nothing else

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, lane: `setActiveExample` / example-switch publication.
2026-09-10. Session ⚪9f5f6952 (Fable 5.1). Repo MCP down (`invalid initialize params`); ticket
bookkeeping is on disk and no ticket was opened/closed/reopened.

Answers `📓️runtime-verification-2026-09-09.md` boot #11 (10:30): with both windows up and no faults,
picking **Box Shell Preview** changed the picker label and left the flow window on the
hexagonal-mushroom-column graph (`height,radius,sides,profile,extrusion-axis,extrude,column-preview`),
with no `action failed` / `typed-operation failed` line.

---

## 0. TL;DR

**The guest does not stall.** Driven through the real reactor/retained path — boot →
`setActiveExample box-shell-preview` → the host's bounded publication/ACK — the switch publishes
`lanes=[Artifact, Config, Transient, Ui, Terminal]`, mints exactly one completion carrying
`UiDirtyScope::Full`, and the flow window's rendered `NodeGraphScene` carries the new example's node
ids, for all eight bundled examples with no leaked node from the previous one. Four new laws pin that
(§1).

**What WAS wrong, and is fixed:** the gesture published `effects=0`
(`✏️editor/🦀️.rs:400` → `🎨️set-active-example/🦀️.rs:37`). It replaced the whole fixture and armed
nothing, so BOTH windows' recovery hung entirely off the host calling
`Generation3dPlayApp::pending_effects` (`✏️editor/🦀️.rs:1655`) through a later `refresh-ui` round
trip — a chain that fails silently, with no fault anywhere, which is exactly the boot-#11 record for
`setActiveExample` AND for `addGeneration`. The switch now arms every attached preview window's
`flowEvalTick` from its own emit (§3.1), so the restart is a consequence of the switch, not of a host
refresh.

**restage required: yes.**

---

## 1. Native reproduction

New law module `✏️editor/🧪️tests/🔬️example-switch/🦀️.rs`, registered at `✏️editor/🦀️.rs`.
It boots the app through `new_app_with_registry`, renders the FLOW WINDOW surface
(`procedural.play.main`) under the shell's own two-window roster, dispatches
`setActiveExample box-shell-preview` through `PluginApp::handle_action` → the retained ladder →
the host's bounded publication/ACK, and reads the node ids back off the RENDERED surface (never off
the document — the live defect is a surface that keeps the previous fixture).


### 1.1 The four laws

`✏️editor/🧪️tests/🔬️example-switch/🦀️.rs`, registered at `✏️editor/🦀️.rs` (`//#region 🧪️ExampleSwitch`):

| law | what it drives |
|---|---|
| `set_active_example_republishes_the_flow_window_graph` | boot → render flow body → `setActiveExample box-shell-preview` → settle → render flow body again |
| `every_example_switch_publishes_its_own_graph_without_leaking_the_previous_one` | all **8** picker entries in order, in ONE session; each hop asserts the published node ids ARE that example's authored widget ids and that no node of the previous example survived (retirement law) |
| `a_shell_dispatched_example_switch_republishes_both_windows_and_rearms_the_eval_chain` | the SERVED shape: the flow window current, both windows in the roster, `ActionMeta.view_state` attached, the boot evaluation already drained to a real preview, then the switch — flow graph, armed tick, and the preview window's own retained evaluation |
| `set_active_example_arms_every_attached_preview_windows_evaluation_chain` | the gesture's own published effects |

### 1.2 What the guest actually does — measured, `--nocapture`

```
[DEBUG] example switch receipt: lanes=[Artifact, Config, Transient, Ui, Terminal] ui_scope=Some(Full) completions=1 effects=0
[DEBUG] flow window node ids after the switch: {"box", "shell", "size", "thickness"}
[DEBUG] example switch hexagonal-mushroom-column -> rectangle-extrude-volume: published 7 nodes ui_scope=Some(Full)
… all eight hops, each publishing its own graph …
[DEBUG] boot preview eval text bytes=Some(1060)
[DEBUG] shell switch: lanes=[Artifact, Config, Transient, Ui, Terminal] ui_scope=Some(Full) own effects=0 armed after switch=1
[DEBUG] switched preview eval text bytes=Some(726)
```

**The guest is not the stall.** Through the real retained ladder the switch publishes all five lanes,
mints exactly ONE `TypedOperationCompletion` carrying `UiDirtyScope::Full`
(`🔌️plugin/🦀️.rs:23101`), replaces the document, and the flow window's rendered
`NodeGraphScene` carries the new example's node ids — for every one of the eight bundled examples,
with no leaked node from the example before it. None of the candidate guest-side stalls named in the
brief is real here: no store-replacement job is inserted at all, `latest_wins` drops nothing, and the
publication is not merely "accepted" — the surface re-render off the published document is green.

---

## 2. The stall point

### 2.1 What the switch owed itself and did not pay — `own effects=0`

`Generation3dPreviewCommandWork::step`'s `SetActiveExample` arm
(`✏️editor/🦀️.rs:400`) built its emit from `set_active_example::emit`
(`✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs:37`) and published it with **zero effects**. The
gesture replaces the whole fixture, removes every generation and resets the config — and then arms
nothing.

The only place a generation3d evaluation chain is ever armed is
`Generation3dPlayApp::pending_effects` (`✏️editor/🦀️.rs:1655`), and the shell reaches that ONLY as
`response.requestedEffects` of a `refresh-ui` round trip
(`🏛️ShellHost/🟦️.tsx:4235`). So after a switch the whole recovery of both windows hung off one
host-side chain:

```
completion frame (ui_scope=Full)
  → ShellHost's subscribeOperationCompletions handler   (🏛️ShellHost/🟦️.tsx:5040-5054)
  → applyHostEffects(..., resolveUiDirtyScope(completion.uiScope), owner)
  → refreshUi(nextSession, uiScope)                     (🏛️ShellHost/🟦️.tsx:4132)
  → buildUiRefreshRequest(scope, …, cache)              (🛠️ShellHelpers/🟦️.tsx:4139) — hash-conditional
  → response.requestedEffects  →  pending_effects  →  flowEvalTick re-armed
```

Every link in that chain is host-side and silent when it does not run: a narrowed scope, a session
that is no longer current (`applyHostEffects`'s own `shellDialogSessionIsCurrentV1` guard,
`🏛️ShellHost/🟦️.tsx:5023`), a dropped completion — each leaves the picker label changed, both
windows on the previous example, and **no fault anywhere**, which is exactly boot #11's record
(and the identical record for `addGeneration` on the same boot).

That is the defect this lane owns and fixes: **the switch's own consequences must not be a
consequence of a host refresh.**

### 2.2 The shell side, read

`🏛️ShellHost/🟦️.tsx` — the picker (`NavbarExampleSelect`, `:7955`) writes the LABEL through the
shell reducer (`dispatch({ type: "SET_ACTIVE_EXAMPLE_ID" })`, `🖥️shell/🟦️.ts:305`) and reaches the
program through `dispatchActiveExample` → `onAction` (`:7940-7947`). It dispatches the right id and
the right action, `setActiveExample` IS a window-kind-declared action so `onAction`'s
`declaredAction` gate (`:5691`) admits it, and the completion handler applies
`resolveUiDirtyScope(completion.uiScope)` — the scope the guest publishes as `Full`. The dispatch
was one inline object literal with the action id spelled at the call site and its promise dropped;
it is now `buildActiveExampleAction` (`🛠️ShellHelpers/🟦️.tsx`), `void`-ed explicitly, so the
dispatched id is a testable value rather than a literal buried in a component.

---

## 3. The fix

### 3.1 Rust — the switch arms its own evaluation restart

`✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs` — new `rearm_attached_previews(window_ids)`,
which turns every attached preview window into one `flowEvalTick` `Effect::DispatchAction`, addressed
to that window (the same `flow_eval_tick::rearm` the chain uses for every other hop, so the window
address, the wire decode and the retained preflight are all the ordinary ones).

`✏️editor/🦀️.rs:400` — the `SetActiveExample` arm reads the attached window roster off the retained
job's own context (`ArtifactOwnedToolJobContext::view_state`, `🔌️plugin/🦀️.rs:13344`) through the
existing `generation3d_preview_window_ids`, and extends the emit's effects with the re-arm.

Nothing else moves: effects are not a store lane, so the route's
`ArtifactToolPublicationContract` (`Artifact`, `Config`, `Transient`) is untouched, and its declared
work capacity is untouched — measured, the gesture publishes exactly one effect for one attached
preview window and zero when no preview window is attached.

### 3.2 TypeScript — the picker's dispatch is a value, not a literal

`🛠️ShellHelpers/🟦️.tsx` — new `SET_ACTIVE_EXAMPLE_ACTION_ID` + `buildActiveExampleAction(controllerId, exampleId)`
(same shape as the neighbouring `buildNoteShellCommandAction`).
`🏛️ShellHost/🟦️.tsx:7942` — `dispatchActiveExample` now builds through it and `void`s the returned
promise explicitly.

---

## 4. Tests

| test | file | level |
|---|---|---|
| 4 example-switch laws (§1.1) | `✏️editor/🧪️tests/🔬️example-switch/🦀️.rs` | native Rust |
| picker dispatch carries the example id, and the completion's UI scope resolves to a full repaint | `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — "the example picker dispatches the chosen example id and its completion refreshes the whole shell" | vitest |

The eight-example sequence law is the retirement half: each hop asserts the published node id set IS
the target example's authored widget set, and that no widget of the PREVIOUS example survived into it.

---

## 5. Commands and tails

Private `CARGO_TARGET_DIR=…/scratchpad/target-example` (seeded from `target/debug`), `RUSTC_WRAPPER=""`,
`RUST_MIN_STACK=134217728`. Raw logs under `🗑️generated/example-*.txt`.

```
$ cargo test -p semio-s-artifact-procedural-generation3d --lib --features component-app-assembly \
      --no-fail-fast example_switch -- --nocapture --test-threads=1
test editor::generation3d::component::example_switch::a_shell_dispatched_example_switch_republishes_both_windows_and_rearms_the_eval_chain ... ok
test editor::generation3d::component::example_switch::every_example_switch_publishes_its_own_graph_without_leaking_the_previous_one ... ok
test editor::generation3d::component::example_switch::set_active_example_arms_every_attached_preview_windows_evaluation_chain ... ok
test editor::generation3d::component::example_switch::set_active_example_republishes_the_flow_window_graph ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 333 filtered out; finished in 6.65s

[DEBUG] setActiveExample armed ticks for ["procedural-preview"] out of 1 published effects
```

```
$ SEMIO_TEST_LEVEL=long bun ./📜️script.ts test ../../../../🧪️tests/🔬️engine-contract/🟦️.ts \
      --reporter=verbose --testNamePattern="example picker dispatches the chosen example id"
 Test Files  1 passed (1)
      Tests  1 passed | 495 skipped (496)
```

(The nx wrapper for that vitest target hung in project-graph creation for 25+ minutes without
starting vitest — see `📓️dev-boot-repo-walks-and-taxonomy-churn.md`; the target's own
`📜️script.ts test` entry point, which is exactly what the nx target invokes, was used instead.)

```
$ cargo check -p semio-s-plugin-procedural --keep-going
    Checking semio-s-plugin-procedural v0.1.0 (…/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 5m 06s          # 0 errors
```

---

## 6. Restage

**restage required: yes** — `✏️editor/🦀️.rs` and `✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs`
changed, so the served `generation3d` guest must be rebuilt and restaged before a browser boot can
observe the switch arming its own evaluation chain. The poll-leak lane owns the next restage on the
shared target; this lane ran only in its own private `CARGO_TARGET_DIR` and never touched
`target/` or the 6018/6118 serves.

## 7. What a follow-up boot must check

The guest half is now pinned by laws and cannot be the cause of a repeat. If boot #12 still shows
"picker label changes, windows unchanged", the remaining chain is entirely host-side and is worth
instrumenting in this order — all of it in the browser console, none of it observable from a native
suite:

1. `[DEBUG] completion apply` (`🏛️ShellHost/🟦️.tsx:5051`) — did the completion arrive at all, and
   what `scope` did it carry? No line means the completion never reached the shell (the
   `OperationCompleted` frame, `AppChannelClient.publishOperationCompletion`, or the typed-operation
   drain that produces the continuation turns).
2. `[DEBUG] applyHostEffects refresh` vs `[DEBUG] applyHostEffects skipped refresh: session not current`
   (`🏛️ShellHost/🟦️.tsx:4196-4199`) — the completion arrived but its session was stale.
3. `[DEBUG] typed-operation drain for instance N exhausted its 4096-poll budget`
   (`🔌️PluginRuntime/🟦️.tsx:2023`) — the operation is running and never terminating.
4. `[DEBUG] skipping undeclared action` / `[DEBUG] action failed` (`🏛️ShellHost/🟦️.tsx:5693`, `:5771`)
   — the dispatch never left the shell.

Both `setActiveExample` and `addGeneration` failed identically on boot #11, and they are the same two
rows of `GENERATION3D_PREVIEW_TOOL_IDS` — i.e. whatever remains is a property of the retained typed
operation's HOST delivery, not of either command.

```
$ cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev --keep-going
      (CARGO_PROFILE_WASM_DEV_DEBUG=false)
    Checking semio-s-plugin-procedural v0.1.0 (…/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust)
    Finished `wasm-dev` profile [unoptimized] target(s) in 7m 01s     # 0 errors
```

### 5.1 Concurrent-lane notes (not this lane's changes)

The `generation3d` lib test target is shared, and three peer edits crossed this lane mid-run:

- `✏️editor/🧪️tests/🔬️unit/🦀️.rs` gained a SECOND `preview_mesh_count` (a new async
  `(app, view)` form next to the existing `(&str)` one) — a hard `E0428` for the whole target.
  Resolved by renaming the new one `rendered_preview_mesh_count` at its two call sites; both forms
  are still used and neither lost work.
- the same file's `snapshot.generation = crate::law_generation_state()` stopped type-checking against
  a peer's new `GenerationPlayRoot`; the compiler's own `.into()` suggestion was applied.
- `👁️viewer/🧪️tests/🔬️unit/🦀️.rs:143` currently calls `app.window_measures(…)` as a method on
  `Generation3dViewerFixture` (`E0599` + two `E0277`s) — a viewer-lane refactor still in flight at
  11:16, untouched here. It blocks the WHOLE-suite regression run
  (`🗑️generated/example-5-suite-blocked-by-viewer-lane.txt`); the four laws of this lane and every
  other editor law compiled and ran green at 10:58 on the same target, before that edit landed.
