# Gap 4: Terrain window Measures trap

Static analysis only. No build, cargo or browser run. The root cause was not proven; no production code was changed.

## What gisterrain contributes

- `Gis3dPlayApp` (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/.../✏️editor/🦀️.rs`) and the Terrain window module declare no `window_measures`, `window_engagements` or `tool_measures`. `terrain::definition()` sets `options: WindowOptions::default()` and `utilities: Vec::new()`.
- Every measures projection for gisterrain therefore falls to the `ArtifactEditor` default, which returns an empty map. No gisterrain Rust code, no 2-65 rectangular grid, no millimetre height and no mode-specific measure is read by the projection. The trap cannot come from document content.
- gismap overrides `window_measures` (`map::window_measures`) and passes, so the difference is framework state, not app code.

## Shared projection the guest calls

- Section render: `RenderSurface::render` (`🧵️instance/🪢️members/📥️staging/🖼️render-surface/🦀️.rs`) matches `UiRefreshSection::Measures`, calls `RenderCtx::window_measures` (`🧵️instance/🖼️render/📥️staging/🪟️projections/🦀️.rs`) and serializes with `canonical_section_json` (infallible, `unwrap_or_else`).
- `RenderCtx::window_measures` builds an `ArtifactView`, then per window instance takes `view_state.scope_window_instance(id)` (a `ScopedWindowView`), captures the window config and calls the app.
- Inspected and cleared: `refresh_cache`, `ArtifactView::with_render_context`, `ChildContentView` clone (plain `Arc`), `WindowConfigAuthority` drop (`SharedUtf8` and `ErasedSnapshotRead` drops do not assert), `canonical_section_json`, `section_component_tree` input for `{}`.

## Only state-dependent assert path found

`ScopedWindowView::drop` (`🧰️framework/🔨️modules/🛂️manifest/🖼️view-model/🦀️.rs`) calls `OrderedMap::return_scoped_alias` on `active_utility_by_window_id` and `tool_run_trace_cursor_by_window_id`. That method (`🌱️value/🗂️ordered/🦀️.rs`) has three hard asserts (`expect` on the original root, `same_original`, `elect_cold().is_none()`), and `OrderedMap` / `SharedOriginalStorage` drops assert that a non-empty root was returned or retired. In a wasm guest a panic is a trap.

- Both maps are empty for gismap (no host utility chosen), so `return_scoped_alias` returns immediately and the asserts never run.
- Terrain is the app whose host utility register is non-empty (Raise, Lower, Smooth, Move Position), and it is the only reported failing case, so it is the first to execute the asserted branch in a live guest.
- The logic reads correct on paper (alias count is bumped by `Clone`, decremented by `elect_cold`), and the existing law `scoped_window_section_returns_its_original_map_aliases` covers it natively. The other `for_window_instance` callers (`🧰️framework/.../⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs` lines ~300 and ~357, context-menu `into_parts`) clone the same maps without returning them; those are only safe where the clone is explicitly retired.
- Any later `ViewModel::clone()` of a Terrain window view that is dropped unretired would trap the same way. I found none inside the measures path.

Conclusion: the trap is most likely tied to a non-empty utility or trace-cursor register reaching the scoped section projection (the one branch gismap never runs), not to Terrain data. A native log naming the failing assert message (`scoped alias requires its exact live root`, `scoped alias cannot own the final root`, `ordered-map root must be explicitly retired before drop`, or `shared original alias must retire or transfer`) will decide the fix; do not guess a change to the alias machinery without it.

## Test written (not run)

- `window_sections_project_while_the_host_utility_register_is_populated` in `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧩️mounted/🦀️.rs`. It builds two Terrain window instances with a populated `active_utility_by_window_id`, runs `window_measures`, `window_engagements` and `tool_measures` twice through the registered fixture, asserts the register is intact afterwards and retires it. If the suspected path is the fault, this test fails natively with the exact assert message.

## Next step

Run that test with the serialized runner. If it panics, the message names the assert to fix in `return_scoped_alias` / `ScopedWindowView::drop`; if it passes, the fault is wasm-only (host-sent register shape or the trace-cursor map) and needs the guest log from the live Chrome run.

## Files

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧩️mounted/🦀️.rs` (new test)
