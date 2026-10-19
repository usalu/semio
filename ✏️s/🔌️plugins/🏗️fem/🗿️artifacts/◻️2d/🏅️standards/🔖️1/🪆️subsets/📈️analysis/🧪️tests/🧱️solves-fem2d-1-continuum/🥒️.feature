@capability-fem2d-1-analysis
@oracle-skfem-fem2d-continuum
@comparison-semantic-fem2d-continuum-v1
Feature: Resolve all continuum geometry and loads before static, modal and buckling analysis

  scikit-fem independently assembles plane-stress elasticity, consistent mass and initial-stress
  geometric stiffness. SciPy solves the restrained eigenproblems. All fixture nodes are authored;
  native mesh-retention tests separately exercise synthesized boundary and interior nodes.
  The hole fixtures apply exact constant-stress boundary tractions on both boundary rings, so
  different valid triangulations must recover the same displacement, stress and support reactions.
  Area pressure is a global downward body load per unit planar area. Self weight uses SI gravity.

  @id-continuum
  @level-exhaustive
  @mode-differential
  Scenario Outline: The <id> answers as an independent plane-stress implementation
    Given the committed model shared://🧱️solves-fem2d-1-continuum/<fixture>
    And the committed reference shared://🧱️solves-fem2d-1-continuum/📊️expected.results.json
    When both implementations solve the complete continuum with its declared restraints and loads
    Then all authored displacements, reactions and continuum stress extrema agree
    And region-only and mixed-model natural frequencies and area and weight prestress factors agree
    Examples:
      | id          | fixture                         |
      | triangle    | 🔺️triangle.snapshot.json        |
      | mixed       | 🌉️mixed.snapshot.json           |
      | hole        | 🕳️hole.snapshot.json             |
      | scaled-hole | 📏️scaled-hole.snapshot.json     |
