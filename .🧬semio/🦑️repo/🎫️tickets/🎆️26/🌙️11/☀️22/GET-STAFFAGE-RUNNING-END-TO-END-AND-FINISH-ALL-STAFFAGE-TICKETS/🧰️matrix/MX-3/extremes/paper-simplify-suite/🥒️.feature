@capability-staffage-paper-simplify
Feature: The paper simplification keeps the extent of a drawn ring
  Every ring that reaches the paper is simplified to a tolerance of 0.01 mm. A tolerance in paper millimetres is a length of the object that grows with the
  denominator: 10 mm at 1:1000. A tapered or rounded tip lies within that tolerance of the chord of its neighbours, so a plain Douglas-Peucker pass cuts it off
  and the drawing is shorter than the body it draws, where the dimension check of the matrix leaves 15 mm. The simplification keeps, for every side of the box of
  a closed ring, the extreme vertex when no kept vertex reaches that side; where the pass already keeps it, the result is the same as before.

  THE REFERENCE is shapely: the bounds of the ring and of its simplification are equal and the Hausdorff distance between the two polygons stays within the tolerance.

  Scenario: The box of the ring survives
    Given the outlines of a hand, a foot and a thin leaf as rings of paper millimetres
    When the rings are simplified to 0.01 mm
    Then the box of each simplified ring equals the box of the ring and no vertex is added that the ring did not have

  Scenario: Nothing changes where nothing was cut
    Given a square with a point in the middle of every edge
    When the ring is simplified
    Then the four corners remain
