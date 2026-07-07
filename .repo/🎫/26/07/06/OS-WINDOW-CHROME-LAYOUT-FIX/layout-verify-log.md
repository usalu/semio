## Outline fix (v2)
- Header row moved **outside** body `tcolorbox` — outer L/R/B frame no longer duplicates tab cap borders (was the offset double-rectangle)
- Body (`semio~window`) draws only L+R+B with `top=-hairline` overlap onto gap bottom stroke → single continuous U outline
- Gap segment uses same `colframe=semio-chrome-border-normal` as tab/controls caps (no `\rule` color drift)
- Standalone `Semiobox` without title uses `semio~window~boxed` (full rectangle)

- U-header gap: replaced gap `tcolorbox` (drew spurious top edge) with canvas-filled flex region + bottom `\rule` only between tab and controls caps
- Hierarchy number: controls cap pinned to right via `gap_dim = linewidth - tab - controls` inside full-width `\hbox`
- Header/body seam: `\vtop` header row + `\vspace{-\semio@stroke@hairline}` overlap (OS `-mt-px`)
- `begindocument` hook: export `\SemioHeaderFooterApply` while `\ExplSyntaxOn` so navbar/footer hook works outside expl3

## Build
- `bun ./script.ts build forschungsbericht` — OK
- `bun ./script.ts build report report-dark` — OK (prior run)
- Removed 218.99994pt header overfull (was caused by `linewidth + 2*padding` bleed at wrong scope)
