Feature: Every norm mutation is reachable through its schema aggregate
  Scenario: Validate document and diff schema requirements
    Given the artifact facets in the language-neutral mutation-aggregate fixture
    When each schema is checked against the JSON Schema metaschema
    Then required property names are unique
    And every closed record declares each property it requires

  Scenario Outline: Admit every native leaf exactly once
    Given the artifact in the language-neutral mutation-aggregate fixture
    When its aggregate schema is compared with all local leaf schemas
    Then every leaf schema id occurs exactly once
    And every external tag requires only its own payload
    And the tag is the title of its payload schema

    Examples:
      | artifact |
      | EN 1990  |
      | EN 1991  |
      | EN 1992  |
      | EN 1993  |
      | EN 1994  |
      | EN 1995  |
      | EN 1996  |
      | EN 1997  |
      | EN 1998  |
      | EN 1999  |
      | DIN 16798 |
      | DIN 18599 |
      | DIN 4108 |
      | ISO 16757 |
      | VDI 3805 |
