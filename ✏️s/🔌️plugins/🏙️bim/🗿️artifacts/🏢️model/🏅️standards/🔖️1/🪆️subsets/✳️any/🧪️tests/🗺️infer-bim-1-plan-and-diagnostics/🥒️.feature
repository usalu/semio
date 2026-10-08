@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the floor plan of every storey and every problem of the model and audit both with shapely
  `s.bim.model@1` stores authored parameters only. `🗺️plan-linework` derives, per storey, the architectural plan cut 1.2 m above the storey
  elevation as typed primitives: the poche of the cut walls (join-trimmed footprints without the gaps of the openings that cut the plane),
  the cut columns and curtain wall mullions, and the outlines of beams, slabs, railings, spaces and grid lines. `⚠️diagnostics` derives every
  finding of the model with a severity, a code, the element ids and an English and German message: clashes between element bodies, dangling
  references, openings outside their host, degenerate elements, storeys that share or skip a level, stairs that break the comfort rule and
  spaces that share a number. The oracle is `🐍️.py` in this directory. It rebuilds the walls (mitered and butted bands), columns, mullions,
  beams and gaps as real shapely polygons from the committed snapshots, measures the poche union and the outlines, intersects every pair of
  bodies of a building for the clash areas and decides the references and levels by set arithmetic. The committed expectations are written by
  that file, never by hand.

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
