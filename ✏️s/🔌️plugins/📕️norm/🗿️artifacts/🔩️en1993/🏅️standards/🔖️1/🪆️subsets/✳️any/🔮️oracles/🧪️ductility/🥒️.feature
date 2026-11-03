@oracle-en1993-1-1-ductility-independent
@comparison-numerical-v1
Feature: EN 1993-1-1 3.2.2 ductility and the plastic analysis gate
  Requirement over value of f_u/f_y, the elongation at failure and eps_u; the largest ratio is the utilisation.

  Scenario Outline: ductility gate
    Given a steel with f_y <fy> MPa, f_u <fu> MPa, elongation <a> and eps_u <eu>
    Then the governing term is <term> with the utilisation <u>

    Examples:
      | fy  | fu  | a    | eu   | term       | u                  |
      | 355 | 510 | 0.20 | 0.12 | fu_fy      | 0.7656862745098041 |
      | 460 | 540 | 0.17 | 0.05 | fu_fy      | 0.9370370370370371 |
      | 355 | 510 | 0.20 | 0.02 | eps_u      | 1.2678571428571428 |
      | 355 | 510 | 0    | 0    | fu_fy      | 0.7656862745098041 |
