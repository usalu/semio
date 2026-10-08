# Fix Table Page Breaks in Mit Bestand Zwischenbericht

Ticket bookkeeping was manual: `ticket_open` of the repo MCP returned a malformed result (`structuredContent: null`) twice and created no folder.

## Symptom

Tabelle AN.2.a (Frankreich, Akteursnetz) ran one to two rows past the page frame into the footer on every page it spans (PDF pages 103–106). Lowest content line measured with `pdftotext -bbox-layout`: 812–826 pt, against about 786 pt on every other long-table page.

## Cause

`GraphSpread` (`🧰️framework/🛍️products/📓️print/🔨️modules/🕸️graph/📐️.tex`) switches to a taller `\newgeometry` for the figure pages and ends with `\clearpage\restoregeometry`.

- `\restoregeometry` assigns `\@colht`, `\@colroom` and `\vsize` locally, inside the environment group.
- On each spread page the LaTeX output routine had set `\@colht` globally to the spread's taller `\textheight`.
- Leaving the group therefore drops the local restore and the stale global spread height comes back.
- The standard output routine heals this at its next shipout, but longtable's `\LT@output` reads `\@colht` and never resets it. The Frankreich table opens on the first page after the spread, so all of its pages were broken against the spread height.

Only the first long table after a spread is affected; the other Akteursnetz tables and all other long tables were already correct.

## Fix

`\semio_graph_spread_end:` now re-seats `\@colht`, `\@colroom` and `\vsize` globally from the restored `\textheight` after `\restoregeometry`.

## Verification

`bun nx run @semio-tech/mit-bestand-bericht:build-zwischenbericht` succeeded (16 min, 193 pages, same page count). Re-measured all 193 pages: pages 103–106 no longer exceed the frame; rendered pages 103, 105 and 106 show the table ending above the footer. Pages 98–102 still reach 817.6 pt: these are the spread pages themselves, whose dropped chrome reservation is the documented `chrome=drop` default and was not changed.

The Forschungsbericht uses the same `GraphSpread` and the same fix applies; it was not rebuilt.
