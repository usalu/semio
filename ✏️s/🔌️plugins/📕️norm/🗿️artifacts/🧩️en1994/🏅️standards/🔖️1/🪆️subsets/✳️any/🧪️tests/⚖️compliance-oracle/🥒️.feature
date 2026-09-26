Feature: EN 1994 compliance oracle
  Scenario: worked examples
    When the python oracle runs self-check
    Then key formulas match the hand derivation
