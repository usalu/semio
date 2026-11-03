@oracle-en1993-1-3-forming-gain-independent
@comparison-numerical-v1
Feature: EN 1993-1-3 3.2.2 enhanced yield strength of cold forming
  f_ya = f_yb + (f_u - f_yb) k n t^2 / A_g, limited to (f_u + f_yb) / 2.

  Scenario Outline: average yield strength
    Given f_yb 350 MPa, f_u 420 MPa, t 3 mm, A_g 600 mm2 and <process> with <n> bends
    Then f_ya equals <fya> Pa

    Examples:
      | process      | n  | fya       |
      | roll forming | 4  | 379400000 |
      | other        | 4  | 371000000 |
      | roll forming | 40 | 385000000 |
      | roll forming | 0  | 350000000 |
