# Lane `contract` handoff

## Changed

UI = `🧰️framework/🔨️modules/🖱️ui`.

- New `UI/🧱️elements/🔲️WindowFrame/`: `🟦️.tsx` (frozen API, exactly the listed names), `frame.schema.json`, `frame.cases.json` (12 cases, array of `{name, metrics, expected{frame, chipRegions, safeClearances, borderPath, contains}}`; normalization rules are in the schema `description`), `frame.test.ts`, `shared-border.ts/.test.ts/.cases.json/.schema.json`, `junctions.cases/.schema.json`, `snap.cases/.schema.json/.test.ts`.
- `sharedOutlineSegments(owner, logical, paint, state, transform, snap?, source = owner, chips = [])`: the new optional last parameter takes `WindowFrameChipRegion[]` in frame-local coordinates. Emits the 4 frame edges (ids `source:0..3`), then per chip the edges not on the frame outline (ids `source:(4 + 4*chip + side)`, side 0 top, 1 right, 2 bottom, 3 left; paint = chip rect inset by 1). Callers that pass no chips get a plain frame.
- Deleted `UI/🧱️elements/🔲️WindowSilhouette/` (chip-span, outline, fusion, snap-of-chips, region and clip-path code and cases dropped; the edge `createSharedBorderSnap` and junction resolvers are kept).
- `UI/🔨️modules/📏️border-presentation/`: `silhouettes.*` -> `outlines.*`; `boxOutlineSegments`, `OutlineBox`, `BoxOutlineInput`, `BoxOutlineError`, `box_outline_segments`, `box_outline_source_segments`, `box_outline_allocation_byte_demand`; `#[path]`/mod wiring in `🦀️.rs`; wording in `states.rs`, `states.cases.json`, `🟦️.ts`; `segments.test.ts` selectors now `data-window-frame*`.
- `UI/package.json`: export `./elements/WindowFrame` (replaces `WindowSilhouette`).
- `UI/🎯️targets/⚛️react/🪞️chrome-exports`, `🧷️panel-exports`, `📦️packages/🟦️typescript/📥️source`, `🧪️tests/🎚️config`: new geometry names exported; WindowChrome renames (`WINDOW_FRAME_BORDER_KINDS`, `WindowChromeFrameBorder`, `measureWindowFrameMetrics`, `resolveWindowFrameBorderKind`, `useWindowFrameGeometry`, `windowFrameBorderPaint`, `WindowFrameBorderKind`) exported, `windowGapFrameClass` removed; test config includes `WindowFrame/{shared-border,frame,snap}.test.ts`.
- `🛍️products/🦑️repo/.../🔣️taxonomy.json`: element id `🔲️WindowFrame`.

## Verified

- `repo test-file` on `WindowFrame/frame.test.ts` (16 pass), `WindowFrame/shared-border.test.ts` (15 pass), `WindowFrame/snap.test.ts` (7 pass), `border-presentation/outlines.test.ts` (6), `segments.test.ts` (13), `states.test.ts` (16): all pass.
- Independent cross-check in `frame.test.ts`: `three` `Box2` for frame containment, chip-in-frame, non-overlap and point containment; seeded brute-force per-unit chip coverage over 200 random frames.
- Not verified: Rust (`outlines.rs`, `outlines.test.rs` renamed by sed only; `semio-ui-border-presentation` is not a workspace member so `repo cargo ... test -p` refuses it); no `tsc` run of the barrels (they depend on the in-flight `WindowChrome` rename).

## Open (other lanes)

- Still import the deleted folder or its old names: `🃏️CardStack/🟦️.tsx` (`WINDOW_SILHOUETTE_PATH_INSET` -> `WINDOW_FRAME_PATH_INSET` from `../🔲️WindowFrame/🟦️.tsx`), `🃏️CardStack/🎯️targets/🧊️wgpu/🦀️.rs` doc, `🛍️products/💻️os/.../🛰️Dock/.../🔲️window-silhouette-and-stack-geometry/🦀️.rs`, and `🎓️teaching/.../🎬️clip/🪟️presentation/` (`render.mjs`, `tests/🪡️axis/🟦️.ts`, `📐️geometry/🐍️.py`). About 27 files in total still name `windowSilhouette*`; owned by compositor, elements, wgpu-os and products lanes.
- `WindowChrome/shared-border.ts` and `chip-abut.ts` must import from `../🔲️WindowFrame/` (`sharedBorderAxisScale`, `sharedOutlineSegments` with the new `chips` argument).
- Rust `🔲️WindowFrame/🦀️.rs` (wgpu-os lane) follows `frame.schema.json` rules and `frame.cases.json`.
