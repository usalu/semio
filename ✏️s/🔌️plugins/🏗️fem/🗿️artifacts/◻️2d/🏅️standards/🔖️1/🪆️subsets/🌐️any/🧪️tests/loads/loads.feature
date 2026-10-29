Feature: Canonical original-owned FEM load records
  Scenario: Scoped identity remains source-owned
    Given actual named load cases and their original nested loads
    When the canonical identity index admits them
    Then same names across cases stay distinct and duplicates within a case refuse
  Scenario: Native source fields survive actual Pack receiving
    Given nodal or member UDL fields and exact binary64 words
    When the original native receiver constructs the persistent snapshot
    Then cases, variants, targets, selected DOFs, self-weight and signed zero remain exact
    And every partial row or refused prefix stays in the original paid recipient
