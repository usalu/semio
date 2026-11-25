# language: en
Feature: A slender body is judged across its width as finely as along its length
  A front or plan view that is a mirrored, rotated or uniformly scaled copy of another view of its band is not a handcrafted view.
  The two views are compared as point sets with a unit longest side; the distance is taken relative to the box of the two views, each coordinate difference as a share of the extent along its axis, and a distance below 0.03 is a copy.
  A copy and a near copy are found on a slender body as on a compact one; two views that differ by a quarter of their width are two drawings, however long the body is.
  THE REFERENCE is shapely: `affinity.scale` to the box and `hausdorff_distance` of the two point sets, under the eight transforms.

  Scenario Outline: The distance equals the distance shapely measures and decides the flag
    Given the side elevation and the front "<case>"
    When the front is compared with the side elevation
    Then the distance equals the shapely distance within a billionth
    And the front is <verdict>

    Examples:
      | case                                                                 | verdict     |
      | an L-shaped outline and its identical copy                           | flagged     |
      | an L-shaped outline and its mirrored copy                            | flagged     |
      | an L-shaped outline rotated by 90 degrees, scaled by 2 and shifted   | flagged     |
      | an L-shaped outline and a handcrafted other outline                  | not flagged |
      | a steel column 3 m high: flange side 300 mm against web side 260 mm  | not flagged |
      | a glulam beam 8 m long: 620 mm high against 160 mm wide              | not flagged |
      | a steel column and its identical copy                                | flagged     |
      | a steel column and a copy 4 mm narrower                              | flagged     |
      | a steel column and a copy with the head plate 60 mm higher           | flagged     |
      | a steel column and a copy that is 600 mm longer                      | not flagged |

  Scenario: The relative distance is never below the plain one
    Given every pair of the vectors
    Then the distance relative to the box is at least the plain distance of the unit-normalised sets, and zero only where that one is zero
