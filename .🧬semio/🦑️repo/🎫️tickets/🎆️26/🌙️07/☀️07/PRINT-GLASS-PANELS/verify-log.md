# Verify Log — Print Glass Panels

## Build

```bash
cd print && bun ./📜️script.ts generate && bun ./📜️script.ts test
```

Result: **all 12 template PDFs built** (6 templates × light/dark).

## Manifest

Example `dist/report.panels`:

```
panel-1;2;412.56505;571.9985;156.49014;244.59569
```

Numeric pt values for page, x, y, width, height.

## Glass PNGs

Pass-2 frosted glass PNGs written under template workdirs:

- `print/template/report/.semio-panel-glass/panel-1.png`
- `print/template/paper/.semio-panel-glass/panel-1.png`
- `print/template/flyer/.semio-panel-glass/panel-1.png`
- `print/template/zukunftbau/.semio-panel-glass/panel-1.png`

(zukunftbau sub-templates share the zukunftbau workdir)

## Warnings

Minor `Overfull \hbox (5.87991pt too wide)` on panel shipout — fbox border/padding rounding; acceptable.

## Fixes applied this session

1. Manifest: `\iow_now:Nx` + `\fp_eval:n` for numeric pt output
2. Glass rasterize: resolve `@napi-rs/canvas` via pdfjs `createRequire` (dedupe Path2D types)
3. Pass-2 layout: `\rlap` + `\resizebox` glass background under centered text
4. Panel content: `minipage` width constraint in `lrbox` capture
5. **Saturation bug (color corruption)**: `sharp.modulate({ saturation })` expects a multiplier (`1.45`), not a percentage. Was `Math.round(glassSaturate * 100)` → `145`, producing neon cyan/magenta. Fixed to `glassSaturate` directly.

## Saturation fix verification (2026-07-08)

Pass-1 crop from `debug-out/pass1.pdf` at manifest coords:

| Stage           | File                        | Result                                          |
| --------------- | --------------------------- | ----------------------------------------------- |
| Raw crop        | `pass1-crop-raw.png`        | Dark bg + red TOC chips/lines visible           |
| Blur + sat 1.45 | `pass1-crop-blur-fixed.png` | Subtle frosted red blurs on dark bg ✓️           |
| Blur + sat 145  | `pass1-crop-blur-bad.png`   | Neon cyan + magenta blobs (reproduces user bug) |

Rebuild: `bun ./📜️script.ts build kompaktbericht` — light/dark PDFs built successfully.

## Border alignment fix (2026-07-08)

TikZ `path picture` + `inner sep` made the border wider than the glass PNG. Fixed by drawing image, inset border stroke, and text in one `tikzpicture` with `\useasboundingbox (0,0) rectangle (#2,#4)` so both share exact manifest dimensions.
