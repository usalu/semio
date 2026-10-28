# Canonical Cursor Interaction Implementation

## Owned Files

- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🔣️.json
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧬️schema.json
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🐍️.py

This implementation does not edit the presentation renderer, authored scientific scenes, native scene click helper, runtime or fonts. Those integrations belong to the parent agent. The testing agent owns portable fixtures and tests in the adjacent tests folder.

## Canonical Source Findings

The existing UI interaction and chrome-control presentation rules, styling contract, dark cursor CSS and WindowChrome/Tree audit are recorded in cursor-ui-audit.md. Ordinary hover emphasizes cream ink/borders. Interactive hover fill is dark-4-7 (#606966). Selected/pressed control fill is primary (#ff344f), with dark foreground (#001117). Active hover uses the canonical direct-sRGB blend, 80% primary and 20% light (#fd5a6d). Normal interactive controls remain transparent with gray text; plain parent glass is retained while its border emphasizes. Tree guide width increases to 1.5.

The exact dark cursor assets are used, with their native two-polygon geometry and SVG group translation:

- Arrow: 🖱️default/🌙️dark.svg, cream fill with dark outline.
- Pointer: 👆️pointer/🖋️dark-source.svg, primary shaft plus cream secondary shape with dark outline.
- Selectable: ☑️selectable/🌙️dark.svg, cream fill with primary outline; used by fullscreen/toggle controls.

The module reads those repository assets rather than inventing a hand cursor, glow, ripple or generic pressed cursor. Fullscreen controls reference the existing maximize-2/minimize-2 icon assets and canonical 22.4-pixel hit cells, 3.2-pixel insets and 20-pixel icon size. A presentation title cap is 44 pixels high.

## Pure Public Interfaces

- interaction_contract() returns the schema-first portable configuration.
- source_files() exposes the module, contract/schema, styling/theme inputs and exact cursor/icon assets.
- hit_test(nodes, point) returns the topmost/deepest enabled node.
- hit_path(point, nodes) returns the real root-to-target ancestry.
- interaction_state(nodes, point, pressed=False) returns target, ancestor_hovered and pressed.
- initial_state() and apply_event(state, event, nodes) replay immutable event projections.
- resolve_visual(role, state) returns native-independent fill, fill_opacity, stroke, text, stroke_width, opacity and cursor kind.
- control_rect(rect, role='fullscreen') places a hit cell at the right of an explicit title cap. Pass the actual cap rectangle, or a window rectangle with title_width specifying cap width.
- cursor_outline(kind='arrow') returns viewBox, hotspot and translated polygon paths with canonical fill/stroke/miter data.
- cursor_timeline(phase, video_id, rect, t) returns point, hover/hovered, pressed, committed action/effects, event milestones, fullscreen state, cursor and zoom motion progress.

Only standard Python libraries are used. Public results are plain serializable data; no rendering-library types cross this interface.

## Events, Hit Regions And Commit Rules

Nodes use explicit IDs, parents, roles, rectangles, optional stacking z, enabled state, optional domain action and optional absolute polygon outline. Bounds are half-open; every ancestor must contain the point and be enabled. An optional silhouette outline rejects the transparent cap gap and clips descendants. Actual ancestors are emphasized; sibling windows and sibling navbar/footer branches are not mistaken for ancestors.

The event stream is enter, move, press, release and click, with leave/cancel support. Release arms an action only on the same enabled target that received the press. A subsequent explicit click commits fullscreen.toggle, toggle or device activation. Drag-out, wrong-target release and disabled-before-click cases do not activate. Effects occur after the click; press alone never changes fullscreen or toggles.

## Timeline And Boundary Rules

Normalized milestones are arrival 0.18, press 0.25, release 0.30, click 0.31, zoom start 0.34 and zoom end 0.86. Fullscreen motion is additionally gated by the actual committed fullscreen.toggle effect.

Focus starts at the canonical overview park [1572,840] and ends at idle park [1624,840] outside the application. Restore independently starts outside and ends at the same overview park. Device routes start and finish outside. Restore is not a reversed focus animation; it has its own press/release/click before motion.

The overview park physically lies within the canvas, so the actual app/canvas hover ancestry is retained at both global boundaries. This is identical across lessons and follows the user's request for regular UI parent hover. The pointer legitimately leaves the application during the body idle interval.

## Test-Driven Verification

The initial five core tests failed because the module did not yet exist. Seven subsequent core tests passed after implementation, including independent Pillow hit masks and transparent-gap clipping, immutable reducer projections, gesture commit/cancellation, NumPy sRGB blending, and native SVG cursor geometry/style parity. All declared source assets exist.

A final no-click/no-zoom edge test was added and observed failing for both focus and restore when the target lies outside the app. The implementation now gates motion on a committed fullscreen effect. All eight core tests passed in 2.157 seconds after this correction.

Native rendered cursor/fullscreen/device integration and visual verification remain with the parent task and native scene helper owner.

## Canonical Border Transition

The pure transition_color(before_hex, after_hex, elapsed_seconds) interface now implements the actual WindowChrome 120 ms CSS ease timing curve. The declarative transition contract contains the standard control points [0.25, 0.1, 0.25, 1]. Standard-library bisection inverts the curve's x coordinate before evaluating y; channels interpolate directly in sRGB and round half-up, matching the existing UI color arithmetic. Elapsed times clamp to the before/after color at 0 and 0.12 seconds. Nonfinite time or malformed colors are rejected.

The new portable test was observed failing before implementation. All eleven pure core/cache/asset tests then passed in 1.841 seconds, including an independent NumPy cubic-root inverse oracle over seven clamped and interior transition samples.

The parent applies this transition only to normal window and ancestor border hover emphasis. Control press/selected palettes keep their existing canonical behavior. No runtime, renderer or native helper source was edited for this small core addition.
