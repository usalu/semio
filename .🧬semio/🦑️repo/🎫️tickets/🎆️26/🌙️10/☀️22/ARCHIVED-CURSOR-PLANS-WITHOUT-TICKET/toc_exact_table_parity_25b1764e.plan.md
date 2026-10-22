---
name: TOC Exact Table Parity
overview: "Fix two remaining root causes so the TOC (and Glossary/Reference registers) render pixel-identical to short tables like Netzwerk: a column-width bug that overflows the table past `\\linewidth` (breaking the right-hand border), and per-row hairlines being unconditionally suppressed in long (`longtable`) mode."
todos:
 - id: reopen-ticket
   content: Reopen TOC-SEMIO-WINDOW-TABLES ticket via repo MCP
   status: completed
 - id: fix-colspec-width
   content: Rewrite the three @long colspecs so column widths subtract tabcolsep/padding/rule so total width equals \linewidth exactly
   status: completed
 - id: reenable-row-sep
   content: Remove \ifsemio@table@long@mode guard in \SemioTableRow so per-row hairlines render in long mode too
   status: completed
 - id: diagnose-pagination-regression
   content: Rebuild and check for pagination regression; if it reproduces, root-cause via literal \hline instead of global-boolean-gated \noalign rather than re-disabling separators
   status: completed
 - id: verify-closing-rule
   content: Confirm the bottom closing rule from \semio@table@long@closing@rule actually renders on the last TOC/Glossary/Reference page
   status: completed
 - id: rebuild-verify-parity
   content: Rebuild light+dark, confirm no Overfull warnings, render TOC/Netzwerk/Glossary pages to PNG and visually confirm pixel parity
   status: completed
 - id: close-ticket
   content: Update verify-log.md and close the ticket with summary and touched files
   status: completed
isProject: false
---

# TOC Exact Table Parity

## Evidence

Comparing the last rendered pages confirms two concrete gaps versus the `Netzwerk` reference table (`makeworkpackages` in [print/tex/semio-components.sty](print/tex/semio-components.sty), rendered via `Window` + `SemioTableThree`):

- Netzwerk: full rectangular border (left/right/top/bottom), header row separated by a rule, and a **hairline between every data row**.
- TOC (`SemioTableRegister`, `longtable`-based): left border and header rule are present, but the **right-hand border appears missing/misplaced** and there are **no hairlines between entries** — every TOC row runs directly into the next.

## Root cause 1: table wider than `\linewidth`

[print/tex/semio-table.sty](print/tex/semio-table.sty:87-91) defines the long colspecs as:

```
\newcommand{\semio@table@colspec@register@long}{@{\hspace{\semio@chrome@padding}}|>{...}p{0.16\linewidth}>{...}p{0.62\linewidth}>{...}p{0.22\linewidth}|@{\hspace{\semio@chrome@padding}}}
```

The outer `@{\hspace{padding}}|` / `|@{\hspace{padding}}` correctly override the default `\tabcolsep` gap at the table's outer edges, but there is **no `@{}` between the three `p{...}` columns**, so `array`/`longtable` inserts the default `\tabcolsep` gap on _each side_ of the two internal column boundaries — 4× `\tabcolsep` of unaccounted width. Since column fractions already sum to exactly `1.0\linewidth`, this pushes the real table width `4\tabcolsep` past `\linewidth`, which is exactly why every build logs `Overfull \hbox (...too wide) in alignment` for these tables, and why the right border pipe lands outside the visible margin instead of aligning with Netzwerk's box edge.

Contrast with the short colspec used by Netzwerk ([print/tex/semio-table.sty](print/tex/semio-table.sty:85)), which explicitly subtracts `\tabcolsep` per column boundary:

```
\newcommand{\semio@table@colspec@three}{@{}T{\dimexpr0.34\linewidth-\tabcolsep\relax}T{\dimexpr0.33\linewidth-\tabcolsep\relax}T{\dimexpr0.33\linewidth-2\tabcolsep\relax}@{}}
```

**Fix:** rewrite `\semio@table@colspec@register@long`, `@reference@long`, `@glossary@long` so each column's `p{...}` width subtracts its share of `\tabcolsep` (mirroring the short colspec's subtraction pattern) and the outer padding/border rule width, so the total resolves to exactly `\linewidth`. This removes the Overfull warnings and puts the right border exactly where the Netzwerk box border sits.

## Root cause 2: per-row hairlines disabled in long mode

[print/tex/semio-table.sty](print/tex/semio-table.sty:194-202):

```
\newcommand{\SemioTableRow}[1]{%
  \ifsemio@table@long@mode\else
    \ifsemio@table@row@started
      \semio@table@row@sep
    \fi
  \fi
  \global\semio@table@row@startedtrue
  #1\\
}
```

`\ifsemio@table@long@mode` (set true only inside `\semio@table@long@render`, i.e. for TOC/Reference/Glossary) unconditionally skips `\semio@table@row@sep` (`\noalign{\hrule}`). This guard was added earlier this session specifically to dodge a historic "one row per page" `longtable` pagination bug — but that fix was never actually re-validated against the _current_ code (the earlier bug's real root cause was traced to an `expl3` header macro in `\endhead`, which is no longer used there). This is the main visible inconsistency: Netzwerk has a rule between every row, TOC has none.

**Fix:**

1. Remove the `\ifsemio@table@long@mode` guard so `\SemioTableRow` always draws `\semio@table@row@sep` between rows, in both modes.
2. Rebuild and check the TOC page/entry count against the known-good baseline (TOC spanning 2 pages, page 2 with ~130+ hierarchy-number matches, page 3 with ~60, from [verify-log.md](.repo/🎫️/26/07/08/TOC-SEMIO-WINDOW-TABLES/verify-log.md)).
3. If the pagination bug reproduces (many near-empty pages), root-cause it directly instead of re-disabling: `\semio@table@row@sep` relies on `\global\semio@table@row@startedtrue`/`false`, and `longtable` internally does a hidden pre-pass to measure row heights — a `\global` boolean flipped during that pre-pass can leak state into the real typesetting pass. Try switching to an unconditional literal `\hline` appended after every row's `\\` (the same plain mechanism already proven safe in `\endfirsthead`/`\endhead`'s header rows), rather than a `\global`-boolean-gated `\noalign`.
4. Keep `\semio@table@long@closing@rule` (in `\endlastfoot`) as the bottom-closing rule; verify it actually renders in the fresh build (previously placed correctly between `\endfoot`/`\endlastfoot` but never visually confirmed post-fix).

## Verification

1. Reopen ticket `TOC-SEMIO-WINDOW-TABLES` via repo MCP.
2. Rebuild `zwischenbericht` (light + dark): `cd mit-bestand/bericht && bun ./📜️script.ts build`.
3. Confirm zero `Overfull`/`Underfull \hbox ... in alignment` warnings tied to the register/reference/glossary long tables.
4. Confirm TOC page count/entry distribution matches the known-good baseline (no pagination regression to near-empty pages).
5. Render TOC pages and the `Netzwerk` page to PNG and visually confirm: right border aligned at the same margin as Netzwerk's box edge, hairline between every TOC row, bottom closing rule present, header shading/padding identical.
6. Render the Glossary page (`\listofglossaries`) to confirm the same fixes apply there.
7. Update `verify-log.md` with findings and close the ticket, listing touched files.
