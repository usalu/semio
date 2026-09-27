# Flow, Cursor, Theme, Table, and Control Parity — Sol Flow 34

## Scope

This continuation resumed the Flow NodeGraph app-backed selection and persisted-movement audit, then implemented the renderer parity packets assigned during the shared WGPU gate: accepted browser cursor presentation, native system-theme invalidation, NodeGraph connection-handle cursor preparation, narrow Table action geometry, and the four current UI control-geometry failures.

## Flow NodeGraph authority

The current app route still carries the real Flow interaction chain rather than a test-only adapter:

- the retained scene owner resolves the live `NodeGraph` surface and its accepted host;
- selection projects framework node identities back to raw scene node ids;
- pointer movement closes through the graph operation vocabulary and publishes the requested widget layout;
- unknown edit operations are rejected instead of silently dropped.

The focused six-law native run is recorded at `🗑️generated/sol-flow34/native-focused-2.log`. It remains queued behind the shared Cargo build lock at the time of this checkpoint. The historical actual-React evidence in `📓️astra-sol-flow-node-graph-parity.md` therefore remains the latest completed runtime oracle; no new live native acceptance is claimed here.

## Accepted browser cursor and native theme

The browser cursor now comes from the atomically accepted product `RenderSnapshot`: the snapshot carries the full `SemioCursor` and the accepted `theme_dark`, and the browser worker serializes `semio_cursor_css(snapshot.accepted_cursor, snapshot.accepted_theme_dark)`. The generic five-value `ui_render::CursorRequest` remains unchanged. Native presentation continues to apply the full `SemioCursor` directly.

`WindowEvent::ThemeChanged` now publishes system appearance and invalidates the same scheduler with `InvalidationReason::THEME`. The neutral law covers idle `pending_reason = None`, existing theme work, and independent `INPUT_STATE`/`PAINT` work, then proves exact reason union and one drain. The React/media-query focused oracle passed 8/8 in `🗑️generated/sol-flow34/native-theme-ts-focused-2.log`.

The general browser cursor oracle passed before the connection-handle extension. The centered connection cursor adds a distinct `CrosshairCentered` fixture projection at hotspot `16 16`, while the general `Crosshair` remains React's `0 0`. The React/CSS/transport oracle passed 33/33 in `🗑️generated/sol-flow34/browser-cursor-centered-focused.log`. The renderer resolves this semantic only from the accepted live NodeGraph pointer owner, revalidates the same-window admitted surface, converts to surface-local coordinates, and asks the live EngineCanvas handle/`DrawEdge` owner. Active generic drag/resize cursors retain precedence. The enum, CSS/native projection, 30-case Rust fixture, live query, and native handle/`DrawEdge` law are all present; the packet was applied after the WASM17 renderer compilation boundary ended.

## Table segmented controls

Table row buttons and steppers now use one fractional segment authority for paint, pointer selection, focus, and accessibility. Pointer selection no longer expands each narrow segment to one pixel. Accessibility intersects each segment with the accepted body in both axes, matching the render scissor. The accepted action lifecycle and menu exclusion remain unchanged.

The language-neutral fixture covers a one-pixel cell split into two distinct half-pixel buttons and a body-edge case clipped in both axes. The actual React/Chromium oracle proves two separately actionable buttons and fractional browser rectangles. It passed 4/4 in `🗑️generated/sol-flow34/table-segment-react-focused.log`. The focused native route law is queued behind the shared Cargo build lock.

## Current Select and Slider geometry

The four UI7 failures had two different causes:

1. React's current `SelectContent` always mounts a 22.4px up-scroll band and a 22.4px down-scroll band around the viewport. With the 1px border, three 25.6px rows, and 3.2px viewport padding, the current menu height is 130px. WGPU already produced 130px; the retained-origin fixture still expected the older 83.2px viewport-only box. Its schema and bottom/top exact geometry now name the border and scroll-band inputs. The focused React test passed 14/14 in `🗑️generated/sol-flow34/ui-control-geometry-select-react-focused-2.log`.
2. The shared Slider presentation reserves a fixed 28.8px readout and accepts pointers only in the remaining track cell. The retained-control fixture and intermediate-drag test still measured fractions over the outer control. They now derive the same track cell. The immediate WGPU slider also used the obsolete two-primitive full-width rendering; it now uses `slider_control_presentation` for rail, range, thumb, formatted readout, hit rectangle, and interaction metadata, matching retained WGPU and React.

The renderer TypeScript twin passed among 174 successful laws in `🗑️generated/sol-flow34/ui-control-geometry-renderer-ts-focused.log`; the target's four failures were unrelated Playwright executable discovery and the repo-owned cached browser path was supplied afterward. The focused four-law native run is recorded in `🗑️generated/sol-flow34/ui-control-geometry-native-focused.log` and remains queued behind the shared Cargo build lock at this checkpoint.

## Remaining validation limits

- Do not claim a fresh native Flow runtime result until `native-focused-2.log` reaches assertions.
- Do not claim native Table or control geometry green until their queued focused laws complete.
- Run the 30-case Rust cursor fixture and the live NodeGraph handle/`DrawEdge` law after the shared Interpreter focus tuple repair admits the next native/full-UI build.
- Root owns the full renderer, WASM, native, and paired browser shell gates.
