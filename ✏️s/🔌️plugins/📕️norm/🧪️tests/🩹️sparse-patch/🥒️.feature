Feature: Sparse native row patches
  Scenario: Edit one typed field without replacing the row
    Given a native patch whose fields have explicit defaults
    When a payload contains only its edited field
    Then its schema and wire reader admit it
    And the payload preserves its original fields and values
    And an unrelated enum value is refused
