---
name: General Mosaic Tiling
overview: "Replace the ad-hoc sprite/cover-zoom math for catalogue tiles with one deterministic windowed-cover algorithm: every tile is a window onto a single cover render of CATALOGUE_FRAME, so tiles match the full catalogue, never over-zoom, and never bleed into neighbors."
todos:
 - id: windowed-fn
   content: Add mosaicWindowedCoverVars + overflowAxisPosition helper implementing the windowed-cover formulas in index.tsx
   status: completed
 - id: route-mosaic
   content: Route detected mosaic cells (rest + grid states) through mosaicWindowedCoverVars regardless of morphTo; keep per-crop centered cover for non-mosaic; rename morphFrame->morphToFrame param
   status: completed
 - id: slide-aspect
   content: Add PresentationSlideAspectContext fed from parsePresentationSlideCssSize and thread slideAspect into FigureCropView/figureCropBackgroundVars (default sourceAspect)
   status: completed
 - id: tests
   content: Update renderer mosaic tests to computed windowed-cover values (size N00% auto, per-cell positions); run renderer + core test suites
   status: completed
 - id: verify
   content: "Browser-verify Bauteilarten grid: size ~500% auto, Rippenplatte 1 no bleed, matches full catalogue, drag stays aligned"
   status: completed
 - id: calibrate
   content: If grid still off the PNG product rows, recalibrate CATALOGUE_FRAME in spec.ts
   status: cancelled
isProject: false
---

# General Mosaic Tiling

## Root cause (confirmed)

Mosaic cells are drawn with the **sprite model** (`background-size: 100/crop.width% ≈ 670%`), which over-zooms by `1/CATALOGUE_FRAME.width ≈ 1.34x` and bleeds into neighbors. Two bugs force this path even after recent edits:

- `figureCropBackgroundVars` ([index.tsx:1560](framework/product/presentation/renderer/react/index.tsx)) param mislabeled `morphFrame` (gets `morphToFrame`); on the Bauteilarten grid the tiles carry a `morphTo`, so `mosaicOnRest` is `false` and rest renders via the over-zoom branch.
- `figureCropCoverVars` ([index.tsx:1434](framework/product/presentation/renderer/react/index.tsx)) / `figureCropBackgroundSize` ([index.tsx:1405](framework/product/presentation/renderer/react/index.tsx)) keep an extra `coverScale` and a mixed-unit axis pick.

## The algorithm (windowed-cover, deterministic)

For tile cell `(column,row)` of a `rows x columns` grid filling normalized `frame`, with `sourceAspect = imgW/imgH` and `slideAspect = slideW/slideH`:

- `fA = (frame.width/frame.height) * slideAspect` // frame's on-screen aspect
- Width-driven when `fA >= sourceAspect`:
  - `size = "${columns*100}% auto"` (image width == frame width == columns cells; e.g. 5 cols -> `500% auto`, NOT 670%)
  - `posX = columns==1 ? 50 : column/(columns-1)*100`
  - `k = fA/sourceAspect` (>=1, vertical overflow); `posY = rows==1 ? 50 : overflowPos(row, rows, k)`
- Else height-driven (symmetric): `size = "auto ${rows*100}%"`, `posY` edge-aligned, `posX = overflowPos(column, columns, k)` with `k = sourceAspect/fA`.
- `overflowPos(i, n, k) = ((1-k)/2 - i/n) / (1/n - k) * 100` // reduces to `i/(n-1)*100` when `k==1`

Properties: no distortion (single-axis size + auto), pixel-continuous windows, and identical to the full-catalogue `cover` render. When `slideAspect` is unknown it defaults to `sourceAspect` (=> `k=1`, exact edge-aligned tiling) so pure/unit contexts stay deterministic.

## Implementation steps

### 1. Add the canonical function — [index.tsx](framework/product/presentation/renderer/react/index.tsx)

Add `mosaicWindowedCoverVars(cell, grid, frame, sourceAspect, slideAspect)` returning `{ size, posX, posY }` per the formulas above, plus a small `overflowAxisPosition(i, n, k)` helper. This supersedes `figureMosaicBackgroundPosition` ([1369](framework/product/presentation/renderer/react/index.tsx)) for rendered tiles.

### 2. Route mosaic cells through it — `figureCropBackgroundVars`/`figureCropCoverVars`

- Detect the cell with `figureMosaicCellIndex(crop, embodiment.mosaic)` using the mosaic's own `frame` (not the slide frame).
- When a cell is detected: compute **rest** and **grid** (`fromMorphToFrame`) states with `mosaicWindowedCoverVars` regardless of any `morphTo` (fixes the `mosaicOnRest` collapse). Keep the **morph** state (`-morph` vars) as the existing centered cover of the crop into the focus box, so grid->focus auto-animate still interpolates.
- When no cell (arbitrary crop / focus / label): keep the existing per-crop centered cover, with `coverScale` only on this non-mosaic path.
- Rename the mislabeled `morphFrame` param to `morphToFrame` for clarity.

### 3. Thread `slideAspect`

Provide a `PresentationSlideAspectContext` (number | undefined) next to `PresentationFigureCropFrameContext` ([3870](framework/product/presentation/renderer/react/index.tsx)), fed from `parsePresentationSlideCssSize(reveal)` ([897](framework/product/presentation/renderer/react/index.tsx)). `FigureCropView` ([1635](framework/product/presentation/renderer/react/index.tsx)) reads it and passes it down; default `undefined` -> function uses `sourceAspect`.

### 4. Keep CSS/morph as-is — [globals.css](framework/product/presentation/renderer/react/globals.css)

Keyframes at [783-852](framework/product/presentation/renderer/react/globals.css) interpolate `--presentation-figure-bg-size/-position` and `-grid-*`/`-morph` variants; the new values are the same shape (`N% auto` + `% %`), so no keyframe changes are needed. Full image stays `cover`.

### 5. Update unit tests (extend existing file only)

In the renderer describe blocks, replace magic-number mosaic assertions ([5489-5492](framework/product/presentation/renderer/react/index.tsx), [5590-5605](framework/product/presentation/renderer/react/index.tsx), [5188-5197](framework/product/presentation/renderer/react/index.tsx), [5365-5371](framework/product/presentation/renderer/react/index.tsx)) with values computed from `mosaicWindowedCoverVars`: assert mosaic size is `"${columns*100}% auto"` (e.g. `500% auto`) and per-cell positions; keep full-image `cover` and single-crop centered-cover tests. Run `bun ./📜️script.ts test` in [renderer/react](framework/product/presentation/renderer/react) and `core`.

### 6. Browser-verify the grid

Open the deck, go to **Bauteilarten**: confirm each tile's `--presentation-figure-bg-size` is `~500% auto` (not `670%`), `Rippenplatte 1` (`tile-r1-c0`) `posX = 0%` showing only its own cell (no Rippenplatte 2 bleed), and the mosaic visually equals the full **Bauteilkatalog** image. Verify dragging the figure keeps tiles aligned (live frame via the crop-frame context).

### 7. Calibration (only if needed)

The algorithm guarantees correct _relative_ tiling. If the 3x5 grid still doesn't land on the PNG's product rows, adjust `CATALOGUE_FRAME` in [spec.ts:105](mit-bestand/präsentation/33.projektetage/spec.ts) — this is content calibration, independent of the tiling math.

## Out of scope

No structural DOM change (single-element + CSS vars retained so reveal auto-animate FLIP and existing morph keyframes keep working). No `git commit` unless requested; temp notes stay under the ticket folder; work tracked in `26/06/02/PROJEKTETAGE-CATALOGUE-GRID-FIDELITY`.
