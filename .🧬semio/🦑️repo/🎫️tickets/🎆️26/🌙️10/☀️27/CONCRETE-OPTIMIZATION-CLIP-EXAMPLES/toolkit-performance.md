# Toolkit Performance: Raster Heat Map And Memoized Live Widgets

Scope after the coordinator's change of plan: only the **heat map rasterization** and the **live-widget memoization** (`LiveGroup`/`_live` and the `key` wiring). Daylight in `room_section`, the demand label and the `plot_axes` tick placement belong to other agents; my drafts for them were reverted from both modules and are kept for reference in `toolkit-daylight-demand-draft.md` (verified on stills before the revert).

Files changed: `C/🎨️visuals/🏠️room/🐍️.py` and one line in `C/🎨️visuals/🐍️.py` (`live_marker` returns `self._widget(build, _room.design_key(trackers))`). No scene file was touched. Scripts (private output under `🗑️generated/toolkit/`): `heatmap-bench.py`, `heatmap-probe-image.py`, `heatmap-patch-raster.py`, `smoke-toolkit.py`, `smoke-toolkit-before.py`, `capture-toolkit.py`, `toolkit-showcase.py`.

## 1. Raster heat map

Why: `heat_map` built ~550 merged rectangles per frame (1,681 cells before merging) and Cairo redrew all of them on every frame the map moved or faded.

What changed (`heat_map`, room module):

- The map is a **bilinearly interpolated raster of the model surface** sampled at the pixel centres of the axes rectangle, coloured through a 256-step LUT of the existing `ramp` (`colour.low … colour.high`), alpha = `opacity`. Smooth (was 24 steps), no seams.
- `image` is an `mn.Group` of `HeatStrip` (an `ImageMobject` subclass), **one strip per WWR column** (41, same iteration order as before, so `LaggedStart(*[FadeIn(c) for c in heat["image"]])` keeps working); `columns=False` gives one strip. The unused `steps=` parameter is gone.
- **Registration is pixel exact:** strip edges are the rounded pixel positions of `axes.c2p` of the column boundaries (half way between grid values, clamped to the data range, like the old cells), rendered 1:1, so neighbours share whole-pixel boundaries (no seams) and the corners are the `c2p` corners of the data range.
- **Pillow 13 problem solved without touching Manim:** Manim 0.22's `Camera.display_image_mobject` crashes with the installed Pillow (`Image.fromarray(np.zeros(...), mode="RGBA")` → "Invalid mode for data type"), which is why the old note said `ImageMobject` was unusable. `RoomVisuals.__init__` now passes `camera_class=RasterCamera` (a `manim.Camera` subclass in the room module) that composites an image mobject's bounding box on whole pixels with PIL `alpha_composite` of just that region (also faster than Manim's full-frame composite). Axis-aligned images only. **Callers must not pass their own `camera_class`** (nobody does).
- **Opacity:** `HeatStrip` answers the vector calls scenes use for fades: `set_opacity(alpha)`, `set_fill(opacity=…)` (sampling's `_fader` calls this on every family member; it crashed on a bare `ImageMobject`), `set_stroke(...)` (ignored), `get_fill_opacity()`, `get_stroke_opacity()`; `FadeIn`/`FadeOut`/`LaggedStart` use Manim's own `fade`. The `opacity=` argument is baked into the pixel alpha; `set_opacity(1)` restores exactly `opacity`.
- `layout_conflicts` only inspects `Text`, images are ignored (0 conflicts on the captured algorithms and sampling frames). Contours, labels, star and legend stay vector objects on top (callers set the z-index).

Callers audited (`heat_map(` in `C/*/🐍️.py`): `🎲️sampling` (z −3, `LaggedStart(FadeIn(column))`, `_fader`) and `🧬️algorithms` (z 0, `LaggedStart(FadeIn(column))`, `FadeIn/FadeOut(heat["image"])`, `_fade`, `map_parts`, fog `Rectangle`). No caller change needed.

Benchmark (`heatmap-bench.py`, 1280×720, map + contours + star + legend + axes, whole scene redrawn per frame; machine loaded by other agents, absolute numbers inflated):

| | build | ms/frame | mobjects in `image` family |
|---|---|---|---|
| before (merged vector runs, 24 colours) | 16.3 s (cold model cache) | **199 ms** | 548 |
| after (raster strips) | 3.3 s (cold model cache) | **47 ms** | 42 |

About 35 ms of the remaining frame is axes, tick glyphs and contour chips; the map itself went from ≈ 165 ms to ≈ 12 ms. Stills: `heatmap-bench.png`, `heatmap-en.png`, and frames under `frames/Beat6_EvolutionarySearch/` (`…-map_2.png`, `…-gradient_result.png`) and `frames/Beat2_GridRandomSampling/` (`…-reveal_2.png`): star, markers, sample points and contours sit on the right cells, no seams.

Scene note (not mine): `🧬️algorithms` centres its fog `Rectangle` on `self.ax.get_center()`, which includes the tick marks and is ~0.03 units left/below the plotting rectangle, so a 1–4 px rim of the map at the top and right edge stays un-fogged. Use `(axes.c2p(x_lo, y_lo) + axes.c2p(x_hi, y_hi)) / 2`.

## 2. Memoized live widgets

`LiveGroup(builder, key=None)` and `RoomVisuals._live(builder, key=None)`:

- `key` is a cheap callable returning a hashable snapshot of **everything the builder reads** (tracker values, counters; language is constant). `refresh()` (the updater) calls it every frame and rebuilds only when the snapshot differs from the last build; unchanged frames reuse the child list.
- `LiveGroup.memoize(key)` sets the key after construction; `RoomVisuals._widget(builder, key)` = `self._live(builder).memoize(key)`. It goes through `_live` on purpose: `🎲️sampling` overrides `_live(self, builder)` with `SnapshotSafeGroup`, which keeps working unchanged (scenes adopting keys can call `self._live(build).memoize(key)` until that override takes `key`).
- `design_key(trackers)` returns the key of the four design trackers; used by `room_section`, `facade`, `plan`, `energy_bars` and `live_marker`.
- **Animation safety** (deferred and deepcopy logic unchanged): updaters are suspended while an animation runs on the widget, so children are frozen exactly as before; `resume_updating` (called when any animation on the widget or a parent group finishes) additionally **invalidates the snapshot**, so a widget that was faded, shifted, transformed or indicated is rebuilt once right after, identical to the old rebuild-every-frame result. `__deepcopy__` calls `refresh()` first and drops builder and key from the copy (static snapshot).
- Verified by real playback (`smoke-toolkit.py`, 320×180, 3 fps) of 🧪️experiments, 🧬️algorithms and 🏆️pareto (all exit 0), and by the showcase `orientation` board (FadeIn on live widgets, `set_design` moves the marker, room, plan and bar together).

Smoke wall times (same scene source, machine under load from other agents in all runs; only the ratio is indicative):

| scene | scene time | before | after |
|---|---|---|---|
| 🧪️experiments `Beat3_FactorialExperiments` en | 280.7 s | 973 s | **260 s** |
| 🧬️algorithms `Beat6_EvolutionarySearch` en | 317 s now (393 s when measured) | 625 s (coordinator's earlier run) | **290 s** (heat map raster + memoization) |
| 🏆️pareto `Beat5_ParetoFront` en | 244 s | not measured (scene file did not run before) | 287 s |

What memoization cannot remove: a mobject with an updater is a *moving* mobject for Manim, so its (unchanged) vector shapes are still drawn every frame; only the Python rebuild is gone (≈ 3–6 ms drawing per section at 1080p).

### Scene-level builders that would benefit from a `key` (scene agents: `self._live(build).memoize(lambda: (…))`)

Pass the tuple of everything `build` reads; for builders reading only design trackers use `_room.design_key(trackers)`.

- 🎲️sampling: counter chips (`_runs`, `_count` → `lambda: (self._runs,)`), WWR/d chip from `self.T` (`_room.design_key(self.T)`), the thumbnail picture `lambda: self.thumbnail(_room.values(T), …)` (`design_key(T)`), the live `build` blocks near lines 222, 238, 249, 341.
- 🏆️pareto: the cost chip (`design_key(trackers)`), `build`s near 495, 532, `trail` (563), `knee_ring` (581), `iso`/`slider` (642).
- 🧬️algorithms: `make` (126), `build` (177, 288, 649), `slopes` (315), `change_row` (326), `steps` / `verdict` (367, 369: key `(reader(),)`), `_chips_live` (420), the per-row thumbnail (653: `(self.runs.get_value(),)`), `knob` (719), `number` (736: `design_key(tracker)`).
- 📉️extrema line 336; ⚖️index line 127 (own holder updater: replace by `_live` + key); 🧪️experiments uses `always_redraw` (141, 166): same idea, rebuild only when the tracked values change.

## 3. Handed over (not done here)

Daylight zone and lamp contrast, thermal-demand-to-electricity label, and the `plot_axes` y tick placement (algorithms ticks ended at x ≈ −7.09; `FRAME_X_LIMIT` is 7.05): a tested draft with the numbers (a 0.1 gap instead of 0.2 plus a clamp gave 0 conflicts on algorithms and sampling) is in `toolkit-daylight-demand-draft.md`. Note for the plot_axes owner: the algorithms scene additionally shifts the labels 0.14 left in `_outdent`.
