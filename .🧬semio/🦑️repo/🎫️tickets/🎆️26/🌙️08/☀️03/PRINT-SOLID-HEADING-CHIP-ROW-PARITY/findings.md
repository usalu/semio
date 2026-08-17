# Findings

## 1. The requested heading-row design is already in the source

`🖋️semio-window.sty` `%region Headings` renders every numbered heading tier as
`[chip]` + transparent gap carrying only a bottom hairline + `[number chip]`:

- `\semio@heading@pair` (solid tiers) → `\semio@heading@chip` … `\semio@window@header@gap@paint` … `\semio@heading@chip`
- `\semio@heading@pair@outline` (outline tiers) → same row, no-fill chips
- `\semio@window@header@gap@paint` paints `\rule{0pt}{body}` (invisible) + `\semio@window@stroke@h` (bottom hairline)

No code path anywhere in print paints a full-`\linewidth` fill for a heading
(grep for `cap@paint` / `rule{\linewidth}` — only chip-width fills exist).

Rendered proof (from the real `.sty`, both themes): `report-light-3.png`,
`report-dark-3.png`, `baseline-1.png`.

The full-width coloured bars in the reported screenshot are the *pre-#1965*
rendering (header gap cutout, ticket 26/07/09 PRINT-WINDOW-BORDER-GAP).

## 2. Why the screenshot is stale: print has not built since the emoji rename

Commit `e01818cc4` (Jul 31) renamed every LaTeX file to the repo emoji
convention (`tex/semio.cls` → `🖋️latex/🖋️semio.cls`, `report.content.tex` →
`🖋️report.content.tex`, `Anta-Regular.ttf` → `🔤️Anta-Regular.ttf`, …).

**TeX cannot resolve file names containing astral-plane characters.** Probe
(`probe/probe5.tex`): a file `x🖋y.tex` makes the filename scanner stop at the
emoji and tectonic looks for `x.tex`; a BMP name (`é.tex`) resolves fine. Same
for `\documentclass{🖋️semio}` / `\RequirePackage{🖋️semio-tokens}` / `\input`.

Consequences, all currently broken:
- `\documentclass{semio}` → `File 'semio.cls' not found`
- `\RequirePackage{semio-window}` etc. — same
- `\input{🖋️report.content.tex}`, `\addbibresource{📚️references.bib}`
- `semio-fonts.sty` font paths (`../../asset/font/anta/🔤️Anta-Regular.ttf`)
- `\SemioImage{../../asset/🖼️demo-strip.png}`
- `♻️mit-bestand/📋️bericht` — `\input{anhang/…}` and ~60 `asset/…/🖼️*.png` refs

Directories may keep emoji names when reached via tectonic `-Z search-path`
(CLI-provided, never scanned by TeX) — only names written inside `.tex` and
package/class names must be ASCII. The ticket harness proves this: ASCII
symlinks in an emoji-named ticket directory compile fine.

## 3. Fixed in this ticket

`📓️print/⚡️implementation/🟦️typescript/📜️script.ts` — stale post-reorg paths:
`tokensPath`, `texDir`, new `templateRoot`, `fontRoot`, font `dir`s, `TEMPLATES`.

## 4. Harness

`harness/build.sh` + `harness/tex/` (ASCII symlinks to the real `.sty`/`.cls`,
plus a local ASCII-path `semio-fonts.sty`) compiles the real print sources.
