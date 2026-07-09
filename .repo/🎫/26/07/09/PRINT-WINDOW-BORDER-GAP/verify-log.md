# Print Window Border Gap

## Problem

Window title bars rendered **below** box content instead of above it; cover titles could clip at the page top. Builds also appeared frozen when many orphaned `tectonic` processes were running.

## Root cause (title bar order)

`\semio@window@header@muted` wrapped the chip row + separator in `\vtop{...}` while working heading chips use `\vbox{...}` in `\semio@heading@row@wrap`.

Per the TeX box model:

- `\vbox{A B}` — reference point at the bottom of the last item → header height is correct; `tcolorbox` body follows immediately after the separator.
- `\vtop{A B}` — reference point at the bottom of the first item (chip row) → most of the header becomes **depth** below the baseline, so labels visually land under content and the cover title can clip at the top.

## Additional fixes uncovered during verification

1. **TeX hang / frozen builds** — `\semio_window_tier_header:nnn` used `\tl_set:Nn` with `{ \tl_use:N \l_semio_window_kind_title_tl }`, storing a self-referential token list. Combined with `\exp_args:Nx` in `\semio_window_header_store:`, this caused infinite expansion. Fixed with `\tl_set:Nx` for titles and `\exp_args:NNo` header store (number tokens deferred to LaTeX2e).
2. **Cover page `\centering`** — global `\centering` in `\makecoverpages` put block-level `\vbox` headers in horizontal mode (`\prevdepth` errors / extreme slowdown). Removed page-level `\centering`; title text centers inside the Titel window body.
3. **Header row wrap** — `\semio@window@header@row@wrap` guards `\par` / `\nointerlineskip` with `\ifvmode`.
4. **Generic `Window`** — `breakable=false` for non-row cover/metadata windows.

## Changes

### `print/tex/semio-window.sty`

- `\vtop` → `\vbox` in `\semio@window@header@muted`.
- Removed abandoned inbox-header / double-hairline dead code.
- `\semio_window_tier_header` title assignment uses `\tl_set:Nx`; number uses `\tl_set:Nn`.
- `\semio_window_header_store` uses `\exp_args:NNo` + `\semio_window_header_store_aux` in `\ExplSyntaxOff`.
- `\semio@window@header@row@wrap` vertical-mode guards.
- Generic `Window` uses `breakable=false` when not `row`.

### `print/tex/semio-components.sty`

- `\makecoverpages` no longer applies global `\centering`; Titel window body uses local `\centering`.

## Verification

```bash
cd print/template/zukunftbau && tectonic --outdir ../../dist zwischenbericht.tex
cd print/template/report && tectonic --outdir ../../dist report.tex
cd .repo/🎫/26/07/09/PRINT-WINDOW-BORDER-GAP && bun rasterize.ts
```

Raster captures:

- `zwischenbericht-p1-windows.png` — cover chips + separators above window bodies; title not clipped.
- `report-p4-window.png` — in-body window chrome (report build succeeds end-to-end).

Both templates build in ~3–4s single-pass after fixes (no infinite TeX loop).
