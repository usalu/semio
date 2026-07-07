---
name: Fix wgpu gis2d selection interaction
overview: Diagnose and restore premigration-parity feature selection (click, rectangle marquee, lasso marquee, with default/additive/subtractive/invertive modes and partial/inclusive coverage) in the wgpu-native gis2d map, which currently does not react to clicks or drag-rectangles even though hover already works correctly.
todos:
  - id: instrument
    content: Add temporary [DEBUG] logs across pointer_down/up, query_map_feature_hits, command dispatch, plugin handle_command, sync_map_host
    status: cancelled
  - id: reproduce
    content: Rebuild/serve wgpu gis2d playground and reproduce click, rectangle (both drag directions), lasso, modifier-merge selection while capturing logs
    status: completed
  - id: diagnose
    content: Pinpoint exact stage where selection interaction breaks from captured logs
    status: completed
  - id: fix-modifiers
    content: Swap modifiers.ctrl for modifiers.ctrl_or_meta() in gis_map_pointer_down call site for Cmd/Ctrl parity
    status: completed
  - id: fix-root-cause
    content: Apply the fix for the diagnosed root cause of click/rectangle selection not reacting
    status: completed
  - id: verify-vocabulary
    content: Manually verify full method/mode/coverage matrix (rectangle, lasso, default, additive, subtractive, invertive, partial, inclusive) matches premigration
    status: completed
  - id: cleanup-tests
    content: Remove debug logs, extend existing gis2d-plugin/gis_2d tests, rebuild and re-verify with evidence
    status: completed
  - id: ticket
    content: Reopen and close MAP-WGPU-RENDERER-PARITY ticket with summary of the selection-interaction fix
    status: completed
isProject: false
---

## Context

Hover already works end-to-end: pointer move (no button) calls [scenes::gis_map_pointer_move](framework/renderer/wgpu/rs/lib.rs) -> `host.hit_test_feature_json(sx, sy)` -> dispatches `setHover` -> plugin `handle_command("setHover", ...)` mutates `play.runtime.hover_json` -> `render_canvas` puts it back on `GisMapScene.hover_json` -> `sync_map_host` applies it to the live `MapHost` -> `append_positions`/`append_routes` render the hover highlight.

Click-select and marquee-select are implemented as a structurally parallel pipeline:

- [scenes::gis_map_pointer_down](framework/renderer/wgpu/rs/lib.rs) (button 0) stores `SceneDragMode::MapMarquee { start_x, start_y, method, merge_mode }` in per-surface state.
- [scenes::gis_map_pointer_up](framework/renderer/wgpu/rs/lib.rs) reads that drag state: for a real drag it calls `query_map_feature_hits` (rect/lasso, crossing vs containing) and dispatches `setFeatureSelection`; for a plain click (distance below `MAP_MARQUEE_THRESHOLD_PX`) it re-runs `host.hit_test_feature_json` (the exact same call hover uses) and dispatches `setFeatureSelection` with a single id.
- Plugin `handle_command("setFeatureSelection", ...)` ([gis/2d/plugin/rs/lib.rs](gis/2d/plugin/rs/lib.rs)) merges into `feature_selection_json` via `merge_feature_selection` (default/additive/subtractive/invertive already implemented) and returns a new document.
- `render_canvas` puts the updated selection back on `GisMapScene.selection_json`; `sync_map_host` applies it via `host.set_selection_json`; `append_positions`/`append_routes` already branch on `self.selected_positions.contains(&pos.id)` for the highlight stroke.

Every function in this chain was re-read line by line and is unit-tested at the plugin layer (`set_feature_selection_updates_runtime_and_host`, `set_feature_selection_additive_merges`, `clear_selection_resets_features`, all passing). No static defect was found that would fully explain clicks and rectangles both doing nothing while hover (which shares the same `hit_test_feature_json` call and the same command-dispatch machinery) works. This means the remaining root cause is very likely in a runtime-only condition (pointer-down/up event delivery, drag-state timing, or a coordinate/threshold edge case) that requires live reproduction with logging to pin down — it cannot be responsibly guessed and "fixed" blind per the repo's rule against claiming behavior is verified without runtime confirmation.

One concrete, already-confirmed parity bug was found while auditing modifiers:

```12575:12587:framework/renderer/wgpu/rs/lib.rs
map_commands.extend(scenes::gis_map_pointer_down(
    surface_id,
    &surface.controller_id,
    surface.bounds,
    x,
    y,
    button,
    modifiers.shift,
    modifiers.ctrl,
    modifiers.alt,
    &surface.selection_method,
));
```

This passes raw `modifiers.ctrl`, whereas the node-graph pointer-down call two blocks above it uses `modifiers.ctrl_or_meta()` (`ui/wgpu/rs/lib.rs`), and every other multi-select/shortcut check in this file (`modifiers.meta || modifiers.ctrl`, e.g. lines 3468, 6741, 7989, 9131) treats Cmd and Ctrl as equivalent. On macOS this means Cmd-drag/Cmd-click on the map silently fails to enter additive/invertive mode even though it works for the node graph — a direct instance of "not matching premigration ... invertive" parity. This will be fixed regardless of the main diagnosis.

## Plan

1. **Instrument the interaction chain with temporary `[DEBUG]`-prefixed logs** (per repo rule, removed before closing) at:
   - `scenes::gis_map_pointer_down` / `gis_map_pointer_up` entry (surface id, button, computed `sx,sy`, drag-state before/after).
   - `query_map_feature_hits` result counts and the `crossing` flag.
   - The `SET_FEATURE_SELECTION` command right before `dispatch_commands`.
   - Plugin `handle_command("setFeatureSelection", ...)` entry/exit in [gis/2d/plugin/rs/lib.rs](gis/2d/plugin/rs/lib.rs).
   - `sync_map_host`'s `selection_json` branch in [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs).

2. **Reproduce live** against the wgpu gis2d playground (wasm/Trunk dev server, now that tiles render) or native-bin: click a pin, drag a left-to-right rectangle, drag a right-to-left rectangle, Cmd/Ctrl-click a second pin, Alt-click a selected pin. Capture console/stdout logs to identify exactly which stage in the chain stops producing the expected data (e.g., drag state never set, hit list always empty, command never dispatched, plugin never invoked, or scene/host never re-synced).

3. **Apply the fix(es)** based on findings from step 2. Known fix to include regardless:
   - Swap `modifiers.ctrl` for `modifiers.ctrl_or_meta()` in the `gis_map_pointer_down` call site (`framework/renderer/wgpu/rs/lib.rs` ~line 12584) for macOS Cmd/Ctrl parity with node graph.

4. **Confirm full premigration selection vocabulary** is exercised and correct once the base breakage is fixed:
   - Method: `rectangle` (default) / `lasso`, switchable via the existing "Selection Method" inspector dropdown.
   - Merge mode: `default` (replace) / `additive` (shift) / `subtractive` (alt) / `invertive` (ctrl/meta) via `map_marquee_mode`.
   - Coverage: `inclusive` (containing, drag left-to-right) / `partial` (crossing, drag right-to-left) via `map_marquee_crossing`, matching the premigration `marqueeCoverageFromGesture` convention (verified against `gis/2d/react/index.tsx` at commit `f8376e8486`).

5. **Remove all temporary debug logging**, extend the existing `gis2d-plugin`/`gis_2d` test files to cover whatever regression is found (no new test files, per repo rules), and rebuild + manually re-verify in the running playground with a screenshot/log as evidence before closing.

6. **Ticket**: reopen the `MAP-WGPU-RENDERER-PARITY` ticket (already used for the tile-proxy regression fix) and close it again with a summary covering the selection-interaction fix and all touched files.
