Feature: Authored IFC Normal Interchange
  The source coordinate and normal index domains remain distinct.
  Authored IFC values remain unchanged during derived GLB preparation.

  Scenario Outline: Exact source pair transport
    Given the closed source row <row>
    When a represented mesh is exported to glTF
    Then usable referenced directions are normalized in the derived NORMAL attribute
    And the raw CoordIndex selects normals before PnIndex selects points
    And POSITION and NORMAL counts match the exact point-normal pair roster
    And a referenced zero causes explicit complete NORMAL omission
    And unused zero or extra normal rows do not erase referenced directions
    And the original source and STEP are unchanged

  Scenario: Shared coordinates have separate authored hard-edge directions
    Given six raw normal indices map to four coordinates across two triangles
    When the mesh is encoded
    Then six exact point-normal pairs remain and both source triangles remain

  Scenario: Matrix and output evidence stay separate
    Given an invertible finite column matrix and a finite nonzero source direction
    When inverse transpose direction is computed
    Then its normalized result matches the independent matrix oracle
    And reflection uses determinant sign without double-swapping triangle corners
    And singular input refuses rather than fabricating a direction
    And float32 rounding and output grants remain separately validated
