# Wave B2 — the host↔guest continuation ceiling: every retirement ladder priced by page

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-B2, 2026-09-11. Implements ranked ceiling **#1** of
`📓️2026-09-11-audit-A3-perf-ceilings.md` ("Wave 1"), the residual W-S2 named in its own §7.
HEAD at start `46c3cb9de0` (2026-09-11 12:39). No git write, ticket not closed, `🗑️generated` untouched.
Shared cargo build-dir, no `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, every run in the foreground.

---

## 1 Root cause, confirmed at file:line

The fault is `[DEBUG] PluginRuntime: actor puzzle#1 did not publish its requested UI surfaces within
4096 continuations (required=[], published=["1:puzzle3d-main-perspective"], effects=0,
status=more-work)` — thrown from
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1409`
(pre-wave numbering), reached from `interactionSelect` on the Nakagin document. Live evidence in this
ticket: `🗑generated/w-ab-41-nakagin-brush4.txt` → `🗑️generated/probe-2026-09-10T19-45-24.md`
(`## faults (1)`). The published set is ONE surface and `effects=0`, i.e. the loop spent 4 096 host
round trips on a guest that published nothing — the `acks=0` streak W-S2 §2.3 already traced.

W-S2 priced ONE ladder per page (`PATCH_CLOSE_UNITS_PER_TURN = 256`,
`PATCH_RETIREMENT_ITEMS_PER_UNIT = 1_024`, `PATCH_RETIREMENT_BYTES_PER_UNIT =
SURFACE_RECONCILE_PAGE_BYTES`, `⚛️reactor/🔄️turn/🦀️.rs:1353-1362`). Its §7 lists what it left at ONE
owner per unit; all four are confirmed live at HEAD and all four are document-scaled:

| ladder | file:line at HEAD | cost |
| --- | --- | --- |
| `SurfaceReconcileTerminal::close_step` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs:3316` → `SurfaceReconcileRetained::close_step_with:2477` | one retained owner per call; the call sites in `PatchTracker::close_step` (`⚛️reactor/🩹️patches/🦀️.rs:834, 923, 951`) were HANDED `(items, bytes)` and dropped them |
| `MountedTreeTerminal::close_step` | `⚛️reactor/🩹️patches/🦀️.rs:51` → `ComponentTreeProducer::close_step` (`🎭️present.rs:257`) | one node / one built-child page per call |
| `close_ui_turn_patch_transport_one` | `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:1736` → `UiTurnPatchTransportArena::close_one:1493`, hardcoded `close_step_with_grant(1, 4096)`, and driven **once per turn** (`🔄️turn/🦀️.rs:458`) | one item per TURN |
| `close_table_rows_view_one` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (`close_table_rows_view_one` → `TableRowsRetireArena::close_one` → `RetiredTableRowsView::close_step` → `TableRow::close_step`) | one row action / cell / column per call, driven **once per turn** (`🔄️turn/🦀️.rs:459`) |

The table ladder is the dominant one and had never been measured. A retained table window is
`TABLE_WINDOW_ROWS = 32` rows × `TABLE_WINDOW_CELLS = 32` cells (`🔌️plugin/🦀️.rs:26598-26601`), i.e.
**1 060 retirement units** — and `TABLE_WINDOW_RETIRE_SLOTS = 64` such windows may queue at once.
Measured this wave: **1 092 turns** for one outliner-scale table alone at the HEAD pacing.

Because every one of those turns answers `MoreWork`, the host counts it as a continuation toward the
SAME `PLUGIN_UI_CONTINUATION_LIMIT` — regardless of which surface is stalling it. That is why
`interactionSelect`, which touches the world-3d surface AND retained table surfaces in one turn, still
throws after W-S2 fixed the world-3d half.

## 2 The architecture change (not a bigger constant)

> W-S2's statement was "retirement is bounded work per TURN, not one item per turn". W-B2 applies it
> to EVERY retirement ladder, and adds its host-side twin: a continuation that publishes,
> acknowledges and emits nothing is not progress, and a settle must fail fast and name what it is
> waiting on instead of spending 4 096 round trips discovering it.

### 2.1 Grant threading, per ladder

| file | change |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs` | `SurfaceReconcileRetained::close_step_with` became `close_unit(items, bytes, registry) -> (complete, consumed)`: the PAGED owner (the pending patch) now gets the caller's whole `(items, bytes)` grant and reports it consumed the grant; every other owner on the ladder is a single retained value and reports 1. New `close_run(items, bytes)` spends a grant either way **without squaring it** (a paged unit ends the run; single owners cost one item each), `close_admitted_run(items, bytes)` keeps the handback reservation step in front of it. New public `SurfaceReconcileTerminal::close_step_with_grant(items, bytes)`; `close_step()` is now `close_step_with_grant(1, SURFACE_RECONCILE_PAGE_BYTES)`. |
| `…/🦀️rust/🎭️present.rs` | new `ComponentTreeProducer::close_step_with_grant(items)` — a bounded run of its own units. **Items only, deliberately**: its owners are whole nodes and built-child pages, nothing on that ladder is byte-priced, and a `bytes` parameter no owner consumes would be a lie in the signature. Documented in the docstring. |
| `⚛️reactor/🩹️patches/🦀️.rs` | `MountedTreeTerminal::close_step(items)` (same reasoning); all five `PatchTracker::close_step` call sites now pass the grant they were already given — four `terminal.authority.close_step_with_grant(items, bytes)`, two `terminal.close_step(items)`. |
| `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` | new `close_ui_turn_patch_transport_with_grant(items, bytes)`; `UiTurnPatchTransportArena::close_one(items, bytes)` passes it to `UiTurnPatches::close_step_with_grant` instead of the hardcoded `(1, 4096)`, and reports the owner's real `released_items`/`released_bytes` (floored at 1 so an empty owner still reports one unit of progress). `close_ui_turn_patch_transport_one()` is now `…_with_grant(1, 4096)`. |
| `🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `TableRow::close_step` answers `(complete, bytes)` — the retired text's own `UiText::len()`. `RetiredTableRowsView::close_step(items, bytes)` runs `close_unit()` until `items` units OR `bytes` of released text are spent (both decrement, so termination is structural). `TableRowsRetireArena::close_one(items, bytes)`; new `close_table_rows_view_with_grant(items, bytes)`, with `close_table_rows_view_one()` = `(1, 4096)`. |
| `⚛️reactor/🔄️turn/🦀️.rs` | the two once-per-turn ladder entries now run through the existing `retire_while_progress(retirement_deadline, …)` driver at `PATCH_RETIREMENT_ITEMS_PER_UNIT` / `PATCH_RETIREMENT_BYTES_PER_UNIT` — the same shape W-S2 gave `close_ui_turn_patch_owner_with_grant`. No constant changed. |

`close_ui_turn_patch_transport_one` / `close_table_rows_view_one` are retained as the `(1, 4096)`
spelling because existing laws pin the one-owner-per-opportunity semantics through them
(`abandoned_table_rows_retire_one_row_action_or_cell_per_opportunity`,
`ui_turn_patch_transport_max_plus_one_…`); production drives the granted variants.

### 2.2 Host: continuation ≠ progress

`🔌️PluginRuntime/🟦️.tsx`. `settlePluginTurn` now tracks a consecutive **zero-progress** streak —
a continuation that published no UI patch, emitted no effect and produced no acknowledgement — and
throws `pluginTurnStalledError` when it reaches the bound. The successful path is untouched: any of
the three resets the streak to 0, so a guest that keeps publishing keeps its whole continuation
budget; the empty-required early stop and the 4 096 backstop both remain.

The bound is **derived, not chosen** (`PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT`):

```ts
Math.ceil(DEFAULT_UI_DOCUMENT_LIMITS.maxPatchBytes / DEFAULT_UI_DOCUMENT_LIMITS.maxTextBytes) * PLUGIN_UI_CONTINUATION_BATCH_SIZE
```

One patch may carry at most `maxPatchBytes` (1 MiB); the smallest owner a single guest retirement turn
is guaranteed to release is one maximal text (`maxTextBytes`, 64 KiB); so a HEALTHY guest needs at most
16 publication-free turns per admitted patch, and one whole continuation batch (8) of slack per such
turn covers this loop's own acknowledgement cadence. **16 × 8 = 128**, i.e. 32× shorter than 4 096, and
it moves with the document contract rather than with a magic number.

The diagnostic names the pending surfaces: the required-but-unpublished ones when a surface set was
requested, otherwise the surfaces the settle DID publish — because those are the owners whose
retirement is holding the guest in `MoreWork`. On the recorded Nakagin fault it would read
`pending=["1:puzzle3d-main-perspective"] … zeroProgressLimit=128`.

## 3 Laws

### 3.1 Native — `a_nakagin_scale_mixed_surface_turn_retires_every_ladder_within_a_handful_of_reactor_turns`

`⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs`, sibling of W-S2's
`a_nakagin_scale_world_publication_reconciles_and_retires_within_a_handful_of_reactor_turns`. It
drives a genuinely MIXED turn — three ladders at once, at the production per-turn pacing:

1. the emitted world-3d patch (`UiPendingPatch::close_step`, the W-S2 ladder),
2. the retained world-3d SURFACE, closed through the real `PatchTracker` instance-close path
   (`reserve_close_instance`/`activate_close_instance` → `TerminalSlot`/`MountedTreeTerminal` →
   `close_surface_reconcile_handback_one`) — the terminal ladders,
3. an outliner-scale retained TABLE surface (`crate::app::TableRowsView`, 32 rows × 32 cells, dropped
   the way its window drops it) — the table ladder,

counting TURNS, and asserting `turns < 10`. A permanent `dripped_units` clause re-queues the same
table and drains it at one owner per turn, so reverting the pacing fails here rather than in a browser.

**Written first, run at HEAD (production sources unmodified) — FAILED:**

```
test component::reactor::reconcile_budget_tests::a_nakagin_scale_mixed_surface_turn_retires_every_ladder_within_a_handful_of_reactor_turns ... FAILED
a mixed world-3d + retained-table turn must retire every ladder inside a single-digit number of
reactor turns; observed 1093 turns against 1060 queued table retirement units
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 647 filtered out
```

**After the change:**

```
test component::reactor::reconcile_budget_tests::a_nakagin_scale_mixed_surface_turn_retires_every_ladder_within_a_handful_of_reactor_turns ... ok
[DEBUG] mixed-surface retirement completed in 5 turns, against 1092 turns for the retained table
        alone at the pre-W-B2 one-owner-per-turn pacing (1060 units queued)
[DEBUG] nakagin publication reconciled in 3 turns (2255 steps) and retires in 5 turns / 1092 units,
        against 137 turns / 1092 units at the pre-W-S2 pacing
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 648 filtered out; finished in 0.30s
```

**1 093 → 5 turns**, i.e. 218× fewer host round trips for the surface set one `interactionSelect`
touches, and W-S2's own law still passes unchanged at its measured 3/5/137.

### 3.2 TS — host fast-fail (`🧪️tests/🔌️plugin-runtime/🟦️.tsx`)

- `fails fast and names the pending surface when a guest answers more-work without publishing anything`
  — a guest stuck at `more-work` with a requested surface: `rejects.toThrow('pending=["7:puzzle3d-main-perspective"]')`,
  and `continuationCount === PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT`.
  Measured: `[DEBUG] zero-progress settle: continuations=128 limit=128 spinLimit=4096`.
- `does not fast-fail a guest that keeps publishing while the requested surface is still pending`
  — 256 continuations, one publication each, required surface last: the settle runs all of them.
  This is the clause that pins "the bound is on zero progress, not on continuations".
- **Rewritten**: `allows a large retained surface to reconcile beyond the former continuation ceiling`
  → `… while it keeps publishing`. That law asserted 1 024 consecutive SILENT continuations are
  acceptable — which is exactly the defect this wave removes (a Nakagin-scale reconcile is 3 turns,
  measured). It now publishes its section surfaces as it goes and the requested one at continuation
  1 025, keeping its real intent (a settle is not capped at the former 1 024 ceiling) under the new
  semantics. Greenfield rule: the old expectation was replaced, not adapted around.

## 4 Verification (foreground; peer breakage separated)

| command | result |
| --- | --- |
| `cargo test -p semio-framework-plugin --lib -- --test-threads=1 reconcile_budget` (law only, at HEAD) | **3 passed / 1 failed** — `observed 1093 turns against 1060 queued table retirement units` |
| the same, after the change | **4 passed / 0 failed**, `5 turns` (tail quoted in §3.1) |
| `cargo test … --test-threads=1 patches` | **42 passed / 3 failed** — all three are `plugin_builder_contract_tests` matched by the word *dispatches*, failing on `interactive-job.missing-factory: typed command 'incrementViaCommand' has no exact controller/owner/factory/tool/schema proof` (peer; the same breakage W-S2 recorded) |
| `cargo test … --test-threads=1 pending` | **16 passed / 2 failed** — both `app-definition.invalid: app id testkit-txn must be a canonical surface id` (peer; identical count to W-S2's run) |
| `cargo test … --test-threads=1 table` | **9 passed / 1 failed** — the failure is the same `testkit-txn` peer break; `abandoned_table_rows_retire_one_row_action_or_cell_per_opportunity` and the five other table laws pass |
| `cargo test … --test-threads=1 reactor::` | **144 passed / 4 failed** — `async_actor_poll_awaits_exchange_and_render_work` (source-text assertion on `plugin_exchange(runtime, cursor.instance, None).await`, wave B0's region), `cancel_of_a_parked_task_drops_it_and_frees_its_slot_for_reuse`, `revision_guard_rejects_an_intent_trailing_by_more_than_the_tolerance`, and `guest_turn_execution_resets_for_every_turn` (`left: Some(3) right: Some(2)` — a 1 µs timing flake in `drive_with_suspension`, a harness that never touches the close ladder) |
| `cargo test -p semio-framework-ui-runtime` | **122 passed / 0 failed**, 2 ignored |
| `cargo test -p semio-framework --lib -- --test-threads=1 ui_turn_patch` | **18 passed / 0 failed** |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | **0 errors** — `warning: semio-s-artifact-puzzle-3d (lib) generated 88 warnings … Finished dev profile in 1m 24s` (the warning count is the proof expansion really ran) |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | **0 errors** — `Checking semio-framework / semio-framework-ui-runtime / semio-framework-plugin … Finished dev profile in 5m 56s`; all three changed crates type-check for the guest target, not only natively |
| `bun ./📜️script.ts test long --run '🔌️PluginRuntime' '🔬️engine-contract'` (from the react target package — the root router no longer accepts `--run`, see §6) | **617 passed / 3 failed of 620** — the three are peer hunks in flight: `buildNoteShellCommandAction` (extra `inverseArgs`/`inverseCommandId`, `🛠️ShellHelpers`), `binds two instances of one body to distinct surfaces` (an extra `window`/`canvas-body` binding), `readAppDocumentPack()` (an extra `ops` field). All three are data-shape mismatches; none involves continuations. `git diff HEAD` on `🔌️PluginRuntime/🟦️.tsx` shows wave B4's section-hash work in the same file |

## 5 Remaining risks

- **The browser has not been re-measured.** This is guest Rust plus one host TS file: nothing changes
  in the browser until `component-release` is rebuilt AND materialised into the served module dir.
  The coordinator owns that run; this wave did not run the wasm build or the probe. Predicted from
  §3.1: the retained-table ladder's ~1 092 round trips per replaced table surface collapse to ≤ 5, so
  `interactionSelect`/`interactionHover` should stop reaching any continuation ceiling at all.
- **The fast-fail is global, not scoped to the Nakagin path.** Any settle whose guest goes 128
  consecutive turns without publishing, acknowledging or emitting now throws instead of spinning. That
  is the intent, but it is a behaviour change for every actor: a guest doing long internal work with a
  NON-empty required set and no intermediate publication will now fault at 128 rather than 4 096. The
  drain case (`requiredSurfaceIds` empty) is unaffected — it still stops at the first ack-less
  continuation, ahead of this check. Worth watching in the first browser run after the rebuild.
- **`MountedTreeTerminal`/`ComponentTreeProducer` take `items` only.** Their owners are node pages with
  no byte accounting; giving them a `bytes` parameter nothing consumes would have made the signature
  uniform and the pricing fictional. If built-child pages ever gain byte accounting
  (`close_built_node_page_one` has no grant variant), that is the follow-up.
- **`COLD_PAIR_INGRESS.advance_close_one` and `step_reactor_close` are still one unit per turn**
  (`🔄️turn/🦀️.rs:433-434`), as at W-S2. Neither holds a document-scaled owner as far as this wave's
  reading goes, and neither was measured — the same caveat W-S2 recorded, now the only one left.
- **Test-process sharing.** The new law drives the process-global table-rows retire arena and the
  surface handback registry; it takes `surface_reconcile_registry_test_guard()` and was verified at
  `--test-threads=1`. Under a fully parallel lib run another test's queued table could be counted.

## 6 Note for the next wave — the root test router changed

`bun ./📜️script.ts test long --run '🔌️PluginRuntime' '🔬️engine-contract'` (the command W-S2 and
W-Z used) now fails with `Unknown workspace test selection: --run 🔌️PluginRuntime 🔬️engine-contract`:
the root `TestScript` was rewritten today (ticket `26/09/09/NX-COMPLETE-TASK-CACHING`) to delegate to
the testing domain's phase vocabulary, and it no longer forwards bare vitest flags. The working
invocation for this lane is the owning package's own script:

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react
bun ./📜️script.ts test long --run '🔌️PluginRuntime' '🔬️engine-contract'
```

(`🔌️PluginRuntime/🟦️.tsx` is an in-SOURCE suite, listed in that config's `longInSourceSuites`; the
filter must name the source file, not the `🧪️tests/🔌️plugin-runtime` directory.)

## 7 Files

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs` — `close_unit`/`close_run`/`close_admitted_run`, `SurfaceReconcileTerminal::close_step_with_grant`.
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🎭️present.rs` — `ComponentTreeProducer::close_step_with_grant`.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` — `close_ui_turn_patch_transport_with_grant`, `UiTurnPatchTransportArena::close_one(items, bytes)`.
- `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🔬️ui-turn-patch/🦀️.rs` — call site updated for the new arity.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs` — `MountedTreeTerminal::close_step(items)`, five granted call sites in `PatchTracker::close_step`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` — transport and table ladders driven through `retire_while_progress` with the page grant.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `TableRow::close_step -> (bool, usize)`, `RetiredTableRowsView::close_step(items, bytes)`/`close_unit`, `TableRowsRetireArena::close_one(items, bytes)`, `close_table_rows_view_with_grant`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs` — §3.1 law, `queue_outliner_scale_table_surface`, `mount_and_publish_nakagin_world`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — `PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT`, `pluginTurnStalledError`, the zero-progress streak in `settlePluginTurn`, export.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` — §3.2 laws.
- this report.

No temporary `[DEBUG]` logging was left in production code. The `[DEBUG] ` prefixes that remain are the
laws' own measurement lines and the thrown diagnostic, matching the prefix the existing continuation
timeout in the same function already uses. `🗑️generated` was not touched.
