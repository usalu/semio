@capability-norm-design-case-edits @comparison-exact
Feature: Design cases are edited like any other entity of the document
  The catalogue browser adds a case seeded from its entry, the properties window sets, retypes and clears one input in its stored unit, and the case list duplicates and removes. Every action is a value-tree edit the artifact resolves to its own `insert-case`, `change-case-number`, `change-case-text` or `remove-case` mutation; the live session re-evaluates the case after each. Executed by `🔬️unit/🦀️.rs` (edit values) and, per artifact, by `assert_case_edit_paths` in the editor tests (mutations applied, live report equal to the full evaluation).

  Scenarios, asserted by the Rust tests and not by the platform adapters:
  - Adding a case from the catalogue: Given the catalogue entry "<entry>" When the browser adds a case Then an insertItem edit on "cases" carries a new case of that entry with the example inputs and a fresh id
  - Setting a number: Given a case with the width 200 mm When the width is typed as "0,25 m" Then the setField edit on cases[id=ID].numbers[id=b].value carries 250
  - Rejecting a number: Given a width with the range 1 to 5000 mm When "0" or "wide" is typed Then no edit is raised and the message names the range or the missing number in the language of the window
  - Setting a choice: Given a class input with the options "C24" and "GL24h" When "GL24h" is chosen Then the setField edit on cases[id=ID].texts[id=class].value carries "GL24h" and any other text is refused
  - Clearing an input: Given a case with the width 200 mm When the width field is emptied Then the setField edit carries null and the artifact removes the number row of the width
  - Duplicating a case: Given a case "c1" When it is duplicated Then an insertItem edit right after it carries the same entry and inputs under the id "c1-2" with the labels marked as copy
  - Removing a case: Given the cases "c1" and "c2" When "c1" is removed Then a removeItem edit with index 0 is raised and index 2 is refused
  - Empty inputs are listed for typing: Given a case whose entry declares the inputs b, angle and class When the properties window opens Then the unset declared inputs appear as empty rows and the document is unchanged
  - A remedy writes the stored unit: Given a width of 9000 mm above the bound 5000 mm When the remedy is applied Then the width becomes 5000 and the case no longer fails
  - The live report is exact: Given any sequence of the actions above When each mutation is applied Then the live report equals the full evaluation of the document
