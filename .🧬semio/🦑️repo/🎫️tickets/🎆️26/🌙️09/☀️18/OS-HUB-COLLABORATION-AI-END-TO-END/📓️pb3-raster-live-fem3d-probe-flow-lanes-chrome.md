# PB3 — 🖨️raster live bar · 🏗️fem3d probe selection · 🌊️flow lanes + `addWidget` · shared panel/rail chrome

Slice **PB3** (2026-09-20, from 17:32). Turns PB1's native proofs into live results and closes its
four named defects. Spec: `📓️pb1-raster-redo-fem3d-undo-flow-addwidget.md` (whole) and
`📓️b3d-cad-lowpoly-draw-shooting-sourcing-demonstrator.md` §B3f.

**Inherited live state at 17:33** (nothing restarted, nothing killed): fleet wasm mutex **FREE**,
`pgrep -fl rustc` = 0, 40 GiB free on `/System/Volumes/Data`. Serves alive and reused as they stood:
6060 raster (pid 98222), 6086 fem2d (823), 6087 fem3d (851), 6216 flow (65151), 6018 generation3d
(16450), 6081 sourcing (15048). No `🗑️generated/pb3-*` capture and no predecessor PB3 report
existed — this slice starts cold on top of PB1's source edits (already in the tree, uncommitted).

## Status

| # | item | state |
|---|---|---|
| 1 | 🖨️raster live re-measure after one activation | ✅ **5/5 live**, `faultLines: 0`, plus paint → undo → redo → undo all four presses green |
| 2 | 🏗️fem3d — press a verb that really mutates, re-measure | 🔴 **PB1's verdict DISPROVEN at runtime**: with `addNode` (args staged, document really changes) fem3d still does not undo. Root named, needs a rebuild to fix |
| 3 | 🌊️flow — `[Child, Ui, Terminal]` lane set + `addWidget`'s composed-child `None` | 🟡 both signatures root-caused and closed: the lane set was flow's law being one page short of the framework's contract, and the live `None` was flow never implementing `genesis_child_pack` (**fixed**, 3 product files, zero suite regression). Live: refusal gone, `faultLines 1 → 0`, `Add Widget↶ / Undo↶ / Redo↶` all land — but the parent-side edit count and projection never see a child-lane edit, so the bar still scores 0/5 |
| 4 | shared chrome — docked panel overlays the Actions rail | ✅ root-fixed in `📐️Layout` (TS-only), 4 vitest laws green, 📐️generation3d's staged-arg verb now clicks and the app is **5/5 live** |

## 1. 🖨️raster — the live bar, 5/5

**One activation, 3 min 18 s.** `📜️pb3-activate.sh raster` through `📜️wasm-build-mutex.sh pb3`
(mutex was free, granted at 17:34:31), **exit=0 at 17:37:49**
(`🗑️generated/pb3-raster-activate.txt`) — 18 nx tasks, `@semio-tech/raster-plugin:component-dev`
1 m 43 s, everything else behind B3f's cache. B3d's serve on :6060 (pid 98222) picked the new guest
up without a restart.

**The five-clause bar, on B3f's exact invocation** (`SEMIO_PROBE_ROW='[role="treeitem"]:has-text("Add
Pixel")'`, `SEMIO_PROBE_SETTLE_MS=60000`, `🗑️generated/pb3-raster-bar.txt`,
`🗑️generated/b3a-pb3-raster-console.txt`):

```
SUMMARY {"ready":"raster","error":null,"exampleRendered":true,"actionCount":0,
         "mutated":true,"undone":true,"redone":true,"panelRoundTrip":true,
         "faultLines":0,"interactionBar":true}
```

with the ledger witness: `edits 2 → 3`, `entry.20: Add Layer create-layer index=3 pixel
id=layer-956935495`, then `entry.21: Undo` (`edits → 2`), then `entry.22: Redo` (`edits → 3`).
B3f's row for 🖨️raster was `redone:false, faultLines:3` — the three fault lines were the
`RasterOwnedMap` `Drop` panic at `🦀️.rs:301` and the two `redo refused: dispatch-failed` lines that
followed it. **All three are gone.** PB1 §1.2's `ReplayProjection` carrier at the fold and §1.3's
removal of raster's blanket populated-map refusal are now proven at runtime, not only by the native
law.

**The brief's four-press walk, separately** (`🐍️pb3-undo-redo-undo.mjs`, keyboard routes only —
raster publishes no `action.undo` row and the History panel's own buttons share a dock with the app
panels, so both answer *"Element is not visible"*; `🗑️generated/pb3-undo-redo-undo-raster.txt`):

```
SUMMARY {"variant":"raster","painted":true,
         "walk":["undo:2->1:true","redo:1->2:true","undo2:2->1:true"],
         "allOk":true,"faultLines":0}
```

paint (`entry.7: Add Layer↶`) → `entry.8: Undo` → `entry.9: Redo` → `entry.10: Undo`. The second
undo is the press the shared bar never makes, and it is the one that re-enters the tail cache after
a redo has re-folded — clean, zero faults.

*(The first run of that script scored `painted: false` because the baseline witness was taken after
the row press; the baseline read now precedes the press — `🐍️pb3-undo-redo-undo.mjs:80`. Both runs
agree on the three history presses.)*

## 2. 🏗️fem3d — PB1's "press it with a node selected" does NOT clear the bar

### 2.1 What the brief asked, and why the first half of it is structurally impossible

PB1 §2 handed on: *"press `addSupport` with a node selected (the probe currently does not), and the
five-clause bar should clear — the lane under it is proven."* **A node cannot be selected into that
press.** `addSupport` is declared as a bare `.mutation(…)` with **no `action_args` at all**
(`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs:1309`) and its command builder reads
`text("nodeId").unwrap_or_default()` (`:1037`) — the args map is the only source, and nothing in the
dispatch chain injects the window's selection into it. So a rail press of `addSupport` always sends
`node_id: ""`, however much is selected, and `action.selectAll` in a `setup` step cannot change that.
(Compare `addAreaLoad`, which declares `solidId` as a required text arg — the same verb shape done
with a settable argument.)

So the brief's second option is the only live route, and the probe needs **no edit**: the shared
`🐍️b3d-bar-probe.mjs` already stages args, so the verb is changed at the call site.

### 2.2 Re-measured with `addNode`, a verb that really moves the document

`SEMIO_PROBE_SETTLE_MS=60000 bun 🐍️b3d-bar-probe.mjs fem3d fem3d 6087 addNode x=3.5 y=4.5 z=2.5`
(`🗑️generated/pb3-fem3d-addnode-bar.txt`, `🗑️generated/b3a-fem3d-console.txt`):

```
SUMMARY {"ready":"fem3d","error":null,"exampleRendered":true,"actionCount":34,
         "mutated":true,"undone":false,"redone":false,
         "panelRoundTrip":true,"faultLines":0,"interactionBar":false}
```

The verb now lands a **real** document mutation — `entry.8: create-node node=node id=n16 x=3.5
y=4.5 z=2.5↶`, `edits 0 → 1`, and the `↶` revert affordance is rendered, i.e. the framework itself
marks the entry undoable. **And the undo still does not move it.** Every route answers `ok` and the
edit count is frozen at 1 for the full 60 s budget with zero refusals anywhere in the console.

**This settles the open question between PB1 §2 and B3f §B3f.6 in B3f's favour: fem3d's undo IS a
live defect, and it is not an artefact of the no-op verb.** PB1's native laws are green and its
reasoning about `node_id: ""` is correct as far as it goes — it just does not explain this run,
where the document genuinely changed.

### 2.3 All three undo routes, measured separately

`🐍️b3d-fem-undo-diagnose.mjs fem3d 6087 addNode x=1.5 y=2.5 z=3.5`
(`🗑️generated/pb3-fem3d-undo-routes.txt`), on a document with `Check In (1)` and
`entry.2 = create-node … ↶` plus its own `entry.2.revert` control:

| route | result |
|---|---|
| A — the active window's rail `action.undo` | `clicked: ok`, `Check In (1)` unchanged after 12 s, **no console line of any kind** |
| B — `framework.history.undo` | `Element is not visible` — the History panel is behind the app panels in that dock |
| C — the ledger row's own `entry.2.revert` | `Element is not visible` — same dock |

So the only route that is even reachable reports success and produces **nothing**: not a refusal, not
a fault, not a `[DEBUG]` line. B and C could not be scored at all because of the docked-panel
occlusion that §4 fixes.

### 2.4 The one live signal: a `live_visual` reconcile that re-spawns forever on an unchanged revision

The whole console for the undo window is this, every ~4–15 s, unprompted:

```
[DEBUG] fem3d live_visual reconcile spawn: instance=1 revision=13329906518788982839 generation=2
        shell=1 job=0xf3d0000000000005 meshItems=149 drawInstances=146 cancelled=0
… job=…0006 … job=…0007   (same revision, same generation, same mesh counts)
```

`cancelled` is `effects.len() - 1`, so `cancelled=0` means `registry.current[slot]` was **`None`**
when the reconcile ran (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/✏️editor/🧵️session/🦀️.rs:3673`,
`:3691`): the previous mounted shell had already gone, the tracker still reports work for a revision
it has already meshed, and it mounts a fresh shell and spawns a fresh job for it — forever. That is
the memory topic *Reconcile Tracker More-Work Spin* on a live guest, and it is the only thing
fem3d's guest is doing while the undo presses arrive. 🏗️fem2d, the SAME component binary and the
same two-window shape, clears the bar 5/5 (B3f §B3f.6) and does not publish this spin.

**Honest boundary.** That a never-settling reconcile spin starves the reserved-job ladder that
`undo` travels is the hypothesis the measurement supports (an accepted press, no guest-side line,
and a job counter climbing on an unchanged revision); it is **not proven**, because proving it means
instrumenting the fem3d guest and rebuilding it. This slice did not rebuild fem: the brief said
fem3d needed no rebuild, the premise that made that true is disproven above, and the remaining mutex
budget was committed to 🌊️flow (§3), which was a named root-fix rather than a new investigation.
The cut is now much narrower than B3f left it: **not "undo silently declines" but "fem3d's mounted
`live_visual` tracker never records a meshed revision as done, and the reserved-job lane that
carries undo produces no guest-side trace while it spins"**.

## 3. 🌊️flow — publication lanes and `addWidget`

### 3.1 The `Ui` lane is NOT a regression — flow's law was pinning a contract the framework does not have

PB1 §3 read `[Child, Ui, Terminal]` ≠ `[Child, Terminal]` as *"a publication-lane set that gained
`Ui`"*. Measured at the source, it is the opposite: **a `Ui` page is what the framework publishes for
every successful emit, and always has been in this tree.** `publish_mounted_typed_operation_unit`
sets `mounted.ui_pending = matches!(publication, ArtifactToolCompletionValue::Emit(Ok(_), _))`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27709`) and the publication ladder turns that
into one `TypedOperationResultLane::Ui` page carrying the emit's `ui_scope` before the terminal
witness (`:28032`). The framework's OWN contract fixture declares exactly that shape —
`"artifactPublications": 1, "uiPublications": 1, "uiScopes": 1, "terminalReceipts": 1`
(`🔌️plugin/🧫️fixtures/⏳️completion/🔣️.json:5`), asserted by
`🔌️plugin/🧪️tests/⏳️completion/🦀️.rs:303`.

**Flow is the only plugin in the tree that pins the exact lane sequence** (grep over every `.rs`:
🔱️trinity asserts only *"contains no `Fault`"*, 🖨️raster and 🧱️block assert the tool's declared
`ArtifactToolPublicationLane`, which is a different thing). So there was nothing to root-fix in the
product; the two laws encoded a sequence one page short. Both now assert
`[Child, Ui, Terminal]` with the reason and the two citations in the message, and both are **green**.

### 3.2 `addWidget`'s composed-child read works natively — including undo AND redo

With the lane assertion corrected, `add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content`
runs to its end and passes: two `addWidget` gestures each publish ONE acknowledged child group, the
parent's `content` coordinate is byte-identical before and after, the child document gains exactly
`note_2` at (40, 40) and `note_3` at (50, 51), and **two undos walk it back node by node in reverse
insertion order**. `retained_add_widget_dispatches_one_acknowledged_child_group_and_retires` passes
with it. So `admitted_child`'s five conditions (PB1 §3) are all satisfiable natively — the `None`
PB1 measured live is a live-only composed-child availability problem, not a code cut.

`undo_restores_fixture_after_add_widget` was rewritten and its three document clauses now pass:
**apply → undo → redo, on the child document, through the full reserved-job ladder**
(`settle_registered_typed_operation` + `settle_history_verb`). Its old body was wrong in two
independent ways, both of which PB1's 4/9 reading attributed to `addWidget`:

- it probed `app.snapshot().to_host_snapshot().widgets.len()` — the PARENT's cached scene on the
  `content` handle (`🌊️flow/🦀️.rs:271`), the exact coordinate the sibling law asserts `addWidget`
  must NOT repoint. A `Child`-lane verb can never move that number, so `3, expected 4` was
  structural, not a failure of the command;
- it called `handle_action("undo"/"redo")` bare, which is a silent no-op for a
  `framework_reserved_job!` route (PB1 §1.1's own finding, applied here).

That green redo is the first native proof of PB2's `child_group_history_target` **redo** branch on
flow's child groups.

### 3.3 Native score 4/9 → 6/9, and what the last three are

`cargo test -p semio-s-artifact-flow-flow --lib -- add_widget`, private target dir per rule 25
(`🗑️generated/pb3-flow-addwidget-before.txt` → `…-after2.txt`):

| test | before | after |
|---|---|---|
| `add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content` | FAILED | **ok** |
| `retained_add_widget_dispatches_one_acknowledged_child_group_and_retires` | FAILED | **ok** |
| `undo_restores_fixture_after_add_widget` | FAILED (clause 1) | document clauses **ok**, fails only in `close_registered_fixture_app` |
| `rename_rejects_blank_unchanged_and_taken_ids` | FAILED (store `Drop`) | dispatch/lane clauses **ok**, fails only in the close |
| `patch_flow_widgets_parses_the_raw_value_string_into_the_slider` | FAILED (value 3.0) | now reaches the route; the publication never retires |
| the other four | ok | ok |

**The two named signatures are closed. The remaining three are one adjacent family, named honestly
rather than folded into the score:**

1. **A store reader outlives an operation that published no durable edit.** Both `undo_restores_…`
   and `rename_rejects_…` now fail at *"document store close awaits a retained reader or owner"* —
   `undo_restores_…` after a child-lane **redo**, `rename_rejects_…` after a **refused** graph
   operation, and the sibling law that publishes a durable child group closes cleanly on the same
   `flow_app()` fixture. Both leave an app that can no longer reach its terminal-empty witness. That
   is a product-side retention defect on the no-durable-edit paths, not a wrong assertion — and it
   sits next to the pre-existing debt `flow_app_closing()`'s own docstring records (*"a registered
   child member keeps the child snapshot disposer waiting on external ownership forever"*,
   `📓️flow-catalog-authority-2026-09-10.md` §7). Not fixed here: it is a second root, in the store's
   reader lifetime, and chasing it would have cost this slice its live measurements.
2. **`patchFlowWidgets` needs a window authority the unit fixture cannot mint.** Its route refuses at
   `context.view_state` (`✏️editor/🦀️.rs:937`, `flow-window-view-required`) — the law dispatched
   through the bare `meta("local")`, whose `view_state` is `None`, so it was measuring a refusal, not
   the parse. A new `flow_main_window_meta()` supplies one live `flow-main` instance and the route now
   accepts, but the publication then never retires inside the 30 s fixture budget: the direct-store
   lane wants a captured `ViewModel` window authority (`window_config` / `window_transient`) that this
   registry-only fixture does not provide. Named, with the meta helper left in place — it is the half
   of the fix that is certainly right.

## 4. Shared chrome — a docked panel must never overlay an interactive rail

### 4.1 Why the mechanism that already exists cannot cover this case

The shell already has a designed answer for "a chrome panel paints over in-window content":
`publishShellChromePanelBox` / `useChromePanelSafeArea` / `chromePanelSafeArea`
(`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:9340`, `:9398`, `:9443`). Every open `Panel`
publishes its measured box (`🖼️Panel/🟦️.tsx:439`), and exactly ONE affordance reads it — the
**folded** engagement's quick-action rail (`🪟️Window/🟦️.tsx:204`). The comment two lines above it
states the rest of the policy outright: *"Window pane toggles … stay on their authored anchors behind
anchored chrome panels"*.

**The expanded Actions rail cannot use it, and the rule itself says why.** `chromePanelSafeArea`
takes an axis only if the affordance can clear the panel *inside its host* — and on PB1's measured
geometry neither can: the rail is 300 × 901 in a 503 × 901 window, so the inline push (union right
1597 − rail left 1094 ≈ 503) exceeds the inline room (503 − 300 = 203), and the block push
(union bottom 996 − rail top 64 = 932) exceeds the block room (901 − 901 = 0). The function's own
docstring names the remedy: *"an axis that cannot clear within `host` is not taken at all … the
answer there is a re-anchoring, not a half-step"*. A full-height rail has nowhere to go; the LAYOUT
has to stop putting the window under the panel.

### 4.2 The fix — the canvas column reserves the docked band

`layoutPanelReserveStyle(panels)` (new, `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🟦️.tsx:106`)
answers the canvas column's own `paddingLeft` / `paddingRight` from the panel props Layout already
holds, and the column carries it (`:175`, now also tagged `data-slot="layout-canvas-column"`):

- a side reserves the **widest open panel anchored to it** — `top/bottom/left-middle` on the left,
  `top/bottom/right-middle` on the right; a `*-middle` anchor is centered and has no edge of its own,
  so it reserves nothing;
- a **folded** panel and an anchor with **no tabs** reserve nothing, so a shell with no open panel
  lays out byte-for-byte as authored (both sides answer `undefined`, not `0px`);
- the reserve is `calc(<size>px + 2 * var(--spacing-single))` — the panel's flush inset counted twice,
  once for the region edge it sits on and once as the gap to the canvas;
- the default width is now the exported `PANEL_DEFAULT_SIZE_PX` (`🖼️Panel/🟦️.tsx:198`), which `Panel`
  itself also destructures (`:400`), so the two cannot drift apart.

**Four vitest laws, green** (`🧱️elements/📐️Layout/🧪️tests/🧩️component/🟦️.tsx`, registered in the
ui-react vitest config `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts:30`;
`🗑️generated/pb3-layout-laws-verbose.txt`):

```
✓ reserves an open panel's own width on the side it is docked to
✓ reserves nothing for a folded panel, an empty anchor or a middle anchor with no edge of its own
✓ takes the widest open panel per side and defaults to the panel element's own size
✓ carries the reserve on the canvas column that hosts the windows, and reserves nothing without a panel
```

and the WHOLE `@semio-tech/ui-react` suite is green beside them — **809 passed / 29 files, exit=0**
(`🗑️generated/pb3-layout-laws.txt`), so nothing else in the shell moved.

*(The fourth law renders only the no-panel shell. Mounting a folded `Panel` in jsdom trips
`PanelTabButton` on the `📐️Layout → 🖼️Panel → 🎯️targets/⚛️react` import cycle — "Element type is
invalid … Check the render method of `PanelTabButton`" — which reproduces with the barrel import too
and is not this law's subject. The applied-reserve half is the three pure laws plus the live
measurement below.)*

### 4.3 Measured live on 📐️generation3d at 1600×1000, no rebuild

The change is TS-only, so FL1's vite serve on :6018 picked it up. Same probe, same panel state as
PB1 (`🐍️pb1-gutter-diagnose.mjs`, `🗑️generated/pb3-generation3d-gutter-after.txt`):

| | PB1 (before) | PB3 (after) |
|---|---|---|
| `clicked` on `BUTTON#kind` | `TimeoutError: click: Timeout 8000ms exceeded` | **`ok`** |
| the listbox it opens | `options: []` | **`["Neuron","Slider","Note","Preview"]`** |
| topmost element at the control's centre | `tree-gutter` of the History panel's own tree | **`select-value` → `select-trigger#kind`** |
| the control's rect | x 1231–1391, under the dock from 1297 | x 453–613, the window re-tiled inside the reserved column |

**And the app clears the five-clause bar with that very verb** — `addWidget` is generation3d's
staged-argument command, the one FL1 and PB1 could never dispatch
(`🗑️generated/pb3-generation3d-bar.txt`):

```
SUMMARY {"ready":"generation3d","error":null,"exampleRendered":true,"actionCount":40,
         "mutated":true,"undone":true,"redone":true,"panelRoundTrip":true,
         "faultLines":0,"interactionBar":true}
```

`filled: ["kind=Note"], submitted: ok` → `entry.9: create-widget index=7 input-note id=note_2 text=""`,
`edits 0 → 1`, then undo and redo both land, zero fault lines.

### 3.4 The live bar on :6216 — baseline taken, re-measure queued behind the fleet mutex

**Baseline, before any rebuild** (`🗑️generated/pb3-flow-bar-before.txt`,
`🗑️generated/b3a-pb3-flow-before-console.txt`), the shared probe with flow's own staged-argument verb
`addWidget kind=Note`:

```
SUMMARY {"ready":"flow","exampleRendered":true,"actionCount":27,
         "mutated":false,"undone":false,"redone":false,"faultLines":1,"interactionBar":false}
```

with the one fault line being exactly PB1 §3's refusal, and it names the window:

```
input #9 addWidget refused: dispatch-failed (user window=flow-main)
  — typed-operation failed: retained command work refused the command before any capacity was measured
```

`user window=flow-main` — **not** `flow-compiled-dag` as PB1 §3 recorded. The staged argument itself
is fine (`filled: ["kind=Note"], submitted: ok`); the refusal is `extent → None`, i.e.
`admitted_child`'s `?` (`✏️editor/🦀️.rs:1439-1445`).

**One activation later, on a guest built from today's whole tree** (`📜️pb3-activate.sh flow` through
the mutex, granted 18:21, **exit=0 at 18:28**, 6 m 17 s, 29 of 34 tasks rebuilt), the identical
summary and the identical fault line. So the live refusal is **not** staleness, and PB1's store fix
and PB2's child-lane projection reaching the guest do not move it.

### 3.5 Root cause of the live refusal: 🌊️flow never derives its `content` child

`ArtifactEditor::genesis_child_pack` is the framework's one hook for *"the initial pack for one
composed child the parent snapshot declares but no archive member carries — the content-addressed
half a parent derives from its own state"*
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11647`), and its default is `None`.
🪵️sourcing implements it for its catalogue child (`🗂️curation/🦀️.rs:250`, wired at `👁️viewer/🦀️.rs:68`
and `✏️editor/🦀️.rs:967`). **Flow implemented it nowhere** — grep over the whole plugin returns
nothing — so in a live shell the `content` child is never composed, `context.children.dialect("content", …)`
answers `None`, and `admitted_child` returns at its second `?` before any capacity is measured. The
native laws never saw it because the unit fixture registered the child **by hand**
(`register_content_child`, `🧪️tests/🔬️unit/🦀️.rs:92`). That is the whole live/native gap in one line.

**Fixed at the root** (3 product files):

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:158` — new `flow_genesis_content_pack`, which derives
  the child's pack from the working scene the parent already caches on its own content handle. It
  borrows the cached `Arc<FlowWorkingScene>` rather than cloning it, so nothing has to be retired
  (the fixture's hand-rolled version had to retire a whole `FlowHostSnapshot`).
- `…/✳️any/✏️editor/🦀️.rs:2359` and `…/✳️any/👁️viewer/🦀️.rs:103` — `genesis_child_pack` forwards to it,
  the same shape as sourcing's two.
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `register_content_child` **deleted**, with its call site. It is
  no longer a fixture concern: the runtime composes the child. The proof that the hook works is the
  failure the deletion fixed — with the hook in and the hand registration still there, every
  child-using law died on `interactive-job.child-member-duplicate: fixed child-member identity is
  already occupied`, i.e. the runtime had already composed it.

**No regression, measured rather than assumed.** The whole `semio-s-artifact-flow-flow --lib` suite
was run with the pre-change shape restored by hand (hook removed, `register_content_child` back) and
then again with the fix:

```
baseline (no genesis hook, fixture registers by hand):  172 passed; 74 failed   🗑️generated/pb3-flow-suite-baseline.txt
after    (genesis hook, no hand registration):          172 passed; 74 failed   🗑️generated/pb3-flow-suite-after.txt
```

identical, and the `add_widget` filter stays at 6/9 with both named laws green. (74 pre-existing
failures, dominated by 28 × *"artifact store reached Drop without its exact terminal-empty
shallow-shell witness"* — the fixture-disposal family B3e and PB1 documented for 🖨️raster, untouched
by this slice and unrelated to child composition, as the identical baseline shows.)

### 3.6 Live after the fix — the refusal is gone and the whole history walk lands

Second activation through the mutex (granted 19:08, **exit=0 at 19:15**), same probe, same serve
(`🗑️generated/pb3-flow-bar-after.txt`, `🗑️generated/b3a-pb3-flow-console.txt`):

```
SUMMARY {"ready":"flow","error":null,"exampleRendered":true,"actionCount":27,
         "mutated":false,"undone":false,"redone":false,"panelRoundTrip":false,
         "faultLines":0,"interactionBar":false}
```

`faultLines: 1 → 0`: **the refusal is gone**, `addWidget` is admitted, the typed operation runs to
completion (`typed-operation slots instance=1 live=1/64 → 0/64`), and the ledger carries the whole
walk:

```
framework.history.entry.9:  Add Widget↶     ← applied and revertible
framework.history.entry.10: Add Widget
framework.history.entry.12: Undo↶           ← after the undo press
framework.history.entry.14: Redo↶           ← after the redo press
```

So on the live shell the composed child now exists, `admitted_child` resolves, the widget is added,
and **undo and redo both land** — the first runtime proof of PB2's `child_group_history_target` on
flow's child groups, in both directions.

**What still does not move, and why the bar reads 0/5 anyway.** `#s-checkin` stays at `edits 0`
across all three presses, and the artifact panel keeps exactly its three rows. Both read the PARENT:
the check-in count is the parent store's uncommitted-edit count, and the panel renders from
`to_host_snapshot()`, i.e. the working scene cached on the parent's `content` handle — the coordinate
`addWidget` is required not to repoint (§3.2). The shared probe scores `mutated`/`undone`/`redone`
from that count, so it cannot see a child-lane gesture at all.

**That is the next cut, and it is now exactly named**: a `Child`-lane edit is journalled and
revertible but invisible to the parent's edit count and to flow's own rendered projection.
PB2's `child_history_tails` already teaches `can_undo`/`can_redo`/`revertible` to read the children
(`🔌️plugin/🦀️.rs:24475`) — the uncommitted-edit count and flow's document/panel projection have not
had the same treatment. Not attempted here: it is a third root (shell witness + plugin projection),
and this slice's two mutex slots were spent proving the first two.

## Files changed

**Product source (3 files, one root fix — the shared panel/rail chrome; every file is shared with
peers holding their own uncommitted edits, so this is only this slice's own change):**

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🟦️.tsx` — `LAYOUT_LEFT_ANCHORS` /
  `LAYOUT_RIGHT_ANCHORS` and `layoutPanelReserveStyle` (+33), applied to the canvas column, which now
  also carries `data-slot="layout-canvas-column"` (§4.2).
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx` — `PANEL_DEFAULT_SIZE_PX` exported (+4) and
  `Panel`'s own `size` default reads it, so the panel and the reserve cannot drift.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — `layoutPanelReserveStyle` re-exported beside
  `Layout` (2 lines).

**Tests (4 files, 4 new laws + 5 corrected ones):**

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🧪️tests/🧩️component/🟦️.tsx` — **new**, the four
  panel-reserve laws (§4.2).
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` — that file registered in the
  ui-react vitest `include` (1 line).
- `✏️s/🔌️plugins/🌊️flow/…/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the `[Child, Ui, Terminal]` lane
  assertion, and `undo_restores_fixture_after_add_widget` rewritten onto the child document through
  the reserved-job ladder, with the new `flow_child_node_count` helper (§3.2).
- `✏️s/🔌️plugins/🌊️flow/…/🎮️commands/➕️add-widget/🧪️tests/🔬️unit/🦀️.rs` — the same lane assertion, the
  undo loop moved onto `settle_history_verb`, the new `flow_main_window_meta()` helper, a stronger
  "publishes no durable lane" clause for `rename`, and the settle/close steps (§3.1, §3.3).

**No fem, raster or flow product source was changed by this slice** — §1 is PB1's code measured live,
§2 is measurement only, §3 is laws plus one live baseline.

**Ticket folder (new):** `📜️pb3-activate.sh`, `🐍️pb3-undo-redo-undo.mjs`. Captures:
`🗑️generated/pb3-*`, plus the shared probe's own `🗑️generated/b3a-pb3-*`. **No `🗑️generated` file this
slice did not create was removed**, `🗑️generated` itself was not swept, and `📌️important.md` /
`🎫️ticket.json` were not touched.

## Servers and processes

**No server started and none killed.** Every serve was inherited and reused as it stood: 6060 raster
(pid 98222), 6086 fem2d (823), 6087 fem3d (851), 6216 flow (65151), 6018 generation3d (16450). The
rebuilt raster guest was picked up by :6060 without a restart, and the TS-only chrome fix by :6018.

**Processes this slice started:** two detached activations through the fleet mutex
(`📜️pb3-activate.sh`, pids 16112 raster / 26684 flow) and three `cargo test -p
semio-s-artifact-flow-flow --lib` runs with the private uplift dir of rule 25
(`⚡️cache/cargo/target-pb3`), one at a time. Nothing of a peer's was stopped.

**The fleet wasm mutex**, by pid, all inherited state left alone:

| time (UTC) | event |
|---|---|
| 17:34:31 | mutex **FREE**, granted to `pb3` immediately; `📜️pb3-activate.sh raster` |
| 17:37:49 | raster **exit=0** after 3 m 18 s (18 nx tasks, mostly B3f cache hits); lock released |
| 17:36 | `gm1` took the lock |
| 17:39 | `a3` queued |
| 17:56:11 | `pb3` queued `flow` — third in line |

`gm1` has held it since 17:36 with `pgrep rustc` = 2 on the whole machine. That is not rule 27(b)'s
signature for MY process and it is a peer's, so nothing was touched (rule 15); reported here for its
owner.

## Honest gaps, in priority order

1. **🏗️fem3d does not undo, and it is a real live defect** (§2) — PB1's handoff (*"press it with a
   node selected and the bar clears"*) is disproven: the press cannot carry a node at all, and with
   `addNode`, which does move the document, undo still does nothing on any of its three routes. The
   guest's only live signal is a `live_visual` reconcile that re-spawns forever on an unchanged
   revision. Narrowed, not fixed; fixing it needs a fem wasm rebuild, which this slice spent on
   🖨️raster and 🌊️flow.
2. **🌊️flow does not clear the bar** (§3.6) even though its document verb now works end to end: a
   `Child`-lane edit is journalled and revertible (`Add Widget↶ / Undo↶ / Redo↶`, zero faults) but
   the parent's uncommitted-edit count (`#s-checkin`) and flow's own rendered projection never see
   it, so every witness the shared probe reads stays flat. Third root, named with its evidence, not
   attempted.
3. **The flow clause itself** — *"evaluate a graph using nodes from three different extensions"* —
   was not attempted: it sits behind the `addWidget` refusal, which had to be root-caused first, and
   behind the child-lane visibility cut above.
4. **Three flow laws still red** (§3.3): two on *"document store close awaits a retained reader or
   owner"* after an operation that published no durable edit (a child-lane redo, a refused rename),
   one on `patchFlowWidgets` needing a captured window authority the unit fixture cannot mint. Both
   are second roots, named with their evidence.
5. **The panel reserve is proven on 📐️generation3d only** (§4.3). It is shell-wide by construction
   and the whole ui-react suite is green (809/809), but no second app was re-probed with a right- or
   left-docked panel open.
6. **The raster crate's own suite** is unchanged by this slice and remains broadly red for the
   fixture-ownership reasons PB1 §"Regression evidence" documents; §1 measures the live bar, not
   that suite.
