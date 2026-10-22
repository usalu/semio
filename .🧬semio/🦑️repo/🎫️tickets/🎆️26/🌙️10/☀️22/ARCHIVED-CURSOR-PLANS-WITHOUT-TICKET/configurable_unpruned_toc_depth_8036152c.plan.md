---
name: Configurable unpruned TOC depth
overview: Decouple the Semio Table-window TOC's inclusion depth from `secnumdepth`, default it to "show everything" (all chapters/sections/subsections/paragraphs, including nested `SemioNest` paragraphs), and expose a way to configure a specific depth when pruning is wanted.
todos:
 - id: decouple-tocdepth
   content: Add unlimited tocdepth default + \SemioTocDepth, remove secnumdepth coupling from \tableofcontents in semio-window.sty
   status: completed
 - id: maketoc-optional-depth
   content: Give \maketableofcontents an optional depth argument in semio-components.sty
   status: completed
 - id: wrap-chapter-part
   content: Add guarded \chapter and \part TOC-tracking wrap helpers in semio-window.sty
   status: completed
 - id: nest-depth-guard
   content: Add tocdepth guard to \semio@nest@paragraph so nested SemioNest entries are prunable too
   status: completed
 - id: verify-build
   content: Rebuild zwischenbericht + report/forschungsbericht templates, confirm unpruned default and working configurable depth
   status: completed
isProject: false
---

## Root cause

The TOC currently prunes entries via `\value{tocdepth}`, but `tocdepth` is force-set from `secnumdepth` on every call:

- [`print/tex/semio-components.sty`](print/tex/semio-components.sty) line 94-97: `\maketableofcontents` does `\setcounter{tocdepth}{\value{secnumdepth}}` before `\tableofcontents`.
- [`print/tex/semio-window.sty`](print/tex/semio-window.sty) lines 1347-1351: the redefined `\tableofcontents` does the same thing again.

`secnumdepth` is meant to control whether a heading _shows a number_ in the body text (checked via `\if\relax\detokenize{\csname the#3\endcsname}\relax` in `\semio@heading@pair` / `\semio@koma@heading@block`), not whether it belongs in the TOC. Tying the two together means:

- Standard class default (`secnumdepth=3`, i.e. up to subsubsection) silently prunes all `\paragraph` entries from the TOC unless a document manually raises `secnumdepth` (as `zwischenbericht.tex` does with `\setcounter{secnumdepth}{4}`).
- There is no way to show a TOC entry without also numbering it in the body, or vice versa.

Additionally, two other gaps prevent "show everything" from actually being everything:

- `\chapter` and `\part` are never wrapped for TOC tracking. `\semio@heading@wrap@install` (lines 1021-1026) only wraps `section`/`subsection`/`subsubsection`/`paragraph`. Templates that use `\chapter` (e.g. [`print/template/report/report.content.tex`](print/template/report/report.content.tex), [`print/template/zukunftbau/forschungsbericht.content.tex`](print/template/zukunftbau/forschungsbericht.content.tex)) never get chapter entries in the TOC at all, regardless of depth.
- Nested `SemioNest` paragraphs are tracked unconditionally in `\semio@nest@paragraph` (line 1103-1108) — there is no depth check at all there, so they can't currently be pruned even if a document wanted a shallower TOC.

## Design

Introduce one depth counter (`tocdepth`, already a standard LaTeX counter) as the single, independent knob for TOC pruning, with a generous "unlimited" default that is never derived from `secnumdepth`.

- Semio's internal level scale (already defined at lines 927-940 of `semio-window.sty`): `part=-1, chapter=0, section=1, subsection=2, subsubsection=3, paragraph=4`. Nested `SemioNest` paragraphs go one level deeper per nesting (`4 + \value{semio@nest@depth}`).
- Add `\newcommand{\semio@toc@depth@unlimited}{99}` and set `\setcounter{tocdepth}{\semio@toc@depth@unlimited}` once during document setup (in the existing `\AfterEndPreamble` block, alongside the other one-time inits) — comfortably above the deepest possible level (paragraph + 5 nest levels = 9).
- Remove the `\setcounter{tocdepth}{\value{secnumdepth}}` line from the `\tableofcontents` redefinition in `semio-window.sty` — `\tableofcontents` should just render at whatever depth is currently set, not silently reset it every call.
- Change `\maketableofcontents` in `semio-components.sty` to take an optional depth argument:

```latex
\NewDocumentCommand{\maketableofcontents}{O{}}{%
  \IfValueT{#1}{\setcounter{tocdepth}{#1}}%
  \tableofcontents
}
```

No argument -> keeps whatever depth is currently active (the unlimited default, unless something else set it). Explicit argument (e.g. `\maketableofcontents[2]`) -> prunes to that level.

- Add a standalone public command for setting depth outside of `\maketableofcontents` (e.g. before a raw `\tableofcontents` call), matching existing naming conventions like `\SemioChromeIconSize`:

```latex
\NewDocumentCommand{\SemioTocDepth}{m}{\setcounter{tocdepth}{#1}}
```

## Fix pruning gaps

1. **Chapter/part tracking** — add wrap helpers mirroring `\semio@heading@wrap@section` (lines 983-990 of `semio-window.sty`), guarded the same way `\semio_header_footer_apply:` already guards chapter usage (line 496: `\cs_if_exist:NF \chapter {...}`):

```latex
\newcommand{\semio@heading@wrap@part}{%
  \let\semio@orig@part\part
  \def\part{\@ifstar{\semio@orig@part*}{\semio@part@tracked}}%
  \def\semio@part@tracked##1{%
    \semio@orig@part{##1}%
    \semio@heading@wrap@tracked{part}{##1}%
  }%
}

\newcommand{\semio@heading@wrap@chapter}{%
  \let\semio@orig@chapter\chapter
  \def\chapter{\@ifstar{\semio@orig@chapter*}{\semio@chapter@tracked}}%
  \def\semio@chapter@tracked##1{%
    \semio@orig@chapter{##1}%
    \semio@heading@wrap@tracked{chapter}{##1}%
  }%
}
```

Update `\semio@heading@wrap@install` to call `\semio@heading@wrap@part` always, and `\semio@heading@wrap@chapter` only `\ifcsname chapter\endcsname` (hyperref never defines a real `\chapter` command on `scrartcl`, so this guard is safe and matches the existing pattern at line 496).

2. **SemioNest depth check** — guard the TOC-tracking call in `\semio@nest@paragraph` (currently unconditional) with the same `>\value{tocdepth}` check used everywhere else, using the nested effective level:

```latex
\newcommand{\semio@nest@paragraph}[1]{%
  \semio@nest@counter@step
  \edef\semio@nest@num{\semio@nest@build@number}%
  \ifnum\numexpr\semio@toc@level@paragraph+\value{semio@nest@depth}\relax>\value{tocdepth}\else
    \semio@toc@track@num{paragraph}{\semio@nest@num}{#1}%
  \fi
  \semio@heading@pair@muted@num{paragraph}{\semio@nest@num}{#1}%
}
```

This only gates the TOC entry — the in-body heading chip (`\semio@heading@pair@muted@num`) always still renders, since depth only concerns TOC pruning, never body-heading visibility.

## Files touched

- [`print/tex/semio-window.sty`](print/tex/semio-window.sty) — unlimited-depth default, `\SemioTocDepth`, drop secnumdepth coupling in `\tableofcontents`, add `\chapter`/`\part` wrap helpers (guarded), fix `\semio@nest@paragraph` depth check.
- [`print/tex/semio-components.sty`](print/tex/semio-components.sty) — `\maketableofcontents` optional depth argument, drop the secnumdepth-derived `\setcounter{tocdepth}{...}` line.

No `.tex` document changes are required: default behavior becomes "show everything" automatically for `zwischenbericht.tex`, `forschungsbericht.content.tex`, and `report.content.tex` alike. `secnumdepth` in `zwischenbericht.tex` (`\setcounter{secnumdepth}{4}`) is left untouched since it still legitimately controls in-body heading number visibility.

## Verification

1. Rebuild `mit-bestand/bericht/zwischenbericht` (`bun ./📜️script.ts build` in `mit-bestand/bericht`) and confirm the Table-window TOC lists every section/subsection/subsubsection/paragraph/nested-`SemioNest`-paragraph with no entries missing.
2. Run `bun print/script.ts test` (or the equivalent build for `report.content.tex` / `forschungsbericht.content.tex`) to confirm `\chapter` entries now appear in those templates' TOCs and nothing regresses/breaks the build (particularly the guarded `\chapter` wrap on `scrartcl`-based documents, which must remain a no-operation there).
3. Spot-check the configurable-depth path by temporarily calling `\maketableofcontents[2]` in one test build and confirming subsubsection/paragraph entries are correctly omitted, then revert.
