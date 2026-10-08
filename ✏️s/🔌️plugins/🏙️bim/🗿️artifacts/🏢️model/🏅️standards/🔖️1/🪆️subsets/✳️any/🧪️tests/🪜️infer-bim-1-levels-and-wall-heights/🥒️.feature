@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer every storey elevation and wall height of the oracle house and audit the wall geometry with shapely
  `s.bim.model@1` stores authored parameters only: a storey's `height` and `level`, a wall's axis, type, base
  offset and top constraint. `🪜️storey-levels` derives each storey's elevation as the sum of the heights of the
  storeys below it in its building, and `🧱️wall-layout` derives each wall's base, top, height, thickness, centreline length,
  face offsets, join-trimmed face curves and footprint loop, side areas and volume from it. The oracle is `🐍️.py` in this
  directory. It reproduces both tables from the committed snapshot (the plan geometry through the sibling oracle
  `../🧱️infer-bim-1-wall-joins`, where `shapely` 2 measures every footprint and join invariant) and proves the parametric
  law: raising one storey height moves exactly the storeys above it and exactly the walls that end at its top. The committed
  expectations are written by that file, never by hand.

  @id-storey-levels
  @level-quick
  @mode-differential
  Scenario: Elevations are the running sum of the storey heights per building
    Given the committed oracle house shared://💡️inferences/🏠️house/📸️snapshot/🔣️.json
    When 🪜️storey-levels is inferred for it
    Then every storey's elevations equal the table shared://💡️inferences/🏠️house/💡️inference/🪜️storey-levels/🔣️.json

  @id-wall-layout
  @level-quick
  @mode-differential
  Scenario: Wall heights, lengths, footprints and volumes follow the storeys and agree with shapely
    Given the committed oracle house shared://💡️inferences/🏠️house/📸️snapshot/🔣️.json
    When 🧱️wall-layout is inferred for it
    Then every wall's layout equals the table shared://💡️inferences/🏠️house/💡️inference/🧱️wall-layout/🔣️.json
