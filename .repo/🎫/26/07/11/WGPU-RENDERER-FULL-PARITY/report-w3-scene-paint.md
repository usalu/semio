# W3 Scene Paint Parity — SceneHost cutover re-verification

Scope: `framework/renderer/wgpu/rs/lib.rs`'s `scenes` region — the 15 `SurfaceKind` `render_*`
functions and shared border/chrome helpers. Compared each against its React host in
`framework/renderer/react/index.tsx` for paint-level parity only.

## Per-surface status
- **Paint2d** — confirmed OK.
- **Table** — confirmed OK on Rust side; React's shared `<Table>` has a dead-code bug (never wires
  `sortColumn`/`sortDirection`), not a Rust regression, not touched.
- **Canvas2d** — **fixed**: selection ring used `theme.accent` (red); React hardcodes an amber
  ring+glow (`rgba(251,191,36,...)`). Added local `CANVAS2D_SELECTION_RING`/`CANVAS2D_SELECTION_GLOW`
  consts, two-pass ring.
- **IconRender** — **fixed**: frame border 1px→2px (`border-2` parity); badge bg `theme.panel`→
  `theme.background`. Extracted `paint_icon_render_chrome` (gpu-free, testable).
- **NodeGraph** — confirmed OK (paint lives in out-of-scope `dock::engine_canvas`).
- **TiledMap** — gap found, **not fixed** (lives in out-of-scope `dock::engine_canvas`): `landStroke`
  alpha hardcoded 0 (invisible), region/hover/route colors use wrong token family (native `Theme`
  lacks `secondary`/`tertiary`).
- **Board2d** — gap found, **not fixed** (out-of-scope `dock`): board canvas theme never synced,
  stuck on light palette.
- **InkCanvas** — **fixed**: item-card bg `theme.panel@0.92`→`theme.background@0.9`.
- **TextEditor** — core buffer OK (shared engine). **Fixed** popups: completions container border +
  active-row accent color; rename-input border+fill; context-menu border stroke.
- **VirtualFileSystem** — **fixed** logic bug: `vfs_glyph_icon` discarded configured icon ids,
  hardcoded `"folder"`. Extension-based file-type table deferred (icon-id availability uncertain,
  overlaps concurrent `IconName` migration).
- **GraphTimeline** — **fixed**: avatar initials (first-two-words → "JD" not "J", extracted
  `graph_timeline_avatar_initials`); guide-line alpha (translucent not opaque). Left label-chip text
  token and avatar-fill token unfixed (no matching native `Theme` field without touching `ui_wgpu`).
- **BlockList** — **fixed**: step cards now four-sided border, not just top/bottom.
- **DiffView** — **fixed** the biggest gap: swapped row-background-wash mechanism for React's
  text-tint mechanism (accent/error/text), fixed inverted equal-line dimming. Gutter numbers/
  monospace font deferred (structural, out of scope).
- **EventFeed** — **fixed**: title text now applies tone color (was always plain `theme.text`).
- **World3d** — confirmed OK/thin; delegate crate not inspected (out of scope).

## Other tasks
- `render_placeholder` guards (12 call sites): all correct, no wiring gaps.
- `SurfaceKind` dispatch: already exhaustive (fixed by prior session), no `_` wildcard.
- Icon tinting: uniform multiply-tint mechanism confirmed consistent everywhere; the one wrong-icon
  (not wrong-tint) bug was the VFS fix above.

## Test results
- `cargo test -p semio-framework-renderer-wgpu --lib scenes::` — **100 passed, 0 failed**.
- Full crate: 264 passed, 1 failed
  (`shell::panel_anchor_model_tests::apply_panel_layout_leaves_widths_untouched_when_absent_from_snapshot`)
  — entirely inside out-of-scope `shell`, unrelated concurrent churn, not touched.
- `cargo check --lib`: clean, 98 warnings (same baseline count, no new warnings).

**Files changed:** `framework/renderer/wgpu/rs/lib.rs` only, within the `scenes` region (Canvas2d,
IconRender, InkCanvas, TextEditor popups, VirtualFileSystem, GraphTimeline, BlockList, DiffView,
EventFeed sub-regions and their paired test modules).
