---
name: Fix Window Row/Stack/TOC Regressions
overview: "Fix three concrete regressions in `print/tex/semio-window.sty` introduced by the recent weighted-window-spacing work: multi-column window rows collapse to near-zero width and overlap, the Stack's per-item height allocation silently no-ops, and the table of contents renders with zero entries."
todos:
 - id: fix-row-width
   content: Fix group-scoping bug in semio_window_row_inner_width:n and semio_window_row_assign_width_set:nn so column widths persist
   status: completed
 - id: fix-stack-height
   content: "Fix group-scoping bug in semio_window_stack_use_end: and semio_window_stack_use_set_height:n so per-item stack heights persist"
   status: completed
 - id: fix-toc
   content: Fix inverted tl_if_empty guard in semio_window_toc_append:nnn and semio_window_toc_aux_line:nnn so TOC entries are written/rendered
   status: completed
 - id: verify
   content: Rebuild zwischenbericht light+dark, rasterize cover + TOC, confirm no overlap and populated TOC
   status: in_progress
 - id: ticket
   content: Reopen PRINT-WINDOW-WEIGHTED-SPACING ticket, close with summary once verified
   status: pending
isProject: false
---

## Root cause

All three bugs share the same mistake: a classic-TeX `\setlength`/`\edef` or an expl3 `\dim_set:Nn` assignment that is meant to persist for the rest of the current call is wrapped inside its own `\group_begin: ... \group_end:` (used only to scope the `\ExplSyntaxOff`/`\ExplSyntaxOn`/`\makeatletter` toggles). `\group_end:` reverts every local assignment made since the matching `\group_begin:`, so the value silently reverts to its previous (usually `0pt`) state the instant the helper returns. This is verifiable by comparing against the correct pattern already used elsewhere in the same file (e.g. `\semio_window_stack_prepare:`, `\semio_block_sep:`), which never wrap the persisting assignment inside the group.

Confirmed by:

- Rasterizing the built cover page of `mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf` — matches the user's screenshot exactly (Zukunft Bau/Titel row and the meta row collapse into overlapping text; Untertitel and Arbeitsprobe, which are single, non-row-grouped windows, render fine).
- `git show 26487040f8 -- print/tex/semio-window.sty`, which introduced these helpers.
- Extracting text from the built PDF: no table-of-contents register table appears anywhere between the cover and the "Ergebnisse" chapter, even though the "Netzwerk" table (a plain, non-`row` `Window`) renders correctly.

## Bug 1 — Row columns collapse to ~0pt width (visible overlap)

`print/tex/semio-window.sty`:

```2174:2188:print/tex/semio-window.sty
\cs_new_protected:Npn \semio_window_row_inner_width:n #1 {
  \group_begin:
  \ExplSyntaxOff
  \makeatletter
  \ifnum#1=2
    \setlength{\semio@window@row@inner@w}{\dimexpr\linewidth-\semio@window@gap\relax}%
  \else
    \setlength{\semio@window@row@inner@w}{\dimexpr\linewidth-2\semio@window@gap\relax}%
  \fi
  \edef\semio@window@row@inner@w@value{\the\semio@window@row@inner@w}
  \makeatother
  \ExplSyntaxOn
  \dim_set:Nn \l_semio_window_row_inner_dim { \semio@window@row@inner@w@value }
  \group_end:
}
```

`\dim_set:Nn \l_semio_window_row_inner_dim {...}` happens _inside_ the group, so it is discarded on `\group_end:`. `\l_semio_window_row_col_a/b/c_dim` are then computed from a permanently-0pt inner width, and `\semio_window_row_assign_width_set:nn` (below) has the identical bug for the classic length registers that hold the final column widths — so both columns of every `SemioWindowRowTwo`/`SemioWindowRowThree` end up ~0pt wide, causing the two/three columns to render on top of each other.

```2164:2172:print/tex/semio-window.sty
\cs_new_protected:Npn \semio_window_row_assign_width_set:nn #1 #2 {
  \group_begin:
  \ExplSyntaxOff
  \makeatletter
  \setlength{#1}{#2}
  \makeatother
  \makeatother
  \group_end:
}
```

Fix: move the persisting assignment out of the group (or drop the group entirely, matching the pattern used by `\semio_block_sep:` and `\semio_window_stack_prepare:`), for both functions.

## Bug 2 — Stack per-item height never actually applied

Same mistake in the two helpers that feed the weighted per-item height into the classic bridge macro consumed by `Window`/`SemioWindowRowTwo/Three`:

```2375:2391:print/tex/semio-window.sty
\cs_new_protected:Npn \semio_window_stack_use_end: {
  \group_begin:
  \ExplSyntaxOff
  \makeatletter
  \semio@window@row@body@h@reset
  \makeatother
  \group_end:
}

\cs_new_protected:Npn \semio_window_stack_use_set_height:n #1 {
  \group_begin:
  \ExplSyntaxOff
  \makeatletter
  \semio@window@row@body@h@set{#1}
  \makeatother
  \group_end:
}
```

`\semio@window@row@body@h@set`/`@h@reset` assign `\semio@window@stretch@height` and `\SemioWindowStretchHeightValue` — both discarded on `\group_end:`. Net effect: every window driven by `SemioWindowStack` always sees `\semio@window@stretch@height = 0pt`, so it silently falls back to auto-sized (no explicit height) boxes instead of the intended proportional slice of `\textheight`. This is why "Untertitel"/"Arbeitsprobe" (plain auto-sized `Window`s) look fine while it defeats the entire purpose of the `SemioWindowStack` feature. Fix: drop the enclosing group in both helpers.

## Bug 3 — Table of contents always empty

Introduced in the same commit, `\semio_window_toc_title_plain:n` clears `\l_tmpa_tl` to empty when a title is junk (contains `Hy@SectionAnchorHref`) and otherwise leaves the real, plain title text in it. But both call sites guard on the wrong branch:

```154:185:print/tex/semio-window.sty
\cs_new_protected:Npn \semio_window_toc_append:nnn #1#2#3 {
  ...
        \semio_window_toc_title_plain:n {#3}
        \tl_if_empty:NT \l_tmpa_tl {
          ... write aux entry / seq_gput_right ...
        }
  ...
}
```

```212:223:print/tex/semio-window.sty
\cs_new_protected:Npn \semio_window_toc_aux_line:nnn #1#2#3 {
  \tl_if_in:nnTF {#2} { Hy@SectionAnchorHref } { } {
    \semio_window_toc_title_plain:n {#2}
    \tl_if_empty:NT \l_tmpa_tl {
      ... seq_gput_right into the seqs read back for rendering ...
    }
  }
}
```

`\tl_if_empty:NT` runs the body only when the title _is_ empty — i.e. only junk entries would ever be appended, and every legitimate heading is silently dropped. Fix: change both to `\tl_if_empty:NF` (append/write only when the plain title is non-empty).

## Verification

- Reopen the `PRINT-WINDOW-WEIGHTED-SPACING` ticket (this is a direct regression from that work) via `ticket_reopen`, per repo workflow rules.
- Apply the four fixes above in `print/tex/semio-window.sty`.
- Rebuild `mit-bestand/bericht/zwischenbericht` (light + dark).
- Rasterize the cover page and visually confirm: Zukunft Bau/Titel row, Aktenzeichen/Förderzeitraum/Berichtszeitraum row, institution row, and the three-logo row all render side-by-side without overlap, and the page still fits within one sheet without overflow.
- Extract text from the rebuilt PDF (or rasterize the TOC page) and confirm the table of contents lists all headings with page numbers, and that the "Netzwerk" table (already correct) is unaffected.
- Close the ticket with a summary of the fix and files touched.
