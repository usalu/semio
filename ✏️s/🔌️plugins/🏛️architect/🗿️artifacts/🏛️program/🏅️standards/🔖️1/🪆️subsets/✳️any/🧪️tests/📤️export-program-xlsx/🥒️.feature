@capability-program-1-xlsx-export
@oracle-architect-program-xlsx-calamine
@oracle-input-subject-raw
@comparison-ordered-json-v1
Feature: Export every architect program table to a real XLSX workbook
  The Rust subject reads a committed program snapshot, maps its full document into seventy worksheets,
  and emits the production ECMA-376 byte stream. The oracle opens those exact bytes with calamine 0.36
  and reports every non-empty cell. It neither links architect nor trusts stdio's XLSX decoder.

  @id-export-xlsx
  @level-quick
  @mode-differential
  Scenario: A populated program remains visible to an independent spreadsheet reader
    Given the committed program snapshot asset://🧬️schema/🧬️mutations/🧱️program-element/🌱️create/🧪️tests/🌱️creates-a/📸️snapshot/➡️after/🔣️.json
    When the subject exports every register through the XLSX serializer
    Then calamine observes the same worksheet names, columns and populated cells
