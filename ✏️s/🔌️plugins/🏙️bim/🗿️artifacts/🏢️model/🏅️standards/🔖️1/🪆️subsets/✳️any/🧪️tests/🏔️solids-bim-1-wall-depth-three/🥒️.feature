@capability-bim-1-infer
@oracle-bim-1-three-mesh
@comparison-floating-point-v1
Feature: Measure the blessed meshes of the walls under roofs, the wall sweeps and the fillers of the attic model with three.js
  The case `🧫️fixtures/💡️inferences/🧗️wall-depth/🏠️attic` commits the authored snapshot and the meshes of the solids the subject inferred from it (welded positions and triangle indices, one per
  element): walls whose top follows a gable or a hip roof or whose base follows a sloped slab, baseboards and a hand rail, and the fillers of a window with a reveal. three.js loads them as
  `BufferGeometry` and measures volume (signed tetrahedra), area (`Triangle.getArea`), bounds (`Box3`) and triangle count. The subject reports the same numbers from its live `element-solids`
  inference, so a committed mesh that is stale, or a volume that is not the volume of the triangles, is a disagreement.

  @id-wall-depth-three
  @level-quick
  @mode-differential
  Scenario: three.js measures the committed meshes exactly as the subject measures its inferred solids
    Given the committed attic meshes shared://💡️inferences/🧗️wall-depth/🏠️attic/🧊️meshes/🔣️.json
    And the committed attic model shared://💡️inferences/🧗️wall-depth/🏠️attic/📸️snapshot/🔣️.json
    When every committed mesh is loaded as a BufferGeometry
    Then the measured volume, area, bounds and triangle count equal the subject's element solids
