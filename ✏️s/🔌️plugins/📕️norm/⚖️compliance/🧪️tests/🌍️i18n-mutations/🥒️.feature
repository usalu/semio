@capability-norm-i18n-mutations
Feature: Mutation labels speak German in the German text
  The German label of every Eurocode mutation leaf is composed from the bilingual entity and field labels and holds no English word and no leaf or field identifier. The Python check `🐍️.py` reads the Rust sources as text and is independent of the Rust kernel.

  Scenario: No mutation label leaks English into German
    Given the committed Eurocode mutation leaves
    When every `LocalizedLabel::native` pair of a mutation leaf is scanned
    Then the German text contains no lowercase ASCII word equal to the leaf name, a field name or a listed English word
