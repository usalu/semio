# r2-lv: visual check of short dimension leaders

Agent `lv`. Checked the leader layout that `r2-ed` only verified with numeric assertions.

## Method
- Exported SVG (print view `dark-line`, scale 1:50, pose per asset, elevation) with the archive export CLI `🔨️modules/📤️export/🎬️cli/🟦️.ts`, `--lang en` and `--lang de`:
  - `tealight` pose `unlit` (height 0.0405 m, 0.81 mm on paper)
  - `thermostatic-radiator-valve` pose `open` (0.145 m, 2.9 mm)
  - `room-thermostat` pose `idle` (0.09 m, 1.8 mm)
  - `gas-hob` pose `off` (0.088 m, 1.8 mm)
- Rasterised with PyMuPDF (`fitz`, 400 dpi) via `🧰️lv/rasterise.py`. Read each PNG with the Read tool.
- Generated SVG and PNG files were deleted afterwards. The script is kept in `🧰️lv/`.

## Result: no defect found
For all four assets, in en and de (8 images):
- The label ("0.02 m", "0.15 m", "0.09 m" / "0,02 m", "0,15 m", "0,09 m") is readable and sits right of the leader, centred on its upper end.
- The leader rises from the upper oblique tick, straight up from the extension line's x. It attaches correctly and is not offset.
- The label does not overlap the drawing, the title block, the scale bar or the lower extension line.
- Upper and lower extension lines and ticks keep their positions. Nothing is cut.
- de uses the decimal comma and the German title and legend.

## Observations (not defects)
- Tealight: the upper extension line (`dimension:extension-top`, `🔨️modules/🎨️depiction/🏷️annotations/🟦️.ts:58`) runs about 0.2 mm above the candle body. The drawing is very thin, so the two lines nearly touch. This comes from the existing extension geometry, not from the leader change.
- Gas hob: the extension lines start at the right edge of the hob base, which is the intended behaviour.

## Code reference (for the record)
- `🔨️modules/🎨️depiction/🏷️annotations/🟦️.ts:16` `DIMENSION_FIT_MM = 3`
- `:17` `DIMENSION_LEADER_MM = 3.2`
- `:67-69` in-between label branch
- `:71-73` leader branch: leader `top` to `lead`, label at `x + 1.6`, baseline `lead + 0.35 * ANNOTATION_TEXT_MM`

## Open
- Only 4 assets and one view (`dark-line`, 1:50, elevation) were checked. Not checked: the front and plan viewpoints, other scales (1:100, 1:200), the `thermal` view, or a leader in a clip frame.
