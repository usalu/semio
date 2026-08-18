# TeX compile / determinism (post-P1)

## Packages

79 `semio-viz-chart-*.sty` files; `semio-viz-charts.sty` requires exactly those 79. Loader matches disk. `unknown-layout` is `\seq_if_in:NnTF` and does not expand a missing family cs.

## P1 gallery recompile

`compile-p1-fix.log`: 1, 2, 3, 6, 7, 8, 10, 15, 17, 26, 51, 76, 78, 79 all PASS (underfull `\vbox` only). `viz-api` failed once on `\SemioVizAxis[scale=apisym]` with domain `-10, 10` (`Invalid operation (0)/(0)` in tick mapping). After binding left to figure scale `y` and bottom to `apiq` with domain `0, 10`, `bun ./print/script.ts build viz api` exits 0 (light + dark).

`viz-79` chapter title still overfull `\hbox`; tectonic exit 0.

## Coverage

`bun ./print/script.ts test viz` → `1966/1966 leaves, API 13/13`.

## Full rebuild

`bun ./print/script.ts test viz full` exit 0 after 15229992 ms.

- 81 galleries × light+dark = 162 PDFs
- `pdfStableHash` on `viz-api` stable: `2e7209f8ba5b`
- No `unknown-layout` / `unknown-chart` / `unknown-demo`
- Underfull `\vbox` accepted throughout
- Overfull `\hbox` previously recorded on the `viz-79` chapter title (H worker); this full run’s stdout has underfull `\vbox` only

**Verdict: PASS**
