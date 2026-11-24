# 🖋️ Summary: Shared Borders and In-Content Chips in the LaTeX Templates

The LaTeX packages now follow the design system's splitting logic: a page is split into closed rectangles, neighbours share the edge between them, and chips lie inside their frame.

## What changed

| Area | Result |
| --- | --- |
| Frame junctions | A frame that directly follows another (heading, window, figure, table, register, list, card table, assignment table) starts on that frame's bottom edge: one hairline shared by both. A paragraph or authored space in between keeps the ordinary separation. |
| Edge colours | A horizontal edge separates two sections and belongs to neither, so every horizontal edge is neutral: the frame primitives carry no colour, the heading top edge, the window top edge and the long-table top edge are neutral, and the top and bottom edges of chips are neutral. Only vertical edges may carry an accent, such as the inner side of a heading chip in the ladder colour. |
| Headings | A heading opens the frame of its section. Only the frame's top edge runs across the column. The chips are in-content: title chip draws right and bottom edge, number chip left and bottom edge, and no rule runs below the chip row. |
| Text column | The column is a closed frame: the navbar rule is its top edge, the footer rule closes it one body padding below the text area, and two side borders run between them. Prose is inset by the window body padding; frames span the column. |
| Page edges | A frame at the top of a page (also a chapter) starts on the navbar rule. Parts of a window split across pages fill the page down to the footer rule. |
| Window rows and columns | Tiles overlap by one hairline instead of leaving a gap. An arrangement is a frame towards its neighbours. |
| Lists | `itemize`, `enumerate` and `description` are split into one cell per point: a hairline opens the list, separates neighbouring points and closes it. Inside a window the lines run to the window border. A frame after a list shares the closing line. |
| Header and footer chips | Chips lie flush inside their band and share dividers. Another session on the shared tree closed the bands into full rectangles while this ticket ran; that work was kept, checked at high zoom against the design, and its page chrome test passes. |

## How it works

- `semio-core.sty`: `\semio@frame@open{sep}`, `\semio@frame@close{sep}`, `\semio@frame@record{gap}`. A closing frame records the glue it leaves below itself; an opening frame that finds the record pending cancels the glue and moves up one hairline. The record is explicit state because the page builder empties the main vertical list between two frames, so `\lastskip` cannot be used. `para/begin` and `\vspace` release the record. No colour travels between frames.
- `semio-window.sty`: `\semio@chip@edges` draws `t` and `b` in `semio-chrome-border-normal` and `l` and `r` in the chip's edge colour.
- `semio-core.sty`: `\semio@column@inset` and `\semio@column@bleed` (prose inset against frame bleed), and the list dividers.
- `semio-window.sty`: tcolorbox `before`, `before nobreak` and `after` are set explicitly, because tcolorbox's own skips depend on the page builder state and on whether a heading precedes. Split parts use `height fixed for=first and middle`. Break strokes are drawn inside the frame instead of centred on its edge.
- `semio-table.sty`: long tables use `\LTpre=0pt` when they open on a pending frame and close as a frame.
- Geometry: `headsep=-hairline`, `footskip=footer height - hairline + body padding`, `\topskip` carries the former head separation for prose, `\maxdepth=0pt`.

## Files changed

- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-core.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-window.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-table.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧩️frame-junctions/` (`🥒️.feature`, `🔣️.json`, `🟦️.ts`), new
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/⚡️quick-pipeline-verification/🟦️.ts` (runs the new test)
- `🧰️framework/🛍️products/📓️print/README.md` (section "Shared borders", English and German)
- `🧰️framework/🛍️products/📓️print/🧪️tests/🏷️page-chrome/🔣️.json` (heading chip edges)

## Verified

Compiled with the pinned tectonic through `build.ts` (it stages sources the way the source staging module does and runs explicit passes), measured from the PDF vector drawings with `measure.py`, and traced with `trace.py`.

| Check | Result |
| --- | --- |
| Fixture `junctions/junctions.tex` | Heading to heading, heading to window, window to window, window to figure, figure to heading, heading to long table, long table to window, row tiles, row to window: every pair of edges coincides within 0.01 pt. Split window parts land on the navbar and footer rules. |
| Fixture `junctions/chapter.tex` | Chapter frame starts on the navbar rule; lists are split into cells and keep the prose inset; a heading after a list shares the closing line; a list inside a window shares the window border or is closed by the paragraph that follows; the equation number stays inside the inset. |
| Report (light and dark), thesis, paper, phd, flyer | Compile without errors or overfull boxes; pages reviewed. |
| Zukunft Bau Zwischenbericht, Forschungsbericht, Kompaktbericht | Compile without errors or overfull boxes; pages reviewed. |
| Energy, parametrization and typology assignments, English and German | Compile without errors or overfull boxes; English pages reviewed. The typology assignment uses the images in its folder; the parametrization icons were rasterized from the SVG sources of its resource plan, checksums verified (`rasterize-resources.py`). |
| `viz-api` | Compiles without errors. It reports 105 overfull boxes, the identical 105 it reports with the unmodified packages from git. |
| All 80 viz gallery documents | Compile without errors; 74 have no overfull box, the other six report the identical boxes with the unmodified packages. See `gallery-results.md`. |
| Edge colours (`edge-colors.py`, after the neutral-edge change) | Every horizontal rule of at least 10 pt that is a frame or chip edge is neutral in the junction and chapter fixtures, report (light and dark), thesis, energy and parametrization assignments. The only other horizontal rules are content: answer underlines, a signature line, and the canvas-coloured long-table break foot. All of them were rebuilt without errors. Images: `evidence/neutral-horizontal-edges.png`, `evidence/report-neutral-edges.png`. The gallery, paper, phd, flyer, Zukunft Bau and typology builds listed above ran before this change and were not repeated. |
| Frame junction test (`run-junction-test.ts`) | Passes (Ajv schema, mathjs arithmetic, the edge colour rule, source assertions). |
| Page chrome test (`run-page-chrome-test.ts`) | Passes after its fixture was updated to the in-content heading chips (title: right and bottom, number: left and bottom). The header and footer bands, closed into rectangles by another session, were checked at high zoom: chips lie inside the band, share dividers, and the navbar rule is shared with the first frame. |
| Quick pipeline verification (`run-quick-verification.ts`) | Passes after one pinned overlay assertion was updated to the shared top edge. |
| Assignment test (`bun test` on `🧪️tests/📝️assignment`) | 20 of 20 pass. |

Images: `evidence/`.

## Not verified

- The `repo` test targets could not run: `repo run @semio-tech/print:test-assignment` timed out after 25 minutes in the dependency stage, and `repo run @semio-tech/print:test -- quick` failed with `BUILD_OWNER_FAILED: Unknown admitted build target: @semio-tech/print:build-flyer`, which `repo` reports as reproducing on unchanged inputs. No tracked project file names that target, so the cause is in the `repo` build graph, not in the print sources. The tests those targets run were executed directly with bun, as listed above.

## Notes

- The pinned tectonic loops on its in-memory rerun for `paper`, `phd` and Zukunft Bau on this Windows machine (confirmed with the unmodified packages for `paper`). Running the passes one by one with `--reruns 0 --keep-intermediates` converges; `build.ts` does that.
- The footer rule closes the column one body padding below the text area, so prose never touches it; a window part that breaks at the page end is drawn across that padding onto the rule.
- A long table that breaks across pages stays open at the break: the rule below its last row on the page is taken back (a zero-height longtable foot), so that row runs down to the footer rule; the table's real last row keeps its rule.
