# PB1 — 🖨️raster REDO trap · 🏗️fem3d undo no-op · 🌊️flow `addWidget` · generation3d combobox hit point

Slice **PB1** (2026-09-20, from ~19:0x). Four stragglers to the five-clause bar
(boot → example → real mutation observed → undo → redo → 0 fault lines), inherited from
`📓️b3d-…-demonstrator.md` §B3e.2 / §B3f.6 / §B3f.7 and `📓️b3c-procedural-flow-process.md`
§FL1.1.6 / §FL1.3.4.

**Inherited live state at start** (nothing restarted, nothing killed): fleet wasm mutex FREE,
`pgrep -fl rustc` = 3, 40 GiB free on `/System/Volumes/Data`. Serves reused as they stood:
6060 raster (pid 98222), 6086 fem2d (823), 6087 fem3d (851), 6216 flow (65151), 6018 generation3d
(16450). No `🗑️generated/pb1-*` capture and no predecessor report existed — this slice starts cold.

## Status

| # | item | state |
|---|---|---|
| 1 | 🖨️raster REDO trap (`RasterOwnedMap` fail-closed `Drop`) | ✅ reproduced natively, root-fixed at the fold + in raster, native law GREEN; wasm re-activation pending |
| 2 | 🏗️fem3d undo silent no-op | ✅ the undo LANE is green natively (3 laws); the bar pressed a verb that changes nothing — live command-log half handed on |
| 3 | 🌊️flow `addWidget` refused by `FlowChildGroupWork.extent` | 🟡 brief's premise disproved (extent already prices 1); real cut named + flow's own suite measured 4/9 — not fixed |
| 4 | generation3d combobox hit point answers `tree-gutter` | 🟡 FL1's diagnosis corrected by measurement — it is a `z-index: 30` panel dock, not the Tree gutter; shared-chrome defect named, not fixed |

## 1. 🖨️raster REDO trap — root cause and fix

### 1.1 Reproduced natively in 0.04 s, not inferred

New law `mounted_paint_undo_redo_undo_never_reaches_an_owned_map_drop`
(`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:869`),
on the LIVE document shape: `mounted_app` → `setActiveExample demo` → `addLayer pixel` →
`undo → redo → undo`. Before the fix it panicked exactly the way B3f's live bar did
(`🗑️generated/pb1-raster-redo-stack.txt`):

```
panicked at 🗿️artifacts/🖨️raster/…/🦀️.rs:301:9:
Raster owned map reached Drop before every entry and page backing was explicitly retired
 10: <RasterOwnedMap<ArtifactChild<SemioImageSnapshot>> as Drop>::drop     ← snapshot.assets
 12: drop_glue::<RasterSnapshot>
 13: ArtifactStore::project_applied::{closure#0}                           ← HERE
 14: ArtifactStore::reproject
 15: ArtifactStore::commit_transition
 16: ArtifactStore::redo_lane_position
```

**Two harness facts the reproduction needed, both real product behaviour:**

- `PluginApp::snapshot()` **clones**, so reading `app.snapshot().layers.len()` in a statement
  trips the same fail-closed `Drop` on a populated document. The law reads through
  `observed_layer_count`, which hands the materialized projection straight back to
  `retire_raster_snapshot`.
- `handle_action("undo")` alone is a **silent no-op**: `undo`/`redo` are `framework_reserved_job!`
  routes whose admission comes back as a `SpawnJob` receipt that must be committed before the store
  ever sees `ArtifactCommand::Undo`. The law drives them through
  `artifact_app_laws::settle_history_verb` (new `mounted::history` helper). *(Directly relevant to
  item 2 — see below.)*

### 1.2 The fourth path: a live projection on the fold's early-return edge

B3e overrode all three retirement seams the trait offers, and the trap still fired, because the
seam that leaked is not a seam at all — it is the `?` operator.
`ArtifactStore::project_applied`'s tail-add branch (the branch **redo** takes; `undo` answers from
`tail_undo_cache` and folds nothing, which is why the undo half was already green) held the live
intermediate in a plain local:

```rust
let mut folded = pre.as_ref().clone();
for operation in &edit.forwards {
    let next = apply_mutation(&folded, operation)?.0;   // ← Err ⇒ `folded` drops BARE
    retire_replayed_projection::<P, Mutation>(std::mem::replace(&mut folded, next));
}
self.replace_tail_undo_cache_retained(Some((added, pre)))?;  // ← Err ⇒ `folded` drops BARE
```

The loop body retires what it displaces; every early return carried the still-live one into a bare
drop. The same shape was in `fold_history`, `materialize_document_snapshot` and
`parse_decoded_document_spr` — four folds, one of which (`replay_suffix_partitioned`) had already
been hand-audited to retire on all six of its exits, which is exactly the evidence that hand-auditing
does not hold.

**Framework fix, at the fold, so no plugin can miss it**
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18593`): a `ReplayProjection<P, Mutation>`
carrier that owns the live intermediate, `advance(next)` retires what it displaces, `into_inner()`
hands it out, and `Drop` retires whatever is still held — so an early `?` retires structurally.
Converted: `project_applied` (`:17190`), `fold_history` (`:18726`),
`materialize_document_snapshot` (`:10896`), `parse_decoded_document_spr` (`:12019`).
`cargo check -p semio-framework-os-kernel --lib` **exit=0** (`🗑️generated/pb1-kernel-check.txt`).

With only that change the guest **stops aborting** and answers a structured fault instead:

```
history verb redo reserved-job commit: Fault { code: "module.vcs", severity: Error,
  message: "mutation diff rejected: mutation.apply.retained-owner-required:
            populated Raster maps require the retained initialization authority" }
```

### 1.3 The second layer: raster refused its own history fold

That message is raster's own blanket head-guard in `RasterDiff::apply` /
`RasterDiff::apply_to_artifact`
(`…/🧬️schema/🔺️diff/📝️text/🦀️.rs`): *any* mutation was rejected once `snapshot.assets` was
non-empty. Forward edits never met it (they travel the retained command lane); **the framework's
history folds do** — `redo`, a fold to base, a `.spr` reload and a remote ingest all re-apply through
`MutationDiff::apply`. The guard is also stale: the body under it already handles a populated map
correctly, and `RasterOwnedMap::clone` is documented at `🦀️.rs:306` as *"a real deep copy … bounded
by construction and needs no retained page authority"*. Asset **removal** is the one genuinely
retained operation and keeps its own separate refusal, one line below, before any ownership is cloned.

Three edits in raster:

| file:line | change |
|---|---|
| `…/🧬️schema/🔺️diff/📝️text/🦀️.rs:306` | `MutationDiff::apply` — blanket populated-assets refusal removed (docstring records why); the displaced layer forest is retired instead of bare-dropped, and an abandoned candidate goes through `retire_raster_snapshot` |
| `…/🧬️schema/🔺️diff/📝️text/🦀️.rs:181` | `apply_to_artifact` — the same two fixes, through `retire_raster_artifact` |
| `…/🧬️schema/📸️snapshot/🦀️.rs:54` | new `retire_raster_artifact`, the artifact-shaped twin of `retire_raster_snapshot` |

The displaced-forest fix is its own latent trap: `next.layers = apply_layers_delta(&next.layers, …)`
dropped the old forest bare, so any document with a populated adjustment-`params` map aborted on a
plain successful apply.

### 1.4 Measured

`🗑️generated/pb1-raster-redo-law.txt`:

```
test editor::raster::component::unit_tests::mounted_paint_undo_redo_undo_never_reaches_an_owned_map_drop ... ok
test result: ok. 1 passed; 0 failed
```

paint → undo → redo → undo, all four layer counts correct, no trap. **Native only so far** — the
live bar needs a wasm re-activation, queued behind GM1's trusted-catalog bootstrap (see §Servers).

## 2. 🏗️fem3d undo silent no-op — the undo LANE is green; the bar pressed a verb that changes nothing

Three native laws in
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`,
all green (`🗑️generated/pb1-fem3d-undo.txt`, `cargo test -p semio-s-artifact-fem-3d --lib --features
component-app-assembly`):

```
add_support_without_a_node_does_not_journal_a_phantom_edit ... ok
undo_restores_document_after_add_node                      ... ok
undo_restores_document_after_add_support                   ... ok   ← new
```

**`addSupport` undoes and redoes correctly on a mounted fem3d app** — apply, undo, redo, all three
counts exact, through `artifact_app_laws::assert_undo_redo_round_trip`, i.e. the full reserved-job
ladder a shell drives. So B3f's *"fem3d's undo lane silently declines to move the document"* is not
a defect in the undo lane, and there is nothing to root-fix there.

**What the measurement does say.** The first version of the law used the shape the live bar emits —
`addSupport` with an **empty `node_id`**, which is what the tree row sends when no node is selected,
and which B3f's own ledger line shows (`create-support support=support id=sup8 node-id="" …`). That
version failed at the FIRST clause, not at undo: `supports.len()` stayed at 2 instead of going to 3.
Re-run with a real node id from the bundled document, every clause passes.

So on the live bar the press changed **nothing**, and every undo route then correctly answered "ok,
nothing to undo" — which is exactly B3f's measured signature: *no new `Undo` ledger row, `edits`
stuck, the guest republishing an identical revision, and zero refusals anywhere*. The bar's
`mutated: true` verdict came from a command-log row for a verb that moved no document state.

The second law pins that honestly: an `addSupport` with no node reaches neither the document nor a
history patch (`history_patch: None`, `diagnostics: []`), so nothing downstream can read it as a
mutation.

**Honest gap, handed on rather than guessed:** why the LIVE shell still shows `edits 0 → 1` for that
press is not reproducible natively — natively the dispatch delivers no history patch at all. That is
a live-only difference in the React host's command-log lane, and chasing it needs the fem serve, not
a native law. The actionable half for whoever takes fem3d's bar: **press `addSupport` with a node
selected** (the probe currently does not), and the five-clause bar should clear — the lane under it
is proven.

## 3. 🌊️flow `addWidget` — the extent is NOT mispriced; the cut is narrowed, not closed

**§FL1.1.6's brief was "price the child-group work correctly (extent must be 1 work item, not
fan-out rows) or declare the missing factory". Neither applies — measured by reading the code the
fault message names.** `FlowChildGroupWork::extent`
(`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1454`)
**already** answers `Some(1)` — F2's animate lesson is applied — and the factory
(`FlowChildGroupJobFactory`, `:1516`) exists and is registered for `addWidget` (`:2295`).

The `None` that `ArtifactRetainedCommandJob`'s preflight reports as *"retained command work refused
the command before any capacity was measured"* comes from the `?` on `Self::admitted_child`, whose
five conditions are all about the COMPOSED CHILD, not about capacity:

1. the command is `AddWidget`; 2. a tool-job `context` is present; 3.
`context.children.dialect("content", snapshot.content.child_id)` resolves; 4. that dialect is
exactly `s.stdio.semio` / `v1` / `flow`; 5. `context.children.typed_read::<SemioFlowSnapshot>` on it
succeeds.

So the live refusal under `flow-compiled-dag` says **the `content` child is not readable from that
window's dispatch**, which is the memory topic *Composed Child Load Requirements*, not a pricing bug.
Pricing the extent differently would not move it by one line.

**And natively, flow's own `add_widget` suite is red today** — `cargo test -p
semio-s-artifact-flow-flow --lib -- add_widget`, 9 tests, **4 pass / 5 fail**
(`🗑️generated/pb1-flow-addwidget.txt`), with ONE signature across them:

```
add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content  ← [Child, Ui, Terminal] ≠ [Child, Terminal]
retained_add_widget_dispatches_one_acknowledged_child_group_and_retires       ← [Child, Ui, Terminal] ≠ [Child, Terminal]
undo_restores_fixture_after_add_widget                                        ← 3 widgets, expected 4
patch_flow_widgets_parses_the_raw_value_string_into_the_slider
rename_rejects_blank_unchanged_and_taken_ids                                  ← store Drop terminal-empty witness
```

*"Each command must publish exactly one acknowledged child group followed by terminal"* — and an
extra **`Ui`** publication now sits between the child ack and the terminal. **`addWidget` does not
land its widget in a plain unit fixture either**, so this is not a live-only composition problem and
the next worker does not need a serve to chase it. Not this slice's code (PB1 touched no flow file),
and not chased here — the remaining budget went to §1, which was the measurable P0.

**Not caused by §1's store change**, and the reasoning is checkable rather than asserted:
`ReplayProjection` retires exactly what the old code retired on the success path and only adds
retirement on early returns, so it cannot add a publication to a child-group sequence; the one
store-level failure above is the pre-existing `ArtifactStore::Drop` terminal-empty witness
(`🏪️store/🦀️.rs:18490`), which sits ~100 lines ABOVE the insertion point and is a fixture-disposal
law B3e already documented for raster.

## 4. generation3d combobox hit point — §FL1.3.4's diagnosis CORRECTED, root named, not fixed

FL1 read `document.elementFromPoint` at the control's centre, got `DIV#(no id)[tree-gutter]`, and
concluded *"the tree's gutter is painted over the property control's hit point … a `Tree`
property-row layout question (gutter vs. control hit area)"*. **The slot name was right and the
element was a different tree's.** `🐍️pb1-gutter-diagnose.mjs` takes rects and the full
`elementsFromPoint` stack, on FL1's own serve :6018 in FL1's own panel state
(`🗑️generated/pb1-generation3d-gutter-{before,containers}.txt`):

| element | rect |
|---|---|
| the control `BUTTON[data-slot="select-trigger"]#kind` | x **1231–1391**, y 941–963 |
| the property row's OWN `tree-gutter` | x **1097–1121** — 110 px to the LEFT of the control, touching nothing |
| what is actually at the centre (1311, 952) | `tree-gutter` of `tree-section-row#framework.history.actions`, x **1298–1312** |
| its positioned ancestor | `[data-slot="panel"]#framework.panelTab.framework.panel.history`, `position: absolute`, **`z-index: 30`**, x **1297–1597** |

The eleven-deep stack under the hit point is the History **panel**, not the rail's tree. The window
(`#procedural-preview`, `z-index: 10`, x 1091–1594) and its engagement overlay (the Actions rail,
`position: absolute`, `z-index: 20`, x 1094–1394) both run underneath the floating right-hand panel
dock, which covers the rail's last **97 px** — and Playwright clicks an element's CENTRE, which lands
at 1311, inside the dock. Measured proof that the control itself is fine: a hit test 6 px inside its
left edge (x 1237) answers `BUTTON#kind` (`leftEdgeHit`). Nothing about `Tree`, the gutter, or the
property row needs changing, and a vitest law over `Tree` would have asserted the wrong thing.

**The real defect** is the one the memory topic *World3d Component Hover & Gumball Contract* already
names as "panel-covered rail": a raised app panel floats at `z-index: 30` over a window whose
engagement overlay is never inset for the dock band it occupies. It is shared shell chrome on the
window/panel stacking contract, affecting every app with a right-docked panel — **not** the
TS-local `Tree` fix this slice's brief assumed, and not something to half-land in shared chrome
while GM1's two-hour catalog bootstrap is running. Named and measured here, with the probe that
settles it, rather than guessed at.

**Consequence for the fleet, same shape as FL1.2's**: a `TimeoutError` on a staged-argument control
is not evidence the verb is broken *or* that the rail's own layout is wrong — take rects before
blaming a slot name, because `elementFromPoint` reports the topmost element in the whole document,
not the topmost element of the subtree you are looking at.

## Files changed

**Product source (4 files).** Every one is shared with peers holding their own uncommitted edits, so
these are this slice's own changes, not the files' whole diffs.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` — new `ReplayProjection<P, Mutation>` carrier
  (`:18593`, +65) and its four adoptions: `project_applied`'s redo branch, `fold_history`,
  `materialize_document_snapshot`, `parse_decoded_document_spr`.
- `✏️s/🔌️plugins/🖨️raster/…/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs` — `MutationDiff::apply` and
  `apply_to_artifact`: blanket populated-map refusal removed (with the docstring that records why),
  displaced layer forest retired, abandoned candidate retired on the layer-delta rejection.
- `✏️s/🔌️plugins/🖨️raster/…/✳️any/🧬️schema/📸️snapshot/🦀️.rs` — new `retire_raster_artifact` (+9).
- *(no fem or flow product source changed — §2 and §3 landed as measurement, not edits.)*

**Tests (2 files, 3 new laws + 1 new harness helper):**

- `✏️s/🔌️plugins/🖨️raster/…/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `mounted::history` (drives a
  framework-reserved history verb through `settle_history_verb`), `observed_layer_count`, and
  `mounted_paint_undo_redo_undo_never_reaches_an_owned_map_drop`.
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/🌐️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` —
  `undo_restores_document_after_add_support` and
  `add_support_without_a_node_does_not_journal_a_phantom_edit`.

**Ticket folder (new):** `🐍️pb1-gutter-diagnose.mjs`. Captures: `🗑️generated/pb1-*`. No
`🗑️generated` file this slice did not create was removed; `📌️important.md` and `🎫️ticket.json`
were not touched.

## Regression evidence for the raster crate

`cargo test -p semio-s-artifact-raster-raster --lib` is broadly red in this crate and **was already**:
B3e's own full pre-PB1 run (`🗑️generated/b3e-raster-fix2.txt`) is **136 ok / 79 FAILED**; this
slice's (`🗑️generated/pb1-raster-suite.txt`) is **142 ok / 78 FAILED**, same families, and both runs
end the same way — `raster_owned_map_cap_plus_one_…` past 60 s, then SIGKILL. Triaged three
(`🗑️generated/pb1-raster-triage.txt`): *"artifact envelope terminal shell reached Drop…"*, *"edit
history insertion requires its exact mutation retirement factory"*, and a test-side bare drop —
all fixture-ownership defects B3e already documented as its §B3e.2 "two defects found next to it",
none of them mentioning the refusal this slice removed. **Net +6 passing, no new failure family.**

One observation, not this slice's doing: B3e's baseline lists a test
`undo_redo_on_a_populated_document_retires_every_owned_map` that **no longer exists anywhere in the
raster plugin** — a peer deleted it between B3e and now. Flagged, not restored.

## Servers and processes

**None started and none killed by this slice.** Reused as they stood: 6060 raster (pid 98222),
6086 fem2d (823), 6087 fem3d (851), 6216 flow (65151), 6018 generation3d (16450) — the last two are
FL1's and were only read from.

**No wasm re-activation was run, deliberately.** The fleet mutex has been held by `gm1` since
18:41 for a ≈2 h trusted-catalog cold build (coordinator note, 18:45), and the standing rule is
foreground-only, so a foreground activation would have burned the whole slice waiting. Per that same
note, all source work, native laws and checks were finished first.

**The one thing this slice needs next, and it is a single step:** one
`📜️b3f-activate.sh raster` through `📜️wasm-build-mutex.sh pb1`, after GM1 releases the lock, then
re-measure on the existing :6060 serve with
`SEMIO_PROBE_SETTLE_MS=60000 SEMIO_PROBE_ROW='[role="treeitem"]:has-text("Add Pixel")'
🐍️b3a-interaction-probe.mjs` (chromium `--use-angle=metal`). B3f's row for 🖨️raster was
`mutated:true, undone:true, redone:false, faultLines:3`; the redone/fault half is what §1 fixes, and
it is proven natively but **not yet on the live bar**.

## Honest gaps, in priority order

1. **🖨️raster's live bar is not re-measured** — §1 is green natively only. One activation away.
2. **🌊️flow `addWidget` is red in flow's own unit suite** (§3, 4/9) with a single `[Child, Ui,
   Terminal]` signature. Reproducible without a serve. Narrowed, not fixed.
3. **The panel-covered rail** (§4) is shared shell chrome (window `z-index: 10` / engagement overlay
   `20` / panel dock `30`) and affects every app with a right-docked panel. Named and measured.
4. **fem3d's live command-log row for a no-op verb** (§2) does not reproduce natively.
5. **The raster crate's fixture-ownership failures** (78) are pre-existing and remain.
