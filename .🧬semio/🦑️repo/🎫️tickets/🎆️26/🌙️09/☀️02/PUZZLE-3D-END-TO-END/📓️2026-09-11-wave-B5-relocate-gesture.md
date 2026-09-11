# Wave B5 — Relocate utility, end to end (checklist §11, audit A2 §11) + Settings scoping (A2 §19)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Implementation wave. No git write, no worktree, no wasm build, no
browser battery. All verification ran in the foreground; every command tail is quoted below.

---

## 1. What A2 §11 found, and what was actually true

A2 §11 is confirmed on the host side and **partly stale on the guest side**:

| A2 §11 claim | verdict at this session's HEAD |
|---|---|
| No gesture anywhere under `📺️renderer` dispatches `worldRelocate` | **TRUE** — `rg worldRelocate` over `World3dHost/🟦️.tsx` returned zero matches before this wave. The utility activated, and nothing else. |
| The only `onRelocate` is `WorldVolumeLayer`'s, gated `activeUtility === "transform"`, and it dispatches `relocateTargetVolume` (target volumes, not objects) | **TRUE**, unchanged by this wave (`World3dHost/🟦️.tsx`, `handleTargetVolumeRelocate`) |
| Checklist reverification §26 item 8: `relocateTargetVolume` / `worldRelocate` declared `ActionKind::View` while mutating | **STALE** — both are already `.mutation(...)` in `create_puzzle3d_app` (`✏️editor/🦀️.rs`, the `.mutation("worldRelocate", …)` and `.mutation("relocateTargetVolume", …)` rows), i.e. `ActionKind::Mutation`. No declaration fix was needed; the new cargo law now **pins** it so the claim cannot come back. |
| A3: `Puzzle3dWorldRelocateWork::extent` fits Nakagin | **TRUE** — `world_relocate_extent_fits_within_cap_for_nakagin`, `world_relocate_on_nakagin_admits_and_completes` and `world_relocate_step_loop_stays_within_its_own_extent_for_nakagin` all pass (quoted below). |

One thing A2 did not mention and this wave did **not** change: `🎮️commands/🌍️world-relocate/🦀️.rs`'s
`world_relocate(ctx, args)` arm (reached from `handle_action`'s `"worldRelocate" =>` match) is **dead in the
app**: `PUZZLE3D_RETAINED_TOOL_IDS` contains `"worldRelocate"`, so `build_tool_job` always intercepts and
routes to `Puzzle3dWorldRelocateWork` before `dispatch_step` runs. It is a duplicate implementation of the
same verb. Deleting it is a separate (larger) refactor — flagged for the coordinator, not taken here.

---

## 2. Gesture design (host)

**Activation.** `relocateMode = activeUtility === "worldRelocate"` (`World3dHost/🟦️.tsx`, alongside the
existing `brushMode`/`volumeBrushMode` flags).

**Grab rule — decided and documented.** `world3dRelocateDragTargetV1(pressedId, selectedIds)`:

- nothing selected → the object **under the pointer** is grabbed outright (direct manipulation, no
  select-then-drag ceremony);
- a selection exists → the selection is the **gate**: a press inside it grabs that object, a press outside it
  grabs nothing and **falls through** to the ordinary marquee/pick path, so the user can re-select without the
  scene jumping under the cursor.

`selectedIds` is exactly `world3dGumballSelectionArgsV1(selection).ids` — the same leftover/object id list the
gumball commit uses, so relocate and transform can never disagree about what "selected" means.

**Ground plane.** Press/move/release each raycast the world Z=0 plane with the existing
`raycastGroundPoint(clientX, clientY, hostRect, camera)` — the same helper catalogue drop, face drag and the
engagement pointer path use. Face-normal dragging is deliberately **not** re-implemented: that gesture already
exists as `worldFaceDragEnd` (`handleFaceDragStart` → `axisDragParam`) and is a different verb.

**Live preview — reuses the existing world ghost, no second render path.** `beginRelocateDrag` /
`updateRelocateDrag` write the shared per-controller ghost store
(`setWorldCatalogueDropPreview(node.controllerId, { objectKind, meshUrl, origin })`), which `World3dHost`
already subscribes to and renders through `CatalogueDropGhost` (the same translucent GLB ghost a catalogue
drop shows). The grabbed object's `meshUrl` comes from `meshes.find(m => m.id === (instance.meshId ?? instance.id))?.url`.
Because the ghost is a shared controller-scoped store, **every sibling pane paints the same in-flight preview**,
which is the behaviour catalogue drop already has. Zero new components, zero new stores, zero changes to
`WorldInstancesLayer`.

**Snapping.** The would-be origin is `origin + (to − from)` passed through the existing
`snapWorldPointToGrid(point, gridSnapEnabled, gridFactor)` — identical to `resolveCatalogueDropOrigin`, so a
relocate and a drop land on the same lattice.

**Commit — exactly one dispatch.** `world3dRelocateDispatchArgsV1(objectId, origin, from, to, snap)` returns
`{ objectId, position }` — **exactly** what the guest decodes
(`Puzzle3dWorldRelocateWork::step`'s `Object` stage reads `args.objectId` and `Self::position(command)` →
`args.position` as a 3-vector; the dead `world_relocate` arm decodes the same two keys). It returns `null`
when the travel is inside `GUMBALL_TRANSFORM_EPSILON` (the gumball's own epsilon, reused), so a
click-without-travel never writes a document edit.

**Why not `dispatchGumballPoseDelta`.** The brief asked to reuse it. It could not be reused *for the dispatch*:
that helper's whole contract is an **incremental** pose delta (`translateSelection {dx,dy,dz}` /
`rotateSelection` / `scaleSelection`), while `worldRelocate` is an **absolute world origin** verb keyed by
`objectId` — feeding it through `gumballTransformDeltaBetweenPoses` would have produced the wrong action with
the wrong args. What *is* reused, so there is still only one gesture vocabulary: `world3dGumballSelectionArgsV1`
(the selection gate), `GUMBALL_TRANSFORM_EPSILON` (the no-op threshold), `raycastGroundPoint`,
`snapWorldPointToGrid`, `resolveClickInstanceId`, and the catalogue-drop ghost store. No second gesture stack,
no second preview mechanism, no new state — the open drag lives in one `useRef`, so mid-drag React re-renders
come only from the ghost store, exactly like a catalogue drop.

**Cancel.** `Escape` (a `keydown` capture listener installed only while the Relocate utility is active) drops
the ghost and dispatches nothing; it `stopPropagation()`s **only** when a drag was actually open, so the
window-search/engagement Escape keybinding is untouched otherwise. `pointercancel` also aborts: the host binds
`onPointerCancel={handlePointerUp}`, so `endRelocateDrag` treats `event.type === "pointercancel"` as a cancel
and only a real pointer-up commits.

**Locked objects are grabbed on purpose.** The host does not refuse the grab for a `disabled`/locked instance —
the guest answers the commit with its `selection_locked` notice, so the user gets a visible refusal instead of a
dead gesture (the principle `refuse_without_selection`'s own doc comment states). The ghost simply snaps back
because no edit is emitted.

---

## 3. Files and functions changed

### Host — `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`

| what | kind |
|---|---|
| `//#region WorldRelocateGesture` — `world3dRelocateDragTargetV1`, `world3dRelocateDispatchArgsV1`, `type World3dRelocateSession` | new exported pure helpers (next to `snapWorldPointToGrid`) |
| `relocateMode` | new derived flag next to `volumeBrushMode` |
| `relocateSessionRef` | new ref next to `gumballDragStartPoseRef` |
| `beginRelocateDrag` / `updateRelocateDrag` / `endRelocateDrag` | new `useCallback`s before `handlePointerDown` |
| Escape `useEffect` keyed on `relocateMode` | new |
| `handlePointerDown` | one new first line: `if (relocateMode && beginRelocateDrag(event)) return;` + deps |
| `handlePointerMove` | one new first line: `if (updateRelocateDrag(event)) return;` + deps |
| `handlePointerUp` | one new line after the capture release: `if (endRelocateDrag(event)) return;` + deps |

### Host re-export — `…/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`

`world3dRelocateDragTargetV1`, `world3dRelocateDispatchArgsV1` added to the World3dHost import **and** export
lists (next to `world3dGumballSelectionArgsV1`).

### Guest — `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

- `Puzzle3dWorldRelocateWork` gained `view_state: Option<ViewModel>` + `bind_view_state`, and `close_step` /
  `terminal_is_empty` now account for it (the close ladder must release everything it holds).
- `Puzzle3dWorldRelocateWork::step`, `Object` stage: a requested object that is **locked or hidden** now
  completes with `puzzle3d_notice_emit(…, labels.selection_locked)` instead of falling out of the loop
  silently. Same notice text, same `UiDirtyScope::None` refusal shape as `Puzzle3dScaleWork`'s
  (`translateSelection`/`rotateSelection`/`scaleSelection`) locked branch.
- `render_body`: `settings_panel::render(&envelope, labels, wid)` (see §5).

### Guest — `…/✏️editor/📌️panels/⚙️settings/🦀️.rs`

`render` takes `window_id: &str` and titles the section `"{Settings} — {window_id}"` (see §5).

### Laws

- `…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — new `it("Relocate-utility drag grabs by selection gate and commits one absolute worldRelocate")` + two imports.
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — new `world_relocate_moves_an_unlocked_object_and_refuses_a_locked_one_with_one_notice`.

**Action declaration fixes: none were needed.** `worldRelocate` and `relocateTargetVolume` are already
`ActionKind::Mutation`; the checklist §26 item 8 row is stale. The new cargo law asserts the kind so it stays
that way.

---

## 4. Laws + quoted outputs

### Vitest — `🔬️engine-contract`

`cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react && bun ./📜️script.ts test long --run '🔬️engine-contract' --testNamePattern='Relocate-utility drag'`

```
 Test Files  1 passed (1)
      Tests  1 passed | 521 skipped (522)
   Duration  6.06s
```

Full lane, `bun ./📜️script.ts test long --run '🔌️PluginRuntime' '🔬️engine-contract'`:

```
 Test Files  2 failed (2)
      Tests  3 failed | 613 passed (616)
```

The three failures are **not** this wave's and do not touch relocate:

```
 FAIL … 🔬️engine-contract/🟦️.ts > noteShellCommand > buildNoteShellCommandAction builds a noteShellCommand action descriptor …
 FAIL … 🔌️PluginRuntime/🟦️.tsx > surface render ViewModel > binds two instances of one body to distinct surfaces and packed window projections
 FAIL … 🔌️PluginRuntime/🟦️.tsx > PluginRuntime documentPack/transaction wire adapter > readAppDocumentPack() …
```

(the `documentPack` diff is a peer's new `"ops": ""` field — B2 territory.)

### `bun x tsc --noEmit` (react target project)

Only three errors resolve into `World3dHost/🟦️.tsx`, **all pre-existing / peer-owned**, none in new code:

```
(3244,12): error TS2741   ← GlbInstanceMesh missing `pickEnabled` (peer added a required prop)
(4083,12): error TS2741   ← same, inside CatalogueDropGhost
(4420,42): error TS2769   ← window.addEventListener("contextmenu", …)
```

and three in `🔬️engine-contract/🟦️.ts` (`4891`, `4892`, `7984`), all on pre-existing lines
(`7984` is the long-standing `world3dGumballSelectionArgsV1({ …, gumballActive: true, … })` call, not my block
at ~7548). **Zero new errors in touched code.**

### Cargo

`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --keep-going`

```
warning: `semio-s-artifact-puzzle-3d` (lib) generated 88 warnings (run `cargo fix --lib -p semio-s-artifact-puzzle-3d` to apply 85 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.43s
```

`grep -c '^error'` over the same run: **0**. (88 warnings prove the crate actually expanded and type-checked;
they are the crate's pre-existing dead-code set.)

`RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib relocate -- --test-threads=1`

```
test editor::puzzle3d::component::tests::relocate_target_volume_undoes_and_redoes_as_one_mutation ... FAILED
test editor::puzzle3d::component::tests::world_relocate_extent_fits_within_cap_for_nakagin ... ok
test editor::puzzle3d::component::tests::world_relocate_hostile_static_law_rejects_whole_proximity_scans ... ok
test editor::puzzle3d::component::tests::world_relocate_moves_an_unlocked_object_and_refuses_a_locked_one_with_one_notice ... ok
test editor::puzzle3d::component::tests::world_relocate_on_nakagin_admits_and_completes ... ok
test editor::puzzle3d::component::tests::world_relocate_step_loop_stays_within_its_own_extent_for_nakagin ... ok
test editor::puzzle3d::component::tests::world_relocate_undoes_and_redoes_as_one_mutation ... FAILED
test result: FAILED. 5 passed; 2 failed; 0 ignored; 0 measured; 666 filtered out; finished in 3.51s
```

**`RUST_MIN_STACK` is mandatory** for this filter — without it the very first case dies with
`has overflowed its stack / fatal runtime error: stack overflow, aborting` (SIGABRT). This is
`📓️2026-09-10-order-dependent-tests-audit.md` family 3; the crate's own `📜️script.ts` sets
`RUST_MIN_STACK ??= 128 MiB`, so a raw `cargo test` outside that script is the broken invocation, not the code.

---

## 5. Settings panel scoping (A2 §19)

Minimal, no redesign: `settings_panel::render(envelope, labels, window_id)` now titles its section
`"{labels.settings} — {window_id}"` (falling back to the bare label when the id is empty), and
`render_body` passes the `wid` it already resolves. The stepper ids
(`#puzzle3d-play-settings.overlap-budget.control` and siblings) and the section id
`#puzzle3d-play-settings` are **unchanged**, so every A2 §19 probe selector still holds. The user can now see
which pane's config the ambient-`windowId` steppers are about to retune — A2's "invisibly, with no per-window
UI cue" is closed without touching the dispatch shape.

---

## 6. Blocker found — undo is broken app-wide right now (NOT this wave)

`worldRelocate` is a declared `Mutation` and its edit lands, but **undo restores nothing** — and that is not
specific to relocate:

`RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib undo -- --test-threads=1`

```
test editor::puzzle3d::component::tests::cut_undoes_as_one_step ... FAILED
test editor::puzzle3d::component::tests::relocate_target_volume_undoes_and_redoes_as_one_mutation ... FAILED
test editor::puzzle3d::component::tests::set_active_example_swaps_the_document_and_undo_restores_it ... FAILED
test editor::puzzle3d::component::tests::world_relocate_undoes_and_redoes_as_one_mutation ... FAILED
test editor::puzzle3d::panels::document::tests::an_outliner_flag_row_undoes_itself_on_the_second_click ... ok
test result: FAILED. 1 passed; 4 failed; 0 ignored; 0 measured; 668 filtered out; finished in 0.53s
```

Failure shape (all four): `dispatch("undo")` **succeeds** (no fault) and the document is unchanged, e.g.

```
assertion `left == right` failed: undo restores the object origin
  left: [9.0, 8.0, 7.0]
 right: [0.0, 0.0, 0.0]
```

`cut_undoes_as_one_step` and `set_active_example_swaps_the_document_and_undo_restores_it` have nothing to do
with relocate and were never touched by this wave, and both `relocate_*` cases fail **alone** as well as in the
suite (`--lib relocate_target_volume_undoes_and_redoes` → `0 passed; 1 failed`). So the checkpoint/undo lane
itself is regressed at this HEAD. **Needs an owner.** The only surviving "undo" case
(`an_outliner_flag_row_undoes_itself_on_the_second_click`) is a toggle re-dispatch, not a history undo, so it
proves nothing about the lane.

Also observed twice during this wave: `✏️editor/⏳️precompute/🪣️fill/🦀️.rs:2909` and
`🧰️framework/…/🔌️plugin/🦀️.rs` failed to compile mid-session from concurrent peer edits (`FixedOwnerVec`
indexing / `get_mut`, and an unclosed delimiter). Both cleared on retry; noted only so a red `cargo` run is not
mistaken for this wave's.

---

## 7. Probe recipe for B1 (`🔍️browser-probe.ts` — NOT edited by this wave)

B1 owns the probe file. Add a `--only=relocate` step shaped like this.

**Selectors (all verified against source this session):**

| thing | selector | source |
|---|---|---|
| Perspective pane | `[id="framework.window.puzzle3dMainPerspective"], [data-element-alias~="framework.window.puzzle3dMainPerspective"]` | A2 cross-cutting (camelCase rule) |
| Utility bar unfold | `#framework.window.puzzle3dMainPerspective.utilityBar.unfold` | `🪟️Window/🟦️.tsx`, `toggleId={childElementId("framework.window", id, "utilityBar", "unfold")}` |
| **Relocate toggle** | `[id="worldRelocate"]` | `deriveUtilityNodes` (`🎯️action-bus/🟦️.ts`) sets the leaf node `id: utility.id`; `UtilityTree` renders `id={entry.id}` verbatim — **the raw utility id, no prefix, no camelCase transform** |
| Viewport + scene mirror | `[data-surface-id]` inside the pane; `data-instances-json`, `data-meshes-json`, `data-camera-json` on the same element | `World3dHost/🟦️.tsx` (`data-surface-id` … `data-camera-json`, the latter now present thanks to B1) |

**Sequence (Concrete Forest is sufficient — one object; Nakagin is not needed, A3 proved the extent fits):**

1. Boot, wait for `data-instances-json` non-empty on the Perspective pane. Parse it; keep
   `target = instances[0]` (`{ id, position }`).
2. Click `#framework.window.puzzle3dMainPerspective.utilityBar.unfold`.
3. Click `[id="worldRelocate"]`. Assert the intercepted dispatch is
   `setActiveUtility { utilityId: "worldRelocate", windowId: "puzzle3d-main-perspective" }`
   (`deriveUtilityNodes` args + `tagSetActiveUtilityWindow`'s `windowId` stamp).
4. Hook outgoing actions (`page.exposeFunction` / console hook on the `onAction` path — same instrumentation
   A2 §2 recommends for `setCamera`, since the ghost lives inside the `<canvas>` and has **no DOM hook**).
5. Over the pane's `[data-surface-id]`: `mouse.move(cx, cy)` → `mouse.down()` →
   `mouse.move(cx + 120, cy, { steps: 8 })` → `mouse.up()`.
   - **Assert exactly one** `worldRelocate` left the page, with args shaped
     `{ objectId: target.id, position: [number, number, number] }` — no `dx`/`dy`/`dz`, no `mode`/`ids`
     (that is `translateSelection`'s shape and would mean the wrong helper got wired).
   - After settle, re-read `data-instances-json`; assert `target.id`'s `position` changed.
   - Assert **zero** `translateSelection` / `worldSelect`-only dispatches replaced it.
6. **Escape law:** repeat step 5 but press `Escape` after the moves and **before** `mouse.up()`.
   Assert **zero** `worldRelocate` dispatches and `data-instances-json` byte-identical to before.
7. **Selection-gate law:** switch to the Transform utility, click object A to select it, switch back to
   Relocate, then press-and-drag on empty space / a different object B. Assert **zero** `worldRelocate`, and
   that the ordinary marquee/pick path ran instead (a `worldPick`/`interactionSelect` went out).
8. **Locked law (optional, browser-only):** lock the object (outliner lock row), then repeat step 5. Assert one
   `worldRelocate` goes out, **no** instance position change, and exactly one notice/toast carrying
   `"Selection is locked"` (EN) / `"Die Auswahl ist gesperrt"` (DE).

The mid-drag ghost itself is screenshot-only (it is a three.js `CatalogueDropGhost` inside the canvas); the
dispatch-count assertions above are the load-bearing ones.
