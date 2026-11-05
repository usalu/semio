Feature: Typed edit boundary capabilities
  Scenario: Structural actions retain their own exact bindings
    Then insertion and removal use collection rules and explicit identity-removal paths
    And setter ancestry grants no structural capability
  Scenario: Typed setters retain concrete selectors
    Then the nearest value boundary wins and exact rules take precedence
    And malformed paths and textual prefix collisions refuse
