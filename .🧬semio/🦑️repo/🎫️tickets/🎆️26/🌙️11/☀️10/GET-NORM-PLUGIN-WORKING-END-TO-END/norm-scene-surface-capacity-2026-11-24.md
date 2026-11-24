# Norm scene surface capacity (2026-11-24)

## Cause
`scene-surface.description` is the accessible description of a canvas, set through `try_describe` (`UiText`, fixed 512 bytes, `UI_TEXT_MAX_BYTES`). It is not a node-count overflow: the string is longer than one fixed text.

`diagram_nodes` in the Norm diagrams window passed `accessible_description(diagram, locale)` as that description. The function enumerates title, basis, description, every axis, every series and one clause per marker (`label (shape) at x; y, utilisation p %`). DIN 4108 (wall, roof, floor, zone and bridge markers with labels like `Bridge tb-window-reveal (categoryA)`, plus basis and dimension markers) and DIN 16798 exceed 512 bytes with that, iso16757 and vdi3805 diagrams carry few markers and stay below. `scene_surface` maps the refusal to `ui.fixed-capacity ... scene-surface.description`, `setActiveExample` is refused and the shell never settles.

The same full text was already published a second time as the chunked sibling text node `<base>.description`, so the canvas description only needs a summary.

## Fix
- `surface_description` (new, `diagrams2d/table-twin`): basis, description, axes with ranges, series and a marker count with the worst utilisation. It does not grow with the marker count; the surface call additionally goes through `UiText::clipped` so a long title or axis label can never refuse admission (the full text stays reachable as the chunked `<base>.description` node and the table twin, which is what `clipped` requires).
- `accessible_description` keeps the complete enumeration (screen-reader `AriaNode` and the chunked text node); both functions share `description_head`, and the three duplicated `t` closures were replaced by the existing `chrome` helper (this also keeps `visual-chrome` under its 4000 active lines source boundary without raising a limit).
- `diagram_nodes` (`diagrams2d/window`) passes the clipped summary to `scene_surface` and the full text to `diagram_text`.
- Framework `scene_surface` is unchanged: the only Norm caller passing a description was this one; every other caller passes `None`.

## Changed files
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/📊️table-twin/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🪟️window/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🧪️tests/🦀️.rs`
- Ticket helpers: `scene-cap-retry.sh` (retries a cargo test while foreign crates are mid-edit), `scene-cap-rebuild.sh` (norm rebuild with exit 75 retry).

## Tests
Crate `semio-s-plugin-norm-app-surface-visual-chrome`, `--lib diagrams2d` (`🗑️generated/scene-cap-t8.log`): 19 passed, 2 failed.
- New `many_marker_diagram_keeps_the_surface_description_within_the_fixed_text_capacity`: 40 DIN-4108-style markers; asserts the full enumeration is above `UI_TEXT_MAX_BYTES`, the summary is below it in en and de, contains the count and worst utilisation, and `render_diagrams` admits the surface. Passes. (Not run against the pre-fix code, because of the long shared-store queue; the first assertion pins the overflow and the production failure is the one in the old probe logs.)
- `accessible_description_names_axes_series_and_markers`, `large_marker_table_renders_the_requested_rows`: pass.
- Not caused by this change: `golden_layers_match_the_committed_fixture` (fixture still holds the old layer wire format, `transform`/`node` vs flat `x/y/text`) and `hit_distances_match_shapely` (`no Python interpreter: SEMIO_PYTHON is unset`; needs `repo dependencies ensure`).

Family examples (`test-family-examples.sh`):
- `[DEBUG] din16798 exit 0: 3 passed, 0 failed`
- `[DEBUG] din4108 exit 0: 4 passed, 0 failed` (first run failed because another editor had a syntax error in `artifact-app-laws`, rerun clean)

## Rebuild and probe
- `build-play-components.ts build norm`: exit 0, published (`🗑️generated/rebuild-norm-scene.log`).
- Probe (`🗑️generated/routes-norm-scene.log`, port 6038, run from the repo root; the probe uses `Path.cwd()` as root): all four routes exit 1, but for a different reason than before.
  - Gone: `ui.fixed-capacity ... scene-surface.description`, `setActiveExample refused` and the din4108 `TimeoutError`. din4108 and din16798 now complete the picker round trip (`selected: No example`, `restored: Compliant ETICS dwelling` / `Compliant office`), draw canvases, and the fault list has no non-debug entry.
  - Remaining: `The ready shell reported browser or guest failures`, caused only by 13 `[DEBUG] ui install ...` console errors per route (including iso16757 and vdi3805, which also fail with 0 non-debug faults). They come from an uncommitted `console.error("[DEBUG] ui install", ...)` in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧺️handle-ui-intake/🟦️.ts` (line 108, edited 04:58 by another agent, not by this ticket). The probe has to be rerun once that line is removed to get exit 0.
