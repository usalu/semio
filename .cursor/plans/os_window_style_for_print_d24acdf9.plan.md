---
name: OS Window Style for Print
overview: Port the semio OS desktop window chrome (title bar + hairline border + window/canvas fill, sharp corners, Anta font) into the print LaTeX framework as a single shared tcolorbox style, and make chapters and sections render as these OS-style windows across all print templates.
todos:
 - id: tokens
   content: Add chrome color resolver + emission to print/script.ts (light+dark)
   status: completed
 - id: theme-alias
   content: Add theme-resolved chrome color aliases in semio-core.sty
   status: completed
 - id: window-sty
   content: Create print/tex/semio-window.sty with base style + chapter/section auto-windowing + Semiobox migration
   status: completed
 - id: cls-wire
   content: Require semio-window.sty from semio.cls; remove old semio box style from semio-components.sty
   status: completed
 - id: template-fixes
   content: Fix kompaktbericht.tex Titelblatt/maketitle and bibliography heading
   status: completed
 - id: verify
   content: Regenerate tokens, rebuild all print templates and mit-bestand zwischenbericht, visually verify chrome styling
   status: completed
isProject: false
---

## Goal

Every `\chapter`/`\section` (and every manual `Semiobox`) in the print framework renders with the exact same visual language as an OS desktop window: a title bar showing the paragraph/chapter name, a hairline `#7b827d` border, sharp `0mm` corners, no shadow, Anta-font title text, `window`-colored title bar (`#ebe8d9` light) over a `canvas`-colored body (`#f0ecdd` light). This becomes one shared mechanism (`semio-window.sty`) used by report, paper, flyer, and the full Zukunft Bau family, including [mit-bestand/bericht/zwischenbericht/zwischenbericht.tex](mit-bestand/bericht/zwischenbericht/zwischenbericht.tex).

## Reference: OS window chrome (source of truth)

From [ui/js/react/index.tsx](ui/js/react/index.tsx) (`ModeDockTabBar`/`ModeDockStack`) and [ui/styling/tokens.json](ui/styling/tokens.json) `themes.light.chrome`:

- Title bar fill: `window` token → `light-6-7` = `#ebe8d9`
- Body fill: `canvas` token → `light-8-9` = `#f0ecdd`
- Border color: `borderNormal` → `gray` = `#7b827d`, active/accent border: `activeBase` → `primary` = `#ff344f`
- Title text color: `borderEmphasized` → `dark` = `#001117`
- Border width: hairline, `strokes.chromeBorderHairline = 1.0`
- Corner radius: `radii.chrome = 0.0` (sharp)
- Font: Anta (sans), no shadows anywhere
- Dark theme equivalents: `window`→`dark-8-9` `#07181d`, `canvas`→`dark-6-7` `#0c1c21`, `borderEmphasized`→`light` `#f7f3e3`

## 1. Emit chrome colors as LaTeX tokens

In [print/script.ts](print/script.ts) `emitSemioTokensSty()`:

- Add a small paint resolver (mirrors `resolvePaint`/`blendHex` in [ui/styling/script.ts](ui/styling/script.ts)) to resolve `tokens.themes.light.chrome` and `tokens.themes.dark.chrome` entries (`{ token }` / `{ mix }` refs) against `tokens.colors`.
- Emit per-theme colors into `semio-tokens.sty`: `semio-chrome-light-window`, `semio-chrome-light-canvas`, `semio-chrome-light-border-normal`, `semio-chrome-light-border-emphasized`, `semio-chrome-light-active-base`, `semio-chrome-light-active-foreground`, and the `-dark-` equivalents.
- Prefer `tokens.strokes.chromeBorderHairline` (falls back to current `gridLarge` heuristic) for `\semio@stroke@hairline`, since this is now the semantically correct source for window borders.

## 2. Theme-resolved aliases

In [print/tex/semio-core.sty](print/tex/semio-core.sty) `semio_theme_apply:`, after existing `\pagecolor`/`\color` calls, add `\colorlet` aliases resolved for the active theme so downstream style code never branches on theme:

```
\colorlet{semio-chrome-window}{semio-chrome-\l_semio_theme_tl-window}
\colorlet{semio-chrome-canvas}{semio-chrome-\l_semio_theme_tl-canvas}
\colorlet{semio-chrome-border-normal}{semio-chrome-\l_semio_theme_tl-border-normal}
\colorlet{semio-chrome-border-emphasized}{semio-chrome-\l_semio_theme_tl-border-emphasized}
\colorlet{semio-chrome-active-base}{semio-chrome-\l_semio_theme_tl-active-base}
```

(expanded via `\edef`/`\str_use:N` since `\l_semio_theme_tl` is a token list, not directly usable in a command name — resolved with `\exp_args:Nc`/similar expl3 expansion).

## 3. New shared style: `semio-window.sty`

New file, required by [print/tex/semio.cls](print/tex/semio.cls) after `semio-components`:

**Base tcolorbox style `semio window`:**

```
\tcbset{
  semio window/.style={
    enhanced, breakable, arc=0mm,
    boxrule=\semio@stroke@hairline,
    colframe=semio-chrome-border-normal,
    colback=semio-chrome-canvas,
    colbacktitle=semio-chrome-window,
    coltitle=semio-chrome-border-emphasized,
    fonttitle=\SemioSans,
    attach title to upper=false,
    left=\semio@spacing@compact, right=\semio@spacing@compact,
    top=\semio@spacing@touch, bottom=\semio@spacing@touch,
  },
  semio window chapter/.style={semio window, fonttitle=\SemioSans\large},
  semio window section/.style={semio window, fonttitle=\SemioSans\small, left=\semio@spacing@compact+\semio@spacing@touch},
}
```

**`Semiobox` becomes sugar for this same style** (replaces the current separate "semio box" style in [print/tex/semio-components.sty](print/tex/semio-components.sty)):

```
\NewDocumentEnvironment{Semiobox}{O{}}{\begin{tcolorbox}[semio window,#1]}{\end{tcolorbox}}
```

This is what makes report/paper/flyer/zukunftbau share one literal style definition for every box, whether auto-generated (chapter/section) or hand-written (funding acknowledgement, work packages, flyer tiles).

**Auto-windowing of `\chapter`/`\section`:**

- State: `\l_semio_window_chapter_open_bool`, `\l_semio_window_section_open_bool` (expl3 booleans in `semio-window.sty`).
- `\semio_window_close_section:` — emits `\end{tcolorbox}` if a section window is open.
- `\semio_window_close_chapter:` — closes section first, then emits `\end{tcolorbox}` if a chapter window is open.
- `\semio_window_open_chapter:n{#1}` — closes previous chapter (and its section), does `\clearpage`, `\stepcounter{chapter}`, `\chaptermark{#1}`, `\addcontentsline{toc}{chapter}{\protect\numberline{\thechapter}#1}`, then opens `[semio window chapter, title={\thechapter\quad #1}]`.
- `\semio_window_open_section:n{#1}` — closes previous section only (chapter stays open), `\stepcounter{section}`, `\sectionmark{#1}`, `\addcontentsline{toc}{section}{...}`, opens `[semio window section, title={\thesection\quad #1}]` (extra left indent when nested inside an open chapter window — this is the "nested window inside" look).
- Starred variants (`\semio_window_open_chapter_star:n`, `..._section_star:n`) skip numbering/counter/mark/TOC — matching plain LaTeX `\chapter*`/`\section*` semantics exactly, so existing manual `\addcontentsline{toc}{chapter}{...}` calls in templates keep working unchanged.
- Public overrides: `\RenewDocumentCommand{\chapter}{s m}{...}`, `\RenewDocumentCommand{\section}{s m}{...}` dispatch to starred/unstarred variants above. Existing template call sites (`\chapter{...}`, `\chapter*{...}`, `\section{...}`) need no syntax changes.
- Hook `\appendix`: `\let\semio@koma@appendix\appendix` then redefine to call `\semio_window_close_section:\semio_window_close_chapter:` before `\semio@koma@appendix`.
- Hook `\AddToHook{enddocument/before}{\semio_window_close_section:\semio_window_close_chapter:}` so a dangling open window never breaks compilation.

**Design note on nesting (technical constraint):** tcolorbox's `breakable` boxes have limited support for true nesting across page breaks. Because a chapter window is only closed when the _next_ chapter starts (or at `\appendix`/end-of-document), any sections opened in between are lexically nested inside the still-open chapter box. This plan keeps that real nesting (both levels use `breakable`), which is supported by the tcolorbox version Tectonic bundles for the simple case here (no watermarks/second skins). If build verification (step 6) shows breakable-nesting failures, the fallback is to make the chapter box a short **non-breakable title band** (just the heading, immediately closed) with section windows following as independent top-level siblings — same visual read, zero nesting risk. This fallback will be applied automatically during implementation if needed, without changing the plan's visual outcome.

## 4. Template content fixes (no structural rewrites)

- [print/template/zukunftbau/kompaktbericht.tex](print/template/zukunftbau/kompaktbericht.tex): remove `\section*{Titelblatt}` immediately before `\maketitle` (the title page is already a full dedicated page; a heading window right before it reads oddly). Add `\section*{Literaturverzeichnis}` immediately before `\printbibliography` so the bibliography gets its own titled window instead of trailing inside "Methodik und Projektverlauf".
- All other templates ([print/template/report/report.tex](print/template/report/report.tex), [print/template/paper/paper.tex](print/template/paper/paper.tex), [print/template/zukunftbau/forschungsbericht.tex](print/template/zukunftbau/forschungsbericht.tex), [print/template/zukunftbau/zwischenbericht.tex](print/template/zukunftbau/zwischenbericht.tex), [mit-bestand/bericht/zwischenbericht/zwischenbericht.tex](mit-bestand/bericht/zwischenbericht/zwischenbericht.tex)): no edits needed — their existing `\chapter`/`\chapter*`/`\section` calls automatically render as OS windows once `semio-window.sty` is loaded.
- [print/template/flyer/flyer.tex](print/template/flyer/flyer.tex): no edits — its manual `Semiobox` tiles automatically pick up the new shared window style.

## 5. Verification

- `bun ./script.ts generate` to regenerate `semio-tokens.sty` with chrome colors.
- `bun ./script.ts test` in `print/` to rebuild all six templates.
- `bun ./script.ts build` in `mit-bestand/bericht/` to rebuild the Zwischenbericht.
- Visually inspect `forschungsbericht.pdf` (chapter+section nesting) and `zwischenbericht.pdf` (section-only) to confirm title bars, borders, and colors match the OS chrome look; fall back to the non-breakable chapter-band design from step 3 if nested breakable boxes fail to compile.

## Out of scope

- `\subsection` is not currently used in any template; left unstyled for now (future extension of the same mechanism if needed).
- Dark theme is wired (colors emitted, aliasing in place) but no template currently sets `theme=dark`, so it stays unverified visually until a template opts in.
