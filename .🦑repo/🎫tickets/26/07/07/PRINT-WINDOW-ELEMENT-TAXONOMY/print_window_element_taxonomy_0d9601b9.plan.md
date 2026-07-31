---
name: Print Window Element Taxonomy
overview: 'Restrict the semio print "window" chrome (currently auto-applied to every chapter/section of body text) to a fixed set of 14 special document elements, grouped into three tiers — Visual, Logical, Structural — each rendered with its own border color and name/number chip fill: primary, secondary, tertiary respectively.'
todos:
 - id: remove-chapter-section-window
   content: Remove chapter/section auto-windowing (booleans, open/close macros, chapter/section overrides, chapter/section/inner tcolorbox styles) from semio-window.sty
   status: completed
 - id: tier-registry
   content: Add tier color mapping (visual/logical/structural -> primary/secondary/tertiary) and tcolorbox tier styles
   status: completed
 - id: parametrize-header-chip
   content: Parametrize semio@window@cap/tab/ctrl/header/stroke macros to take fill+border color instead of hardcoded chrome-canvas/border-normal
   status: completed
 - id: kind-registry
   content: Add semio_window_kind_new factory and define the 14 kind environments (Image, Photo, Figure, Table, Listing, Pseudocode, Theorem, Lemma, Proof, Equation, Glossary, Abbreviations, Blockquote, Epigraph) with counters and optional title override
   status: completed
 - id: retire-semiobox
   content: Remove the public Semiobox environment
   status: completed
 - id: migrate-components
   content: Migrate semio-components.sty (makefundingacknowledgement -> Blockquote, makeworkpackages -> Table)
   status: completed
 - id: migrate-templates
   content: Migrate report.content.tex and flyer.content.tex Semiobox usages to Blockquote
   status: completed
 - id: migrate-mit-bestand
   content: Migrate zwischenbericht.tex's 3 highlight boxes to Blockquote
   status: completed
 - id: build-verify
   content: Run bun print/script.ts test and build mit-bestand zwischenbericht to verify all templates compile with the new taxonomy
   status: completed
isProject: false
---

# Print Window Element Taxonomy

## Goal

`print/tex/semio-window.sty` currently wraps **every** `\chapter` and `\section` body in a `tcolorbox` window (name = heading text, number = chapter/section counter), and offers a generic `\Semiobox` for ad-hoc callouts. This is being replaced: windows will no longer wrap ordinary text. Windows are reserved for 14 named element kinds, grouped into 3 tiers, each tier with its own border color and name/number chip fill color:

- Visual (`Image`, `Photo`, `Figure`) → **primary** border + primary fill on the name/number chip
- Logical (`Table`, `Listing`, `Pseudocode`, `Theorem`, `Lemma`, `Proof`, `Equation`) → **secondary** border + secondary fill
- Structural (`Glossary`, `Abbreviations`, `Blockquote`, `Epigraph`) → **tertiary** border + tertiary fill

Per your decision, existing generic `Semiobox` callouts that don't correspond to a real Figure/Table/etc. (funding-acknowledgement quote, report "Key findings" callout, flyer marketing cards, the 3 itemized highlight boxes in the real `zwischenbericht.tex`) are reclassified as `Blockquote` (Structural/tertiary). The one genuine table (`makeworkpackages`'s AP table) is reclassified as `Table` (Logical/secondary).

`semio-primary` (`#ff344f`), `semio-secondary` (`#34d1bf`), `semio-tertiary` (`#fa9500`) are already emitted into `print/tex/semio-tokens.sty` for every entry in `ui/styling/tokens.json`'s `colors`, so **no token changes are needed** — only `print/tex/semio-window.sty` and its call sites change.

## Chapter/section: drop window chrome entirely

In `print/tex/semio-window.sty`, remove the "text is windowed" machinery:

```61:126:print/tex/semio-window.sty
  semio~window~chapter/.style={...},
  semio~window~section/.style={...},
  ...
\cs_new_protected:Npn \semio_window_open_chapter:n #1 { ... }
```

Delete: `\l_semio_window_chapter_open_bool`, `\l_semio_window_section_open_bool`, `\semio_window_close_chapter_body:`, `\semio_window_close_section:`, `\semio_window_close_chapter:`, `\semio_window_open_chapter:n`, `\semio_window_open_section:n`, the `\RenewDocumentCommand{\chapter}`/`\section` overrides (and the `tableofcontents`/`listoffigures`/`listoftables` chapter-swap hooks that existed only to bypass the override), `\SemioWindowCloseAll`, `\SemioWindowCloseChapter`, the `semio~window~chapter`/`semio~window~section`/`semio~window~inner` tcolorbox styles, and the `enddocument` hooks that closed them. `\chapter`/`\section` fall back to plain KOMA-script behavior — no box, no header chip.

Consequently `\semio_window_semiobox_begin:n` (the nested-inside-chapter branching logic) also disappears — there's nothing left to nest inside.

## Tier + kind registry (new, in `semio-window.sty`)

```latex
%region Window Tiers
\cs_new_protected:Npn \semio_window_tier_color:n #1 {
  \str_case:nn { #1 } {
    { visual } { semio-primary }
    { logical } { semio-secondary }
    { structural } { semio-tertiary }
  }
}
\cs_new_protected:Npn \semio_window_define_tier:n #1 {
  \tcbset{ semio~window~tier~#1 /.style={ semio~window, colframe=\semio_window_tier_color:n{#1} } }
}
\semio_window_define_tier:n { visual }
\semio_window_define_tier:n { logical }
\semio_window_define_tier:n { structural }
%endregion

%region Window Kind Registry
% one counter + one environment per kind; header chip fill+border+strokes all use the
% tier color, chip text uses semio-chrome-canvas (inverted-chip contrast against the
% saturated tier color, mirrors the existing canvas/border-emphasized pairing)
\cs_new_protected:Npn \semio_window_kind_new:nnn #1 #2 #3 { % #1 label/env name, #2 tier, #3 counter id
  \newcounter{#3}
  % NewDocumentEnvironment with a dynamic name via \cs_generate_from_arg_count:/\use:c,
  % taking an optional [title=...] key that overrides the displayed name (number always shown)
}
\semio_window_kind_new:nnn { Image }         { visual }     { semioimage }
\semio_window_kind_new:nnn { Photo }         { visual }     { semiophoto }
\semio_window_kind_new:nnn { Figure }        { visual }     { semiofigure }
\semio_window_kind_new:nnn { Table }         { logical }    { semiotable }
\semio_window_kind_new:nnn { Listing }       { logical }    { semiolisting }
\semio_window_kind_new:nnn { Pseudocode }    { logical }    { semiopseudocode }
\semio_window_kind_new:nnn { Theorem }       { logical }    { semiotheorem }
\semio_window_kind_new:nnn { Lemma }         { logical }    { semiolemma }
\semio_window_kind_new:nnn { Proof }         { logical }    { semioproof }
\semio_window_kind_new:nnn { Equation }      { logical }    { semioequation }
\semio_window_kind_new:nnn { Glossary }      { structural } { semioglossary }
\semio_window_kind_new:nnn { Abbreviations } { structural } { semioabbreviations }
\semio_window_kind_new:nnn { Blockquote }    { structural } { semioblockquote }
\semio_window_kind_new:nnn { Epigraph }      { structural } { semioepigraph }
%endregion
```

Each generated environment: increments its counter, renders the header via `\semio_window_header:nnn {name}{number}{tier-color}` (name defaults to the kind label, e.g. "Figure"; overridable via `[title=...]` so migrated callouts like "Design"/"Build" keep a custom name), opens `\begin{tcolorbox}[semio~window~tier~<tier>]`, closes with `\end{tcolorbox}`. Counters are plain document-wide `\arabic{}` (not chapter-scoped) since the package must work identically under `scrreprt` (report/forschungsbericht, has chapters) and `scrartcl` (paper/flyer/zwischenbericht/kompaktbericht, no chapters).

`\Semiobox` itself is retired as a public environment — all its call sites migrate to one of the 14 named kinds.

## Header chip rendering: parametrize color

`\semio@window@cap`, `\semio@window@tab`, `\semio@window@ctrl`, `\semio@window@header`, `\semio@window@stroke@v`, `\semio@window@stroke@h` currently hardcode `semio-chrome-border-normal` (stroke) / `semio-chrome-canvas` (fill) / `semio-chrome-border-emphasized` (text). Add a color parameter threaded through all of them so the chip's fill+border become the tier color and text becomes `semio-chrome-canvas` (readable inverted chip on a saturated background, consistent in both themes since `chrome-canvas` is already the light/dark-appropriate near-white/near-black token).

## File-by-file migration of existing usages

- `print/tex/semio-components.sty`
  - `makefundingacknowledgement`: both `Semiobox` → `Blockquote`.
  - `makeworkpackages`: the `[title=Netzwerk]` box wraps a real `tabular` → `Table` (with `[title=Netzwerk]` preserved).
- `print/template/report/report.content.tex`: "Key findings…" `Semiobox` → `Blockquote`.
- `print/template/flyer/flyer.content.tex`: the centered emblem box and the `[title=Design]`/`[title=Build]` cards → `Blockquote` (titles preserved via `[title=...]`).
- `mit-bestand/bericht/zwischenbericht/zwischenbericht.tex`: the 3 itemized highlight boxes (Recherche/Entwurfswerkzeug/Bauteilportal) → `Blockquote`.
- `print/template/paper/paper.content.tex`, `print/template/zukunftbau/forschungsbericht.content.tex`, `.../zwischenbericht.content.tex`, `.../kompaktbericht.content.tex`: no `Semiobox` usage today — only need to confirm `\chapter`/`\section` still read correctly once window chrome is removed (plain headings).
- Generated `*-dark.tex` files are build artifacts (`deriveDarkTexSource` in `print/script.ts`) — not hand-edited.

## Verification

- `bun print/script.ts test` (from `print/`) rebuilds `semio-tokens.sty` and compiles all 6 templates (light+dark) via Tectonic; must succeed and produce all PDFs.
- Spot-check rendered PDFs (report, flyer) to confirm: chapters/sections are plain (no chrome), and `Blockquote`/`Table` boxes show the correct tier border + colored name/number chip in both light and dark theme.
- Build `mit-bestand/bericht/zwischenbericht/zwischenbericht.tex` (via its `mit-bestand/bericht/script.ts`) to confirm the real report still compiles after the `Blockquote` migration.

## Process note

Per repo workflow, implementation must happen inside a ticket (none currently covers this — the closed `OS-WINDOW-STYLE-FOR-PRINT` ticket introduced the chapter/section auto-windowing this plan now reverts). A new ticket should be opened under the appropriate goal before editing.
