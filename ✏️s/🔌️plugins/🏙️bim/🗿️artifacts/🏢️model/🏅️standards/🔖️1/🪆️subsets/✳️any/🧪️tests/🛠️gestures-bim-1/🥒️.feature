@capability-bim-1-author
@oracle-bim-1-numpy-shapely-geometry
@comparison-floating-point-v1
Feature: Drive every BIM authoring tool from the keyboard with typed points and audit the numbers with numpy
  The authoring tools of `s.bim.model@1` (`✏️editor/🧵️gestures`) turn pointer events into model mutations, and the same tools take a typed line as a click
  at the point the line names, so geometry can be placed without a pointer. The line is read as one of four forms, in metres and degrees counter-clockwise
  from +X: `x, y` (also `x; y` and `x y`) is the absolute point, `@dx, dy` is the offset from the point the gesture is anchored at (the origin before the
  first point), `length<angle` (also `@length<angle`) is the polar offset from the anchor, and a bare `length` goes that far from the anchor towards where
  the pointer last was (towards +X when it never was). An empty line finishes the gesture, as Enter does. The oracle is `🐍️.py` in this directory: it
  parses the same lines with its own grammar and recomputes each point with `numpy` (trigonometry and vector arithmetic), then writes the expectations into
  `🧫️fixtures/🛠️gestures/🔣️.json`, which the Rust subject replays through `typed::parse` and `typed::resolve`. The tools themselves (a wall chain, an arc wall, a
  slab polygon, a column, a window on the wall under the point, a move, a rotation, a split) are exercised by the subject's unit tests with the same lines.
  The same tools take the arrow keys: a keyboard cursor starts where the pointer last was (else at the point the gesture hangs on, else at the centre of the view),
  steps 0.1 m with an arrow key and 1 m with Shift, and a click key places the point there. The oracle sums the steps with `numpy` into the `cursor` cases of the same file.

  @id-typed-point
  @level-exhaustive
  @mode-differential
  Scenario Outline: The typed line <entry> names the point the oracle computed
    Given the committed gesture cases shared://🧫️fixtures/🛠️gestures/🔣️.json
    And the gesture is anchored at <anchor> and the pointer last was at <toward>
    When the line <entry> is resolved
    Then the point equals <point> within 1e-8 metres
    Examples:
      | entry          | anchor  | toward  | point                   |
      | 3, 4           |         |         | [3.0, 4.0]              |
      | 3;4            |         |         | [3.0, 4.0]              |
      | -3.5 4e-1      |         |         | [-3.5, 0.4]             |
      | 10 ; 5         | [7, 7]  |         | [10.0, 5.0]             |
      | @2, -1         | [1, 1]  |         | [3.0, 0.0]              |
      | @ 2 -1         |         |         | [2.0, -1.0]             |
      | 2<90           | [1, 1]  |         | [1.0, 3.0]              |
      | 2<180          | [1, 1]  |         | [-1.0, 1.0]             |
      | @2.5<45        | [0, 0]  |         | [1.767766953, 1.767766953] |
      | 4.2426406871<45 | [1, 2] |         | [4.0, 5.0]              |
      | 1<-30          | [0, 0]  |         | [0.866025404, -0.5]     |
      | 3              | [1, 1]  | [1, 9]  | [1.0, 4.0]              |
      | 3              | [1, 1]  | [-4, 1] | [-2.0, 1.0]             |
      | 3              | [1, 1]  |         | [4.0, 1.0]              |
      | 3              | [1, 1]  | [1, 1]  | [4.0, 1.0]              |
      | -2m            | [0, 0]  | [0, 5]  | [0.0, -2.0]             |

  @id-typed-wall
  @level-exhaustive
  @mode-scenario
  Scenario: A wall is drawn without a pointer
    Given an empty plan window with the wall tool armed and a wall type in the library
    When the lines "0, 0", "@4<0", "@0, 3" and an empty line are entered
    Then the model holds the walls (0, 0) to (4, 0) and (4, 0) to (4, 3)
    And an invalid line such as "not a point" is refused and leaves the gesture untouched

  @id-cursor-point
  @level-exhaustive
  @mode-differential
  Scenario: The keyboard cursor stands where numpy summed the arrow keys
    Given the committed gesture cases shared://🧫️fixtures/🛠️gestures/🔣️.json
    When the arrow keys of each cursor case are pressed from its start: a fine key steps 0.1 m, a far key 1 m
    Then the cursor stands on the point of the case within 1e-8 metres

  @id-cursor-wall
  @level-exhaustive
  @mode-scenario
  Scenario: A wall is drawn with the arrow keys alone
    Given an empty plan window with the wall tool armed and a wall type in the library
    When the cursor is moved with the arrow keys and placed, then moved and placed again
    Then the model holds the wall between the two placed points
    And moving the cursor alone writes nothing and shows the rubber band of the tool

  @id-component-fit
  @level-exhaustive
  @mode-differential
  Scenario: A wall-mounted component stands on the face of the wall with its back to it
    Given the committed gesture cases shared://🧫️fixtures/🛠️gestures/🔣️.json
    When the plan point of each component_fits case is fitted to the wall axis of its thickness
    Then the family origin lies on the face of the side the point is on (the left face for a point on the axis) within 1e-8 metres
    And the local depth axis points away from the wall along the outward normal of that face

  @id-component-frame
  @level-exhaustive
  @mode-differential
  Scenario: The family frame is mirrored, then turned, then moved to its origin
    Given the committed gesture cases shared://🧫️fixtures/🛠️gestures/🔣️.json
    When each local point of each component_frames case is placed in the plan
    Then it lands on the point numpy computed with a rotation matrix within 1e-8 metres

  @id-component-place
  @level-exhaustive
  @mode-scenario
  Scenario: A family is placed with a ghost, turned in steps of fifteen degrees, and written by a click
    Given a plan window with the component tool armed and a table family and a basin family in the library
    When the ghost is turned three times by fifteen degrees, mirrored and raised by two tenths of a metre and the plan is clicked
    Then the model holds one component of the table with a rotation of 45 degrees, mirrored, 0.2 m above the storey elevation
    And the committed turns cases of numpy equal the summed rotation modulo 360 degrees for every run of turn keys
    And the typed lines "z 0.8", "rot 45", "mirror", "sys water" and "fam basin" set the same options without a pointer

  @id-component-host
  @level-exhaustive
  @mode-scenario
  Scenario: A basin clings to the wall beside the pointer and Ctrl places it free
    Given a plan window with the component tool armed and the basin family selected in the library
    When the pointer is half a wall thickness and three centimetres from a wall and the plan is clicked
    Then the component is hosted by that wall and keeps the pointer as its position
    And with Ctrl held the component has no host
    And a table family is never hosted

  @id-route-length
  @level-exhaustive
  @mode-differential
  Scenario: A route is written from its clicks and measures what numpy measures
    Given the committed gesture cases shared://🧫️fixtures/🛠️gestures/🔣️.json
    When the vertices of each routes case are clicked at their elevations, typed as "z <metres>", and the run is finished
    Then the rubber band label shows the length numpy summed from the segment norms
    And the model holds one MEP element whose path is the clicked vertices at their elevations

  @id-route-keys
  @level-exhaustive
  @mode-scenario
  Scenario: A route is edited with the keys while it is drawn
    Given a plan window with the MEP route tool armed
    When two vertices are clicked, the system is changed with Alt+T, the section kind with Tab, the elevation with Alt+PageUp and a third vertex is clicked
    Then Backspace takes the last vertex back, Enter writes the element with the chosen system and section, and Escape drops a route in progress before it leaves the tool
