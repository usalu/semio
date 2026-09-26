Feature: DIN V 18599 balance oracle
  Scenario: Hand-derived H_V and H'T limit
    When the oracle self-checks run
    Then they pass
