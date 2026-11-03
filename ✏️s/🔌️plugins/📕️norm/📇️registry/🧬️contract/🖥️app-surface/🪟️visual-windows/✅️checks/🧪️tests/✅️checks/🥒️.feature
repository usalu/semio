@capability-checks-window
@oracle-serde-json-and-std-sort
Feature: Checks window shows every compliance check accessibly and in sync with the model
  Executed by `🦀️.rs` beside this file. Independent oracles: `serde_json` parses the projected tree, `std` sorting confirms the order.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Redundant status coding: Given a report with a passing, a warning, a failing and a not-applicable check When the window renders in English and in German Then every row carries a status glyph, a localized status word and a patterned utilisation bar with a tick at 100 percent
  - Group, sort and filter: Given checks on several entities, parts and clauses When the view state groups by entity, part or clause, sorts worst-first, by clause or by title and filters by status or text Then the plan equals the std sort of the same data and clause 6.2.10 follows 6.2.9
  - Selection sync: Given an editor controller and an interaction domain When a check row is activated Then the selected-check action fires and the check selects its entity id and field path
  - Remedies: Given a failing check with an applicable remedy and a choice remedy When the window expands the row Then one chip per remedy or option is shown with its change, and actions are bound only for an editor
  - Locale-aware quantities: Given a length of 12.3 millimetres When rendered in German Then the decimal separator is a comma
  - Shared row component: Given the inspection panel When it renders a check Then it uses the same detail rows as the window
