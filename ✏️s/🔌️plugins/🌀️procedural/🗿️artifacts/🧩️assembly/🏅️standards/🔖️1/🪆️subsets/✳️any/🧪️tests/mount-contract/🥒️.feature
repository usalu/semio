Feature: Procedural assembly is a real mounted app
  Scenario: the plugin declares editor and viewer apps
    Given the procedural plugin manifest
    Then an app "s.assembly@1/*#editor" exists
    And an app "s.assembly@1/*#viewer" exists
    And the editor window kind "framework.window.tree" is labeled "Structure" / "Struktur"

  Scenario: bundled examples are real documents
    Given the two-room-corridor and wall-roof-facade-strip examples
    Then each example DSL is non-empty
    And each example pack envelope is non-empty
    And labels exist in English and German
