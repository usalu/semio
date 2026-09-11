# Wave B23 — the rendered document vs the mutated one: the family's one hop is unobservable by construction

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-B23, 2026-09-12. Brief: kill H1–H4 for the
selection-scoped family that is red on wasm `#48` (`🗑️generated/lanes-2026-09-12-48.txt`, FAULTS=0) —
`inspection-object-fields`, `locked-flag-row`, `locked-refusal-notice`, `gumball-scene-delta`,
`brush-preview-place`, `volume-brush-add-target-volume`, `relocate-pose-delta` — while `clipboard`,
`volume-brush-arm` and catalogue drag-drop pass.

No git write, ticket not opened/closed/reopened, `🗑️generated` written to and never deleted, **no wasm
build run**, every command foreground, the one temporary `[DEBUG] b23` host tap added, used and
**removed** (`rg -an "DEBUG\] b23"` over the repo → no match).

**Headline.** The family is not "the mutation never reaches the document it renders from". Every guest
half of it is green natively, including through the panel render route no previous law used (§3). What is
actually broken is that **the one hop the six previous waves kept guessing at cannot be seen from a
browser at all**: both witnesses the framework already owns for "a pick disappeared between the dispatch
and the render" go through `crate::plugin_runtime::debug_runtime_line`, which is gated on
`semio_framework_trace::runtime_diagnostics_enabled()` — an environment read that a jco-transpiled
component in a browser tab can never answer true. Measured, not reasoned: **zero** `debug_runtime_line`
records in the 4 000-line console of the `#48` `selection-surfaces` lane, against thousands of plain
`eprintln!` lines from the same guest in the same run (§2.3). And the more specific of the two witnesses is
additionally gated on `persisted != persisted_before`, so the exact failure mode "validation pruned the
pick to the same empty state that was already stored" emits nothing at any diagnostics level, anywhere.

Three deliverables, each proven red-then-green:

| # | what | where |
| --- | --- | --- |
| 1 | `interaction_selection_loss_v1` — the named predicate for the four ways a pick disappears, reported on a channel the browser actually shows, at the exact hop | `🔌️plugin/🦀️.rs` |
| 2 | `data-guest-selection-json` — the pane publishes the GUEST's own unmerged selection lane beside the merged one it paints, so "the guest lost the pick" and "the host is painting it for the guest" stop reading identically from outside | `🌐️World3dHost/🟦️.tsx` + its language-neutral fixture + the engine-contract law |
| 3 | `a_browser_shaped_pick_survives_every_render_route_and_both_mutating_dispatches` — the browser sequence natively, across BOTH host render routes, and the statement of exactly which hop the testkit short-circuits | `✏️editor/🧪️tests/🔬️unit/🦀️.rs` |

---

## 1 Hypothesis table — each killed with its evidence

| # | hypothesis | verdict | killing evidence |
| --- | --- | --- | --- |
| H1 | B4's `uiRefreshSectionUnchanged` skip keeps a stale body forever because the guest's reported hash is computed off a store the mutation does not touch | **DEAD** | The skip compares the host's cached hash against `surface.view.hash` — the hash of the RETAINED surface, i.e. of the tree the guest last PUSHED (`🔌️PluginRuntime/🟦️.tsx:1665`, `:1838`, `:1865`). It is downstream of the guest's render, not a substitute for it, so it can only skip a projection of a tree that really is unchanged. And the guest is asked to re-render regardless of any hash: `uiRefreshSurfaceEvents` (`:1610`) emits one `surface-visible` per requested window/panel/section unconditionally, and `Event::SurfaceVisible` (`⚛️reactor/🔄️turn/🦀️.rs:529`) mounts the surface and marks it dirty, which `plugin_render_surface` then re-renders. On the `#48` lane the host additionally omitted the cached hash outright — `[DEBUG] leftover Inspection refresh {"epoch":1,"selectedIds":["seed-left-001"],"hashBust":true,"cached":"5bd00904:1"}` — and the panel STILL rendered `puzzle3d-play-inspector.empty`. A hash skip cannot explain a body that is re-rendered with the hash deliberately busted. |
| H2 | two documents — the mutation lands on the typed half while `render_body`/the world lane read `puzzle3d_fixture_from_projection`, or the reverse | **DEAD** | The two halves are bridged symmetrically and the render reads the persisted projection directly. `Puzzle3dActionPrologue::scene_step` (`✏️editor/🦀️.rs:3324`) takes `before` from `puzzle3d_projection_value(snapshot.value())` and the mutated `after` from `scene_from_snapshot(snapshot.typed())`, but `puzzle3d_operations_from_fixture_change` (`:612`) runs BOTH through `puzzle3d_snapshot_from_fixture` before diffing, so the typed/projection asymmetry cancels. `render_body` (`:8171`) decodes `doc.snapshot.value()` — the projection — every call, and `app.geometry_jsons`'s memo key `main::fixture_geometry_fingerprint` (`🧊️main/🦀️.rs:261`) hashes the FULL JSON of `objects`/`references`/`target_volumes`/`meta`, so a pose, a `hidden` or a `locked` change busts it exactly like an added object does. There is no field-blind memo and no second document. |
| H3 | the mutations resolve `selected` from a leftover InteractionView that names vortex uuids / the wrong instance and silently act on an empty set | **REFINED, and it is the family's shape — but not its cause** | Correct that the whole family is selection-scoped and the passing lanes are not: `clipboard` (paste creates), `volume-brush-arm` (arms a utility) and catalogue drag-drop need no guest-side selection, while all seven failing verdicts need one. But the resolution itself is right: `Puzzle3dInteractionSnapshot::from_interaction` (`✏️editor/🦀️.rs:1036`) already falls back to `leftover_selected_ids()` when `selection("vortex")` is empty, and `inspection::selected_section` (`🔍️inspection/🦀️.rs:196`) falls through granularity to a bare id match against the fixture in two further arms. `.empty` therefore means `interaction.selected` is genuinely EMPTY at that render — not a granularity or uuid mismatch. |
| H4 | per-instance addressing — the mutation is applied to the window KIND's document while the panes render instance documents | **DEAD for this family** | The Inspection body is a PANEL: `inspection::render(&envelope, interaction, labels)` reads neither `wid` nor any window-owned partition, and the same is true of the selection half of `main::render`. §3's law renders the same pick through the panel route (`ViewModel::for_panel()`, no `window_id` at all) and the window-instance route on a two-pane session and gets the object fields from both. |

---

## 2 What the `#48` browser log actually says

All from `🗑️generated/probe-2026-09-11T19-11-07.md` (the `--only=selection-surfaces` lane of
`lanes-2026-09-12-48.txt`), read line by line rather than through the probe's own filtered tails.

### 2.1 The guest HAS the pick, and says so

```
warning: [DEBUG] leftover InteractionView {"selectedIds":["seed-left-001"],"publishedIds":["seed-left-001"],
                                           "locked":{"seed-left-001":false},"gumball":true,"hoverTarget":…}
```

That object is `leftover_interaction_view_from(&state, …)` (`🔌️plugin/🦀️.rs`), built inside
`dispatch_interaction_action` from the state the guest had just computed — one line after
`overlay_leftover_ids_into_vortex(&mut state)` and one line before
`self.interaction_leftover_selection = Some(state.clone())`. So at that instant the guest's own
in-memory leftover carries `vortex=object:seed-left-001`, and it keeps carrying it: every later hover
leftover in the run republishes the same ids.

### 2.2 The guest's interaction reaches the WORLD body's render, in the same window of time

The vortex-marker lane is gated on `Puzzle3dInteractionSnapshot::touches_object`, so its payload size is a
direct read-out of "is this render's interaction empty":

```
line 286  debug: [DEBUG] puzzle3d.vortices.publish utility= bytes=2    brush_or_volume=false
line 308  debug: [DEBUG] puzzle3d.vortices.publish utility= bytes=2640 brush_or_volume=false
line 331  warning: [DEBUG] leftover InteractionView {"selectedIds":["seed-left-001"],…}
line 345… debug: [DEBUG] puzzle3d.vortices.publish utility= bytes=2640 brush_or_volume=false   (×60)
```

`2 → 2640` is the guest's own render switching from "nothing marked" to "this object is marked", and it
never goes back for the rest of the lane. The same flip appears in §3's native law
(`vortices=2` before `select_id`, `vortices=2640` after). So the guest renders SOME body with a live
interaction while `puzzle3d-play-inspector.empty` is what the Inspection body publishes at
`[20.6s] selection inspection-wait populated=false empty=true id=null`.

That is the sharpest statement of the defect this ticket has had: it is **not** "the guest has no
selection". It is that two bodies of one instance disagree about it — and nothing in the system is
allowed to say so out loud (§2.3).

### 2.3 Both witnesses for that disagreement are off in a browser

| witness | site | channel | present in the `#48` console |
| --- | --- | --- | --- |
| `interaction selection lost dispatched=… validated=… readback=… domains=…` | `revalidate_and_persist_interaction_state` | `debug_runtime_line` | no |
| `interaction-store snapshot unavailable, selection read as empty: …` | `interaction_selection_snapshot` | `debug_runtime_line` | no |

`debug_runtime_line` (`🔌️plugin/🦀️.rs:30279`) is
`if semio_framework_trace::runtime_diagnostics_enabled() { eprintln!(…) }`, and for the guest's target
(`wasm32-wasip2`) `runtime_diagnostics_from_environment` (`⏱️trace/🦀️.rs:199`) reads the
`RUNTIME_DIAGNOSTICS_ENV` variable — which a component in a browser tab does not have. Control reading:
`[DEBUG] plugin_exchange entry`, a `debug_runtime_line` on the hottest path in the runtime, occurs
**0** times in that 4 000-line console, while the puzzle3d app's plain `eprintln!`s
(`puzzle3d.vortices.publish`, `puzzle3d.utility.publish`, `puzzle3d.brushPreview.lane`) occur in the
hundreds. The channel is not quiet — it is structurally off in exactly the place the defect lives.

Worse, the first witness is nested inside `if persisted != persisted_before`, so the failure mode
"`validate_state` pruned the pick back to the empty state already stored" produces no record at ANY
diagnostics setting. B6's own docstring cites this ticket's browser symptom as the reason that witness
exists; it could never have fired for it.

### 2.4 What is NOT wrong

- No `refreshUi dropped requested body …` in the run — the Inspection surface is retained and requested.
- No fault of any kind: `faults=0 hard=0 collateral=0 guest-death-faults=0`. Nine `error:` lines, all
  `contributions push refused empty pack` / `contributions document sources`, unrelated.
- The `#48` release wasm (`dist/release/…/semio_s_plugin_puzzle_component.core.wasm`, 20:35:09) does
  contain `interaction-store snapshot unavailable` — a string absent from `HEAD` — so the B6/B19
  `interaction_selection_snapshot` work is genuinely inside `#48`, and the symptom survives it.

---

## 3 The law, and the hop the testkit short-circuits

`✏️editor/🧪️tests/🔬️unit/🦀️.rs` →
`a_browser_shaped_pick_survives_every_render_route_and_both_mutating_dispatches`. It plays the browser's
sequence on the two-pane session the browser boots: render both panes, pick the object, read Inspection
through the **panel route** (`render_panel_body`, `ViewModel::for_panel()`, no `window_id` at all — the
projection a real Inspection refresh receives, and the one every previous Inspection law skipped by using
`render_body`) and through the **window-instance route**, then lock the selection through the
context-menu path (no explicit ids), then translate it and compare the world lane the pane publishes.

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly \
  --lib -- --test-threads=1 a_browser_shaped_pick
test editor::puzzle3d::component::tests::a_browser_shaped_pick_survives_every_render_route_and_both_mutating_dispatches ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 715 filtered out
```

Green — every row of the `#48` family passes natively, including across the two
`revalidate_interaction_state_after_document_change` passes the two document-intent dispatches run.

**The hop it cannot reach, stated exactly.** The browser renders a body only through
`Event::SurfaceVisible` → `plugin_mount_surface` → `SurfaceContexts` → `plugin_render_surface`, where the
`ViewModel` a body is rendered against is reconstructed by `SurfaceContexts::get`
(`⚛️reactor/🪟️surfaces/🦀️.rs:52`) from **one shared `view_state` field that every mount overwrites with
its own projection** — a panel's `get` therefore calls `for_panel()` on whatever view the last WINDOW
mount stored. Every testkit render helper (`render_body`, `render_panel_body`, `render_window_refresh`,
`render_window`, `render_composite`) instead calls `PluginApp::render` directly with a hand-built
`ViewModel`, so a body that is never re-rendered, re-rendered against another surface's retained view, or
deferred by `PATCHES.reserve_mounted` is invisible to every puzzle3d law that exists. That route is
`pub(crate)` to `semio-framework-plugin`, so the missing testkit shape cannot live in the puzzle3d crate:
it belongs beside `⚛️reactor/🪟️surfaces/🧪️tests/🪟️surface-context-lifecycle/🦀️.rs`, driving
mount(window A) → mount(window B) → mount(panel) → render(panel) and asserting the panel's own
projection. **Named here rather than built, because it is a framework-crate wave, not this one's file
budget** — and because §4's instrument answers the same question from the browser without it.

---

## 4 Fix 1 — the loss is named, at the hop, on a channel the browser shows

`🔌️plugin/🦀️.rs`:

- `interaction_selection_loss_v1(dispatched, validated, readback, minted) -> Option<&'static str>` — the
  ONE predicate, with four verdicts: `validate-state-pruned` (`protocol::validate_state` dropped the ids
  against the topology it was handed), `persist-skipped` (the validated half compared equal to the stored
  one so no edit was minted and the pick never reached the render), `store-readback-lost` (the store took
  the edit and did not answer with it), and `None` — which correctly covers the healthy case where the
  store never changed but `interaction_selection_snapshot`'s leftover overlay puts the ids back.
- `InteractionRevalidateOrigin::{Pick, DocumentChange}` — the same pruning is the mechanism working in one
  pass and a defect in the other. `revalidate_interaction_state_after_document_change` passes
  `DocumentChange` (it exists to drop ids the document no longer has); `dispatch_interaction_action` and
  `apply_interaction_writes` pass `Pick`.
- `report_interaction_selection_loss` — deliberately a plain `eprintln!`, with the docstring stating why
  it is NOT `debug_runtime_line` (§2.3, with the measurement). It fires only when a loss actually
  happened, the same rule the renderer's permanent `refreshUi dropped requested body` record follows.
- `revalidate_and_persist_interaction_state` now computes the `dispatched`/`validated` witnesses
  unconditionally and takes the readback through `interaction_selection_snapshot()` — the source the
  render really reads — OUTSIDE the `persisted != persisted_before` gate that used to hide the
  `persist-skipped` case entirely.

**Red, against the code it covers** (`if false && validated != dispatched`):

```
assertion `left == right` failed: validate_state dropped the pick against its topology
  left: Some("store-readback-lost")
 right: Some("validate-state-pruned")
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 663 filtered out
```

**Green, restored:**

```
RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib interaction_selection_loss -- --test-threads=1
test component::app::interaction_selection_loss_names_every_way_a_pick_can_disappear ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 663 filtered out; finished in 0.01s
```

The law states all seven rows as browser shapes, including the two that must stay `None` (a dispatch that
carried no id; a present-but-empty domain `vortex=object:`).

## 5 Fix 2 — the pane publishes the guest's own lane, unmerged

`🌐️World3dHost/🟦️.tsx`: new `worldSurfaceGuestSelectionDomV1(selectionJson)` and
`data-guest-selection-json` beside B20's `data-selection-json`.

B20's attribute publishes the state the pane PAINTS — `mergeWorldSelectionWithLeftoverV1(guest, leftover)`
— so a guest that renders from an empty interaction and a guest that renders from the picked one are
byte-identical from outside the moment the host's leftover overlay carries the ids. That is precisely why
§2.2's disagreement needed a 4 000-line console read to find. The new attribute carries the four fields
that make the comparison exact (`selectedIds`, `activeObjectId`, `hoveredId`, `gumballActive`) straight
off `scene.selectionJson`, with no overlay.

Language-neutral fixture `🌐️World3dHost/🧫️fixtures/🪪️world-surface-identity.json` gained two rows, and
they are the two directions of the discriminator: `guestPublished` (this fixture's guest sent nothing
while the pane paints `seed-left-001`) and `guestCarriedPick` (a guest that kept the pick says so on its
own lane). The law is B20's own engine-contract test.

**Red, against the code it covers** (`selectedIds: guest.ids ?? []` → `selectedIds: []`):

```
AssertionError: a guest that KEPT the pick must say so on its own lane: expected { selectedIds: [], …(3) } to deeply equal { …(4) }
-   "selectedIds": [
+   "selectedIds": [],
 Test Files  1 failed | 23 skipped (24)
```

**Green, restored:**

```
SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts \
  --testNamePattern='names its OWN window surface'
 Test Files  1 passed | 23 skipped (24)
      Tests  1 passed | 893 skipped (894)
```

---

## 6 Verification — every command foreground, tails quoted

| command | result |
| --- | --- |
| `cargo check -p semio-framework-plugin --lib` | `warning: semio-framework-plugin (lib) generated 6 warnings` / `Finished dev profile … in 6.83s` — **0 errors** (B19 recorded the same 6) |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: semio-s-artifact-puzzle-3d (lib) generated 88 warnings` / `Finished dev profile … in 19.50s` — **0 errors**, the warnings are the proof expansion ran |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Checking semio-s-plugin-puzzle v0.1.0` / `Finished dev profile … in 31.97s` — **0 errors** |
| `RUST_MIN_STACK=… cargo test -p semio-framework-plugin --lib interaction_selection_loss -- --test-threads=1` | `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 663 filtered out; finished in 0.01s` |
| `RUST_MIN_STACK=… cargo test -p semio-s-artifact-puzzle-3d … --lib -- --test-threads=1 leftover_ first_pick_ hover_after_first_pick a_browser_shaped_pick empty_target_interaction_select outliner_` | `test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 689 filtered out; finished in 1.48s` |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config …/⚛️react/vitest.config.ts` (whole config) | `Test Files 3 failed \| 21 passed (24)` / `Tests 9 failed \| 885 passed (894)` |
| `bun x tsc --noEmit -p …/⚛️react/tsconfig.json` | 852 errors repo-wide — B19's exact peer baseline. In `World3dHost/🟦️.tsx`: the same FOUR pre-existing errors B20 recorded, shifted by this wave's insertions (1303→1304 `leftoverSelectIdsMustNameHoverPickV1`, 3302/4160→3326/4184 `pickEnabled`, 4497→4521 overload). In `🔬️engine-contract/🟦️.ts`: three pre-existing, all above the edited region (4900, 4901, 8057). **No new error.** |

The vitest nine are **exactly** B18/B20's nine — 1 engine-contract `buildNoteShellCommandAction`, 6
`🧪️tests/🧩️package-integration` wgpu-worker, 2 `🔌️PluginRuntime` — with the totals moved
`887 → 894` / `878 → 885` by peers' and this wave's added tests. **No new failure.**

### `cargo test -p semio-framework-plugin --lib interaction` — a PEER's red, not this wave's

That filter reports `42 passed; 7 failed`. All seven fail on the same line, before any interaction code
runs:

```
seed label: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"),
  message: "typed command 'setLabel' has no exact controller/owner/factory/tool/schema proof" }
```

i.e. at the fixture's FIRST `dispatch_typed(TestCommand::SetLabel)`, inside
`🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — the file a peer last wrote at **21:45:19**,
after this wave started. `revalidate_and_persist_interaction_state`, the only function this wave touched
on that path, is never reached. Flagged for whoever owns that fixture; not this wave's to repair, and not
reverted.

---

## 7 Host-live vs `#49`

| change | lands |
| --- | --- |
| `data-guest-selection-json` + `worldSurfaceGuestSelectionDomV1` (`🌐️World3dHost/🟦️.tsx`) | **host-live on `:6013` now** — pure renderer TypeScript, no rebuild. The next probe any wave runs already carries it. |
| fixture + engine-contract law | vitest only, live |
| `interaction_selection_loss_v1` + the report at the hop (`🔌️plugin/🦀️.rs`) | **rides `#49`** — guest Rust |
| `a_browser_shaped_pick_survives_every_render_route_and_both_mutating_dispatches` | a TEST, rides no wasm |

**No probe run this wave.** `pgrep -f "browser-probe|lane-probe|b2[0-9]-"` was polled every 55 s for
~50 minutes across four windows and was never empty — peers held `:6013` continuously (`bun
🔍️browser-probe.ts --only=engagement-bar --port=6013` and successors). The temporary `[DEBUG] b23` tap
that was waiting on that slot was therefore removed and replaced by §5's permanent attribute, which
answers the same question without a probe run of this wave's own.

## 8 Handover — the two reads that close this family

1. **On the next `--only=selection-surfaces` run on `:6013`, read `data-guest-selection-json` beside
   `data-selection-json` at the moment `inspection-object-fields` reds.** They are now different
   attributes with the same three fields:
   - guest lane `selectedIds: []` while painted `selectedIds: ["seed-left-001"]` → the guest really has
     lost the pick between `dispatch_interaction_action` and `render`, and §4's report names which of the
     four ways on `#49`.
   - guest lane `selectedIds: ["seed-left-001"]` with Inspection still `.empty` → the guest HAS it and the
     Inspection body is not being re-rendered (or is rendered against another surface's retained view),
     which is §3's `SurfaceContexts` route — build the missing testkit shape there.
   §2.2 already leans to the second reading, but the vortex-marker lane cannot separate hover from
   selection and this attribute can.
2. **`#49` must carry `🔌️plugin/🦀️.rs`.** Until then the family's own diagnosis is still mute in the
   browser: every `interaction selection lost reason=…` line is a guest `eprintln!`.
