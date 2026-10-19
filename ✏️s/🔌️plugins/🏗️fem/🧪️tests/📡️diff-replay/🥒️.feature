Feature: FEM Diff Replay and Collaboration Isolation
  Scenario Outline: Whole-row identity changes are refused by the shared central applier
    Given a FEM <dimension> document with stable node and load identities
    When a node diff renames a row or duplicates another node identity
    Then the complete diff is refused with mutation.apply.identity-change
    And the original document remains unchanged
    When a nested load diff changes a load identity after an earlier node change
    Then the complete diff is refused and the earlier node change is not published

    Examples:
      | dimension |
      | 2D        |
      | 3D        |

  Scenario Outline: Independent collaborator edits survive inverse replay
    Given a FEM <dimension> document with two nodes and two loads in one load case
    When one author changes the first node and modal count
    And another author changes the second node and buckling count
    And the first author's exact inverse diff is replayed
    Then the second author's node and buckling count remain changed
    When the authors independently change different loads of one load case
    And the first author's exact inverse diff is replayed
    Then the second author's load remains changed

    Examples:
      | dimension |
      | 2D        |
      | 3D        |
