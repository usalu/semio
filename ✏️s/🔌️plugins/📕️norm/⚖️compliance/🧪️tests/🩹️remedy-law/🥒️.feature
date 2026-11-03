@capability-norm-remedy-law
Feature: Every check that can fail or warn names what to change
  A failing check without a remedy is a dead end for the user: the checks window offers no chip and no one-click write. Every check construction site of the ten Eurocode artifacts (EN 1990 to EN 1999) and of the shared norm crates therefore attaches a remedy (dimension, material or arrangement to change, EN and DE), or names the post-pass function that attaches it (`remedies_attached_by`). The Python check `🐍️.py` scans the Rust sources statically and is independent of the Rust kernel; the Rust kernel enforces the same law at run time (`CheckBuilder::build` rejects a failing check without a remedy in every profile, unit tests in `../🔬️unit`).

  Scenario: Every failing or warning construction site carries a remedy
    Given the committed Rust sources of the norm plugin
    When every check construction chain is scanned
    Then no chain that can fail or warn lacks a remedy or a named post-pass

  Scenario: The scanner classifies synthetic sites correctly
    Given synthetic construction chains with and without remedies
    When the scanner classifies them
    Then every verdict matches the expected one

  Scenario: No typed remedy targets a text or enum field and no solver invents a value
    Given the committed Rust sources of the norm plugin and the snapshot schema of each Eurocode artifact
    When every typed remedy, every empty option list and every solver fallback is scanned
    Then no typed remedy names a text or enum field, every empty option list is an advisory and no fallback value is unreachable

  Scenario: Every remedy of every check of every palette document is effective
    Given the example documents, the fixtures and the full showcase of each Eurocode artifact
    When every remedy of every check is applied and the document is evaluated again
    Then the target exists and is numeric or a valid enum token, the document still decodes, the written value lies inside the field range and the utilisation or the severity of the check falls
