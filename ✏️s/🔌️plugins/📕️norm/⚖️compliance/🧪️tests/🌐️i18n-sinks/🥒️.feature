@capability-norm-i18n-sinks
Feature: No user-visible sink of the norm plugin is English only
  Spec-trace descriptions, gate reasons, outcome steps, mechanics errors, clause and NDP locators carry both languages, and no enum Lang, Locale or Language has a default variant. The Python check `🐍️.py` reads the sources as text and is independent of the Rust kernel.

  Scenario: Every sink is bilingual
    Given the committed norm artifacts and shared crates
    When every spec-trace row, gate reason, outcome steps text, mechanics error, clause label and NDP locator is scanned
    Then each one has an English and a German text, or a designator rendered per locale
