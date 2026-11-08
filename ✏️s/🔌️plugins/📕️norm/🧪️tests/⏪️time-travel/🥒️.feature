Feature: Every norm artifact is a visual, live, persisted editor
  Scenario Outline: The hosted s app opens each artifact and mounts its five visual windows
    Given the norm plugin is loaded in the hosted s app in locale <locale>
    When the artifact <artifact> is opened on its sample document
    Then its viewport, diagrams, properties, checks and outliner windows are mounted

    Examples:
      | artifact | locale |
      | en1990   | en     |
      | en1990   | de     |

  Scenario Outline: A pinned numeric edit changes the check result, undoes, redoes and persists
    Given the pinned sample of artifact <artifact> in locale <locale>
    And its artifact is attached to a fresh persisted local folder
    When the pinned numeric field commits its after value
    Then exactly one document history entry is added
    And the field and the pinned check follow the after state
    When the edit is undone and redone through the shell
    Then both the field and check follow the before and after states
    When the page reloads and the same local folder is reopened
    Then the edited field, result and document history entry are restored

    Examples:
      | artifact | locale |
      | en1990   | en     |
      | en1990   | de     |
