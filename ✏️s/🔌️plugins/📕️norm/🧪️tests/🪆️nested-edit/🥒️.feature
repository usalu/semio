Feature: Nested Norm Input Mutations
  Scenario: A child input uses the nearest typed semantic value boundary
    Given a norm document and an existing typed parent replacement rule
    And the input is visible in the English or German inspector
    When a user changes an existing nested scalar through its value path
    Then the exact scalar rule takes precedence when it exists
    And otherwise the nearest enclosing replacement rule emits its concrete mutation
    And the mutation preserves every sibling value and the parent's selectors
    And selectors within the replaced value remain within that value
    And the original document is unchanged while resolving the command
    And the payload equals the independent JSON Pointer oracle
    And an unknown field or entity is refused

  Scenario: Several child inputs change in one interaction
    Given two distinct fields inside the same typed replacement boundary
    When one interaction submits both field writes
    Then both changes survive in one atomic mutation bundle
    And a later refusal commits none of the bundle
