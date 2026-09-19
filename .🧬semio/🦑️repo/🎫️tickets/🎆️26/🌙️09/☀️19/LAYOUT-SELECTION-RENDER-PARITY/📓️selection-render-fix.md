# Layout selection render parity

## Root cause

Framework-owned `"elements"` selection/hover was updating in dispatch (`canvasPointerDown` / `interactionSelect`) but **never reached the render path**:

- `canvas_layers` always passed empty `selected_ids` / `hovered_id`.
- `render_with_request_context` ignored `InteractionView`.
- Inspection panel had no live selection.

## Fix (gis2d / puzzle2d pattern)

1. `LayoutInteractionSnapshot` — copies `InteractionView::selection("elements")` and `hover("elements", "pointer")`.
2. `LayoutPlayApp::render_body` — single render funnel; `render_with_request_context` passes `from_interaction`, plain `render` uses default empty snapshot.
3. `canvas_layers` — threads snapshot into `build_display_list_for_page` (blueprint chrome strokes for selected/hover).
4. Inspection panel — summary + first selected frame read-only fields.
5. Test `selected_and_hovered_frames_get_chrome_strokes` restored in `🖼️canvas` unit tests.

## Note

Preview window still uses `blueprint: false` (no edit chrome); selection state is passed but stroke highlighting is blueprint-only by design.
