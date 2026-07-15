# Verify Log — Window Row Bottom-Border Alignment

## Fix

`SemioWindowRowTwo` / `SemioWindowRowThree` measure natural column heights in `lrbox`, take the max, and pass it to row `Window` instances via `height=` on tcolorbox (`\SemioWindowStretchHeightValue` bridge).

`anchor` sets expl3 `\l_semio_window_valign_tl` / `\l_semio_window_halign_tl` so `valign` / `halign` work inside expl3 `\begin{tcolorbox}` (LaTeX `\semio@...` macros cannot be used there).

## Results

| Fixture                               | Build | Visual                                                   |
| ------------------------------------- | ----- | -------------------------------------------------------- |
| `verify-window-alignment.tex` p2      | OK    | Short/Tall bottom borders aligned                        |
| `verify-zwischenbericht-cover.tex` p1 | OK    | Institution + logo rows share row height; logos centered |

## Notes

- Stretched row windows emit ~6pt overfull hbox / ~3pt overfull vbox warnings (tunable later).
- Full `zwischenbericht.tex` still fails at `\makeworkpackages` (`Misplaced \noalign` in `\semio@table@row@sep`) — separate from window rows; cover verified via ticket fixture.

## Artifacts

- `verify-window-alignment-p2.png`
- `zwischenbericht-p1-cover.png`
