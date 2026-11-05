Feature: Consistent applied mutation witnesses
  Scenario: A typed mutation changes its declared row
    Given a committed applied-operation case
    When the mutation is validated independently against its schema
    Then its target exists in the before and after snapshots
    And its new value differs from the previous value
    And the after snapshot and diff contain the same new value
    And the generated aggregate reader preserves the mutation
