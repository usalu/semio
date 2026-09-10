# Wave W-Z — `[DEBUG]` close-out sweep

Working tree swept 2026-09-10 against `HEAD=6ad7b0e7bce8bf0c00089f162d5e9da6224741b8`. The list was
re-derived live (`git diff HEAD -- <path> | grep -a -n '^+.*\[DEBUG\]'`), NOT read off
`📓️2026-09-10-debug-trace-inventory.md` — that audit was taken at 00:00 against an older HEAD and is
now stale in both directions: the repo auto-committed most of the traces it listed (they read as HEAD
context today, not as `+` lines), waves W-G…W-H3 removed others, and peer ticket
`26/09/09/PROCEDURAL-3D-END-TO-END` added a whole new diagnostics facility on top of some of them.

## 0. Attribution rule actually applied

Three signals decided every hunk, in this order:

1. **Peer-owned** — the adjacent docstring/comment names another ticket slug, or the trace text is
   quoted in another ticket's notes (`grep -a -rl '<trace text>' .🧬semio/🦑️repo/🎫️tickets/`). Untouched.
2. **At HEAD** — `git show HEAD:<file> | grep -a '<trace text>'` finds it verbatim. Untouched, even
   when the working tree shows the line as `+` because a peer only wrapped it in a gate.
3. Otherwise **this ticket's** — removed, with any helper that existed only to feed it.

Two traces the task named for removal fell into class 2/1 and are documented in §3 instead of §2.

## 1. Removed (14 items, 11 traces + 3 dead helpers)

| File | Line (pre-edit) | Trace / helper | Action | Justification |
|---|---:|---|---|---|
|`🧰️…/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`|4679-4681|`console.warn("[DEBUG] actor dispatch", JSON.stringify(invocation), …result.slice(0,800))` + its `undo`/`redo` guard|removed 3 lines|Coordinator undo-funnel probe, named a close-out sweep target in `📓️2026-09-10-cursor-coordination.md` 11:35. Ungated, stringified the whole invocation plus 800 B of result on every browser-actor undo/redo.|
|same|5669|`console.warn("[DEBUG] undo route", …)`|removed|Same probe family; `📓️2026-09-10-cursor-coordination.md` 08:15 says it "lives in ShellHost until undo is browser-green". Routing is covered by `shellHistoryUndoRouteV1`'s own laws.|
|same|5679|`… console.warn("[DEBUG] undo funnel", …)`|removed|Same family (11:35 entry: "5 more `[DEBUG] undo funnel*` traces").|
|same|5682-5685|`… console.warn("[DEBUG] undo funnel dropped: stale effect owner")`|removed; guard collapsed back to `if (!isCurrentEffectOwner(actionOwner)) return;`|Same family; the block existed only to host the trace.|
|same|5705-5708|`… console.warn("[DEBUG] undo funnel dropped: no dispatch view state")`|removed; guard collapsed back to `if (!dispatchViewState) return;`|Same family; same reason.|
|same|5738|`… console.warn("[DEBUG] undo funnel branch", …)`|removed|Same family.|
|same|5757|`… console.warn("[DEBUG] undo funnel handleAction response", …)`|removed|Same family; sat inside the `handleAction` `.then` on the hot action path.|
|`🧰️…/🔌️plugin/🦀️.rs`|~21935|`eprintln!("[DEBUG] group history action={action} group={group_id} undone={} skipped={:?}", …)`|removed|Guest-side half of the same undo diagnosis (`📓️2026-09-10-fill-build-host-tick.md`). Allocated a `Vec<String>` of skipped members on every group undo/redo.|
|same|~22019|`eprintln!("[DEBUG] history route action={action} tail_group_id={group_id:?}")`|removed|Same; fired on every `undo`/`redo` commit.|
|same|~22043|`eprintln!("[DEBUG] history route action={action} benign-collapse (…)")`|removed; the `NothingToUndo \| NothingToRedo \| ForeignEdit` arm restored to its one-line `Ok(Self::empty_result(…))`|Same; the block form existed only to host the trace. `benign_undo_with_nothing_to_undo_stays_unlogged_with_scope_none` already asserts this branch.|
|`🧰️…/🧱️elements/🔌️PluginRuntime/🟦️.tsx`|~404|`console.warn(\`[DEBUG] spawn-job routed kind=… job=…\`)`|removed|W-F5/W-G3 spawn-job conversion probe (`📓️2026-09-10-wave-F5-fill-budget-cancel.md`). Sat inside `routeHostEffects`, i.e. once per spawned job — every fill/suggestions tick.|
|`🧰️…/🔌️plugin/🕹️interaction/📃️query/🦀️.rs`|109-112|`LocalInteractionQuery::debug_state`|removed|Docstring: "`[DEBUG]` state summary — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END". Its only reader was the `typed-operation publication` `eprintln!` this tree already deleted, so it was dead (`method … is never used`).|
|`🧰️…/🔌️plugin/🕹️interaction/📡️live/🦀️.rs`|214-227|`LiveLocalInteraction::debug_state`|removed|Same docstring, same orphaned reader; it also was the only caller of the query one above.|
|`🧰️…/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`|57-58, 368|`TURN_TRACE` thread-local + `let _turn_seq = TURN_TRACE.with(…)`|removed|Docstring: "turn sequence for the phase trace — temporary, ticket 26/09/02". The trace it counted for is gone; the binding is `_`-prefixed and never read, so it was a per-turn `Cell` bump for nothing.|

## 2. Kept — this ticket's, deliberately (rule b)

One-shot `[DEBUG]` summaries at the END of a passing law, restating exactly the values the
surrounding assertions already pin. Rule (b) admits these; none is inside a loop, and none survives a
law it does not belong to. **13 lines:**

| File | Lines | Why it stays |
|---|---:|---|
|`✏️s/…/🧩️puzzle/…/🧊️3d/…/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs`|91, 123, 180|Each is the last statement of one `#[async_test]`, reporting `census.nodes` / instance+mesh counts / fit revisions that the `assert!`s directly above already bound.|
|`✏️s/…/🧩️puzzle/…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`|146|Reports the node count the same law asserts against `PANEL_RECONCILE_NODE_BUDGET`.|
|`🧰️…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`|5 lines (carrier packing, lane paging, Nakagin-scale nodes, 100 KiB measures)|Each follows the `assert!` on the very number it prints; the packing figures are the wave's own evidence and are cheap (one line per test).|
|`🧰️…/🔌️plugin/⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs`|1|Prints the four turn/unit figures the W-S2 law asserts bounds on.|
|`🧰️…/🧱️elements/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx`|3|One `console.info` per `it()`, restating the lane counts/bytes the `expect`s pin.|

Hot-loop probes were already gone before this sweep: `probe_sp`, `probe_future_sizes`,
`probe_stack_sizes`, `probe_fill_build_*` and the `[DEBUG] tick=` / `FINAL retained=` loops named in
the task have **zero** matches anywhere under `✏️s/🔌️plugins/🧩️puzzle/` today, and no `[DEBUG]` line
survives in puzzle 3d production code at all (`grep -a -rn '\[DEBUG\]' --include='*.rs'
'✏️s/🔌️plugins/🧩️puzzle/' | grep -av 🧪️tests` → empty), including the W-U3
`🎮️commands/🪣️fill-build-tick/🦀️.rs` `eprintln!` the 07:30 inventory refresh flagged as "strip LAST".

## 3. Kept — NOT this ticket's to remove

**28 in-scope `+` lines.** Each was checked against HEAD and against every peer ticket's notes.

| Where | Lines | Why untouched |
|---|---:|---|
|`🏛️ShellHost/🟦️.tsx` — `extension invocation faulted`, `invokeExtension unresolved`, `contributions push`|3|Inside hunks whose own docstrings name `ticket 26/09/09/PROCEDURAL-3D-END-TO-END` (the `extension.missing` address fix and the four-gate contributions push).|
|`🏛️ShellHost/🟦️.tsx` — `applyHostEffects refresh`, `applyHostEffects skipped refresh`, `completion apply`|3|**The task named these for removal, and they stay.** `git show HEAD:…\|grep` finds all three verbatim at HEAD (4875/4878/4903): the working tree's `+` is only the peer's `runtimeDiagnosticsEnabled()` gate wrapped around them. That peer's `//#region 🩺️RuntimeDiagnostics` docstring adopts them by name ("the refresh/completion chatter a boot emits once per dispatched action"), they are OFF by default, and `🧪️tests/🔬️engine-contract/🟦️.ts` pins the arming key. Deleting them would gut a live peer facility to satisfy a rule ("added by this tree") they no longer meet.|
|`🏛️ShellHost/🟦️.tsx` — undeclared-action comment|1|A comment, not a trace, and the peer's: it documents why the adjacent `console.error` is deliberately un-prefixed. Rule (d).|
|`🔌️PluginRuntime/🟦️.tsx` — `wireEffectToFriendly … empty action id`, `extension completion submitted`|2|Peer: `📓️renderer-fixes-2026-09-10.md` / `📓️hotpath-optimization-2026-09-10.md`.|
|`⚛️reactor/🔄️turn/🦀️.rs` — `turn phase`, `guest linear memory`, `continuation resolve dropped`, `more-work streak`, `streak ended`|5|The first three are peer (`📓️idle-turns-…`, `📓️guest-memory-…`, `📓️runtime-hotpath-audit-…`). The streak pair is at HEAD; the `+` is the peer re-gating it behind `runtime_diagnostics_enabled()` and re-shaping the format around its own `TurnMoreWorkSources`. `PatchTracker::debug_state` / the pending-patch `debug_state` stay for the same reason — both are at HEAD and both still have live readers (this trace, plus assertion messages in `🩹️patches/🧪️tests/🔬️unit/🦀️.rs`).|
|`⚛️reactor/🦀️.rs` — `extension response dropped the oversized request field`|1|Peer extension-correlation work.|
|`🔌️plugin/🦀️.rs` — `cooperative maintenance callback overran …`|1|At HEAD; the `+` is the peer's diagnostics gate.|
|`🔌️plugin/🦀️.rs` — `runtime close pending authority turn=…`|1|Peer: `📓️close-ladder-2026-09-10.md`.|
|`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` — `shard worker: retryable lifecycle deadline`|1|Peer: `📓️first-step-deadline-2026-09-10.md` (with its own law in `🎭️actor/📮️shard-client/🧪️tests/⏱️retryable-lifecycle-deadline`).|
|Peer test files: `🖥️host/🧪️tests/🔬️poll-turn-memory` (5), `⚛️reactor/🔄️turn/🧪️tests/📏️future-size` (2), `…/⏱️execution` (2), `🧵️retained-command/🧪️tests/🔬️unit` (1)|10|All four files are peer-authored (`poll-turn-memory`'s header literally opens "Boot #9b of `26/09/09/PROCEDURAL-3D-END-TO-END`"). The two hot-loop prints inside `poll-turn-memory` are the peer's to prune.|

`LAST_MAINTENANCE_STAGE` and its `maintenance stage=` `eprintln!` also stay: both are at HEAD,
neither appears in this tree's diff, and the atomic still has a live store/read pair.

Out of scope and untouched by this wave: `🕸️NodeGraph`, `🎯️targets/🧊️wgpu/**`, `🐚️Shell/…/🧊️wgpu`,
`🎭️actor/📮️shard-client`, `🎠️kernel`, `🌊️flow/**` and the `🌀️procedural`/`🌊️flow` plugin trees — all
carry peer `[DEBUG]` additions belonging to `26/09/09/PROCEDURAL-3D-END-TO-END`.

## 4. Verification

All under `CARGO_TARGET_DIR=…/scratchpad/target-p3d-f RUSTC_WRAPPER="" CARGO_INCREMENTAL=0`.

| Command | Result |
|---|---|
|`cargo check -p semio-framework-plugin`|**0 errors**, 1 warning (`associated function 'new' is never used`, macro-generated by `framework_reserved_job!`, pre-existing). Re-run after the peer's latest `turn/🦀️.rs` landed: **0 errors, 3.35s**. Crucially **no `debug_state is never used`** — the two orphaned interaction helpers are gone.|
|`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly`|**0 errors**, 87 pre-existing warnings, **1m 44s**.|
|`cargo test -p semio-framework-plugin --lib -j 4 <9 law names> -- --test-threads=1`, `RUST_MIN_STACK=67108864`|**9 passed / 5 failed / 619 filtered out**, 1.83 s. See the failure attribution below.|
|`bun ./📜️script.ts typecheck` (react target)|**826 `error TS`** — the expected pre-existing count, all in peer files.|
|`bun ./📜️script.ts test long --run '🔌️PluginRuntime' '🔬️engine-contract'`|**2 files passed, 600 tests passed / 0 failed**, 24.07 s.|

### The 5 red laws are peer-blocked, none reachable from this sweep's edits

W-S2 already recorded the state this lane is in: "The whole framework-plugin `--lib` run is
peer-blocked (54 failures from other lanes' in-flight app-definition/classification work)"
(`📋️master-plan-2026-09-08.md`). All five failures are that class, and each panics strictly
UPSTREAM of anything this wave touched — the sweep removed `eprintln!`s and collapsed one match arm
back to the identical one-line `Ok(Self::empty_result(…))`, which cannot move any of these:

| Law | Panic | Why it is not this sweep |
|---|---|---|
|`mutation_fixture::transaction::undo_and_redo_by_group`|`app-definition.invalid: app id testkit-txn must be a canonical surface id: surface id "testkit-txn" missing '#'`|Fails during app-definition validation, before a single dispatch. `testkit-txn` is the fixture's `APP_ID` in `🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs`; the validation lives at HEAD, the fixture that violates it is a peer's.|
|`composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles`|`composite edit: Fault … interactive-job.missing-factory: typed command 'compositeEdit' has no exact controller/owner/factory/tool/schema proof`|Fails on the **composite edit** that sets the test up, before any `undo` is issued. Classification/proof work, another lane's.|
|`group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child`|same `interactive-job.missing-factory` on `compositeEdit`|Same, same reason.|
|`benign_undo_with_nothing_to_undo_stays_unlogged_with_scope_none`|`🏪️store/🦀️.rs:17917: artifact store reached Drop without its exact terminal-empty shallow-shell witness`|A store TEARDOWN invariant, in a file this wave never opened and that peers changed by +374/−223 lines in this same tree.|
|`undo_on_empty_history_is_a_benign_no_operation`|same store Drop witness|Same.|

Green in the same run, and these are the ones that actually exercise the touched code: all seven
`local_interaction_live*` / `local_interaction_runtime_query_generation_*` laws (the two
`debug_state` helpers were removed from exactly those types), plus
`a_nakagin_scale_world_publication_reconciles_and_retires_within_a_handful_of_reactor_turns` and
`reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps` (the reactor turn
whose `TURN_TRACE` counter was removed). **9 passed / 0 failed among the laws that touch this wave.**

### One peer repair, to make the lane runnable at all

`cargo test -p semio-framework-plugin --lib` did not COMPILE on the first attempt: a peer had changed
`plugin_continue_typed_operations` to return `TypedOperationScan { runnable, contended }` instead of
`bool` without following the change into `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
(4 errors: 2 × `E0600 cannot apply unary operator '!' to type TypedOperationScan`, 2 × `E0308
expected TypedOperationScan, found bool`). That blocked every wave, not just this one. Repaired at the
two call sites — `let (output, scan) = …;` then `let more = scan.runnable || scan.contended;`, which
preserves the old combined meaning of the driver loops. Not part of the sweep; recorded here so the
hunk is not mistaken for one.

### The three react `typecheck` errors inside touched files are peer symbols

`🏛️ShellHost/🟦️.tsx` reports `TS18048` on `baseDispatchViewState.windowInstances` at the peer's
`undeclaredActionDiagnostic(…)` call and `TS2559`/`TS2339` at the peer's
`pasteActionWithRetainedFragment({ action: definition.id }, …)`; `🔌️PluginRuntime/🟦️.tsx` reports
`TS2353` on a `FaultScope` literal. All three are in peer-authored expressions this sweep never
touched, and removing a `console.warn` cannot produce any of these classes.

## 5. Residual `[DEBUG]` count

`git diff HEAD | grep -a -c '^+.*\[DEBUG\]'` → **420** across the whole tree, of which **~340 are
ticket-note markdown** (both tickets quote console output verbatim) and the rest are out-of-scope peer
plugin/renderer files. In the scoped areas **41 `+` lines remain**: 13 this ticket's deliberate
test one-shots (§2), 27 peer or at-HEAD (§3), and 1 comment. Zero remain in this ticket's production
code.

## 6. Landed DURING the sweep — left for whoever owns them

The tree is live and a wave WAS editing while this ran, contrary to the sweep's brief. Two things
changed under it, both re-verified at the end:

- **`🌐️World3dHost/🟦️.tsx` — 4 new lines, mtime 12:16.** The wave's original three
  (`[DEBUG] world dispatch`, `[DEBUG] frame visible instances` ×2) were removed by someone else
  between this sweep's first and second capture; the file then gained a **different** set —
  `[DEBUG] vortex marker hover`, `[DEBUG] vortex marker click`, `[DEBUG] world dispatch addBrushObject`.
  These are an ACTIVE browser diagnosis of the vortex-hover / brush-placement defect (W-AB), still
  unfinished. Deleting a live probe mid-run would destroy the diagnosis, so they stay. **They must be
  swept the moment that wave closes** — they are ungated `console.log`s on the hover and click paths.
- **`⚛️reactor/🔄️turn/🧪️tests/📄️command-page-authority/🦀️.rs` — 1 new line.**
  `eprintln!("[DEBUG] command page authority: {census}, {per_command} B/command")`, W-F6's own
  one-shot at the end of its law, printing the `4 098 B`/command figure the law asserts. Rule (b)
  admits it; stays.

Everything this sweep removed was re-verified as still gone after those landings.
