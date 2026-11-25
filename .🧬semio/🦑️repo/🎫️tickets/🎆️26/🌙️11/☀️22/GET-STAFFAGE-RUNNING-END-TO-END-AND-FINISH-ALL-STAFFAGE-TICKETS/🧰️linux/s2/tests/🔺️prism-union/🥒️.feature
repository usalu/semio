# language: en
Feature: The outline of a swept profile is built at every orientation
  An extruded shape is drawn as the union of its profile at both ends and the sweep of every edge.
  When the profile plane turns through the view direction the faces project to slivers, edges overlap a few nanometres apart and corners arrive twice; the union must still be one outline of the right area and never an error.
  THE REFERENCE is shapely `unary_union` (the oracle of 🧪️tests/🔺️polygon-union) and polygon-clipping.

  Scenario Outline: The recorded faces of a catalog person that could not be united
    Given the projected faces of "<shape>" at the instant "<instant>"
    When they are united
    Then the union is one ring whose area equals the shapely and the polygon-clipping union within a ten-thousandth (the tolerance of the polygon-union vectors)

    Examples:
      | shape                                              | instant                    |
      | humans/pregnant-person 1:50 front shank-r (a corner arrives as two vertices 48 nanometres apart at the seam of the boundary) | clip waddle-walk at 0.133 s |
      | humans/pregnant-person 1:50 front shank-r          | clip waddle-walk at 0.289 s |
      | humans/tour-guide 1:50 hand-r swept along w, plan  | clip gesture-point at 7.992 s |

  Scenario Outline: A traced profile turned through the view direction
    Given the profile "<profile>" swept along its normal and tilted by <tilt> degrees
    When the prism is turned from <from> to <to> degrees past edge-on in steps of <step> degrees
    Then at every step the union is built and its area equals the shapely union within a ten-thousandth (the tolerance of the polygon-union vectors)

    Examples:
      | profile                              | tilt | from  | to   | step    |
      | hand of humans/tour-guide, 53 points | 0    | -1.5  | 1.5  | 0.0125  |
      | hand of humans/tour-guide, 53 points | 37   | -1.5  | 1.5  | 0.0125  |
      | shank of humans/pregnant-person      | 45   | -1.5  | 1.5  | 0.0125  |
      | shank of humans/pregnant-person      | 42.5 | -0.15 | 0.15 | 0.00125 |
