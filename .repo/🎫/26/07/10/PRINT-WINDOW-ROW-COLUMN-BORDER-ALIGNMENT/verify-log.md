# Verify Log

## Builds

- `bun run nx run mit-bestand-bericht:build` — success (zwischenbericht light + dark)
- `bun ./verify.ts` — success (verify-window-alignment light + dark)

## Rasterized outputs

- `verify-window-alignment-p1.png` — row/column bisect fixture
- `zwischenbericht-p1-cover.png` — mit-bestand cover page with logo row

## Notes

- Row macros measure each column at final width, set shared `height=` on child `Window` tcolorboxes, then render once more for output.
- Column macros measure natural width inside `\vbox` + `lrbox` with `fit=true`, stretch to max width, stack vertically with `\semio@block@sep`.
- Cover logo windows use `anchor=center` so logos center inside the stretched row height.
