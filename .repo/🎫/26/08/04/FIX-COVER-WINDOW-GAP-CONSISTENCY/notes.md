# Cover window gap consistency

## Symptom
Zwischenbericht cover (`\makecoverpages`) showed alternating wide/narrow vertical gaps between `WindowColumn` tiles; horizontal gaps could also diverge from vertical.

## Root causes
1. **`after~skip` leak** — `semio~window` sets `after~skip=0.85em`. Standalone column windows leaked it into the outer vertical list; `WindowRow` siblings did not (trapped in minipage) → alternating vertical seams.
2. **`\parskip` leak on titlepage** — `WindowRow` ends with `\par` while document `\parskip` is `0.5em`. After row items: `\parskip + \vspace{4pt}` ≈ 10pt; after single windows: `4pt` only.
3. **Stray `\par` in arrangement items** — non-`text-size=fit` windows called `\par` before `\end{tcolorbox}` inside column items.

## Fix
- `semio~window~row`: `before~skip=0pt`, `after~skip=0pt` — only `\semio@window@gap` governs arrangement spacing.
- `WindowArrangement`: `\parskip=0pt` for its body group.
- `makecoverpages` titlepage: `\parskip=0pt` belt-and-suspenders on cover.
- `semio_window_generic_end`: skip trailing `\par` when `\l_semio_window_in_item_bool`.

## Verification (PDF pixel probe, page 1)
After rebuild: meta row horizontal gaps `3.50pt` (n=1), logos row `3.50pt` (n=2); full-page border-gap medians H=V≈`3.50pt`.

## Token alignment (second pass)
Cover had `\setlength{\semio@window@gap}{4pt}` — off-document. Arrangement gaps now track **block↔block** spacing:

| Junction | Token | Value |
|---|---|---|
| Paragraph ↔ paragraph | `\parskip` | `\semio@spacing@par` (0.5em) |
| Block ↔ block (incl. window tiles) | `\semio@block@sep@skip` = `\semio@window@gap` | `\semio@spacing@single` (0.2em) |
| Heading → following block | `\semio@block@before` + `\semio@heading@topskip` / `\semio@heading@bottomskip` | separate ladder |
| Standalone table/figure window | `after~skip` | `\semio@spacing@table@outer` (0.85em) — zeroed inside arrangements |

`\AfterEndPreamble` sets `\semio@window@gap` from `\semio@block@sep@skip`. Removed cover `4pt` override.
