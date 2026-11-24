# 🖋️ Summary: Shared Borders and In-Content Chips in the LaTeX Templates

The LaTeX packages now follow the design system's splitting logic: a page is split into closed rectangles, neighbours share the edge between them, and chips lie inside their frame.

## What changed

| Area | Result |
| --- | --- |
| Frame junctions | A frame that directly follows another (heading, window, figure, table, register, list, card table, assignment table) starts on that frame's bottom edge: one hairline shared by both. A paragraph or authored space in between keeps the ordinary separation. The shared edge takes the accent colour when one of the two frames has one. |
| Headings | A heading opens the frame of its section. Only the frame's top edge runs across the column, in the ladder colour. The chips are in-content: title chip draws right and bottom edge, number chip left and bottom edge, and no rule runs below the chip row. |
| Text column | The column is a closed frame: the navbar rule is its top edge, the footer rule its bottom edge (each overlaps the text area by one hairline), and two side borders run between them. Prose is inset by the window body padding; frames span the column. |
| Page edges | A frame at the top of a page (also a chapter) starts on the navbar rule. Parts of a window split across pages fill the page down to the footer rule. |
| Window rows and columns | Tiles overlap by one hairline instead of leaving a gap. An arrangement is a frame towards its neighbours. |
| Lists | `itemize`, `enumerate` and `description` are split into one cell per point: a hairline opens the list, separates neighbouring points and closes it. Inside a window the lines run to the window border. A frame after a list shares the closing line. |
| Header and footer chips | Chips lie flush inside their band and share dividers. Another session on the shared tree closed the bands into full rectangles while this ticket ran; that work was kept as it is. |

## How it works

- `semio-core.sty`: `\semio@frame@open`, `\semio@frame@close`, `\semio@frame@record`. A closing frame records the glue it leaves below itself; an opening frame that finds the record pending cancels the glue and moves up one hairline. The record is explicit state because the page builder empties the main vertical list between two frames, so `\lastskip` cannot be used. `para/begin` and `\vspace` release the record.
- `semio-core.sty`: `\semio@column@inset` and `\semio@column@bleed` (prose inset against frame bleed), and the list dividers.
- `semio-window.sty`: tcolorbox `before`, `before nobreak` and `after` are set explicitly, because tcolorbox's own skips depend on the page builder state and on whether a heading precedes. Split parts use `height fixed for=first and middle`. Break strokes are drawn inside the frame instead of centred on its edge.
- `semio-table.sty`: long tables use `\LTpre=0pt` when they open on a pending frame and close as a frame; the accent edge shared with a heading is painted from the first body row, so continuation pages keep the neutral top edge.
- Geometry: `headsep=-hairline`, `footskip=footer height - hairline`, `\topskip` carries the former head separation for prose, `\maxdepth=0pt`.

## Files changed

- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-core.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-window.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-table.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🧩️frame-junctions/` (`🥒️.feature`, `🔣️.json`, `🟦️.ts`), new
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/⚡️quick-pipeline-verification/🟦️.ts` (runs the new test)
- `🧰️framework/🛍️products/📓️print/README.md` (section "Shared borders", English and German)

## Verified

Compiled with the pinned tectonic through `build.ts` (it stages sources the way `📥️source-staging` does and runs three explicit passes), measured from the PDF vector drawings with `measure.py`, and traced with `trace.py`.

| Check | Result |
| --- | --- |
| Fixture `junctions/junctions.tex` | Heading to heading, heading to window, window to window, window to figure, figure to heading, heading to long table, long table to window, row tiles, row to window: every pair of edges coincides within 0.01 pt. Split window parts and the long table meet the navbar and footer rules. |
| Fixture `junctions/chapter.tex` | Chapter frame starts on the navbar rule; lists are split into cells and keep the prose inset; a heading after a list shares the closing line; a list inside a window shares the window border or is closed by the paragraph that follows; the equation number stays inside the inset. |
| Report (light and dark), thesis, paper, phd, flyer, Zukunft Bau Zwischenbericht | Compile without errors or overfull boxes; pages reviewed. |
| Energy, parametrization and typology assignments (English) | Compile without errors or overfull boxes; pages reviewed. Parametrization and typology used grey placeholders for the image resources that only the Print pipeline generates (`build-with-placeholders.py`). |
| `run-junction-test.ts` | `verifyFrameJunctions` passes (Ajv schema, mathjs arithmetic, source assertions). |

Images: `evidence/`.

## Not verified

- `repo run @semio-tech/print:test -- quick` and `test-assignment` were not run through `repo`; the new test ran directly with bun.
- The page chrome test (`🧪️tests/🏷️page-chrome`) is being changed by another session together with the header and footer bands and was not run here.
- The viz gallery and `viz-api` documents, the German assignment editions and the Forschungsbericht and Kompaktbericht variants were not recompiled; they use the same packages.

## Notes

- The pinned tectonic loops on its in-memory rerun for `paper`, `phd` and Zukunft Bau on this Windows machine (confirmed with the unmodified packages for `paper`). Running the passes one by one with `--reruns 0 --keep-intermediates` converges; `build.ts` does that.
- A long table that breaks across pages ends at a row boundary; the strip down to the footer rule is an empty cell closed by the column borders.
- Prose that fills a page ends directly above the footer rule (`\maxdepth=0pt`).
