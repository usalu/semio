# SemioProject overview band inset

## Cause

`\semio@project@overview@band` put photo + meta in a top box with no top air.
The image sat flush on the chip baseline (**0.00pt** measured on Kopfbau /
P.K.1). Body cells / window bodies use `\semio@window@bodypad` (5.5pt).

Plain `\vskip` / `\vspace` at a `p`-column / minipage top is discarded or
picks up `\baselineskip`; unpainted glue also shows page cream under the chip
hairline (reads as a chip-seam gap).

## Fix (`print/tex/semio-table.sty` ProjectCatalogue)

1. `\semio@project@band@toppad` — canvas-coloured `\rule{\linewidth}{\semio@window@bodypad}`
   so the inset is painted (no cream slit under the chip baseline).
2. Photo / meta wrapped in `\vbox` (meta: `\vbox to \semio@project@bandheight` +
   `\vss`) so both cells share one height and stay top-aligned.
3. Horizontal: `\hspace{\semio@table@long@cellpad}` + photo width
   `\linewidth-2\semio@table@long@cellpad`; meta gets `\semio@cell@inset`.
4. Photo height absorbs top bodypad (`bandheight - bodypad`) so the band stays
   ~26mm. Component colspec / `\semio@cell@struttop` untouched.

Chip seam welding for muted chips remains in `semio-window.sty` (open-chip
geometry); title chrome already pulls with
`\noalign{\vskip-\extrarowheight-\dp\@arstrutbox}`.

## Measurements (Kopfbau Halle 118)

| | photo top inset | notes |
|---|---|---|
| **before** | **0.00pt** | flush on chip baseline |
| **after** | **5.50pt** | page 23 full Zwischenbericht |
| target | 5.5pt | `\semio@window@bodypad` |
| meta after | 6.50pt | first dark ink (label) |
| chip seam after | single rule | welded |

Probe: `probe-project-band.tex` → photo 5.75pt / meta 6.75pt.
