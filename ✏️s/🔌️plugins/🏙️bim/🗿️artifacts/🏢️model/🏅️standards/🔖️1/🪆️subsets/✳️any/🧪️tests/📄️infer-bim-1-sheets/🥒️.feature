@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the composed geometry of every sheet of a room and audit its frame, title block, revision table, windows and findings with shapely
  `s.bim.model@1` stores sheets (number, name, paper, orientation and the authored title block fields), the viewports that place authored views on them (position, scale, crop,
  label) and the rows of their revision tables. It stores no geometry of the paper. `📄️sheet-layout` derives, per sheet, the paper as it lies, the frame (20 mm at the binding
  edge, 10 mm elsewhere), the title block in the bottom right corner of the frame with the text of its cells (the scales of the viewports and the last revision mark fill the cells
  the author left empty), the revision table directly above it, the window of every viewport (its crop at the scale of the viewport, at least 10 mm a side) and the findings: a
  window beyond the frame, windows that overlap, a window over the title block or the revision table, a view that draws nothing. The oracle is `🐍️.py` in this directory. It
  reproduces the whole table from the committed snapshot with `shapely` 2: every rectangle is a `box`, containment in the frame is `covers`, an overlap is the area of an
  `intersection`, and the metamorphic laws hold that moving a viewport moves its window without resizing it and that halving the scale denominator doubles the window. The windows of
  uncropped drawn views depend on the extent of the linework, which the view oracle owns; the committed room crops every drawn viewport. The committed expectation is written by that
  file, never by hand.

  @id-sheets-room
  @level-quick
  @mode-differential
  Scenario: Three sheets with an A3 landscape, an A4 portrait and a custom paper resolve to their frames, title blocks, revision table, windows and findings
    Given the committed room shared://💡️inferences/📄️sheet-layout/🏠️room/📸️snapshot/🔣️.json
    When 📄️sheet-layout is inferred for it
    Then every sheet's paper, frame, title block, revision table, viewport windows and findings equal the table shared://💡️inferences/📄️sheet-layout/🏠️room/💡️inference/📄️sheets/🔣️.json
