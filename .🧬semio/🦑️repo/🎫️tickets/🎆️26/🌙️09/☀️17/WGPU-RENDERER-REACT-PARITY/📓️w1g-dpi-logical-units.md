# 📐️ W1g — DPI / logical units (P0 root cause: "elements placed totally different")

Packet W1g of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Implemented 2026-09-18.
Source: `📓️audit-shell-window-system.md` §0.3 — the single highest-confidence finding, reached
independently by two audit passes.

---

## 1. The bug, precisely

Before this packet the wgpu renderer mixed two units in one coordinate space:

| Owner | Unit before | Source |
|---|---|---|
| `ShellState::screen_w`/`screen_h` (the content root) | **physical** (`css * dpr`) | `🧊️renderer/🦀️.rs` `resize` + `boot_runtime`, `🌐️browser-worker/🦀️.rs` boot |
| winit pointer positions | **physical** (winit's own contract) | `🪟️winit-app/🦀️.rs` `normalize` |
| browser pointer positions | **physical** (`offsetX * devicePixelRatio`) | `🚚️browser-frame-transport/🟦️.ts` `physical()` |
| `chrome_px()`, `navbar_height`, `footer_height`, `control_height`, `stroke_hairline`, resize-handle hit widths, every font size | **logical/CSS** — taken verbatim from the SAME generated `ui_styling` tokens React's DOM consumes | `🎨️theme/🦀️.rs`, `🔤️tokens/🦀️.rs` |
| glyph atlas raster size | **logical** (`ensure_glyph(ch, size_px)` rasterised at `size_px`) | `📝️text/🦀️.rs` |
| projection divisor (`globals.screen_size`) | **physical** | `🧊️gpu/🦀️.rs` → `update_globals` |

Hit-testing was internally consistent (root and pointer were both physical), which is why the audit
found no desync *inside* the dock module — the defect is upstream of it. At `scale_factor == 1.0`
nothing is visible. At 2× (this environment's own macOS Retina host) the canvas is 2560×1600 while
the navbar is still 22.4 tall, a tab chip is still 22.4 tall, and a split resize handle is still
~10 wide — every fixed-size chrome element renders and grabs at `1/dpr` of its intended size
against a full-size canvas, and text is rasterised at 1× then stretched 2×. That is exactly
"ui elements are placed totally different, the window system doesn't work".

One consequence worth naming separately: `plan_dock_windows` sets
`dock.mobile = screen_w <= MODE_DOCK_MOBILE_MAX_WIDTH_PX` (767), the exact mirror of React's
`UI_MOBILE_MEDIA_QUERY`. With a physical `screen_w`, a 375-CSS-px phone at 3× measured 1125 and the
dock's mobile mode could never engage at all — the media-query parity was structurally unreachable,
not merely off by a constant.

## 2. The unit model now

**One rule: everything is LOGICAL (CSS) pixels except three named consumers.**

| Stays PHYSICAL (device pixels) | Why |
|---|---|
| `GpuContext::width`/`height` + `wgpu::SurfaceConfiguration` | the swapchain is a device-pixel resource |
| every `set_scissor_rect` / `set_viewport` argument | `wgpu` clips in device pixels |
| `FontAtlas::pixels`/`color_pixels` and `GlyphEntry::{atlas_x, atlas_y, width, height}` | atlas texels are device pixels so text is crisp |
| the icon atlas cell edge | same |
| the `resize` wire event's `width`/`height` (browser) and `WindowMetrics::physical` (native) | they *are* the surface extent |

Everything else — `ShellState::screen_w/h`, every `Rect` in a draw list, every hit rect, every
pointer/wheel coordinate on the wire, `GlyphEntry::{advance, bearing_x, bearing_y}`,
`GlyphEntry::logical_width()/logical_height()`, `measure_text`, and of course every theme
constant — is logical, exactly like React's DOM.

The scale factor reaches production through exactly **four** doors:

1. `GpuContext::resize` → `SurfaceConfiguration.width/height` (the swapchain).
2. `GpuContext::resize` → `UiPipelines::set_surface_scale` → every `set_scissor_rect`/`set_viewport`.
3. `AppRuntime::resize` → `FontAtlas::set_raster_scale` (glyph raster size).
4. boot → `icon_atlas::build_icon_atlas_scaled` (icon cell size).

The projection needs **no multiplication**: `update_globals` is handed the LOGICAL extent, so
`ndc = pos / screen_size` maps a logical draw list onto the full physical surface for free.

## 3. Seams changed

### `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/`

- **`📝️text/🦀️.rs`**
  - `GlyphEntry` (`:24-57`) — new `raster_scale` field; new `logical_width()`/`logical_height()`;
    docstring states the texel-vs-logical split.
  - `RasterizedGlyph` (`:196-206`) — new `raster_scale` (shaped path honours the atlas scale, the
    fixed 8×16 ASCII bitmap fallback cannot and reports `1.0`).
  - `FontAtlas::raster_scale` field (`:267`), `set_raster_scale` (`:401`), `raster_scale()` (`:420`).
    Changing the scale clears the glyph cache, zeroes and rewinds both pages and raises both dirty
    flags — a display change re-rasterises and fully re-uploads; re-setting the same scale, or a
    non-finite/non-positive one, is a no-op.
  - `ensure_glyph` (`:437`) — keys on the **device** size `quantize(size_px * raster_scale)` and
    rasterises there; `render_resolved` (`:485-494`) and `rasterize_shaped_glyph` (`:509`) divide
    `advance`/`bearing_*` back to logical.
  - `measure_text` (`:581`) — `glyph.logical_height()` instead of the raw texel height.
- **`🧊️gpu/🦀️.rs`**
  - `GpuContext` (`:222-232`) — new `logical_width`/`logical_height` beside the physical pair, with
    the unit contract in the docstring.
  - `from_surface` (`:287`, `:312-313`) — publishes the surface scale to the pipelines and seeds the
    logical extent.
  - `resize` (`:338-353`) — params renamed to logical; **a scale-only change now lands** (the old
    early-return compared physical extent only, so a same-size display swap silently kept the old
    projection divisor and scissor scale).
  - `encode_prepared_draw_scalar` (`:598-599`) — the projection divisor is now the LOGICAL extent.
  - New accessors `logical_width()` / `logical_height()` / `scale_factor()` (`:775-788`).
- **`🖍️draw/🦀️.rs`**
  - `UiPipelines::surface_scale` (`:2049`, seeded `:2670`), `set_surface_scale` (`:2676`),
    `physical_scissor` (`:2683`).
  - `set_pass_scissor` (`:2126`) — takes the scale; converts the logical clip (and the logical
    full-viewport fallback) into the physical rectangle `wgpu` wants.
  - Scene-pass viewports scaled: batch path `:2815-2837`, `encode_prepared_world_instance` `:3225-3247`,
    `encode_prepared_world_line` `:3274-3292`.
  - Remaining bare scissor resets scaled: `:2886`, `:2964`, `:3055`.
- **`🖌️paint/🦀️.rs` `:120-133`** and **`🪀️widgets/🦀️.rs` `:582-583`, `:598-599`, `:614-615`** — the
  four glyph-quad call sites now size the quad from `logical_width()`/`logical_height()`.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`

- **`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`**
  - `AppInteractionState::resize` (`:14040-14047`) — content root is the LOGICAL extent; `dpr` is
    now `_dpr` and documented as never reaching layout. **This is the P0 one-line root cause.**
  - `AppRuntime::resize` (`:11025-11056`) — new inherent method that shadows the `Deref` route so the
    glyph atlas raster scale follows the window's scale factor before the content root moves.
  - `boot_runtime` (`:14392-14400`) — `atlas.set_raster_scale(dpr)`,
    `icon_atlas::build_icon_atlas_scaled(dpr)`, `shell.screen_w/h = css_width/css_height`
    (was `css * dpr`).
- **`🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs`**
  - `BrowserRendererBootstrap` (`:528-536`) — carries `dpr`; `width`/`height` documented as physical.
  - Boot phase 0 (`:548-551`) sets the atlas raster scale, phase 1 (`:558`) builds the scaled icon
    atlas, phase 5 (`:575-576`) divides the physical canvas extent into the logical content root.
  - `semio_wgpu_worker_bootstrap` (`:712`) threads `dpr` into the bootstrap.
- **`🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs`**
  - New `pointer_scale_factor` (`:689-702`) and `normalize` (`:704-712`, `:731-732`) — winit hands
    PHYSICAL positions; cursor, mouse-button, wheel and touch coordinates are divided at ingress, so
    the native half matches the browser half.
- **`🎯️targets/🧊️wgpu/🎮️input-wire/🦀️.rs` `:12-16`** — module docstring restates the wire contract
  (pointer/wheel logical, `Resize` the one physical carrier).
- **`🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`**
  - `physical()` is now used for the SURFACE EXTENT ONLY; `pointerFields` no longer takes or applies
    `devicePixelRatio`; the `wheel` branch stops scaling its position. `resize` is unchanged.
- **`🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts`**
  - `publishMetrics` extracted, and a `matchMedia("(resolution: Ndppx)")` watcher added that re-arms
    itself on every change — a `ResizeObserver` never fires on a density-only change (window dragged
    to another display, browser zoom), so the surface, the projection and the atlas raster used to
    stay at the boot density forever. Torn down on `pagehide`.
- **`🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs`**
  - `icon_cell_size` (`:19-24`) — `ICON_SIZE * round(scale).clamp(1,3)`; 3× is the ceiling that keeps
    250 icons × 16 columns inside the fixed 2048 UV space (16 rows × 72 = 1152).
  - `rasterize_svg` takes the cell edge; `build_icon_atlas_scaled` (`:46`) is the real constructor and
    `build_icon_atlas()` is the 1× alias. UVs stay normalised against `ICON_ATLAS_TEXTURE_SIZE`, so no
    draw call changed.

### Shared oracle

- **`🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json`** — `why` header rewritten; the
  `pointer-move-on-a-retina-surface` and `wheel-over-the-preview` rows now assert **zero** dpr
  multiplications (they were the rows that pinned the bug in place); the `resize` row's `why` names
  it as the one physical carrier. Answered from both sides by
  `🧪️tests/🎮️wgpu-browser-input-wire/🦀️.rs` and `…/🟦️.ts`.

## 4. Tests

| Test | File | Asserts |
|---|---|---|
| `window_metrics_answer_one_logical_extent_at_every_density` | `🧱️elements/🐚️Shell/🧪️tests/📐️wgpu-dpi-logical-units/🦀️.rs` | one CSS viewport at 1/1.5/2/3× → one logical extent; the physical extent is what grows |
| `chrome_layout_is_independent_of_the_scale_factor` | same | navbar/footer/control heights, `ShellState::body_rect`, `DockState::stack_corner_tab_bar_rects` rects + per-tab chip widths, and every `HitKind::DockSplit` rect registered by `DockState::register_resize_hits` are **byte-identical** across all four densities |
| `resize_handle_hit_width_stays_a_logical_constant` | same | every split grab target stays ≥ 8 logical px at every density |
| `atlas_raster_size_scales_with_the_scale_factor_while_metrics_stay_logical` | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs` | at 2× the texel extent doubles (±2) while `logical_width/height`, `advance` and `measure_text` stay the 1× values |
| `changing_the_scale_factor_re_rasterises_and_re_setting_it_does_not` | same | a scale change drops the cache and raises `dirty`; an identical, zero or NaN scale does not |
| `the_glyph_cache_key_is_the_device_size_not_the_logical_one` | same | 16 logical px at 2× caches under key `('A', 32)` |
| `never scales a pointer coordinate by the device pixel ratio` | `🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts` | replaces the old "scales exactly once" law |
| `scales only the resize extent by the device pixel ratio` | same | the one remaining `physical()` consumer |
| `never scales a scroll delta or its position` | same | position joined the delta |

Test module mounted at the tail of `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
(`mod dpi_logical_units_tests`).

## 5. Verification

All run in the foreground. Logs under `🗑️generated/w1g-*.txt`.

| Gate | Result |
|---|---|
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --keep-going` | **0 errors** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` | **0 errors**, `Finished` (`w1g-native-check.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --profile test --keep-going` | **0 errors** — the `cfg(test)` route that type-checks the new law module (`w1g-native-test-check.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going` | **0 errors**, `Finished` — covers the browser-worker `dpr` path (`w1g-wasm-check.txt`) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib dpi_logical_units` | **3 passed, 0 failed** (`w1g-native-dpi-test.txt`) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib input_wire` | **3 passed, 0 failed** — the Rust half of the shared fixture (`w1g-native-input-wire-test.txt`) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- text::tests layout` | all 11 `wgpu::text::tests` pass, incl. the 3 new ones; 60 passed / 5 failed (`w1g-ui-text-test.txt`) |
| `vitest 🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts` | **22 passed** (`w1g-ts-input-wire-test.txt`) |
| `nx run @semio-tech/framework-renderer-wgpu:lint` | pass |
| `nx run @semio-tech/framework-renderer-wgpu:check-browser-worker` | pass |
| `nx run @semio-tech/framework-renderer-wgpu:generate-browser-boot --skip-nx-cache` | pass; the regenerated `🚀️browser-boot/🤖️generated/🟨️.js` carries the `dppx` watcher |

**The 5 ui-crate test failures are a peer's, not this packet's.** They are
`engine::large_layout_and_shaping_job_keeps_every_observed_slice_below_eight_ms` (a wall-clock budget
law; the machine was swap-thrashing under a full agent fleet and observed 67 ms),
`layout::tree_row_rect_tests::every_fixture_row_is_published_at_the_rect_the_painter_draws_it_at`
(a collapsed row's WIDTH, `got (0,0,0,0) want (0,0,200,0)`) and three
`mounted_layout` alignment laws (`align:center` expected 19.0, got 19.2 — the peer's own brand-new
`text::line_height` ramp, landed mid-packet, against a not-yet-updated expectation). None involve the
scale factor, and W1g provably cannot move any of them: at `raster_scale == 1.0`,
`GlyphEntry::logical_width()`/`logical_height()` are `width as f32 / 1.0`, bit-identical to the
`width as f32` they replaced, and `set_raster_scale` is never called by any of these tests.

**Concurrent-churn note.** Three peer refactors crossed this packet in the same crates: a `🎨️theme`
palette change (`Rgba::TRANSPARENT`, `from_chrome` arity), a `📐️flex`/`📌️mounted_layout`
`LayoutJobStage` rework (81 transient errors), and a `🎞️Scenes` blend-mode edit (13 transient
errors, all `canvas_lum`/`canvas_set_sat` not-in-scope from a mid-edit snapshot). All cleared on
their own; none touched a W1g file.

**Build-environment note for whoever re-runs this.** Under a full agent fleet the machine's 5 GB swap
sat at ~4.3 GB used with ~128 MB free pages, and macOS jetsam `SIGKILL`ed (`exit=137`) every
`cargo check` of this crate — detached, backgrounded and foreground alike, at a different crate each
time. Passing `CARGO_INCREMENTAL=0`/`CARGO_PROFILE_DEV_DEBUG=false` makes it *worse*, because it
changes the fingerprint and discards the peers' shared-build-dir artifacts. The recipe that worked:
plain env, `-j 2`, foreground, re-run until it gets through (each pass keeps the crates it finished).

## 6. Remaining gaps

1. **Icon atlas does not re-rasterise on a post-boot scale change.** `build_icon_atlas_scaled` is
   called once at boot on both hosts. Re-running 250 `usvg`/`resvg` rasterisations inside a resize
   step would blow the interactive deadline, and the atlas owner (`AppRuntime::icons`) would also
   need a re-upload step. Until then, dragging a window onto a higher-density display keeps
   boot-density icons (they scale smoothly, they just are not re-sampled). The glyph atlas DOES
   re-rasterise. A bounded multi-step icon re-raster job is the clean fix.
2. **Icon raster scale is capped at 3×** by the fixed 2048 UV space with 16 columns. A 4× surface
   falls back to 3× cells. Raising the cap means paging the icon atlas.
3. **`quantize_size` rounds the device size**, so a fractional scale factor (1.5) quantises 11 px
   text to 17 device px rather than 16.5 — a sub-pixel metric difference from the browser, invisible
   in practice but not bit-identical to React.
4. **Nothing re-runs the glyph atlas's own upload budget check after a scale change.** The
   re-rasterisation itself is lazy and per-glyph, but the `FrameFinishPhase::GlyphUpload` step
   re-uploads the WHOLE 2048×2048 page when `take_dirty()` fires. That is already the existing
   behaviour for any atlas growth; a scale change just makes it certain on the frame after a display
   swap. Worth a look if a display swap ever shows a frame hitch.
5. **No live-surface confirmation yet.** Every gate here is a compile or a unit law. The packet has
   not been run against a live `dev`/`serve` shell on the 2× host, which is where "elements placed
   totally different" was observed — that visual confirmation is the next thing to do.
6. This packet fixes the *unit* mismatch only. The audit's other P0s (the hardcoded 2-slot panel
   anchor model §3, the single-axis production layout engine, the dead third tab button, the missing
   agent-approval overlay) are untouched and still explain their own share of "placed differently".
