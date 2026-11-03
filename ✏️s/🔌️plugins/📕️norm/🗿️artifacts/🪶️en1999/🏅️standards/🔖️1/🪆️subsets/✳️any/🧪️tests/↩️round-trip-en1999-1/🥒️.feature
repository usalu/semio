@capability-en1999-1-round-trip
@oracle-en1999-1-python-independent
@comparison-ordered-json-v1
Feature: EN 1999 mutations are undone by their own computed inverse
  Representative committed vectors of the typed en1999 mutations (one change, one insert, one remove) are applied to their committed before-snapshot and then undone by the mutation's own computed inverse. Both implementations must restore the before-snapshot exactly; the exhaustive form of the same law over every kind is the `inverse-<kind>` scenarios of `🪶️mutate-en1999-1`.

  @id-mutation-round-trip
  @level-quick
  @mode-differential
  Scenario Outline: Undoing <id> restores the before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.before.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🔣️.mutation.json
    When the mutation is applied and then its computed inverse
    Then the document equals the before-snapshot
    Examples:
      | id | dir | fixture |
      | change-annex | 🌍️change-annex | ✅apply |
      | change-reliability-class | 🎯change-reliability-class | ✅apply |
      | remove-member | ➖remove-member | ✅apply |
