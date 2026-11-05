Feature: Complete Native Inspector Payloads
  Scenario: Selecting a single input supplies its declared operation arguments
    Given the canonical native mutation payload schema
    When a path selects an axis-specific input
    Then the payload includes the declared z axis
    And all other axes remain unchanged

  Scenario: Editing bolt rows preserves the other count
    Given a connection with three bolts per row
    When its row count changes from two to four
    Then native admission receives both required counts
    And the number of bolts per row remains three

  Scenario: A required sibling is absent
    When the draft addresses a row without the retained sibling
    Then payload preparation refuses the edit

  Scenario: Alle fünfzehn Normen verwenden vollständige Argumente
    Given die kanonischen Payload-Schemata
    When alle deklarierten Bearbeitungsregeln geprüft werden
    Then fehlen keine erforderlichen Argumente
    And unbekannte und doppelte Argumente werden zurückgewiesen
