# w3-atlas-sharing-fix — final report

## New `Ui` API surface (`ui/wgpu/rs/lib.rs`, `engine` region)

`Ui` no longer owns a `FontAtlas`/`Option<IconAtlas>`:

```rust
pub struct Ui {
    windows: HashMap<String, UiWindow>,
    shell: Shell,
    theme: Theme,
    scene_host: Option<Box<dyn SceneHost>>,
    pending_commands: Vec<UiCommand>,
}
```

- `Ui::new()` — no longer builds `FontAtlas::builtin()`/sets `icons: None`.
- `set_icons` method — removed entirely (nothing left to set it on).
- `apply_tree(&mut self, window_id: &str, ui_node: &UiNode)` — unchanged. Confirmed `UiTree::apply_tree`/`reconcile` never measures text, so no atlas parameter needed there.
- `frame`'s new signature:
  ```rust
  pub fn frame(&mut self, window_id: &str, viewport_width: f32, viewport_height: f32, atlas: &mut FontAtlas, icons: Option<&IconAtlas>) -> Option<&DrawList>
  ```
  Body now threads the caller-supplied `atlas`/`icons` into `window.layout.compute(..., atlas, ...)` and `paint_tree(..., atlas, icons, ...)` instead of `&mut self.atlas`/`self.icons.as_ref()`.

## `paint`/`flex` — no signature changes needed

Verified both already take the atlas as a parameter, not a struct field: `flex::LayoutEngine::compute(&mut self, tree, root, atlas: &mut FontAtlas, theme, ...)` and `paint::paint_tree(tree, root, theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw)`. `engine` was the only region storing the atlas as a field.

## Test updates (`engine::tests`)
All 5 call sites constructing `Ui` and calling `frame`/using `retained_stats` now create `let mut atlas = FontAtlas::builtin();` and pass `&mut atlas, None`.

## Re-exports
No changes needed — `FontAtlas`/`IconAtlas` were already curated exports; only the method signature changed, not the path.

## `interpreter` follow-up (made directly, not filed)
`framework/renderer/wgpu/rs/lib.rs`, `RetainedEngineCutover` sub-region of `interpreter`: `render_ui_node`'s `UI_ENGINE.with` closure changed `engine.frame(window_id, viewport_w, viewport_h)` → `engine.frame(window_id, viewport_w, viewport_h, ctx.atlas, ctx.icons)`. `FrameworkWidgetContext<'a>` already carries `pub atlas: &'a mut FontAtlas` and `pub icons: Option<&'a IconAtlas>` — exactly the shell's real, already-GPU-uploaded instances. Rewrote the now-stale `RetainedEngineCutover` doc comment to record the fix as resolved.

## Build/test results
- `ui_wgpu --features engine`: `cargo check` clean. `cargo test`: **152 passed, 1 failed** — matches baseline exactly (the 1 failure is the pre-existing, unrelated `component::ui::ui_node_wire_format_tests::scene_records_serialize_to_golden_json` TiledMap JSON golden-string mismatch).
- `semio-framework-renderer-wgpu --lib`: `cargo check` clean. `cargo test`: **178 passed, 1 failed** (baseline was 147/0). The 1 failure, `shell::chrome_overlays_tour_tests::render_engagement_input_click_accepts_ghost_completion`, panics inside `ShellState::render_engagement_input` on a test-local `gpu_free: Option<GpuContext> = None` — entirely in `shell`, has nothing to do with `render_ui_node`/`UI_ENGINE`/`Ui::frame`/atlas threading; confirmed by inspecting the test body (never touches the retained engine). **This is `w3-overlays-chrome-polish`'s in-progress work (test name matches their scope: chrome_overlays_tour_tests, engagement ghost completion) — flag for them to fix, not a regression from this fix.**

## Confidence on retained-mode text/icons rendering
Confident the PLUMBING gap is now closed: `Ui::frame` receives the exact same `FontAtlas`/`IconAtlas` instances the shell already uploads every frame for chrome/dock/panel text, so retained-mode glyph/icon lookups during `paint_tree` will resolve against the real, populated, GPU-resident atlas instead of a private empty one — the non-deterministic-clobber risk AND the "never uploaded" blank-text risk are both eliminated by construction. Residual risk: could not launch the winit app in this sandbox, so glyph positioning/UV correctness under real GPU rendering is unconfirmed — that's Wave 4's job. The 6 known-gap kinds (Field/Section layout divergence, NumberStepper border nesting, etc.) are unrelated to this fix and remain as previously documented.

## Files touched
- `ui/wgpu/rs/lib.rs` — `engine` region (`Ui` struct, `Ui::new`, removed `set_icons`, `Ui::frame` signature+body) and its `#[cfg(test)] mod tests`.
- `framework/renderer/wgpu/rs/lib.rs` — `interpreter`'s `RetainedEngineCutover` sub-region only.

No other files touched; `Cargo.toml` untouched in both crates.
