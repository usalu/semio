Feature: EN 1990 engineering results follow persisted edits
  Scenario Outline: Edit the bridge deck acceleration through the actual UI
    Given the road-bridge-compliant example in locale <locale>
    And its artifact is attached to a fresh persisted local folder
    When the bridge SLS row sls-D1 is selected
    Then the deck acceleration is 0.2 metres per second squared
    And check en1990.a2.acceleration.deck-D1.sls-D1 has utilization 0.4
    When its numeric control commits the value 0.4
    Then exactly one document history entry is added
    And the field is 0.4 and the same check has utilization 0.8
    When the edit is undone and redone through the shell
    Then both the field and check follow the before and after states
    When the page reloads and the same local folder is reopened
    Then the edited field, result and document history entry are restored

    Examples:
      | locale |
      | en     |
      | de     |
