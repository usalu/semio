# 📓️ Wave 2I — the product defects the extended ◻️2d battery exposed, fixed at the root (2d, 3d and the shell)

Slice 2I. Five hand-offs from `📓️wave-2G-report.md` §4 plus the two long-standing 🧊️3d battery reds from
`📓️E11-3d-baseline-battery-triage.md`. Paths are relative to `/Users/ueli/Documents/semio`.
`EDITOR2 = ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`,
`EDITOR3 = …/🧊️3d/…/✏️editor`, `BOARD = 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs`,
`B2H = 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx`,
`W3H = …/🧱️elements/🌐️World3dHost/🟦️.tsx`, `HELPERS = …/🧱️elements/🛠️ShellHelpers/🟦️.tsx`,
`SHELLHOST = …/🧱️elements/🏛️ShellHost/🟦️.tsx`.

---

## 0. TL;DR

| # | defect | root cause | fixed in |
|---|---|---|---|
| 1 | `deleteSelection` ignores the lock in 2d; no refusal notice anywhere | no single lock gate; each verb decided for itself, and the two that honoured the lock did it silently | 2d **and** 3d |
| 2 | fill `Step` places nothing; `Abort` from Paused never ends the run | `Step`'s single unit of fuel was spent on a *rejected candidate*, not a placement; `Abort` was refused as `Stale` and the rejection published no repaint, so the button never recovered | 2d + framework (fixes 3d too) |
| 3 | the Actions-panel `undo` row is inert while the History panel's undo works | an armed utility gated **every** row of the pane, including the framework-injected history/clipboard rows — while `mod+z` for the same verb is not gated | framework |
| 3b | `framework.history.revert` does not exist | it never did: revert-to-command is an **id-less** `↶` glyph on each ledger row. Also four English string literals in a bilingual panel | framework |
| 4 | handle ids never reach the DOM | `data-board-positions-json` publishes nodes only; nothing named a handle | board engine + wasm bridge + `B2H` |
| 5 | catalogue kind rows unbound for hover | the engine's kind-hover has existed all along and nothing ever called it | `B2H` (host-local, no verb, no round trip) |
| 3d-A | `gumball-scene-delta` (long-standing) | the gesture's ids were re-resolved per dispatch, and a `componentIds` fallback minted numeric ids no object can answer to → the guest mutated nothing | `W3H` |
| 3d-B | `locked-refusal-notice` (long-standing) | `Effect::Notify` from a retained job **is** delivered; the guest simply never reached the `selection_locked` branch, and when it did it could be lying | `W3H` + `EDITOR3` |

**Verdicts: `cargo check -p semio-s-artifact-puzzle-2d` exit 0 (5 crate warnings, none mine); `-p semio-s-artifact-puzzle-3d` exit 0 (103 crate warnings, none mine); `cargo test … --lib -- lock_tests` 6 passed / 0 failed; `cargo test -p semio-framework-os-infinite --lib -- handle_vitals` 1 passed / 0 failed.** Outputs in `TICKET/🗑️generated/2I/`.

---

## 1. Item 1 — one lock, one promise, one sentence

### 1.1 ◻️2d

The lock was decided verb by verb: `puzzle2d_transform_selection` skipped locked nodes *silently*, `cut` refused
with a notice, `deleteSelection` did not look at all (2G measured `before=12 after=0` on a node whose inspector
flag read `locked true`). There is now exactly one predicate and one refusal helper.

| what | where |
|---|---|
| `puzzle2d_addresses_locked_entity(fixture, ids) -> bool` — node, handle, edge **and** target region | `EDITOR2/🦀️.rs` (just above `puzzle2d_selection_is_locked`) |
| `Puzzle2dActionCtx::notice(..)` — at most ONE `Effect::Notify` per action (3d's `Puzzle3dActionCtx::notice` twin) | `EDITOR2/🦀️.rs` `//#region 🔖️ActionContext` |
| `Puzzle2dActionCtx::refuse_when_locked(&ids) -> bool` — notice + `UiDirtyScope::None`, no edit, no fault | same |
| label `selection_locked` EN `"Selection is locked"` / DE `"Die Auswahl ist gesperrt"` (+ the `reuse` axis) | `EDITOR2/🗣️terminology/🦀️.rs` beside `cut_locked` |

Gated verbs (each `if ctx.refuse_when_locked(&ids) { return }`):
`🎮️commands/🗑️delete-selection`, `🚀️translate-selection`, `🔄️rotate-selection`, `📏️scale-selection`,
`🩹️patch-inspector`. `cut` already refused and is unchanged.

The **drag** is not a verb — it arrives as `applyBoardEvents`. `apply_board_events_from_json` now returns
`bool` ("a locked entity refused a gesture in this batch") and refuses the three moving rows whole:
`nodeMove` (per id), `nodeDragEnd` (the whole `moves` list), `nodeRotate` (the whole `ids` list).
`apply_board_events` raises the sentence **once per batch** — a drag streams one `nodeMove` per pointer tick
and a notice per row would bury the board. The dispatch epilogue in `EDITOR2/🦀️.rs` answers the same way for
rows the engine drains itself (`apply_host_events` now returns the same bool), never twice.

**Semantics chosen: any locked entity refuses the WHOLE gesture.** That is what `cut` already did, and a
half-applied drag is worse than a refused one. `hidden`/`locked` are never gated by `patchInspector`, or a
locked node could never be unlocked (the inspector's own flag rows send `setSelectionFlag`, but the field
names are honoured on this route too so no path can wedge the document).

### 1.2 🧊️3d — the same audit

`translateSelection` has refused a locked grab since wave B31; `deleteSelection` erased the same object
without a word. `EDITOR3/🎮️commands/🗑️delete-selection/🦀️.rs` now asks the `locked` flags of the selected
objects, vortices, target volumes and references and calls the existing `ctx.refuse_when_locked()`.
(3d attractions carry no `locked` field — `Puzzle3dAttraction`, `EDITOR3/🦀️.rs:251`.)

`Puzzle3dScaleWork`'s terminal refusal used to read "the work produced no mutation ⇒ the selection is locked",
which is a **misreport** for ids that name nothing. It now asks the flags — see §7.2.

### 1.3 What 🖐️5d needs (hand-off to the 5d integrator)

5d inherits the ◻️2d board pane. Three things to port, in this order:

1. `selection_locked` in `…/🖐️5d/…/✏️editor/🗣️terminology/🦀️.rs` (EN+DE, plus the `reuse` axis).
2. `notice` + `refuse_when_locked` on 5d's action ctx — copy `EDITOR2/🦀️.rs`'s pair verbatim; 5d's ctx already
   carries `effects` and `labels`, so it is a paste.
3. The same five guards + the `applyBoardEvents` bool. If 5d reuses `apply_board_events_from_json`, it gets
   the drag half for free and only needs to raise the notice at its own call site — **the signature changed
   from `()` to `bool`, so 5d will not compile until that one call site takes the return value.**

---

## 2. Item 2 — fill `Step` and `Abort`

### 2.1 `Step` placed nothing — the unit of fuel was not the declared unit

There is **no missing verb and no dead transition arm**. `toolRunStep`/`toolRunAbort` are framework-reserved
and short-circuit before the app registry (`🔌️plugin/🦀️.rs` `dispatch_tool_run_action`); the state machine's
arms are correct (`🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs` — `(Paused, Step) → DriveOneUnit`,
`(…, Abort) → CloseJob`). `Step` grants the job **exactly one unit of fuel**.

This run declares its unit as `placements` (`🗣️terminology` `fill_unit`) and reports
`completed = placements.len()` — but `Puzzle2dFillRunJob::decide` charged a unit for every *decided
candidate*, and on a 29-node board almost every candidate is a `host-collision` rejection. So `Step` burned
its one unit on a rejection and the census never moved (`atPause=29 after=29`).

**Fix** (`EDITOR2/⏳️precompute/🪣️fill/🦀️.rs`, `fn decide`): the verdict record no longer consumes fuel; only
`accept` does. A single `Step` now runs until one placement lands. The tick stays bounded without that
charge — `context.deadline_exceeded()` and the trace writer's `FILL_RUN_TICK_FLUSH_BYTES` both still gate the
loop, and a rejection costs bytes.

(3d charges per candidate *deliberately* and documents it; its battery never presses `Step`. Left alone — its
unit label and its progress counter agree with each other. Noted for whoever unifies the two.)

### 2.2 `Abort` was rejected as `Stale`, and the rejection hid itself

The tool-run panel's buttons carry the run identity **captured when the panel last rendered**. A settings
change — `fill 12`, a count stepper, which is exactly what the battery does immediately before pressing
Abort — bumps the live generation (`(Running|Paused, SettingsChanged) → Reconfigure, increment = true`). The
stale press is then rejected, and a rejection published `UiDirtyScope::None`, so the button kept its stale
identity and **every** later press was rejected too, forever. Silently: a rejection is a no-op.

Two fixes, both in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs`:

- `dispatch_tool_run_action`: `Rejected(_)` now publishes `self.tool_runs.dirty_scope()` instead of
  `UiDirtyScope::None`, so a stale press refreshes the buttons and the next press lands. This unsticks
  `Step`, `Pause`, `Resume` and `Finalize` as well, which share the defect.
- `apply_selected_tool_run_action`, the `Abort` arm: an abort is idempotent in the generation — it ends the
  run whatever that run was last reconfigured to, exactly as `Dismiss` already ignores generation. Abort now
  takes the live slot's generation when the named `runId` matches. The one control whose whole job is to get
  out of a run must not become unreachable.

**Both fixes apply to 🧊️3d and 🖐️5d unchanged** — this is framework code shared by every tool run.

### 2.3 Not done — a retargetable 2d fill job (owed)

2d ships no `ToolRunRetargetableJob`, so `ToolRunReconfigurePolicy::Resume` **closes the run job outright**
on every count nudge (`retarget_current_job`'s else-branch), while 3d retargets in place
(`EDITOR3/🦀️.rs` `build_retargetable_tool_run_job` → `…/🛠️tools/🪣️fill/🦀️.rs` `reconfigure`/`rebind` →
`FillRunJob::retarget`). That is very likely 2G finding #5 (`fill <n>` not retargeting the count mid-run).
The port is mechanical and self-contained; it is the single highest-value remaining fill item.

---

## 3. Item 3 — the inert Actions-panel rows

**Both routes dispatch the identical descriptor.** `#action.undo` sends
`{ controllerId: app.controllerId, action: "undo" }` (`HELPERS` `buildActionCategoryTree`, bound through
`windowActionPaneNode`'s `controllerId={app.controllerId}`), which is byte-for-byte what
`framework.history.undo` sends (`SHELLHOST` `frameworkUtilitiesHistoryTab`). The ticket-memory window-level
hazard does **not** apply: the descriptor was already app-scoped.

The difference is upstream of the dispatch. `buildActionCategoryTree` took one pane-wide `disabled` boolean
— true whenever a utility with `allows_actions_while_active: false` is armed, which is the **default**
(`🛂️manifest/🦀️.rs` `UtilityDefinition::new`) and which all three puzzle-2d utilities take. It stamped
`pointer-events-none opacity-50` on the row element that also carries the DOM id **and** short-circuited the
handler, so a click could neither reach nor fire it — no fault, no console line. Every battery lane after the
first `armUtility` measured every row of that pane as dead, including the framework-injected
`undo`/`redo`/`commitCheckpoint`/`copy`/`cut`/`paste`. The History panel is not utility-gated at all; that is
the whole asymmetry.

**Fix** (`HELPERS`, `buildActionCategoryTree`): the gate is now per row and never covers a framework-reserved
verb — `const rowDisabled = disabled && !FRAMEWORK_RESERVED_ACTION_IDS.has(action.id)` (that set is already in
the same file). `mod+z` for the same verb is bound outside the pane and was never gated, so the pane was
inconsistent with its own chord. The gate still covers every app verb, which is what it exists for.

Laws: `HELPERS/🧪️tests/🧩️component/🟦️.ts` region `🧰️ActionPaneGate` — an armed utility gates
`action.deleteSelection` and not `action.undo`; with nothing armed both press. **Written, not executed** (see §8).

### 3b `framework.history.revert`, and the four English literals

It has never existed. What the panel offers today (`SHELLHOST`, `frameworkUtilitiesHistoryTab`) is:
`framework.history.{undo,redo,checkpoint,checkin}` plus `framework.history.entry.<seq>` rows.
`checkin` is the VCS publish row (`#s-checkin` → `commitCheckpoint` with a message) — never a revert.
**Revert-to-command is the per-entry `↶` control**, and it had no id of its own, so nothing but a mouse
could reach it: both the 2d and the 3d battery looked for a `framework.history.revert` and scored the panel
as having no revert at all.

Fixed:
- the control is now `id="framework.history.entry.<seq>.revert"`, with `title`/`aria-label`.
- `Undo`, `Redo`, `Checkpoint`, `Commands` were English string literals in a bilingual panel. They now go
  through `historyPanelText(key, uiLocale)` (`HELPERS`, `HISTORY_PANEL_LABELS`, EN+DE, sitting beside the
  existing `CHECKIN_*` frozen labels), which also supplies the revert control's label
  (`Revert to Command` / `Auf Befehl zurücksetzen`).

The History panel is framework-shared, so **2d and 3d are identical here by construction** — "make 2d
identical to 3d" needed no 2d-side work, only the addressable control both were missing.

Still absent from the React panel and present in the wgpu twin (`🔌️plugin/🦀️.rs` `framework.history.*`):
`createAlternative`, `checkoutCheckpoint`, and the command filter. Out of scope; flagged.

---

## 4. Item 4 — handle ids in the DOM

`BoardHost::handle_positions_json()` (`BOARD`, beside `interaction_json`) publishes every handle the pane can
actually be **pointed at**:

```
{"total":358,"onScreen":41,"published":41,"capped":false,
 "rows":[["seed-left-001:v0", -30, 0, "seed-left-001", "b-l", true], …]}
```

Row = `[handleId, worldX, worldY, nodeId, handleKind, open]`, `open` = no incident edge (the precondition
`openHandleSuggestions` and `createEdge` both need). World coordinates, the same space as
`data-board-positions-json`, so the probe's existing `nodeScreen` camera maths applies unchanged.

**Bounded twice**: the row set is the camera's own viewport (a handle off-screen cannot be clicked, so
publishing it buys nothing), then a hard `BOARD_HANDLE_VITALS_CAP = 128`. `total`/`onScreen`/`capped` say
exactly what was left out, so a caller can zoom in and read more rather than guess. Nakagin's 358 handles
never grow the attribute.

Chain: `BOARD` → `EDITOR2/🌉️wasm/🦀️.rs` `handlePositionsJson` → `Board2dWasmSession.handlePositionsJson?()`
(`🪪️WasmSessionLoader/🟦️.tsx`) → `B2H` `publishBoardVitals` → **`data-board-handle-positions-json`**.
Written only from `publishBoardVitals` (rAF + post-render effect), never from JSX, so a re-render cannot
stamp a stale frame — 2E's rule for `data-board-interaction-json`/`-transform-json`.

Coordinated with 2E: one line added inside `publishBoardVitals`, nothing else in that region touched.

Law: `BOARD`'s standalone tests, region `🩺️HandleVitals` — names all three on-screen handles with the
handle's own world point (not its node's), reports every handle open on an edgeless document, and publishes
`onScreen:0 rows:[]` with `total:3` intact once the camera leaves the document. **Passes.**

---

## 5. Item 5 — catalogue kind hover

**3d does it**, and it is fully kind-driven: `hoveredKindId` in `world_selection_json`
(`EDITOR3/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` `hovered_kind_id`) → `W3H`'s instance chrome store
(`isHighlighted(objectKind) === objectKind === snapshot.hoveredKindId`) → the `highlighted` palette entry.

**2d's engine already does it too, and nothing ever called it.** `BoardHost` carries
`hovered_kind: Option<(String,String)>`, `set_hovered_kind_silent(domain, kind_id)`,
`ids_matching_kind_hover()` and `hovered_style_kind()`; `setHoveredKindSilent` is bridged in
`EDITOR2/🌉️wasm/🦀️.rs`. A repo-wide grep found **zero** JS callers. That is 2B's open question: the
mechanism was complete except for its trigger.

**Delivered, host-local**, in `B2H`:

- `board2dKindDomainById(glyphCatalogsJson)` — the board's own `nodeKinds`/`handleKinds`/`edgeKinds`/
  `wireKinds` as `kindId → domain`, using the engine's own domain words.
- `board2dKindHoverFromElementId(elementId, map)` — a catalogue row is `<sectionId>.<kindId>` in 2d
  (`puzzle2d-play-kinds.nodes.beam`) and a bare kind id in 3d; both resolve, and **only against kinds this
  board carries**, so the host hard-codes no panel id scheme and no plugin name.
- a capture-phase `pointerover` listener that paints through `session.setHoveredKindSilent(domain, kindId)`
  and publishes **`data-board-hovered-kind`** (`"<domain>:<kindId>"`, absent when none). Pointing anywhere
  else clears it.

A hover deliberately costs **no verb, no guest round trip and no repaint**: it is not a document act, and a
per-row `UiValue` arg map is exactly the tree-row cost that starves sibling panels (the reason 2B rejected
the per-row route). Hovering a kind row therefore highlights every node of that kind on **all three panes** at
once, which is what the 3d viewport does.

Laws: `B2H/🧪️tests/🧩️component/🟦️.ts` (new) — the domain map over all four slices, malformed JSON, and the
row-id resolution including the refusals. **Written, not executed** (see §8).

---

## 6. Registries

**None were touched, and none needed to be.** No verb was added anywhere in this slice: every fix is a guard,
a helper, a host listener or a framework routing/label change. `puzzle2d_command_variants!`, `TOOL_JOB_IDS`,
`PUZZLE2D_RETAINED_TOOL_IDS`, `bounded_first_step_tool_proofs!`, `build_tool_job`, `PUBLICATION_CONTRACTS`,
`.action_with`/`.action_interactive_job`, `command_from_action` and the retained-job fixtures are unchanged.
The `toolRun*` verbs are framework-injected and routed before the app registry, so §2 needed no registry
either. New **labels** (`selection_locked` in 2d, `HISTORY_PANEL_LABELS` in the shell) are EN+DE on every axis.

---

## 7. The two 🧊️3d battery reds

### 7.1 `gumball-scene-delta` — the gesture lost its own ids

Three faults on one path, in `W3H`:

1. `world3dGumballSelectionArgsV1` fell back to `(selection.componentIds ?? []).map(String)` — face/vertex
   indices, i.e. ids like `"0"`, `"1"` that **no `Puzzle3dObject.id` can ever match** — contradicting its own
   doc comment. The guest collected them, matched nothing, mutated nothing, and completed with a refusal.
   Removed; an empty list is the honest answer.
2. The ids were resolved **per dispatch**, not at drag start. A gumball drag routinely changes the selection
   under itself: a press that misses the handle is a canvas pick that **clears** it (the probe tries up to
   four points along the axis). Now pinned in `gumballGestureArgsRef` at `handleGumballDragStart` and cleared
   after `transformEnd`.
3. An id-less delta was dispatched anyway and failed silently for 30 s. It is now refused loudly, with the
   existing permanent `console.info("gumball pose delta skipped", { reason: "no-selection-ids", … })` record.

This is exactly the difference between this lane and the structurally identical Relocate lane that **passes**:
`world3dRelocateDragTargetV1` has an id fallback; the gumball lane had none.

Ruled out, with evidence, so nobody re-investigates:
- **The retained path does NOT drop `Emit.effects`.** They drain one per turn through the typed-effect
  outbox, ride the completion as `requestedEffects`, and `typedOperationCompletionRefreshV1` explicitly keeps
  a `none`-scope completion alive when effects exist → `showTransientNotice` → `role="status"`, the exact
  selector the probe sweeps.
- **The publication-lane contract does not gate `effects`** — only mutations/presence/transient/interaction
  lanes — so `translateSelection`'s `lanes: &[Artifact]` never rejects a Notify-only emit.
- **`work_items` cannot truncate a retained job.** The extent is a preflight pacing counter only
  (`🎮️commands/🧵️retained/🦀️.rs`), never compared against `work_cursor`. It *was* dishonest by 4 for
  `Puzzle3dScaleWork`; corrected for hygiene (`EDITOR3/🦀️.rs` `fn extent`, `+4` for the four terminating steps).

### 7.2 `locked-refusal-notice` — and a refusal that could lie

Same upstream cause: with ids that name nothing the job short-circuits on `nothing_selected`, which the
probe's `/locked/i` predicate correctly rejects.

Independently, `Puzzle3dScaleWork`'s terminal branch inferred the *message* from "produced no mutation", so a
stale leftover id made it announce **"Selection is locked"** for a document with nothing locked. It now asks
the `locked` flags of the collected objects and volumes and says `nothing_selected` otherwise. A refusal that
names the wrong cause is worse than none.

### 7.3 A probe bug, not a product one — `gumball-handle-enter` can never pass

The 3d probe requires console lines `[DEBUG] gumball drag entered … kind: move`. Those were removed (rule 6:
`[DEBUG]` logs go before a slice finishes); drag entry is now the `__gumballDragEntered` global the probe
*also* reads. `moveEntered` is therefore permanently false regardless of product health.
**Probe fix**, `…/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts:2052`:
`verdict("gumball-handle-enter", drag.entered, …)` — drop the `&& moveEntered` conjunct and the
`enteredLogs` read above it.

---

## 8. Commands run — real verdicts

| command | verdict |
|---|---|
| `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --message-format=short` | **exit 0**, `Finished dev profile in 1m 14s`, 5 crate warnings (`unused imports: Buildable, HasBase` in `🎭️modes/✏️edit/🦀️.rs` — another slice's), 72 warning lines across all deps → `🗑️generated/2I/check-2d-final.txt` |
| `… -p semio-s-artifact-puzzle-3d …` | **exit 0**, `Finished in 1m 29s`, 103 crate warnings, none in the lines I touched → `check-3d.txt` |
| `CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly --lib -- lock_tests` | **6 passed, 0 failed** → `test-2d-locks.txt` |
| `CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-infinite --lib -- handle_vitals` | **1 passed, 0 failed** → `test-board-handle-vitals.txt` |
| first (pre-fix) 2d check | exit 101 — **one** error, `E0502` in `try_begin_region_drag_at` (region-drag code, not mine), blocking the whole crate → `check-2d.txt` |

**One edit outside my slice, declared:** `BOARD` `try_begin_region_drag_at` held an immutable borrow of
`self.regions` across `self.set_selection_ids_gestured(..)`. It now reads `(start_bounds, locked)` out of the
region first. Semantics unchanged; it was the only thing standing between the crate and a verdict.

No `nx`, no dev server, no activation, no Playwright, no git write. Nothing was written outside
`TICKET/🗑️generated/2I/` and the source files listed here.

---

## 9. NOT verified / owed to integration

1. **No TypeScript typecheck, and 5 TS files changed** (`B2H`, `W3H`, `HELPERS`, `SHELLHOST`,
   `🪪️WasmSessionLoader`) plus two new `.ts` test files. This is the slice's biggest gap. The three new
   exported helpers are pure and small; the risky surface is the `useEffect` in `B2H` and the `historyPanelText`
   import in `SHELLHOST`.
2. **The three new TS laws are written, not executed** (`HELPERS/🧪️tests/🧩️component/🟦️.ts` `🧰️ActionPaneGate`,
   `B2H/🧪️tests/🧩️component/🟦️.ts`) — vitest needs nx, which this slice may not run.
3. **`cargo check --target wasm32-wasip2`** not run. The wasm bridge method `handlePositionsJson` is
   `wasm32 && !p2`, so wasip2 cannot see it; **integration must run
   `cargo check -p semio-s-plugin-puzzle --target wasm32-unknown-unknown --no-default-features`** — the only
   check that compiles it — before trusting `session.handlePositionsJson` exists on the JS side.
4. **`bun ./📜️script.ts publication-authority-audit`** not run. This slice added no verb and touched no lane,
   so it should be neutral; 2B/2D own its one known red.
5. **Nothing here has been driven in a browser.** Every claim about the battery reds is a source argument
   plus the unit laws, not a measurement. The fill `Step` fix in particular changes tick pacing (a running
   tick now does up to `INTERACTIVE_LANE_FUEL` *placements* instead of decisions) — bounded by the deadline
   and the flush budget, but **unmeasured under the 8 ms reactor law**. Watch it in the next battery.
6. **The 2d fill job is still not retargetable** (§2.3) — owed, and probably 2G finding #5.
7. **The `Abort` rejection-repaint change makes every rejected tool-run press repaint the panel.** Correct,
   but it is a new repaint on a hot path; if a battery shows tool-run churn, this is the line.
8. The catalogue kind hover uses a document-level capture listener. It is generic (it recognises rows by the
   board's own kind catalogs) but it is a *host* observing chrome it does not own. The principled home is a
   framework tree-row hover channel — `item.onPointerEnter` exists on the Tree element and **nothing in the
   React Interpreter ever sets it**. Worth a framework ticket.

---

## 10. Exact DOM ids and attributes for the battery (slice 2G)

### New
| selector | where | shape |
|---|---|---|
| `[data-surface-id="window:<pane>"][data-board-handle-positions-json]` | every 2d board surface | `{"total":n,"onScreen":n,"published":n,"capped":bool,"rows":[[handleId,worldX,worldY,nodeId,handleKind,open]]}` — world coords, same camera maths as `data-board-positions-json`; viewport-bounded, capped at 128 |
| `[data-board-hovered-kind]` | every 2d board surface | `"<domain>:<kindId>"`, e.g. `"node:Hexagonal Cut Concrete Forest Left"`; **attribute absent** when nothing is kind-hovered |
| `#framework.history.entry.<seq>.revert` | History panel, per ledger row | the revert-to-command button (`revertToCommand {entrySeq}`); present only when `entry.revertible && !isViewer`. **This is what the lane calling for `framework.history.revert` should use** |

### Changed behaviour behind existing selectors
- `#action.undo` / `#action.redo` / `#action.commitCheckpoint` / `#action.copy`/`cut`/`paste` are **no longer
  disabled while a utility is armed**. The row id is `action.commitCheckpoint`, *not* `action.checkpoint`.
  ⚠️ All three panes still author the same unqualified `action.*` ids — the `undo` lane
  (`…/☀️06/PUZZLE-2D-END-TO-END/🔍️browser-probe.ts:1107`) is the one lane still using a page-wide
  `.first()` locator and can be aiming at a hidden Detail-pane twin. **Use the window-scoped `fireAction` /
  `actionRow` helpers (`:816`) like every other lane**; I deliberately did not window-qualify the ids, because
  that would break three probe files owned by other slices for no gain the scoped locator does not already give.
- History panel button text is now localized: `Undo`/`Rückgängig`, `Redo`/`Wiederholen`,
  `Checkpoint`/`Checkpoint`, section `Commands`/`Befehle`. A lane matching the literal `"Undo"` must match
  `/undo|rückgängig/i`.
- `locked-refusal-notice` (both 2d and 3d): the refusal is an `Effect::Notify` → `role="status"` transient
  that **auto-dismisses after 4000 ms**. 2d's sentence is `Selection is locked` / `Die Auswahl ist gesperrt`.
  Assert `/lock|gesperrt/i`, and poll rather than sample.
- `8-locked/locked-node-refuses-delete` should now measure `before == after` and exactly one notice; the same
  is true for a drag (`nodeMove`/`nodeDragEnd`/`nodeRotate`), `translate/rotate/scaleSelection` and an
  inspector stepper. **A whole drag batch raises ONE notice, not one per row.**
- `12-fill/fill-step-advances-one-placement`: `Step` now advances by one **placement**. It may take longer
  than before on a crowded board (it runs until a placement lands or the tick deadline) — budget for it.
- `12-fill/fill-abort-ends-the-run`: Abort no longer goes stale after a count nudge, and a rejected press now
  repaints the panel so the next press has a live identity. If it still fails, read the dispatch result —
  `{"toolRun":"closeJob"}` vs `{"rejected":"stale"}` — instead of only the panel text; that turns the
  remaining inference into a fact.
- `gumball-handle-enter` (3d): see §7.3 — a probe fix, not a product one.

### Unchanged, restated because it bit twice
`data-board-selection-json` is the **local/optimistic** echo since 2E; the committed set is
`data-board-guest-selection-json`.
