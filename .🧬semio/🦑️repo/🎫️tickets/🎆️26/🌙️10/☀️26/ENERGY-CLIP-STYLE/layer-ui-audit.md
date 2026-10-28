# Layered UI Audit

## Exact Existing Components

- Shared API imported by demonstrator/play from @semio-tech/ui-react: LayeredOverview, OverviewCard, OverviewCardOpenChip, centeredLastRowCells, nearSquareGrid, LayeredPane, LayeredCardState.
- Shared component: 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx.
- Types: same component folder /📋️types/🟦️.ts.
- Card: 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx.
- Pure geometry: 🧰️framework/🔨️modules/🖱️ui/🔨️modules/🥞️layered-overview-geometry/🟦️.ts.
- Existing direct composition: ♻️mit-bestand/🧺️demonstrator/🟦️.tsx and 🏢️semio-tech/🎡️play/🟦️.tsx.
- Exact nested course composition: 🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🏠️home/🟦️.tsx. Architecture course entry delegates to @semio-tech/course-react/app.

## Mechanism

LayeredOverview fills a positioned parent. Each pane renders a real full-size page on a grid strip behind one dialog-level ui-veil; cards are fixed above it in strip mode, or travel with their pane in swipe mode. cells maps each id to {column,row}. renderCard receives {mode,revealed,opened,open}. Place OverviewCardOpenChip in footerLeft for the required bottom-left Ansehen chip: it already supplies data-layered-zoom. Whole-card button onClick is state.open and lifted is state.revealed.

Hover only the zoom chip triggers reveal, not the whole card. Keyboard focus reveals accessibly. Reveal calls focusView, which glides to cellOffset(cells[id]) using the same function as opening; thus hover and opened focus positions match. The hovered card becomes data-tinted, other card children opacity zero while preserving their hit targets. Leaving restores pointer camera. Move source is /🔸️move/🟦️.tsx; trigger and host implementation is /🧵️pane/🟦️.tsx; pointer listener and normalized camera are /🟠️pane-styles/🟦️.tsx. Existing behavior includes prevention of inherited-hover reveal on newly active nested grids via layeredZoomArm/Move.

Tint is exact shared design-system CSS: 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css lines 7126 onward. ui-veil uses backdrop blur, saturate and color-mix(surface-bg, veil-alpha); data-tinted adds matching wash overlay. Shared CSS should supply it, not a custom approximation.

## Nested Topic To Video Composition

Mirror course HomeOverview: recursively render a nested LayeredOverview as the topic pane content, with enabled/interactive only for deepest active level, openedId controlled from one address, routing=none, and onOpenedIdChange opening its child or parent. For six topics use cells three columns/two rows, and each topic pane contains the topic video grid; each video pane renders the full Video component. Each level receives panes, cells, card renderer and explicit labels. lifecycle={budget:panes.length,warmStartMs:0,warmIntervalMs:0} keeps all lightweight backgrounds live as the course does.

Course crucial pointer mechanism: PreviewPointerContext holds createPreviewPointerBus(). Active grid publishes onCameraChange; each background grid subscribes via useInheritedPreview(activeGrid || opened!==null), passes inherited camera to LayeredOverview and interactive=false. Background previews remain live/inert and follow pointer without zooming. Reusing only LayeredOverview without the shared pointer bus does not reproduce moving deeper background levels. Course functions appear at home lines 117-156, 237-246, and LayeredOverview usage 309-338.

## Validation Hooks

- data-layered-overview / data-interactive / data-mode.
- data-layered-strip (transform), data-layered-veil (clip path), data-layered-overlay.
- data-layered-card=id, data-revealed, data-reveal=pointer or focus, data-tinted, data-concealed.
- data-layered-pane=id with data-opened.
- data-layered-zoom is the chip activation surface.
- Shared component tests: 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🧪️tests/🧩️component/🟦️.tsx; hover/tint tests near lines 263-303.
- Course acceptance: 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🥞️layered-home/🟦️.ts; six-topic/video acceptance should mirror nested camera tests there.

No source files changed by scout.
