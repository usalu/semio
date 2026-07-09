---
name: Print Spacing Single Unit
overview: Refactor the `print` LaTeX chrome/typography system to use exactly one spacing unit everywhere (mirroring `ui`'s `--ui-spacing`/`--spacing-single`/`--spacing-double` pattern), fixing a dead-padding bug in window tab captions and replacing the unrelated "touch" value and a hardcoded `3pt` literal that currently make gaps inconsistent between horizontal/vertical and between navbar/box spacing.
todos:
 - id: ticket
   content: Open a new ticket for the print spacing refactor via repo MCP
   status: completed
 - id: script-ts
   content: Update print/script.ts to emit a single base unit + single/double multiples, drop tokens.spacing.touch usage
   status: completed
 - id: core-sty
   content: Update print/tex/semio-core.sty public spacing aliases (SemioSpacingUnit/Double)
   status: completed
 - id: window-sty
   content: Fix dead fboxsep bug in semio@window@cap and replace touch/3pt with unit multiples in semio-window.sty (headsep, footskip)
   status: completed
 - id: components-flyer
   content: Replace touch vspace usages in semio-components.sty and flyer.content.tex with double-unit spacing
   status: completed
 - id: regen-verify
   content: Regenerate tokens, rebuild all template PDFs, visually verify consistent spacing
   status: completed
 - id: close-ticket
   content: Close the ticket with a summary of files touched
   status: completed
isProject: false
---

# Print Spacing: Single Unit Refactor

## Root cause analysis

`ui` defines exactly one base spacing variable, `--ui-spacing` (defaults to `--spacing-compact: 0.2rem`), and every other gap/padding/size is a clean multiple of it: `--spacing-single: calc(1 * var(--ui-spacing))`, `--spacing-double: calc(2 * ...)`, `--size-medium: calc(7 * ...)`, etc. (`ui/styling/js/ui.css:672-706`). `--spacing-touch` only replaces the _value_ of `--ui-spacing` on touch devices (`.touch { --ui-spacing: var(--spacing-touch); }`, `ui/styling/js/ui.css:582-583`) — it is never a second, independently-used spacing token.

`print` currently has two disconnected literal tokens generated 1:1 from `tokens.spacing` (`print/tex/semio-tokens.sty:50-51`):

```50:51:print/tex/semio-tokens.sty
\newcommand{\semio@spacing@compact}{0.2em}
\newcommand{\semio@spacing@touch}{0.275em}
```

`compact` is used as the real base unit for chrome padding/heights (via `print/script.ts:198-208`, all derived correctly as `compactFactor * NUiSpacing`). But `touch` (a value that is **not** a clean multiple of `compact`, 1.375×) is separately spliced into structural spacing in five places, breaking the "one unit" rule:

- `print/tex/semio-window.sty:484` `\setlength{\headsep}{\semio@spacing@touch}` — the gap between the navbar/window header and the content box below it.
- `print/tex/semio-window.sty:485` `\setlength{\footskip}{...+\semio@stroke@hairline+3pt+\semio@spacing@touch}` — also has a hardcoded `3pt` literal with no token at all.
- `print/tex/semio-components.sty:16,34` `\vspace{\semio@spacing@touch}` (title page emblem→title gap, cover page title→metadata gap).
- `print/template/flyer/flyer.content.tex:12` `\vspace{\SemioSpacingTouch}`.

Separately, `print/tex/semio-window.sty:254-288` (`\semio@window@cap`, used for every chapter/section/Semiobox title tab and the numbered corner control) sets `\fboxsep` to the padding token but never actually applies it — the caption is placed in an `\hbox to \semio@window@cap@w` where `\semio@window@cap@w` is set to `\wd\semio@window@cap@slot` (the box's own natural width), so the surrounding `\hfil ... \hfil` is a no-op and the caption text touches its border with **zero** padding, while every other box (navbar/footer chips via `\fcolorbox`, window bodies via tcolorbox `left=/right=/top=/bottom=`) correctly gets 1 unit (`\semio@chrome@padding`) on all sides. This is exactly the "text-to-border spacing is inconsistent" symptom.

## Refactor

### 1. `print/script.ts` — generate one unit + clean multiples

In `emitSemioTokensSty`, replace the two independent `spacing.*` emissions with a single base-unit emission plus named multiples, generated the same way `ui.css` derives `--spacing-single`/`--spacing-double`:

```226:1:print/tex/semio-tokens.sty
\newcommand{\semio@spacing@unit}{0.2em}     % 1 unit — from tokens.spacing.compact
\newcommand{\semio@spacing@single}{0.2em}   % 1 × unit
\newcommand{\semio@spacing@double}{0.4em}   % 2 × unit
```

Drop `tokens.spacing.touch` from print codegen entirely — it is a touch-device override of `--ui-spacing` in `ui`, not a second spacing concept, and print has no touch context. Keep `compactFactor` (renamed `unitFactor`) as the single source used for `\semio@chrome@padding`, titlebar/navbar/footer heights (unchanged derivation, still `unitFactor * NUiSpacing`).

### 2. `print/tex/semio-core.sty` — public API

Replace:

```94:95:print/tex/semio-core.sty
\newcommand{\SemioSpacingCompact}{\semio@spacing@compact}
\newcommand{\SemioSpacingTouch}{\semio@spacing@touch}
```

with `\SemioSpacingUnit` / `\SemioSpacingDouble` (aliases of `\semio@spacing@single` / `\semio@spacing@double`).

### 3. `print/tex/semio-window.sty` — fix dead padding + replace `touch`

- Fix `\semio@window@cap`: compute `\semio@window@cap@w` as the slot's natural width **plus** `2\semio@chrome@padding`, and center the caption inside that wider box so the now-meaningful `\hfil`s actually add 1 unit of padding on each side (horizontal). Remove the dead `\setlength{\fboxsep}{\semio@chrome@padding}` (unused) and its stray reset.
- `\headsep`: `\semio@spacing@double` instead of `\semio@spacing@touch` — the same unit family used for box padding, just the "gap" multiple instead of the "padding" multiple, so navbar-to-box spacing is a clean multiple of the same base unit as text-to-border spacing.
- `\footskip`: drop the `3pt` literal and `\semio@spacing@touch`; express purely as `\semio@chrome@footer@height + \semio@stroke@hairline + \semio@spacing@double`.

### 4. `print/tex/semio-components.sty` and `print/template/flyer/flyer.content.tex`

Replace the two `\vspace{\semio@spacing@touch}` calls and the one `\vspace{\SemioSpacingTouch}` with `\vspace{\semio@spacing@double}` / `\vspace{\SemioSpacingDouble}` respectively — same visual role (breathing room between stacked title-page elements), now expressed as a clean multiple of the one unit.

### 5. Regenerate + verify

- Run `bun ./script.ts generate` to rewrite `semio-tokens.sty`.
- Run `bun ./script.ts test` to rebuild all 6 templates × light/dark (12 PDFs) with Tectonic and confirm no build errors.
- Visually spot-check `zwischenbericht.pdf`/`zwischenbericht-dark.pdf` (the template shown in the reference screenshot) to confirm: caption text now has visible, equal padding on all tab/control boxes; the header-to-box gap and the flyer/title-page gaps read as a consistent, single-unit-derived rhythm instead of the previous mismatched `compact`/`touch`/`3pt` mix.

## Files touched

- `print/script.ts`
- `print/tex/semio-tokens.sty` (generated, regenerate not hand-edit)
- `print/tex/semio-core.sty`
- `print/tex/semio-window.sty`
- `print/tex/semio-components.sty`
- `print/template/flyer/flyer.content.tex`

## Ticket

Per repo workflow rules, open a new ticket (`.repo/🎫/YY/MM/DD/PRINT-SINGLE-SPACING-UNIT` or similar) before implementing, since the closed `OS-WINDOW-STYLE-FOR-PRINT` ticket covered chrome color tokens, not spacing — a different concern.
