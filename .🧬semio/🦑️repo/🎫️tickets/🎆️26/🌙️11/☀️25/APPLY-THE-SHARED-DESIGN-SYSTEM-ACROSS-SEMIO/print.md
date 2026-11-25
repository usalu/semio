# Print design alignment

The prior print ticket records the shared-border work and its 98-document verification. Heading chips now use window-level glass and put the existing hierarchy accent on the inner vertical edge. Heading titles, window and table chip labels, and every segment of a linked hierarchy number use the appearance foreground. Chapter, part, and nested headings use the same heading frame. Silent table headings still register their hierarchy and labels.

## Verification

- The page chrome, frame junction, and quick pipeline verifiers pass through the repo-dispatched Bun runtime. The page chrome regression reproduced the previous accent fill and muted pane labels before those were corrected.
- All 57 assignment, token paint, and thesis paint-reader tests pass through `repo test-file`. The light and dark window and pane composites clear the 4.5:1 text contrast threshold against the canvas with both the framework calculation and the independent Colord calculation.
- The previous dark pane foreground for number prefixes measured 4.30:1 against the composited canvas. The themed foreground measures 11.65:1 there.
- The thesis paint reader checks the heading title and the entire numeral against the expected appearance foreground. Publication and rendering fixtures now assert the shared glass fill and composited contrast.
- The light and dark reports compiled with pinned Tectonic 0.16.9 in three passes, producing 12 pages each with zero overfull boxes. Poppler previews of pages 5 and 9 show in-content chips over full-bleed images, a neutral shared line at the top of the appendix heading, and a single shared edge between the adjoining figure and table. Independent PDF readers confirmed the window, pane, and foreground fills and their actual opacity values of 0.88, 0.76, and 0.10.
- The thesis and flyer compiled in three passes, producing 32 and 2 pages with zero overfull boxes. The first thesis body page retains the chapter and section numbers and the resolved citation. The flyer tiles share their interior divider and meet the footer rule.
- The dark specimen in `print-chips.tex` compiled to eight pages with zero overfull boxes. Its part opens on page 1, and page 3 shows the chapter, section, and nested headings. Page 4 shows the opaque section and table number with the silent heading suppressed. Independent PDF inspection found both glass opacity operators on page 3 and only opaque paints on page 4.
- The graph-backed `repo run @semio-tech/print:test -- quick` waited for host memory and reached the 600-project graph inventory. It was stopped during the inventory after the graph-free test and render checks passed. The full 98-document catalog count describes the earlier shared-border verification.
