# Canvas2d Input Parity

Implementation packet completed on 2026-09-20. This closes the two Canvas2d interaction gaps identified in `📓️astra-terra-surface-completeness.md`: captured pointer cancellation with all four modifiers, and catalogue drag/drop plus double-click. It also corrects the Display transfer native law to follow the projected retained row identity that the accepted browser runtime publishes.

## Neutral contract

The shared fixture is `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧫️fixtures/🖱️input-contract/🔣️.json`. It contains no product or renderer implementation types. It fixes:

- a 100×100 surface and controller identity;
- one primary press carrying `shift`, `ctrl`, `meta`, and `alt` together;
- one cancelled terminal release at the last owned sample;
- the exact catalogue MIME, raw payload, advertised MIME list, and local drop point;
- a second-click point and interval.

The mounted React oracle and native WGPU laws consume the same fixture.

## Production implementation

### Captured pointer ownership

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` now snapshots a captured `ComponentScene` before releasing pointer capture. A release goes to the current scene first and to the captured scene only when they differ. This gives an outside release to the gesture owner while preserving cross-surface Table/BlockList destination-first transfer and preventing same-scene duplication.

`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` owns one bounded `CanvasGesture` containing window, document generation, surface/controller, viewport/rect, button, last sample, and cancellation state. The Canvas wire carries all four modifiers. Escape, outside move/release, window closure, and a replaced or removed retained generation publish exactly one `canvasPointerUp { cancelled: true }`, clear scene drag state, and retire authority. Cancellation uses false modifiers and the last owned sample, matching React's `JsonLayersCanvasSession.pointerCancel`.

The Interpreter terminal lane checks the owning retained generation before processing another scene intent. This closes the case where a surface disappears and therefore cannot repaint itself to discover staleness.

### Catalogue and double-click

Click intervals and drag-over throttling read `semio_framework_job::default_now_ms()`, the shared real monotonic host clock backed by `Instant` on native and the installed `performance.now()` authority in the browser. They do not infer event time from render input or synthesize time when the browser clock is unavailable.

The Canvas authority accepts only `application/x-semio-catalogue-item`, caps MIME count and raw payload bytes, preserves the raw MIME value, and emits React-shaped actions:

- `canvasDragOver { surfaceId, x, y, width, height, types }`, throttled by the same 50 ms / 4 px policy as React;
- `canvasDragLeave { surfaceId }` when ownership moves away or is cancelled;
- `canvasDrop { surfaceId, x, y, width, height, dragData }`;
- `canvasDoubleClick { surfaceId, x, y, width, height }` after two complete primary gestures within the bounded click interval and distance.

Shell resolves catalogue move/drop against the actual retained `ComponentScene` hit, converts global coordinates through the owning window body, and uses the live Interpreter tree revision. An accepted Canvas drop releases the retained source capture at a hitless point instead of also sending an unrelated `PointerUp` into the destination Canvas. Duplicate release is inert. Outside drop emits leave and no drop.

### Display native law correction

The Display producer already passes its window-template payload through projection and the checkpoint-10 browser journey creates one new window. The native law looked for the authored id `tree.drag.transfer.framework.display.windows.main.kind`, but `PanelProjection` correctly scopes retained keys as `<surface>/<authored-id>`. The law now derives the projected TreeItem key before asserting the painted transfer handle and forbidden sort handle. Its payload, NewWindow promotion, hitless release, exactly-once creation, and capture retirement assertions remain intact.

## Laws and oracle

`🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs` adds physical production-path laws which publish and paint real `Component::Surface(Canvas2d)` documents, register actual retained hits, and use `ShellState::handle_pointer_button`, move, keyboard Escape, and release:

- full modifier preservation and exactly one cancelled terminal action;
- hitless captured release and duplicate-release refusal;
- real retained Tree catalogue source to published Canvas destination, including raw MIME and local coordinates;
- drag-leave/no-drop on outside cancellation;
- two complete clicks producing exactly one double-click.

`🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs` adds the removed/replaced-generation terminal law. Existing standalone Canvas down laws now explicitly retire their gesture so thread-local test authority cannot contaminate later cases.

The mounted React oracle is `📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx`. It mounts the real `Canvas2dHost`, uses React DOM drag/drop, and uses the installed infinite-canvas session boundary for pointer cancel and double-click. The external renderer is replaced only at its canvas attachment seam.

Focused command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache -- long ../../../../🧱️elements/📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx --silent=false --reporter=verbose
```

Result: 1 file passed, 3 tests passed, 0 failed. Rustfmt check-only parser passes reached every touched Rust file; those commands return nonzero because these large shared files already differ from rustfmt layout. Native16 compiled far enough to expose four invalid `InputState.time_seconds` references before running tests; the source now uses the real monotonic clock above and removes those fixture-only writes. Native17 compiled and reduced the new Canvas failures to JSON integer/float representation drift; the laws now compare every coordinate and extent numerically while retaining the exact expected values.

Native18 then exposed one real release-only defect. After a retained catalogue transfer consumed its capture and emitted one `canvasDrop`, a duplicate physical release landed on the Canvas and entered `canvas_pointer_button_into` with no active `CanvasGesture`. The prior inside-rect exception emitted an unrelated `canvasPointerUp`; it was not a second drop, but it still violated the bounded gesture lifetime. Every Canvas release now requires the matching window/surface/document-generation gesture. Catalogue drop remains destination-owned, and a retired capture cannot alias a Canvas terminal event, click, double-click, or drop. Native19 is the pending verification receipt. Per task ownership, no Cargo, native renderer, WASM, generator, or browser build was started. Root owns native and runtime acceptance.

## Acceptance boundary

The source packet is stable. Runtime acceptance still requires a fresh canonical WGPU build and the paired browser journey. The checkpoint-10 finding where a newly created Display tab lacks its initial World3d body is handled separately in `📓️astra-sol-window-body-publication.md`.
