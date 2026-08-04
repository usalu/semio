## Root cause

Muted title/number chips are open-bottom tabs: one shared hairline baseline closes both chips (not per-chip bottom borders in the gap).

## Fix (single line, no double stroke)

1. **tcolorbox windows** (`Table` + `SemioTable`, `Figure`, `Window`):
   - Inline: `\semio@window@header@invoke@tcb` (chips only, `\semio@window@header@draw@baselinefalse`).
   - Overlay page 1: `\semio_window_header_overlay_baseline:` draws one line at `frame.north~west`–`frame.north~east` (must use `~` in anchor names; `frame.north west` broke the build).
   - Overlay continuation: full `\semio@window@header@muted` (with baseline) in a `\node` at `frame.north`.
   - Gap spacer `\semio@window@header@gap@open` no longer draws a mid stroke; full-width baseline row in `\semio@window@header@muted@core` supplies the bottom edge for both chips.

2. **Long tables** (`SemioTableLong`, TOC, glossary, `\SemioProject`):
   - `\semio@table@long@title@chrome@row` uses `\semio@window@header@invoke` (baseline inline).
   - No extra `\hrule` after title chrome (was parallel line with invoke baseline).

## Follow-up: the build the chip/border rewrite left broken

Four defects, each of which halted `zwischenbericht` on its own.

1. **`\SemioTableBegin` colspec dispatch** — `\begin{SemioTable}{0.07,…}` failed with
   `Missing number, treated as zero` (`<to be read again> \setbox`). The dispatch was
   written as several delimited `\def`s sharing one name
   (`\semio@table@begin@window@route@spec` etc.); TeX has no overloading, so each later
   `\def` silently replaced the previous one and `\SemioTableWindowColspec` leaked into
   the fraction list. The first column width then read
   `\dimexpr\SemioTableWindowColspec 0.07\semio@table@long@inner@w…`, re-entering
   `\semio_table_window_prepare:n` (and its `\setbox`) inside a number scan.
   Fixed by deleting the dispatch: every call site passes a fraction clist, so
   `\SemioTableBegin` now calls prepare + tabular open directly. Removed with it:
   `\SemioTableWindowColspec(Use)`, `ll`/`lll`/`nrp` specs, `\SemioTableThree`,
   short `colspec@three/register/glossary`, columntypes `U`/`F`, `\SemioTablePad`,
   `\semio@cell@preR`, `\semio@table@window@cellfill@first`, `\SemioTableWindowPrepare@do`,
   `\g_semio_table_window_colspec_tl` — all unreachable.

2. **Segmented row rules** — `\semio@table@long@rules@scan` emitted `\hrule` inside an
   `\hbox` (`You can't use \hrule here`) and its recursion had no terminator
   (`\sentinel` was never followed by the `,` its parameter text required).
   Replaced by `\semio_table_long_rules_segments:n`, a `\clist_map_inline:nn` loop of
   `\vrule height\arrayrulewidth`, with `\nointerlineskip` on both sides instead of the
   `\vskip\arrayrulewidth` pair. `\semio@table@long@rules@draw` stays plain-`\def`
   expandable so `\noalign` is still visible to TeX after `\\`.

3. **`\nointerlineskip` in horizontal mode** — the baseline row added to
   `\semio@window@header@muted@core` sits in a `\vbox` that opened with
   `\hskip-\semio@stroke@hairline`, which starts a paragraph. Both branches now use
   `\moveleft\semio@stroke@hairline\hbox…`, keeping the vbox in vertical mode.

4. **`\exp_not:N` in the long-table head** — `\tl_put_right:Nn` stores verbatim, so the
   marker survived as `\noexpand` in front of `\semio@table@long@hhline@current`. TeX's
   post-`\\` peek for `\noalign` is an expanding read, so the shielded macro read as an
   ordinary token, the row opened, and the rule ran inside a cell (`Misplaced \noalign`
   — and, before this rewrite, the long-standing `Misplaced \omit` for `\hhline`).

Also restored: `\SemioProject`'s `P.K.x` key chip. `\semio@table@long@title@chrome@row`
now renders through `\semio@window@header@invoke`, which reads
`\semio@window@header@number@page1`, but the card only set `\semio@window@header@number`.
New `\semio@window@header@number@set` sets both for callers outside a window.

## Nx caching for the print/report builds

`build` is cacheable via `nx.json` `targetDefaults`, but neither `print` nor
`mit-bestand/bericht` declared `outputs` — so Nx replayed a "success" that wrote no
PDFs at all, and their inputs did not cover `print/tex`, so shared LaTeX edits never
invalidated the cache. Both build targets now declare `outputs` and a new shared
`print` named input (`print/tex/**`, `print/script.ts`, `ui/styling/tokens.json`).
A named input rather than an implicit dependency: `build` has `dependsOn: ["^build"]`,
so making the report depend on `@semio-tech/print` would drag all six templates into
every report build.

Verified: cold build 166s → cache replay 3.6s with both PDFs restored into
`zwischenbericht/dist`; a one-line edit to `print/tex/semio-table.sty` invalidates it
(197s full rebuild).

## Build status

- Print unit tests: pass.
- `mit-bestand/bericht` `zwischenbericht`: light + dark build clean, 108 pages.
- `print` templates: report, paper, flyer, forschungsbericht, zwischenbericht,
  kompaktbericht — all light + dark build clean.
- Rendered checks (`render.ts`, PNGs in this folder): `zb-*`/`fin-*` cover the
  Meilensteine short table, TOC register, project cards (chips back) and a
  `SemioTableLong` appendix page; `probe*.tex` are the reduced reproductions.

## Files

- `print/tex/semio-window.sty`
- `print/tex/semio-table.sty`
- `nx.json`
- `print/project.json`
- `mit-bestand/bericht/project.json`
