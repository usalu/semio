@oracle-en1993-1-1-effective-length-independent
@comparison-numerical-v1
Feature: EN 1993-1-1 6.3.1.3 buckling length factor of a column in a frame
  The textbook nomogram equations of Jackson and Moreland are solved with scipy brentq and agree with the pole-free forms solved by the implementation.

  Scenario Outline: effective length factor K
    Given the distribution factors G_A <ga> and G_B <gb>
    When K is solved for the <mode> frame
    Then K equals <k> within 1e-9

    Examples:
      | mode     | ga   | gb   | k                  |
      | non-sway | 1.0  | 1.0  | 0.7742650686480754 |
      | non-sway | 0.5  | 2.0  | 0.7646542638997111 |
      | non-sway | 10.0 | 10.0 | 0.9625009769338223 |
      | non-sway | 0.1  | 0.1  | 0.5486975576213342 |
      | non-sway | 2.0  | 5.0  | 0.891362426734382  |
      | sway     | 1.0  | 1.0  | 1.3172751026289118 |
      | sway     | 0.5  | 2.0  | 1.3667664809123463 |
      | sway     | 10.0 | 10.0 | 3.010392659171542  |
      | sway     | 0.1  | 0.1  | 1.0333048474667663 |
      | sway     | 2.0  | 5.0  | 1.86959151611498   |

  Scenario: limits of the nomogram
    Given a column fixed at both ends
    Then K is 0.5 in the non-sway frame and 1.0 in the sway frame
    And a column fixed at one end and pinned at the other has K 0.6991556596428412 in the non-sway frame and 2.0 as a cantilever in the sway frame
