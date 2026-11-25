# 🪟 Rectangular Frames with Glassy In-Content Chips: Design

## Before

A window, panel, card, menu or dock stack was a *silhouette*: a body rectangle plus chips that stick out of it in a cap row and a footer row. Between the chips the cap row was a *gap*: a transparent, hit-free cutout that showed the floor behind the element. The outline was a concave rectilinear polygon (`window-silhouette-geometry/v1`), the content was clipped to the body-and-chip union (CSS `clip-path: polygon`, wgpu stencil union mask), and fused context menus added stepped wings to the same polygon.

Two such elements never compose: side by side or stacked they leave gap shapes between each other.

## After

One model for React, wgpu, TUI and print.

1. **Frame.** Every such element is one axis-aligned rectangle: the *frame*. Its outline is its four edges. Nothing sticks out of it and nothing is cut out of it.
2. **Content plane.** The content fills the complete frame, edge to edge, and is clipped by the frame rectangle only (`overflow: clip` on the web, a scissor rect in wgpu, the cell rect in the TUI).
3. **Chips.** Chips are overlays *inside* the frame, anchored to a frame edge (`top` or `bottom`) and an alignment (`start`, `center`, `end`). They are painted above the content plane.
4. **Chip glass.** A chip has its own glass surface, one elevation level above the frame's content surface, translucent with backdrop blur, so the content stays visible through it and the chip still reads as a different surface. Reduced transparency, missing backdrop support and forced colours fall back to the opaque chip surface.
5. **Chip borders.** A chip draws only its *inner* edges, the ones that do not lie on the frame outline. Its outer edges are the frame outline. Chips are shared-border boxes whose parent is the frame, so coincident edges of abutting chips resolve to one divider.
6. **No gaps.** The space of a chip row that no chip covers is content. It paints content, and pointer input goes to the content. Gap elements, gap selectors, gap hit-testing and cutout fills do not exist.
7. **Safe clearances.** The frame reports the chip row depth per edge. Document content starts below the top row and ends above the bottom row by that clearance and scrolls beneath the chips; edgeless content (scenes, canvases, maps) is full-bleed from the first frame.
8. **Composition.** Frames abut on shared edges. The shared border compositor resolves a coincident edge to one line in the highest-priority state. A fused context menu is a set of abutting frames (primary column and submenu wings), not one stepped polygon.
9. **States.** Border state ownership and closure (`resolveBoxStates`, `boxSilhouetteSegments` → `boxOutlineSegments`) stay as they are, only on rectangles.

## Contract `window-frame-geometry/v1`

Schema first (`frame.schema.json`, cases `frame.cases.json`), implemented in TypeScript and Rust, validated against the cases by both and against a third-party rectangle implementation in tests.

```
metrics   { width, height, top: { depth, chips: [{ left, right }] }, bottom: { depth, chips: [...] } }
geometry  { schema, state: "pending" | "ready", metrics,
            frame: { x: 0, y: 0, width, height },          // outline and content clip
            borderPath,                                    // inset rectangle path
            chipRegions: [{ x, y, width, height, dock }],  // glass overlays inside the frame
            safeClearances: { top, right, bottom, left } }
contains(geometry, x, y) = point in frame
```

Removed without replacement: outline polygons, `contentClipPath`, `contentPath`, `bodyRegion`, `contentRegions`, gap regions, outline violations, context menu fusion metrics, stencil union masks, gap pass-through hit testing.

Kept: the junction resolver and the snap of `WindowFrame/shared-border.ts` (`junctions.*`, `snap.*`). They resolve where the edges of abutting frames and chips meet, which the rectangle model needs as much as the silhouette did; `WindowChrome/shared-border.ts` (the compositor) consumes them, and the geometry lives in `WindowFrame`.

## Naming

| Before | After |
| --- | --- |
| `🔲️WindowSilhouette` element | `🔲️WindowFrame` |
| `WindowSilhouette*` types, `windowSilhouette*` functions | `WindowFrame*`, `windowFrame*` |
| `data-window-silhouette`, `-chip`, `-content`, `-border` | `data-window-frame`, `-chip`, `-content`, `-border` |
| `data-window-silhouette-gap`, `window-chrome-gap`, `mode-dock-tab-gap` | removed |
| `--window-silhouette-*-clearance` | `--window-frame-*-clearance` |
| `.window-silhouette-*` classes | `.window-frame-*` |
| `boxSilhouetteSegments`, `silhouettes.*` (border presentation) | `boxOutlineSegments`, `outlines.*` |

The word *silhouette* stays where it means something else (staffage and icon silhouettes, 3D model outlines, map policies).

## Work packages

1. Contract: frame schema, cases, TypeScript and Rust geometry, border presentation rename.
2. React compositor: `WindowChrome`, shared border, chip abut, styling.
3. React elements that host frames: Panel, PanelTabBar, ContextMenu, Popover, Select, OverviewCard, CardStack, Skeletons, Canvas dock, Window, Introduction, SharedBorderRow, Surface.
4. wgpu: UI paint and draw pipeline, scene glass primitives, OS shell staging.
5. TUI.
6. Products: course, pets, os, print and every plugin that names the old contract.
7. Tests, fixtures, stories, docs.

## Frozen API (all lanes code against this; do not wait for another lane)

TypeScript, `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowFrame/🟦️.tsx` (replaces `🔲️WindowSilhouette/🟦️.tsx`, which is deleted):

```ts
export const WINDOW_FRAME_GEOMETRY_SCHEMA = "window-frame-geometry/v1" as const;
export const WINDOW_FRAME_PATH_INSET = 1;
export const WINDOW_FRAME_CHIP_EPSILON = 0.5;
export interface WindowFrameChip { readonly left: number; readonly right: number }
export interface WindowFrameEdge { readonly depth: number; readonly chips: readonly WindowFrameChip[] }
export interface WindowFrameMetrics { readonly width: number; readonly height: number; readonly top: WindowFrameEdge; readonly bottom: WindowFrameEdge }
export type WindowFrameDock = "top" | "bottom";
export interface WindowFrameRect { readonly x: number; readonly y: number; readonly width: number; readonly height: number }
export interface WindowFrameChipRegion extends WindowFrameRect { readonly dock: WindowFrameDock }
export interface WindowFrameSafeClearances { readonly top: number; readonly right: number; readonly bottom: number; readonly left: number }
export interface PendingWindowFrameMetrics { readonly width: number; readonly height: number; readonly topClearance: number; readonly bottomClearance: number }
export interface WindowFrameGeometry {
  readonly schema: typeof WINDOW_FRAME_GEOMETRY_SCHEMA;
  readonly state: "pending" | "ready";
  readonly metrics: WindowFrameMetrics;
  readonly frame: WindowFrameRect;
  readonly borderPath: string;
  readonly chipRegions: readonly WindowFrameChipRegion[];
  readonly safeClearances: WindowFrameSafeClearances;
}
export function normalizeWindowFrameChips(chips, x0, x1): WindowFrameChip[];
export function normalizeWindowFrameMetrics(metrics): WindowFrameMetrics;
export function windowFrameBorderPath(metrics, inset = WINDOW_FRAME_PATH_INSET): string;
export function windowFrameChipRegions(metrics): WindowFrameChipRegion[];
export function windowFrameSafeClearances(metrics): WindowFrameSafeClearances;
export function windowFrameContains(metrics, x, y): boolean;
export function pendingWindowFrameMetrics(pending?): WindowFrameMetrics;
export function createWindowFrameGeometry(metrics: WindowFrameMetrics | null, pending?): WindowFrameGeometry;
```

`🔲️WindowFrame/shared-border.ts` keeps the export names of the former `🔲️WindowSilhouette/shared-border.ts` (`sharedBorderAxisScale`, `sharedOutlineSegments`, junction resolvers), with `sharedOutlineSegments` emitting the four frame edges plus the inner edges of each chip region.

`WindowChrome` keeps its component name and props except: `gapProps`, `footerGapProps` and `silhouetteSlot` are removed, `frameBorderSlot` (default `"window-chrome-frame-border"`) is added. `WindowChromeSilhouetteBorder` → `WindowChromeFrameBorder`, `WindowSilhouetteBorderKind` → `WindowFrameBorderKind`, `WINDOW_SILHOUETTE_BORDER_KINDS` → `WINDOW_FRAME_BORDER_KINDS`, `resolveWindowSilhouetteBorderKind` → `resolveWindowFrameBorderKind`, `windowSilhouetteBorderPaint` → `windowFrameBorderPaint`, `measureWindowSilhouetteMetrics` → `measureWindowFrameMetrics`, `useWindowSilhouetteGeometry` → `useWindowFrameGeometry`. `windowGapFrameClass` is removed.

DOM: `data-window-frame` (stack), `data-window-frame-chip` + `data-dock` (chip overlay), `data-window-frame-content` (content plane), `data-window-frame-border` (overlay), `data-frame-state` (`pending`/`ready`), `data-frame-remeasure`. Chip rows are `position: absolute` overlays inside the stack (`data-slot="window-chrome-cap"`, `"window-chrome-footer"`) with `pointer-events: none`; chips re-enable pointer events.

CSS: `.window-frame-border*`, `.window-frame-content-plane`, `--window-frame-top-clearance`, `--window-frame-bottom-clearance`.

Border presentation: `boxOutlineSegments`, `OutlineBox`, files `outlines.{rs,schema.json,cases.json,test.ts,test.rs}`; Rust `BoxOutlineInput`, `box_outline_segments`.

Rust: pure geometry in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowFrame/🦀️.rs` (`WindowFrame`, `WindowFrameEdge`, `WindowFrameChip`, `chip_regions()`, `safe_clearances()`, `contains()`, `content_clip_rect()`), validated against the same `frame.cases.json`. Scene builder: `begin_silhouette_clip`/`end_silhouette_clip`, the stencil mask pipeline and `SilhouetteMask` tessellation are removed; frames clip with the existing rectangular clip and paint chip glass with the existing glass rect primitive after the content.
