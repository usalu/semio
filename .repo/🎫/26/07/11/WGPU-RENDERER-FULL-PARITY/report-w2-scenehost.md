# W2 — SceneHost Bridge (WS1) — verified complete

The implementing agent's final chat message was a non-answer ("waiting for a monitor"), but its actual
work had already landed and auto-committed (commit `af14f953b9`, tag 353) before that message. This
report is written from direct verification of the committed code, not the agent's own summary.

## What's confirmed in place
- `ui/wgpu/rs/lib.rs`: `pub struct SceneSlot<'tree>` (borrowed payload, not just a rect) and
  `pub trait SceneHost` (paint-only contract) both exist at line ~16044/16067.
- `Ui::frame(...)` (line ~16699) takes `scene_host: Option<&mut dyn SceneHost>` as a per-frame
  parameter — not a stored field. `Ui::set_scene_host`/stored `scene_host` field are gone.
- `framework/renderer/wgpu/rs/lib.rs`: `struct FrameworkSceneHost<'ctx>` (line ~5774) implements
  `ui_wgpu::SceneHost` (line ~5788) and is constructed and passed into `engine.frame(window_id,
  viewport_w, viewport_h, ctx.atlas, ctx.icons, Some(&mut scene_host))` (line ~5876).
- `paint_unbridged_scene_and_image_leaves` — **deleted** (zero matches repo-wide).
- The entire legacy immediate-mode path — `ui_node_to_widget`, `render_widget` (framework-side wrapper),
  `measure_ui_node`, `layout_vertical`, `layout_horizontal` — **deleted** (zero matches in
  `framework/renderer/wgpu/rs/lib.rs`; `interpreter`'s import list was trimmed accordingly, confirmed
  via `git diff`).

## Test verification (run directly, not taken from the agent)
- `cargo test -p ui_wgpu --features engine`: **205/205 PASS**.
- `cargo test -p semio-framework-renderer-wgpu --lib`: **232 passed, 2 failed**
  (`shell::command_registry_tests::build_command_panel_ui_groups_rows_under_category_headers`,
  `shell::ui_prefs_themes_i18n_tests::persist_ui_prefs_if_changed_is_idempotent_when_nothing_changed`).
  Both failures are in the `shell` region owned by the concurrently-running W2/W3 shell-audit
  workstream, not in `interpreter`/`scene_slots`/`engine`/`paint` — treated as that workstream's
  in-flight state, not a SceneHost regression. Will be re-verified once shell-audit reports.

## Assessment
The core architectural goal of this ticket — a single retained-mode layout/paint source of truth for
window content, with scenes/images painted through a real host bridge instead of a second
divergent layout pass — is done. This unblocks accurate scene/image rect placement under
`Field`/`Section`/`Group`/`Tree` ancestors and removes the dead-weight legacy interpreter path.
