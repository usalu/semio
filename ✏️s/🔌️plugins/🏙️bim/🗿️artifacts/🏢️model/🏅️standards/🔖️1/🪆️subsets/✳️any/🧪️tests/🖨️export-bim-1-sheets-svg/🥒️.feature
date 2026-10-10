@capability-bim-1-export-svg
@oracle-bim-1-lxml-shapely-svg
@comparison-floating-point-v1
Feature: Open the SVG sheets of a BIM room with lxml and measure their windows with shapely
  The subject writes every sheet of the committed room (an A3 landscape sheet with a revision table, an A4 portrait sheet and a custom 500 by 350 mm sheet) as one SVG 1.1 file in paper
  millimetres: the viewBox is the paper, every viewport is a group clipped to its window that holds the linework of its view already scaled to millimetres of paper, and the title block and the
  revision table are groups of text runs. The lxml and shapely oracle never sees the subject's writer: it opens the committed files as namespaced XML, requires the SVG root with a millimetre
  size equal to its viewBox, follows the `clip-path` of every viewport to its `clipPath` rectangle and measures that window as a shapely `box`, reads the scale of the group, lists the text runs
  of the title block and of the revision table, and audits that the window of a cropped viewport is exactly the crop of the committed model at the scale of the viewport and that the drawing of
  every viewport lands on its window. The subject reports the same table from the `sheet-layout` and the marks it draws. The committed files are written by the subject's export test
  (`BIM_BLESS=1`), never by hand.

  @id-export-sheets-svg-room
  @level-quick
  @mode-differential
  Scenario: Paper size, viewport windows, scales and title block texts of the exported room equal the subject's report
    Given the committed room shared://💡️inferences/📄️sheet-layout/🏠️room/📸️snapshot/🔣️.json and its exports shared://🚪️sheets/🏠️room/A-101.svg shared://🚪️sheets/🏠️room/A-901.svg shared://🚪️sheets/🏠️room/A-902.svg
    When the files are parsed and every viewport window and title block is measured
    Then the paper, the windows, the scales and the sorted title block and revision texts equal the subject's within 1e-9
