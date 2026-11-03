@capability-norm-design-case @oracle-python @comparison-exact
Feature: Catalogue entries become editable, evaluated design cases
  Every catalogue entry of an artifact is addressable from a document: a design case names the entry, stores typed inputs (stored unit, EN and DE label, bounds, choices) and is evaluated into checks of the document report. Executed by `🔬️unit/🦀️.rs` and the independent oracle `🔮️oracles/🐍️.py`, which share the fixture `🧫️fixtures/🔣️.json`.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Searching the catalogue for "<query>": Given the catalogue entries of the fixture When the catalogue is searched for "<query>" Then the matching ids are "<ids>" And the oracle search returns the same ids
  - A new case carries the example inputs of the declared inputs: Given an entry whose width and depth declare examples When a case is seeded Then the numbers and choices with examples are present and the others are absent
  - Invalid inputs fail with a remedy: Given a case with a choice outside its options and a number outside its bounds When the case is evaluated Then each issue is a failing check with a one-of or a bound remedy on the input path
  - A failing case sizes the first lever input: Given a failing case with utilisation 1.3 and a lever width of 200 mm When the case is evaluated Then the remedy asks for at least 260 mm on the width input
  - A case is evaluated into checks addressed to its row: Given a passing case When the case is evaluated Then every check carries the clause of the entry, the case label in both languages and the subject path cases[id=ID]
  - The artifact vocabulary is consistent: Given the typed input vocabulary of an artifact Then rows are unique per key and scope, labelled in both languages, units are derivable by the unit oracle and choices are unique
