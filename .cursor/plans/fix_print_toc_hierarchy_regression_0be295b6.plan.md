---
name: Fix Print TOC Hierarchy Regression
overview: Restore the full TOC/register hierarchy rendering in the semio print pipeline, which was accidentally regressed to a single hardcoded row and a 5-row cap during the Jul 9 "TOC/register empty-row fix" commit.
todos:
 - id: reopen-ticket
   content: Reopen TOC-SEMIO-WINDOW-TABLES ticket via repo mcp (read repo://goals first if needed)
   status: completed
 - id: fix-materialize
   content: Restore dynamic loop bound in semio_window_register_body_materialize:n (print/tex/semio-window.sty line ~66)
   status: completed
 - id: fix-render-toc
   content: Restore \csname semio@register@body@toc\endcsname in \semio@render@toc (print/tex/semio-window.sty line ~891)
   status: completed
 - id: rebuild-verify
   content: Rebuild zwischenbericht PDF and verify full TOC hierarchy renders on page 2 via pdfjs text extraction
   status: completed
 - id: close-ticket
   content: Close ticket with summary and list of touched files
   status: completed
isProject: false
---

## Root cause

The semio print TOC is not LaTeX's native `\tableofcontents` mechanism — it's a custom `.sctoc`-backed register table (Window + `SemioTableRegister`), populated by every tracked heading (section/subsection/subsubsection/paragraph) via `\semio_window_toc_append:nnn` in [print/tex/semio-window.sty](print/tex/semio-window.sty). For `zwischenbericht`, `dist/zwischenbericht.sctoc` correctly contains 110 entries (all sections/subsections/paragraphs), so the data pipeline is intact.

The regression is purely in **rendering**, introduced in commit `96cf4167` ("Print TOC/register empty-row fix", Jul 9 20:03) while debugging the _original_ empty-cell bug (ticket `TOC-SEMIO-WINDOW-TABLES`, now closed). Two debug artifacts were left in place instead of being reverted:

1. **`\semio@render@toc` hardcodes one row** instead of using the materialized body:

```887:895:print/tex/semio-window.sty
\newcommand{\semio@render@toc}{%
  \semio@register@window@open{\SemioTocTitle}%
  \SemioTableRegister{%
    \semio@register@table@header
    \SemioTableRegisterRow{1}{Ergebnisse}{semio-toc-section-1}%
  }%
  \semio@register@window@close
  \clearpage
}
```

Compare with the sibling `\semio@render@register` (used for `\listof*`), which correctly uses the dynamic body:

```867:876:print/tex/semio-window.sty
\newcommand{\semio@render@register}[1]{%
  \ifnum\value{#1}>0
    \semio@register@window@open{\SemioRegisterTitle{\csname semio@window@#1@kind\endcsname}}%
    \SemioTableRegister{%
      \semio@register@table@header
      \csname semio@register@body@#1\endcsname
    }%
    \semio@register@window@close
  \fi
}
```

2. **`\semio_window_register_body_materialize:n` is capped at 5 rows** instead of iterating the full sequence count (this helper is shared by TOC and all `\SemioWindowListOf` registers, so any register with >5 entries is silently truncated too):

```64:78:print/tex/semio-window.sty
\cs_new_protected:Npn \semio_window_register_body_materialize:n #1 {
  \tl_clear:N \l_tmpa_tl
  \int_step_inline:nn { 5 } {
    ...
  }
  \exp_args:No \cs_gset_nopar:cpn { semio@register@body@#1 } { \tl_use:N \l_tmpa_tl }
}
```

Confirmed via `git log -p` that both lines changed from correct dynamic code to these hardcoded/capped forms in that single commit — this was mid-debug bisection that never got reverted before commit/ticket close.

The literal-value baking (`\exp_not:n` + `\tl_use:N`/`\str_use:N`, lines 70-75) from that same commit is correct and must be kept.

## Fix

In [print/tex/semio-window.sty](print/tex/semio-window.sty):

1. Restore the dynamic loop bound in `\semio_window_register_body_materialize:n` (line 66):

```latex
\int_step_inline:nn { \seq_count:c { g_semio_register_ #1 _num_seq } } {
```

2. Restore the dynamic body call in `\semio@render@toc` (line 891):

```latex
\SemioTableRegister{%
  \semio@register@table@header
  \csname semio@register@body@toc\endcsname
}%
```

No other files need to change — `.sctoc` writing, `--reruns 2` in `print/script.ts` (line 478), and the `dist/*.sctoc` fallback are already correct and unaffected.

Note: there is currently unstaged, unrelated WIP in the same file (glossary mechanism ticket `PRINT-GLOSSARY-MECHANISM`, also mid-debug with its own hardcoded rows in `\semio@render@glossary` and `semio-table.sty`). The fix must touch only the two TOC/materialize lines above and must not disturb that unrelated WIP.

## Ticket workflow

The bug reintroduces exactly what ticket `.repo/🎫/26/07/08/TOC-SEMIO-WINDOW-TABLES/` (status `closed`) claimed to fix — its closing summary describes the correct materialize/render logic that is no longer in the file. Per repo rules, reopen this ticket with `ticket_reopen` rather than opening a new one, then close it again with an updated summary once verified.

## Verification

1. Rebuild `zwischenbericht` PDF (`mit-bestand/bericht` build, then `print/script.ts test`, or equivalent per `print/script.ts`) — this is slow (was >8 min in a prior agent run), run with a generous timeout/background.
2. Extract text from PDF page 2 (the TOC page) with `pdfjs-dist`, reusing the check pattern from `.repo/🎫/26/07/08/TOC-SEMIO-WINDOW-TABLES/verify-log.md`: confirm `Ergebnisse`, `Forschungsfragen`, and nested numbering like `1.2.1` all appear, and that the row count roughly matches the 110 entries in `dist/zwischenbericht.sctoc`.
3. Save verification screenshot(s)/notes inside the reopened ticket folder per repo rules (temp files/logs must live under the ticket folder, never deleted).
