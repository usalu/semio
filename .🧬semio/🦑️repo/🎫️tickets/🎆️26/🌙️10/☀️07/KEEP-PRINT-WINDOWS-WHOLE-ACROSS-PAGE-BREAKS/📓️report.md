# Keep Print Windows Whole Across Page Breaks

Ticket bookkeeping was manual: `ticket_reopen` of the repo MCP returned a malformed result (`structuredContent: null`).

## Symptom

In the Zwischenbericht a window's title tab stayed at the bottom of one page while its body started on the next page without any header (Abbildung Stützengenerator, Abbildung 1.2.3.2.a, Tabelle 2.1.0.3.a Risiken und Maßnahmen).

## Cause

A kind window (`Figure`, `Table`, …) emitted its title tab into the page before opening a breakable tcolorbox.

- tcolorbox only learns the body height after the tab is already on the page.
- When the body did not fit, tcolorbox broke the page itself, behind the tab.
- The body then restarted on the next page as an unbroken box, which carries no header.
- The same happened to windows taller than a page whose first block (an image) did not fit under the tab.

## Fix

All in `🧰️framework/🛍️products/📓️print/🖋️latex/semio-window.sty`.

- Breakable kind windows no longer emit the tab up front. A new split start (`WindowWholeKeep` region, style `semio~window~whole`) emits it once the body height is known.
- A window that fits on one page is never broken: it moves to the next page whole, tab included.
- A window taller than a page still breaks, and every part carries the full chrome. It starts on a fresh page when the room under the tab cannot take its first lines or its leading unbreakable block.
- A body with no break point that is taller than a page stays one block under its tab (it overflows as before; such content belongs in a long table).
- Unbreakable windows (`break=false`, generic `Window`, windows in rows) now always tie the tab to the body with `\nobreak`.

## Verification

Probe (`probe.ts`, `probe-body.py`, `probe.tex`) under the live Zwischenbericht preamble, seven scenarios:

| Scenario | Before | After |
| --- | --- | --- |
| Figure with `break=false`, does not fit | whole on next page | unchanged |
| Default figure, does not fit | tab orphaned, body headerless | whole on next page |
| Short table, does not fit | not probed before | whole on next page |
| Window that fits in place | in place | pixel-identical |
| Tall text window | breaks, chrome repeated | pixel-identical |
| Tall window starting with an image | tab orphaned, first part headerless | starts on fresh page, chrome on both parts |
| 30-row unbreakable table | not probed before | one block under its tab, overflows the page |

The tab-to-frame seam of a moved window measures the same as the `break=false` reference (baseline rule at 31.38 pt, body at 37.56 pt from the crop origin).

Full build: `bun nx run @semio-tech/mit-bestand-bericht:build-zwischenbericht` succeeded (7 min, 193 pages). Abbildung Stützengenerator (PDF page 18), Abbildung Ansichten der Entwurfsumgebung (page 20) and Tabelle Risiken und Maßnahmen (page 23) each sit whole under their title tab at the top of a page. The `\nobreak` for unbreakable windows was edited seconds after that build started and is covered by the probe only; the Forschungsbericht and the dark variant were not rebuilt.
