# DOCX Table Primary Projection Repair

## Finding and Source Repair

Root review found that the canonical primary document renderer counted top-level paragraphs and tables, then called a paragraph-only run-count helper on every block. Any table therefore returned an error and aborted the whole main window. The top-level run projection now accepts either canonical paragraph or table blocks and resolves nested runs in XML document order to their actual physical node addresses. The separate semantic nested-block selection helper retains its paragraph-specific contract. Unused ordinal fields were removed from the projected editable run; text and its canonical XML address remain.

## Authored Validation

A language-neutral fixture in the DOCX base subset `🧫️fixtures/🧭️table-run-projection` carries an aliased WordprocessingML document, a paragraph before and after a table, two table cells, a nested table, five expected texts, and all exact physical XML paths. A native Quick-XML law independently reads the five texts, then checks each projected address resolves. Base, strict, and transitional main-window laws require all five editable drafts and successful English/German rendering. The shared Office TypeScript schema task now validates this fixture and its row/path cardinalities. These new laws have not executed; DOCX native current7 remains in the shared compilation queue.

## Remaining Primary Experience Work

This repair prevents a complete table-containing document from being refused at projection. It does not supply a rich table layout, format toolbar, blank paragraph insertion, native page layout, footnote/header navigation, or bounded lazy run enumeration. The current primary projection still scans and materializes all runs before UI windowing. Those remain completion requirements.
