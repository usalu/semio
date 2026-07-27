# Final status — Zwischenbericht layout/rendering pass (non-table scope)

Scope per user: implement everything EXCEPT the tables (the table title
separator/borders were set aside on a separate branch, and the table files
`semio-table.sty` were left untouched).

## Committed & verified fixes (branch kinan/bericht-diff)

Verified on a clean build: **0 undefined-cs, 0 overfull, 105 pp, no blank/near-empty pages.**

| # | Fix | File | Verification |
|---|---|---|---|
| 2.6 | `\parskip` reset in itemize/enumerate/description + bibliography list | semio-window.sty | compile-clean |
| 2.7 | Heading reserves a second title-bar (keep-with-next) | semio-window.sty | 105pp, no orphan regressions |
| 2.8 | Running headers via `\markright`/`\rightmark` (fixes early advance) | semio-window.sty | p45=P.K.39, p55=P.K.59 name the page's own section |
| 2.9 | Running-header chip width-capped (no overlap with centre chip) | semio-window.sty | compile-clean |
| 2.11 | `\SemioCardKeep` needspace before all 67 project cards | semio-window.sty + anhang/projekte.tex | P.K.67 heading + Bauteile table co-located on p59; 105pp unchanged |
| 2.15 | Contents `\&` → plain `&` (write-path normalization) | semio-window.sty | "Kultur- & Gewerbehaus" plain in TOC/header/bib |
| 2.16 | Glossary header `VerwendungimBericht` → "Verwendung im Bericht" | zwischenbericht.tex | spaces present |
| 2.17 | `\bibitemsep` shrink so final bib page isn't sparse | semio-window.sty | last page 1513 chars |

Commits: the fixes landed across the user's "style" commits plus
`07a18eff2` (2.6/2.17 bib) and `c32ba3340` (2.11 card-keep).

## Already correct in source (verify-only, no change)
- **2.1** blank pages (`\SemioOpenAny{true}` + `\SemioSignaturePadding{false}`)
- **2.5** external table spacing (centralized token; must not grow — nested cards)
- **2.14** framed system (one shared cap+box system on two tokens)

## Not done — table files, excluded per "don't touch the tables"
- **2.2 / 2.3 / 2.14-table** — table title separator/borders (user's separate branch).
- **2.4** cell padding, **2.13** hyphenation — `semio-table.sty`.
- **2.10** title-bar padding — shares the cap macro that table titles use.
- **2.12** continuation "(Fortsetzung)" markers — `semio-table.sty` (`\semio@table@long@continuation@chrome`).

## Environment note
No PDF rasterizer available (only `pdftotext`); verification was text/log/page-count
based. Pixel-level appearance of padding and header layout was not rasterized.
