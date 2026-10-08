@capability-bim-1-infer
@oracle-bim-1-three-mesh
@comparison-floating-point-v1
Feature: Measure the blessed element solid meshes of walls, curtain walls and fillers with three.js
  Each case of `🧫️fixtures/💡️inferences/🧊️element-solids` commits the authored snapshot and the meshes of the solids the subject inferred from it
  (welded positions and triangle indices, one per element). three.js loads them as `BufferGeometry` and measures volume (signed tetrahedra), area
  (`Triangle.getArea`), bounds (`Box3`) and triangle count. The subject reports the same numbers from its live `element-solids` inference, so a
  committed mesh that is stale, or a volume that is not the volume of the triangles, is a disagreement.

  @id-element-solids-three
  @level-quick
  @mode-differential
  Scenario: three.js measures the committed meshes exactly as the subject measures its inferred solids
    Given the committed straight wall cases shared://💡️inferences/🧊️element-solids/🚪️straight-openings/🔣️.json
    And the committed joined room case shared://💡️inferences/🧊️element-solids/🧩️room-joins/🔣️.json
    And the committed curtain wall case shared://💡️inferences/🧊️element-solids/🏬️curtain-grid/🔣️.json
    When every committed mesh is loaded as a BufferGeometry
    Then the measured volume, area, bounds and triangle count equal the subject's element solids
