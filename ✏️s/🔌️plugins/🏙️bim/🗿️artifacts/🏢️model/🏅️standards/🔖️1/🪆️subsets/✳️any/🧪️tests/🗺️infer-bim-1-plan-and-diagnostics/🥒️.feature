@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the floor plan of every storey and every problem of the model and audit both with shapely
  `s.bim.model@1` stores authored parameters only. `🗺️plan-linework` derives, per storey, the architectural plan cut at the cut height of the storey (1.2 m above its
  elevation unless the storey authors another) as typed primitives: the poche of the cut walls (join-trimmed footprints without the gaps of the openings that cut the plane),
  the cut columns and curtain wall mullions, and the outlines of beams, slabs, railings, spaces and grid lines. `⚠️diagnostics` derives every
  finding of the model with a severity, a code, the element ids and an English and German message: clashes between element bodies, dangling
  references, openings outside their host, degenerate elements, storeys that share or skip a level, stairs that break the comfort rule and
  spaces that share a number. The oracle is `🐍️.py` in this directory. It rebuilds the walls (mitered and butted bands), columns, mullions,
  beams and gaps as real shapely polygons from the committed snapshots, measures the poche union and the outlines, intersects every pair of
  bodies of a building for the clash areas and decides the references and levels by set arithmetic. The committed expectations are written by
  that file, never by hand. The findings are also exported as one RFC 4180 CSV table (`s.stdio.csv`) and one RFC 8259 JSON document (`s.stdio.json`) with the
  counts, the findings with their numbers and messages in both languages, and the index by element and storey. The committed exports are written by the subject and
  read back by the standard `csv` and `json` modules of python: the JSON measures must equal the shapely table and the CSV must hold one record per finding.

  @id-plan-metrics-house
  @level-quick
  @mode-differential
  Scenario: The plan of a two-storey house measures as shapely measures its poche, columns, mullions and outlines
    Given the committed plan house shared://💡️inferences/🗺️plan-linework/🏡️house/📸️snapshot/🔣️.json
    When 🗺️plan-linework is inferred for it
    Then every storey's plan measures equal the table shared://💡️inferences/🗺️plan-linework/🏡️house/💡️inference/🗺️plan-metrics/🔣️.json

  @id-plan-metrics-curved
  @level-quick
  @mode-differential
  Scenario: A curved wall with a window and a round column keep their exact areas and agree with the sampled arcs of shapely
    Given the committed curved plan shared://💡️inferences/🗺️plan-linework/🌀️curved/📸️snapshot/🔣️.json
    When 🗺️plan-linework is inferred for it
    Then every storey's plan measures equal the table shared://💡️inferences/🗺️plan-linework/🌀️curved/💡️inference/🗺️plan-metrics/🔣️.json

  @id-plan-metrics-cut-heights
  @level-quick
  @mode-differential
  Scenario: A storey cut low and a storey cut high open and close the gaps of its poche exactly as shapely cuts them
    Given the committed plan with authored cut heights shared://💡️inferences/🗺️plan-linework/🔪️cut-heights/📸️snapshot/🔣️.json
    When 🗺️plan-linework is inferred for it
    Then every storey's plan measures equal the table shared://💡️inferences/🗺️plan-linework/🔪️cut-heights/💡️inference/🗺️plan-metrics/🔣️.json

  @id-diagnostics-clean
  @level-quick
  @mode-differential
  Scenario: A clean house has no clash, no dangling reference and no level problem
    Given the committed clean house shared://💡️inferences/⚠️diagnostics/🏡️clean/📸️snapshot/🔣️.json
    When ⚠️diagnostics is inferred for it
    Then the adjudicated findings equal the table shared://💡️inferences/⚠️diagnostics/🏡️clean/💡️inference/⚠️diagnostics/🔣️.json

  @id-diagnostics-defects
  @level-quick
  @mode-differential
  Scenario: Clashes, a missing wall type, an orphaned opening, an opening outside its wall, a zero-length wall, duplicate and skipped levels and shared space numbers are found
    Given the committed defect house shared://💡️inferences/⚠️diagnostics/💥️defects/📸️snapshot/🔣️.json
    When ⚠️diagnostics is inferred for it
    Then the adjudicated findings equal the table shared://💡️inferences/⚠️diagnostics/💥️defects/💡️inference/⚠️diagnostics/🔣️.json

  @id-diagnostics-export-json
  @level-quick
  @mode-differential
  Scenario: The JSON export of the findings of the defect house reads back with the json module of python as the table shapely adjudicates
    Given the committed defect house shared://💡️inferences/⚠️diagnostics/💥️defects/📸️snapshot/🔣️.json and its JSON export shared://💡️inferences/⚠️diagnostics/💥️defects/📤️export/⚠️diagnostics.json
    When the export is read back
    Then the adjudicated findings of the export equal the table shared://💡️inferences/⚠️diagnostics/💥️defects/💡️inference/⚠️diagnostics/🔣️.json

  @id-diagnostics-export-csv
  @level-quick
  @mode-differential
  Scenario: The CSV export of the findings of the defect house reads back with the csv module of python as one record per finding, and holds every finding shapely adjudicates
    Given the committed defect house shared://💡️inferences/⚠️diagnostics/💥️defects/📸️snapshot/🔣️.json and its CSV export shared://💡️inferences/⚠️diagnostics/💥️defects/📤️export/⚠️diagnostics.csv
    When the export is read back
    Then every record equals the finding the subject writes: its position, code, elements, severity, storey, missing ids and the English and German message

  @id-diagnostics-panel-groups
  @level-quick
  @mode-differential
  Scenario: The diagnostics panel groups the findings of the defect house by severity, storey level and kind as plain dictionaries group the exported findings
    Given the committed defect house shared://💡️inferences/⚠️diagnostics/💥️defects/📸️snapshot/🔣️.json and its JSON export shared://💡️inferences/⚠️diagnostics/💥️defects/📤️export/⚠️diagnostics.json
    When the findings are grouped by severity, then by storey in level order with the model-wide findings last, then by kind
    Then every group holds the count of findings the export gives for its severity, storey and kind
