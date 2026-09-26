# Select Popup Geometry, Reveal, and Locale Continuation

## Contract completed

The retained and immediate WGPU Select paths now share one popup geometry with React's measured outer-box structure: a 128 px minimum width, a hairline border, two permanently mounted 22.4 px scroll bands, and a distinct padded scroll viewport. Row paint is clipped to that viewport, while the chevrons paint after the viewport clip closes.

The scroll calculation uses the viewport's real `clientHeight` for React's `max(24, floor(clientHeight * 0.8))` step. Its maximum keeps the viewport padding in the DOM-equivalent `scrollHeight`, including the constrained case where the viewport is shorter than both padding edges. Wheel movement continues to update the existing bounded per-Select offset and is not reset by later retained synchronization.

Nearest-edge keyboard reveal is one-shot. Popup geometry captures the theme-derived row height and viewport padding used to create it, so event routing does not substitute the default theme when a customized theme is active. Opening reveals the selected row once; Arrow, Page, Home, End, and typeahead movement reveal their highlighted row without committing the selected value.

The language-neutral schema and fixture remain at:

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔽️select-popup-geometry/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🔽️select-popup-geometry/🔣️.json`

The fixture is consumed by the React third-party DOM law, the pure WGPU geometry laws, and the retained keyboard-routing law.

## Validation

The React component/schema oracle passed on 2026-09-26:

```text
Test Files  1 passed (1)
Tests       14 passed (14)
Duration    4.56s
```

Command:

```text
SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🧩️component/🟦️.tsx --config /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts
```

The focused native command was handed to the root-owned Cargo session and was not run in this lane:

```text
SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/ui-rs:test-wgpu-engine -- -E 'test(the_popup_outer_box_keeps_reacts_minimum_border_and_two_permanent_scroll_bands) or test(nearest_reveal_uses_the_custom_theme_metrics_captured_by_the_popup) or test(keyboard_movement_reveals_the_highlighted_select_row_without_committing_it) or test(wheel_over_an_escaped_select_popup_scrolls_its_accepted_viewport_only) or test(retained_select_popup_is_viewport_clamped_scrolled_and_glass_foreground)'
```

The first native slice exposed an oversized-row nearest-reveal defect. A 3.2 px constrained viewport opened on row zero, whose leading edge was already visible, but the earlier formula scrolled 25.6 px to its trailing edge. `select_revealed_scroll` now keeps the offset while an oversized row's leading edge remains visible and otherwise aligns that leading edge. The first correction exposed only f32 edge noise (`3.1999998` versus `3.2`), so the edge comparison now carries a 0.001 logical-pixel epsilon.

The root-owned UI37 rerun passed the four selected native laws on 2026-09-26:

```text
Tests       4 passed, 0 failed, 676 skipped
Test time   0.101s
Nx time     7m10s
```

This verifies the oversized-row edge, retained popup geometry and scroll, customized-theme nearest reveal, and keyboard highlight movement without committing the value.

## Locale continuation

`📓️astra-sol-locale59-repair.md` contains only its title. The complete locale failure chronology and correction are recorded in `📓️astra-sol-locale-retained-refresh.md`: Native59 exposed stale law assumptions, Native60 through Native62 repaired typed labels and the live-session dock fixture, and Native63 passed both selected locale laws with 1,242 tests outside the filter. No unresolved locale production change was identified in that continuation.
