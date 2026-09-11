# Wave B12 — WindowConfig Publish / Refresh Round-Trip

Implementation pass, 2026-09-11. Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Closes B10 handover item 1:
the gestures land (B10 removed Tree `preventDefault`) but the measures rail never showed the new
`Puzzle3dWindowConfig` because a changed snapshot was dropped on publish and, even when it published,
the host actor refused to refresh on `mutationCount: 0`.

No git write, no worktree, no `CARGO_TARGET_DIR` / `RUSTC_WRAPPER`, shared cargo cache
`.🧬semio/🦑️repo/⚡️cache/cargo`. Puzzle wasm component-release was **not** started (#45 already
activated; two `serve puzzle3d react release` processes already live). B8/B9/B10/B11 hunks were not
reverted.

Every command tail quoted below is real output from this pass.

---

## 0. Verification status up front

| lane | result |
|---|---|
| guest static law `window_config_publish_hostile_static_law_rejects_silent_ok_into_iter_drop` | **ok** (1 passed, 694 filtered) |
| guest rail law `window_option_rail_round_trips_grid_lod_and_vortex_into_published_config` | **ok** (1 passed, 0.21 s) |
| existing `grid_window_options_control_one_visible_grid_spacing` | **ok** |
| existing `vortex_show_window_option_defaults_to_selected_and_switches_to_always` | **ok** |
| existing `one_window_config_mutation_publishes_exactly_one_generation_and_quiesces` | **ok** |
| host vitest `dirties the whole shell when a direct browser-actor dispatch applied a mutation` | **ok** — Tests 1 passed, 527 skipped (528), 26.71 s |
| **browser probe on :6014** | **NOT RUN.** First `curl` this session answered `200` in 60 ms; a later `curl --max-time 3` timed out (`000`). A `browser-probe` on any port would also stall Claude's wait loop (`pgrep -f browser-probe|lane-probe` before its :6013 battery). Guest rust rides the next component; host TS is vite-live. |
| puzzle wasm component-release | **not started** |

[DEBUG] window-option rail round-tripped grid/lod/vortex into published WindowConfig generations=4

---

## 1. Defect (user-visible)

Toggling grid visible / snap, bumping grid spacing, flipping LOD auto, or changing vortex show/direction
now receives the click (B10), but the rail stays on the previous value. `data-camera-json` moving is
**not** evidence this lane works: that attribute mirrors `World3dHost`'s local `cameraState`.

B1 already saw the intermittent hard fault on `setGridSpacing`:

`SemioFaultError: fixed typed-operation and segmented-output authorities did not pre-admit the exact operation slot`.

---

## 2. Hop chain (traced)

### 2a. Silent `.ok().into_iter()` drop — guest `dispatch_step`

`✏️editor/🦀️.rs` `Puzzle3dActionPrologue::dispatch_step` used to build:

```
view_state.and_then(|view| window_ownership::addressed_config(view, window_after).ok()).into_iter().collect()
```

`addressed_config` required `view.window_id`. The host often stamps `windowId` on the **command**
(`taggedOnAction`) while `view.window_id` stays `None` (settings `panelViewContext` clears it; some
browser-actor views omit it). The mutation became `None` and was discarded with no fault. Scene.runtime
updated this turn; the next render reconstituted from the **old** window-config store, so the rail
did not move. The same drop existed on `addressed_transient`.

Testkit always stamps `app.window_view(window_id)` via `for_window_instance`, so the existing WindowOptions
laws never saw the drop.

### 2b. `WindowCommandWork` already resolved the instance, then ignored it

`Puzzle3dWindowCommandWork::step` already did `view.window_id.or(command.window_id())` and passed that
`wid` into `dispatch_step`. Addressing still read `view.window_id` only.

### 2c. Mount-time capture still needs a view window id

`plugin.rs` `start_typed_command_operation` captures with `window_config_store.capture(meta.view_state)`.
That returns `Ok(None)` when `view.window_id` is `None`, and a later publish faults:

`window config emission requires one exact captured ViewModel window authority`.

This wave stamps the view **on the work** before dispatch and tags settings steppers with `windowId`.
A capture that already ran with an empty view is **not** retroactively fixed — leftover, section 6.

### 2d. Preflight leftover (B1 `setGridSpacing` hard fault)

Framework window-config `preflight` documented `retained_bytes` as **item cost**, then still returned
`O::MAXIMUM_PUBLICATION_BYTES` (65_536) against a publisher grant of 4_096. `advance()` used real item
bytes; preflight did not, so the slot was not pre-admitted.

### 2e. Host actor refresh hole (B9)

`browserActorDispatchUiScopeV1` returns `{kind:"none"}` when `mutationCount===0`. Window-config-only
verbs publish **no document mutations**. A successful guest publish therefore never called `refreshUi`.
Making every guest-applied dispatch `full` would refresh on `worldPointerDown` every click — that helper
stays as B9 wrote it.

### 2f. Settings steppers omitted `windowId`

`📌️panels/⚙️settings/🦀️.rs` dispatched overlap-budget / proximity / chunk-size / grid-spacing with
ambient view only. `panelViewContext` clears `windowId`, so grid-spacing shared this exact drop.

---

## 3. What landed

### 3.1 Guest address helpers

`✏️editor/🪟window/🦀️.rs`:

- `addressed_config_for(window_id, config)` / `addressed_transient_for(window_id, transient)`
- `addressed_config` / `addressed_transient` now delegate after reading `view.window_id`

### 3.2 Guest `dispatch_step`

When the snapshot actually changes:

```
vec![window_ownership::addressed_config_for(&wid, window_after)]
vec![window_ownership::addressed_transient_for(&wid, transient_after)]
```

No `.ok().into_iter()` drop.

### 3.3 Guest `Puzzle3dWindowCommandWork::step`

Resolve `view.window_id` or `command.window_id()`. If the view lacks that id, `for_window_instance` or
stamp `window_id` on a clone. `resolved_window_id: String` then `window_id = resolved_window_id.as_str()`
so the rest of `step` can borrow.

### 3.4 Framework preflight

`preflight` now:

```
let retained_bytes = Self::item_retained_bytes(...)?;
Ok(Footprint { work_items: 2, retained_bytes })
```

### 3.5 Settings steppers

`stepper_window_args(window_id)` builds `{windowId}`. All four steppers
(`setBrushPlacementOverlapBudget`, `setProximityRadius`, `setChunkSize`, `setGridSpacing`) pass it.
Doc comment: steppers tag the instance.

### 3.6 Host refresh (B9 preserved)

`🛠️ShellHelpers/🟦️.tsx`:

- `WINDOW_CONFIG_RAIL_ACTION_IDS` — WindowConfig-only action ids (grid / lod / vortex / camera / sun /
  projection / chunk / proximity / selectable / gumball / voxel).
- `browserActorWindowConfigDispatchUiScopeV1(result, actionId)` — if the B9 helper is `none`, the guest
  applied, and `actionId` is in that set → `{kind:"full"}`. `worldPointerDown` and rejected stays `none`.

`🏛️ShellHost/🟦️.tsx` `dispatchDirectBrowserActorCommand`:

```
const actionId = "actionId" in invocation.address ? invocation.address.actionId : undefined;
const dirty = browserActorWindowConfigDispatchUiScopeV1(result, actionId);
```

---

## 4. Laws

### 4.1 Guest static — hostile drop

`✏️editor/🧪️tests/🔬️unit/🦀️.rs`
`window_config_publish_hostile_static_law_rejects_silent_ok_into_iter_drop`

Source must contain `addressed_config_for(&wid, window_after)` and
`addressed_transient_for(&wid, transient_after)` and must **not** contain
`addressed_config(view, window_after).ok()).into_iter()`. Replacing the `vec![addressed_config_for…]`
publish with the old `.ok().into_iter()` collect fails the predicate.

```
test editor::puzzle3d::component::tests::window_config_publish_hostile_static_law_rejects_silent_ok_into_iter_drop ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 694 filtered out; finished in 0.03s
```

### 4.2 Guest runtime — rail round-trip

`window_option_rail_round_trips_grid_lod_and_vortex_into_published_config`

Dispatches `setGridVisible` pressed false, `setGridSpacing` 25, `setLodAutomatic` pressed false,
`setVortexShow` ALWAYS. Asserts no pending typed ops, `window_config_generation` increases by 4, and
the measures rail reads:

- `puzzle3d-play-grid-visible` = false
- `puzzle3d-play-grid-spacing` = 25
- `puzzle3d-play-lod-auto` = false
- `puzzle3d-play-vortex-show` = ALWAYS

```
test editor::puzzle3d::component::tests::window_option_rail_round_trips_grid_lod_and_vortex_into_published_config ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 694 filtered out; finished in 0.21s
```

Existing WindowOptions laws still pass (they already used a stamped view and therefore never caught
the silent drop):

```
test editor::puzzle3d::component::tests::grid_window_options_control_one_visible_grid_spacing ... ok
test editor::puzzle3d::component::tests::vortex_show_window_option_defaults_to_selected_and_switches_to_always ... ok
test editor::puzzle3d::component::tests::one_window_config_mutation_publishes_exactly_one_generation_and_quiesces ... ok
```

Crate: `semio-s-artifact-puzzle-3d --features component-app-assembly`. `RUST_MIN_STACK=134217728`.
Shared cache. A first `cargo test -p semio-s-plugin-puzzle --lib <filter>` compiled the plugin and
ran **0** matching tests (that crate's `--lib` suite is not the editor tree) — do not read those
`b12-cargo-static.txt` / `b12-cargo-rail.txt` files as this wave's guest proof.

### 4.3 Host vitest

`🔬️engine-contract/🟦️.ts` keeps B9: `mutationCount: 0` → `none` on `browserActorDispatchUiScopeV1`.
Adds:

- `browserActorWindowConfigDispatchUiScopeV1(..., "setGridVisible"|"setGridSpacing"|"setVortexShow")`
  on guest-applied / 0 → `{kind:"full"}`
- still `none` for `worldPointerDown` and for rejected `setGridVisible`

```
 Test Files  1 passed (1)
      Tests  1 passed | 527 skipped (528)
   Duration  26.71s
```

---

## 5. Files touched

- `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🪟window/🦀️.rs` — `addressed_*_for`
- `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🦀️.rs` — `dispatch_step` + `WindowCommandWork::step`
- `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/📌️panels/⚙️settings/🦀️.rs` — stepper `windowId`
- `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — static + rail laws
- `🧰️framework/…/🔌️plugin/🪟window/🎚️config/🦀️.rs` — preflight item bytes
- `🧰️framework/…/🛠️ShellHelpers/🟦️.tsx` — WindowConfig actor scope helper
- `🧰️framework/…/🏛️ShellHost/🟦️.tsx` — wire helper with `actionId`
- `🧰️framework/…/🔬️engine-contract/🟦️.ts` — host laws
- this report

Logs: `🗑️generated/b12-cargo-puzzle3d-laws.txt`, `🗑️generated/b12-vitest-actor-scope.txt`.

---

## 6. Handover leftovers

1. **Mount-time capture** still returns `Ok(None)` when `view.window_id` is missing at
   `start_typed_command_operation`. This wave stamps the work and the settings steppers; a host path
   that still mounts with a window-less view will fault on publish instead of silently dropping.
   Folding `command.window_id()` into the capture view (or capturing by the resolved instance id)
   is the remaining guest authority hop.
2. **B10 item 2** — `record_command`'s promised `View`/`Shell` folding is still unimplemented
   (`push_log_entry` always appends `count: 1`).
3. **B10 item 4** — `world3d_sun_measures` / `world3d_projection_measures` still hard-code English
   labels.
4. **Live wasm** — guest rust is native-proven only. The running #45 component does not include this
   `dispatch_step` until the next puzzle wasm component-release. Host refresh is vite-live on reload.
5. **Re-run probe** against `:6014` (this Cursor session) after that wasm, **not** `:6013` (Claude's
   battery). `--only=window-option-puzzle3d-play-grid-visible,window-option-puzzle3d-play-grid-spacing,window-option-puzzle3d-play-lod-auto,window-option-puzzle3d-play-vortex-show`.
   Do not treat `data-camera-json` moving as proof.
6. **`setBrushPlacementOverlapBudget`** is a settings stepper now tagged with `windowId`, but it is
   not in `WINDOW_CONFIG_RAIL_ACTION_IDS`. If that verb is shared app config rather than WindowConfig,
   it still reports `mutationCount: 0` and will not refresh through the new helper.

---

## 7. Probe / port notes

- `:6013` — Claude battery (`bun 🔍️browser-probe.ts --battery --reload-between-groups --port=6013`).
  Do not touch.
- `:6014` — this Cursor session. Intermittent this pass (`200` then `000` / 3 s timeout). Skipped.
- Do not start a second `browser-probe` while Claude's waiter is `pgrep -f browser-probe|lane-probe`.
