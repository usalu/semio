@capability-bim-1-export-pdf
@oracle-bim-1-pypdf-pdf
@comparison-floating-point-v1
Feature: Open the PDF sheet set of a BIM room with pypdf and read its pages
  The subject writes the sheets of the committed room as one PDF 1.7 document, one page per sheet in the order of the sheet numbers, through the existing stdio PDF writer (`s.stdio.pdf@1.7/*`):
  the page is the paper in points, the drawing is vector operators in millimetres of paper, and the title block prints its text in a standard font. The pypdf oracle never sees the subject's
  writer: it opens the committed file with an unrelated PDF reader, counts the pages, converts the media box of every page from points to millimetres and extracts the text of the page with its own
  content-stream interpreter, which must show the number and the title of the sheet. The subject reports the same table from the `sheet-layout`. The committed file is written by the subject's
  export test (`BIM_BLESS=1`), never by hand.

  @id-export-sheets-pdf-room
  @level-quick
  @mode-differential
  Scenario: Page count, page sizes and the printed number and title of every sheet of the exported room equal the subject's report
    Given the committed room shared://💡️inferences/📄️sheet-layout/🏠️room/📸️snapshot/🔣️.json and its export shared://🚪️sheets/🏠️room/sheets.pdf
    When the file is opened and every page is measured and its text extracted
    Then the page count, the page sizes to a tenth of a millimetre and the printed number and title of every page equal the subject's
