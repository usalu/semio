---
name: Window Row/Column Border Alignment
overview: Make `SemioWindowRowTwo/Three` stretch every window in a row to the tallest sibling's height (bottom borders align, top stays flush), add a new symmetric `SemioWindowColumnTwo/Three` that stretches naturally-sized windows to the widest sibling's width (right borders align, left stays flush), and add a shared `anchor` key (`nw`, `n`, `ne`, `w`, `center`, `e`, `sw`, `s`, `se`) so content can be positioned within a stretched box instead of always sticking to the top-left.
todos:
 - id: ticket
   content: Open new ticket under goal r2602/updateddocs
   status: completed
 - id: anchor-key
   content: Add shared anchor choice key (nw/n/ne/w/center/e/sw/s/se) + valign/halign macros to semio-window.sty
   status: completed
 - id: stretch-splice
   content: Add stretch height/width + fit registers and splice extra tcolorbox options into both window-opening call sites
   status: completed
 - id: row-rewrite
   content: Rewrite SemioWindowRowTwo/Three to measure natural heights, stretch all columns to the max, and reset after
   status: completed
 - id: column-new
   content: Add new SemioWindowColumnTwo/Three (fit-content width measurement, stretch to max width, vertical stacking)
   status: completed
 - id: cover-anchor
   content: Add anchor=center to the 3 logo Windows in makecoverpages
   status: completed
 - id: verify
   content: Add bisect tests, build+rasterize zwischenbericht cover, confirm alignment, close ticket
   status: completed
isProject: false
---

# Window Row/Column Border Alignment

## Confirmed bug (motivating example)

`\makecoverpages` in [print/tex/semio-components.sty](print/tex/semio-components.sty) (lines 149-161) already renders the exact "zwischenbericht has window rows" case cited by the dev: a `\SemioWindowRowThree` of three logo `Window`s (Zukunft Bau, BBSR, BMWSB). The logos have wildly different aspect ratios:

- `zukunft-bau.png` 669x126 (~5.3:1, short/wide)
- `bbsr.png` 397x438 (~0.9:1)
- `gefördert-durch-bmwsb.png` 573x455 (~1.26:1)

`\semio@logo@slot[\linewidth]{...}` (`print/tex/semio-logo.sty` line 44-50) does `\includegraphics[width=...]` with natural aspect ratio, so each column's content height differs a lot. Each column is currently just a `minipage[t]` (`\semio@window@row@col`, [print/tex/semio-window.sty:1628](print/tex/semio-window.sty)), which only aligns tops — the three windows' bottom borders currently land at three different heights.

## 1. Shared `anchor` key — [print/tex/semio-window.sty](print/tex/semio-window.sty)

Add to the existing `keys_define:nn { semio / window / kind }` block (lines 458-461, alongside `title`/`row`) an `anchor .choice:` key with values `nw, n, ne, w, center, e, sw, s, se`, each setting two classic macros used later by the tcolorbox options:

```latex
anchor / nw .code:n = { \exp_args:Nno ... \semio@window@valign{top} \semio@window@halign{left} },
anchor / n  .code:n = { valign=top,    halign=center },
anchor / ne .code:n = { valign=top,    halign=right },
anchor / w  .code:n = { valign=center, halign=left },
anchor / center .code:n = { valign=center, halign=center },
anchor / e  .code:n = { valign=center, halign=right },
anchor / sw .code:n = { valign=bottom, halign=left },
anchor / s  .code:n = { valign=bottom, halign=center },
anchor / se .code:n = { valign=bottom, halign=right },
```

(`valign`/`halign` are native tcolorbox keys — confirmed they exist and only matter once `height`/`width` is forced beyond natural size, so this is safe to always emit.)

Store into two classic macros `\semio@window@valign`, `\semio@window@halign` (default `top`/`left`, i.e. today's visual behavior). Reset them to the default at the top of both `\semio_window_kind_begin:nnn` (line 512) and `\semio_window_generic_begin:n` (line 791), before `\keys_set:nn`, exactly like `\l_semio_window_kind_title_tl` is already cleared there.

## 2. Stretch registers + option splicing — same file

Declare two classic lengths near the `WindowRow` region: `\semio@window@stretch@height`, `\semio@window@stretch@width`, both `0pt` by default (sentinel = "no override"), plus `\newif\ifsemio@window@fit` (default false) for the Column "size-to-content" mode (see §4).

Build one extra-options token list consumed by both window-opening call sites:

```latex
\newcommand{\semio@window@extra@opts}{%
  valign=\semio@window@valign,halign=\semio@window@halign%
  \ifdim\semio@window@stretch@height>0pt ,height=\semio@window@stretch@height\fi
  \ifdim\semio@window@stretch@width>0pt ,width=\semio@window@stretch@width\fi
}
```

Splice it into the four `\begin{tcolorbox}[...]` call sites that currently hard-code the option list:

- `\semio_window_kind_begin:nnn` (lines 539-541, both the `semiotable` and default branch)
- `\semio_window_generic_begin:n` (lines 800-804, both the `row` and default branch)

e.g. `\begin{tcolorbox}[semio~window~tier~#2,\semio@window@extra@opts]`. Since these functions are `\ExplSyntaxOn`, reading the classic (`@`-named) macros needs the same `\group_begin: \ExplSyntaxOff \makeatletter ... \makeatother \ExplSyntaxOn \group_end:` bridge already used by `\semio_window_header_store_aux:nn` (lines 469-477) — mirror that pattern.

## 3. Rewrite `SemioWindowRowTwo/Three` — height stretch, bottom-border align

In the existing `%region WindowRow` (`print/tex/semio-window.sty:1623-1646`):

- Add `\newsavebox{\semio@window@measure@box}`.
- Add a measuring helper:

```latex
\newcommand{\semio@window@row@measure}[3]{% width, content, \dimen-out
  \setlength{\semio@window@stretch@height}{0pt}%
  \begin{lrbox}{\semio@window@measure@box}\semio@window@row@col{#1}{#2}\end{lrbox}%
  \setlength{#3}{\dimexpr\ht\semio@window@measure@box+\dp\semio@window@measure@box\relax}%
}
```

- `\SemioWindowRowTwo`/`Three`: before building the real `\hbox to \linewidth{...}`, measure each column at its final column width into scratch lengths, take the max via nested `\ifdim ... \fi`, `\setlength{\semio@window@stretch@height}{<max>}`, then render the row exactly as today (unchanged `\hbox to \linewidth{...}` body — the stretch now takes effect _inside_ each `Window`'s tcolorbox because of §2). Reset `\semio@window@stretch@height` to `0pt` immediately after the row so later windows aren't affected.
- This only works correctly when each row cell's content is exactly one `Window` (today's only usage pattern) — nesting a _registered_ kind (`Image`/`Table`/...) would double its counter/TOC entry because the content is typeset twice (once to measure). Document this constraint as a code comment; not a regression since no call site does that.

## 4. New `SemioWindowColumnTwo/Three` — width stretch, right-border align

New `%region WindowColumn` right after `WindowRow`, symmetric to §3 but on the width axis and stacked vertically instead of side-by-side:

- `fit` key: add `fit .bool_set:N = ...` (bridged to `\ifsemio@window@fit`) to `semio / window / kind`. When true, the generic `Window`'s natural width is measured (single-line content, mirroring tcolorbox's own `\tcbox` limitation) instead of defaulting to `\linewidth`, via an unrestricted `\hbox` measuring the raw content width plus `2(\semio@chrome@padding+\semio@stroke@hairline)` chrome allowance, forced as an explicit `width=` for that box.
- `\semio@window@column@measure{content}{\dimen-out}`: renders `content` (with `\semio@window@fittrue` and stretch-width `0pt`) into an `lrbox`, stores `\wd`.
- `\SemioWindowColumnTwo/Three`: measure natural width of each argument, take the max, `\setlength{\semio@window@stretch@width}{<max>}`, then render each argument in sequence (not an `\hbox`; a vertical stack) separated by `\semio_block_sep:` (the same spacing unit already used between windows), each with `\semio@window@fittrue` so it uses `width=` instead of `\linewidth`. Since all columns start flush at the same left edge and are now forced to the same (max) width, right borders coincide automatically — no separate packing/anchoring logic needed. Reset `\semio@window@stretch@width` to `0pt` and `\semio@window@fitfalse` after.
- No current call site needs this yet; it is added as a general-purpose primitive per the confirmed direction (symmetric counterpart to Row), to be exercised by the ticket's own verification test.

## 5. Showcase the anchor on the real bug — [print/tex/semio-components.sty](print/tex/semio-components.sty)

In `\makecoverpages`'s logo `\SemioWindowRowThree` (lines 149-161), add `anchor=center` to the three logo `Window`s so once they're stretched to the tallest logo's height, each (differently-shaped) logo is vertically centered rather than pinned to the top:

```latex
\begin{Window}[title=Zukunft~Bau, row, anchor=center]
```

(same for BBSR / BMWSB). All other `SemioWindowRowTwo/Three` call sites (Titel/Zukunft-Bau header pair, Aktenzeichen/Förderzeitraum/Berichtszeitraum, Antragstellende Institution/Kooperationspartner) need no changes — they automatically get bottom-border alignment while keeping today's default top-left anchor.

## 6. Verify

- Open a new ticket (goal `🎯r2602🎯updateddocs`, same bucket as the prior `PRINT-WINDOW-COMPOSABILITY`/`PRINT-WINDOW-BORDER-GAP` tickets which are both closed and don't cover this).
- Add a temp bisect `.tex` in the ticket folder exercising: (a) `SemioWindowRowThree` with three placeholder boxes of very different natural heights (mimicking the logo aspect ratios) to confirm stretched/flush bottom borders and `anchor=center` behavior; (b) `SemioWindowColumnTwo` with a short-label vs. long-label window to confirm stretched/flush right borders.
- Build `mit-bestand/bericht/zwischenbericht` (light + dark) via the existing `print` nx test task, rasterize the cover page, and visually confirm the 3-logo row now shares one flush bottom border with vertically-centered logos, and the other existing rows/stacks are unaffected.
- Close the ticket with a summary and the full list of touched files.

## Files touched

- `print/tex/semio-window.sty` (anchor key, stretch registers/splicing, `SemioWindowRowTwo/Three` rewrite, new `SemioWindowColumnTwo/Three`)
- `print/tex/semio-components.sty` (`anchor=center` on the 3 cover logo windows)
- new ticket folder under `.repo/🎫/26/07/10/...` (bisect/verify temp files, screenshots, ticket.json)
