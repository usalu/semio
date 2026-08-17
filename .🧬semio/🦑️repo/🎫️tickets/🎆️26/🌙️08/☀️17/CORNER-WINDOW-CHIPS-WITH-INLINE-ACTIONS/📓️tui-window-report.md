# TUI Window Chrome — Four Corner Tab Groups

## Summary

Refactored TUI window chrome so stack tabs dock into up to four corners, each tab carrying inline action glyphs (`⤢` maximize, `⧉` new window, `✕` close). `window_chip_layout` remains the single source of truth for paint and hit-testing.

## Model

- `layout::WindowStackCorner` — `TopLeft` (default) / `TopRight` / `BottomLeft` / `BottomRight`
- `WindowLayoutWindowNode.corner: Option<WindowStackCorner>`
- `WindowStackTabState { label, corner }` on `WindowState.stack_tabs` (replaces `Vec<String>`)
- Empty `stack_tabs` synthesizes one top-left tab from `title` (+ number prefix)

## Layout

`WindowChipLayout` now exposes:

- `groups: Vec<WindowCornerChipGroup>` (≤ 4)
- each `WindowCornerTab` has `x`, `interior`, glyph absolute columns (`maximize_x` / `new_x` / `close_x`)
- top/bottom body hairline edges between opposing corner groups

## Paint / hit

- `paint_window` paints corner tab boxes via `paint_corner_tab` (top bends down; bottom bends up)
- Body hairlines connect between chip groups; missing corners stay flat
- `window_hit` resolves per-tab glyphs → `WindowClose` / `WindowMaximize` / `WindowNewTab` / `WindowTabActivated`
- `window_control_at` delegates to `window_hit` (close/maximize only)

## Mount

`mount_stack` copies each child's `corner.unwrap_or_default()` into `WindowStackTabState`.

## Tests updated

| Test | Change |
|------|--------|
| `window_chrome_recesses_tabs_into_the_top_corners_of_a_closed_shape` | Asserts single top-left chip with inline glyphs + flat right body corner |
| `window_chrome_flattens_the_right_side_when_no_controls_tab_is_wanted` | Title chip without action glyphs; flat right |
| `window_chrome_hides_both_tabs_when_too_narrow_for_even_the_title` | Width 4 → `has_tabs == false`, flat box |
| `window_control_clicks_resolve_to_close_and_maximize_signals` | Glyphs hit-tested on the corner tab row |
| `window_hit_resolves_tab_activation_and_new_tab_signals` | Multi-tab top-left group; `⧉` → `WindowNewTab` |
| `window_stack_tabs_paint_on_body_top` | Labels + glyphs on tab text row (y+1) |
| `window_stack_tabs_respect_bottom_corners` | **New** — top-left + bottom-right paint |

## Verification

```
cargo test -p semio-framework-ui --features tui --lib
# 92 passed; 0 failed
```

Artifacts: `🧪️tui-chrome-test.txt`, `🧪️tui-full-test.txt`, `🎯️tui-target/`.

## Files

- `framework/ui/tui/component.rs` — layout corner enum, chrome state/layout/hit, mount, tests
- `framework/ui/elements/Window/tui component.rs` — `paint_window` / `paint_corner_tab`
