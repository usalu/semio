@capability-view-steps
@oracle-hand-derived-orders
Feature: Stepping shortcuts change the ephemeral view lane and never the document
  Executed by `🧪️tests/🦀️.rs`; the walking orders are derived by hand from the example report (five checks) and the toolbar help is compared with the shortcut table.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Next check: Given the example report and no selected check When "]" is pressed five times and once more Then the view lane selects the checks 1, 4, 2, 0, 3 and wraps to 1
  - Previous check: Given check 1 is selected When "[" is pressed Then the last check of the list order is selected
  - Governing check: Given the example report When Alt+G is pressed Then the checks are selected from the highest utilisation down and the not applicable check comes last, Alt+Shift+G walks backwards
  - Combination: Given a scene with a combination track of three stops When "." is pressed Then the lane track, value and item index follow the next stop and wrap, "," walks backwards, and a scene without the track changes nothing
  - Framing: Given a lane check When the viewport is lowered Then the camera frames the selected entities
  - Help: Given the viewport panel When the keyboard help is read in English and German Then every shortcut of the table appears with its label and chord
