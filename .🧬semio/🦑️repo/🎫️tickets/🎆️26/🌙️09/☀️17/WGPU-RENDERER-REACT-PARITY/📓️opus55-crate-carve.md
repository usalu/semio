# Crate carve for the source boundary gate (semio-ui-wgpu-paint / semio-ui-wgpu-events)

Status: INCOMPLETE. Files are wired but not compiling. The integration agent owns the remaining work. No cargo is running from this session.

## Gate
`readRustUnitSourceClosureV1` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️inputs/🟦️.ts` follows the `#[path]` closure from the crate entry and sums active lines, files and bytes. The caps are not raised.

## New crates (in-place; crate root is the existing folder's `🦀️.rs`, `[lib] path = "../../🦀️.rs"`)
Each has Cargo.toml (4000 active lines / 32 files / 262144 bytes) and `📋️project.json`.
- `semio-ui-wgpu-events-commit` from `⚡️events/🧮️control-commit-rules`
- `semio-ui-wgpu-events-focus` from `⚡️events/🧭️event-model-and-focus` (depends on events-commit)
- `semio-ui-wgpu-paint-widgets` from `🖌️paint/📶️progress-celebrate-and-widget-painters`
- `semio-ui-wgpu-paint-sync` from `🖌️paint/🔄️interactive-state-sync`

Seams not carved: the remaining events modules all `impl EventRouter` (a type in `🧲️overlays-drag-and-commands`), which is not allowed from another crate.

## Edits
- Parents: the `#[path]` mods for these folders were replaced by `use`/`pub use` from the new crates (`⚡️events/🦀️.rs`, `🖌️paint/🦀️.rs`). Parent Cargo.toml gained the path deps.
- New crates: `use super::*;` replaced by explicit imports; `pub(super)` made `pub`.
- Moved code: `text_field_key` + `text_input_kind` (commit rules to focus crate); `TREE_NAV_ROW_CAP`, `TREE_NAV_DEPTH_CAP`, `tree_row_owner`, `collect_visible_rows`, `tree_visible_rows`, `FocusState::roving_stop` (`🌳️tree-roving-and-keys` to focus crate). `🌳️tree-roving-and-keys` now imports them from `semio_ui_wgpu_events_focus`.
- `🎛️control-nodes`: `crate::progress_celebrate_and_widget_painters::hover_owner` became `semio_ui_wgpu_paint_widgets::hover_owner`.
- Registered: `🧰️framework/Cargo.toml` members (4), `📋️build-owners.json` (4 owners), `UI_WGPU_PACKAGES` in `🖱️ui/📦️packages/🦀️rust/📜️script.ts`. Root `Cargo.toml` has no wgpu entries.
- Lockfiles: both `Cargo.lock` files already contained the 4 crates when checked (written by other cargo runs); diff showed no external version changes (only an unrelated `semio-ui-wgpu-widgets` removal by another agent). My own `update --offline` was cancelled by someone else.

## Check result (events-commit, events-focus, events parent)
`semio-ui-wgpu-events-commit` fails, 11 errors (events-focus and parent not reached); the other two crates' errors are not seen yet. Verbatim causes:
- E0433 `semio_ui_wgpu_controls` unresolved (lines 67, 234): add dependency `semio-ui-wgpu-controls`.
- E0425 not found in scope: `INTENT_VALUE_FIELD` (353, 384; from `semio_ui_action`), `constrain_number_input` (126, 161; from `semio_ui_wgpu_widget_kit`), `slider_control_presentation`, `number_stepper_segments`, `ring_t_at` (from `semio_ui_wgpu_layout::layout`), `ring_orb_hit`, `icon_select_rows` (likely controls/layout).
Fix: add imports (`semio-ui-action`, `semio-ui-wgpu-controls`, `semio-ui-wgpu-layout`, `semio-ui-wgpu-widget-kit` deps to events-commit Cargo.toml). Expect the same kind of errors in events-focus, paint-widgets and paint-sync (full parent `use` block was copied into the paint crates; unused-import warnings expected).

## Remaining
1. Fix the compile errors above, then check parents, new crates, `semio-ui-wgpu-engine`; run `semio-ui-wgpu-paint-laws` and `semio-ui-wgpu-events-laws`.
2. Restore `🖌️paint/📦️packages/🦀️rust/Cargo.toml` `maxActiveLines` from 4300 to 4000 (not done).
3. Count active lines. Estimates only: paint about 4209 - 846 = 3360, events about 4027 - 914 = 3100; both above the 3000 target. Candidates for more headroom: paint `🖼️ui-image-sources-skeleton-and-overlays` (needs cursor consts and `retained_fixed_output`) and `🔤️retained-glyph-and-node-cursors`.
4. Check `semio-ui-wgpu-events` downstream users of items that were `pub(super)` (now `pub`).
