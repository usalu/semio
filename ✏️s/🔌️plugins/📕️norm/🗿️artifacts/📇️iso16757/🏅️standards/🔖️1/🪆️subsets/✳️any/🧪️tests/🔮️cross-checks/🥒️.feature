@capability-iso16757-1-evaluate
@oracle-iso16757-1-cross-check
@comparison-numerical-v1
Feature: ISO 16757-2 geometry volumes reproduce the third-party cross-checks
  The independent oracle `🔮️oracles/🌐️cross-checks/🐍️.py` (shapely polygon booleans on z-sections integrated with scipy quad, scipy ConvexHull and closed forms) writes the vectors in `🧫️fixtures/🔮️cross-checks/🔣️.json`; every implementation must reproduce the rows tagged @agree.
  Primitive volumes are exact closed forms, rotations are rigid, disjoint booleans are summed exactly and overlapping booleans are integrated with a relative quadrature tolerance of 1e-8 of the bounding volume.

  @id-volume-box
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the box volume equals the convex hull volume
    Given the vector "<vector>" of the section "volumes"
    When the geometry volume is evaluated
    Then it equals solidVolumeM3 within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 1e-9 |
  @id-volume-solid
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the volume of curved, rotated and boolean geometry
    Given the vector "<vector>" of the section "volumes"
    When the geometry volume is evaluated
    Then it equals solidVolumeM3 within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 1 | 2e-5 |
      | 2 | 2e-5 |
      | 3 | 2e-5 |
      | 4 | 2e-5 |
      | 5 | 2e-5 |
      | 6 | 2e-5 |
      | 7 | 2e-5 |
      | 8 | 2e-5 |
      | 9 | 2e-5 |
      | 10 | 2e-5 |
      | 11 | 2e-5 |
