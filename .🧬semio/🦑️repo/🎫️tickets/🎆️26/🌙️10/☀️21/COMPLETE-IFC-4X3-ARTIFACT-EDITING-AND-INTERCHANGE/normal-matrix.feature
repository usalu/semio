Feature: Finite Normal Covector Direction
  Scenario: Moderate finite transforms
    Given identity rotation reflection nonuniform scale and shear inputs
    Then the owned inverse-transpose direction agrees with independent Three and high precision
  Scenario: Extreme finite columns
    Given an extreme finite binary64 input matrix and direction
    Then common-scale exact cofactors preserve relative component magnitude
    And the result matches the high-precision oracle
  Scenario: Non-direction or singular transform
    Given a zero normal or singular matrix
    Then no nonzero direction is fabricated
