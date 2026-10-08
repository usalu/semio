@capability-bim-1-infer
@oracle-bim-1-ifcopenshell-kernel
@comparison-floating-point-v1
Feature: Measure every straight wall of the oracle house with the IfcOpenShell geometry kernel
  The sibling case `🪜️infer-bim-1-levels-and-wall-heights` commits each wall's resolved `base_z`, `top_z`,
  `height`, `thickness` and `length`. This case hands those numbers to IfcOpenShell 0.8.4.post1: each straight
  wall is written as an `IfcWall` extruded by `add_wall_representation`, placed at its committed base `z`,
  serialised to Part-21 text, re-opened and tessellated by the library's own kernel. The measured `z` extent and
  volume must equal the committed `base_z`, `top_z` and `volume`. Arc walls have no IFC standard-case form and
  are audited by `shapely` in the sibling case.

  @id-wall-solids
  @level-quick
  @mode-differential
  Scenario: The kernel's z extent and volume of every straight wall equal the committed layout
    Given the committed oracle house shared://💡️inferences/🏠️house/📸️snapshot/🔣️.json
    When every straight wall of the committed layout shared://💡️inferences/🏠️house/💡️inference/🧱️wall-layout/🔣️.json is built as an IfcWall
    Then the tessellated walls have the committed base_z, top_z and volume

  @id-wall-solids-openings
  @level-quick
  @mode-differential
  Scenario: The kernel volume of every straight wall with its openings subtracted equals the subject's wall solid
    Given the committed element solids case shared://💡️inferences/🧊️element-solids/🚪️straight-openings/🔣️.json
    When every straight wall of the case is built as an IfcWall with one IfcOpeningElement per window, door and void
    Then the tessellated walls have the volume and z extent of the subject's wall solids
