@capability-direct-manipulation
@oracle-numpy-snap
@comparison-snapped-value-within-1e-12
Feature: Handle drags preview in the transient lane and commit once
  Executed by `🔬️unit/🦀️.rs`; the snap oracle is `🔮️oracles/🐍️.py` (numpy).

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - A pointer drag previews and a release commits one write bundle: Given a length handle with 50 mm increments limited to 0.5 m to 12 m When the pointer presses at 4 m, moves to 4.27 m and releases Then the preview carries "members[id=b1].length" = 4.25 And the release commits exactly one bundle with 4.25
  - Escape aborts without trace: Given a drag in progress When Escape is pressed Then the gesture returns to hover at the origin and the effect is cleared
  - A drag released at its origin commits nothing: Given a pressed handle When it is released without moving Then no commit is raised
  - The keyboard reaches every gesture: Given a focused handle When Tab, an arrow, typed digits, Enter and Escape are pressed Then they cycle, nudge by the increment (Shift x10, Alt x0.1), override the value, commit and abort
  - Typed entry overrides the drag value: Given a dragging handle When "4,237" is typed Then the preview carries the exact value 4.237 within the limits
  - Snapping equals the numpy oracle: Given a table of raw values, targets and increments When Rust and numpy snap them Then both agree within 1e-12
  - Picks name sub-parts and grips win: Given the pick "b1#face:3" and a hit grip When the hit is resolved Then the grip wins and the pick round-trips through its wire form
  - A nudge moves the bound field by exactly its schema step: Given a handle bound to a field quantised to 0.01 m When an arrow key is pressed Then the field changes by 0.01 m And Shift changes it by 0.1 m And Alt by 0.001 m
  - Enter opens an inline value field: Given a focused numeric handle resting at 4.25 m When Enter is pressed Then a field pre-filled with 4.25 opens And typing 4.7 and pressing Enter commits one bundle with 4.7 And Escape instead closes it without a trace And a token handle opens no field
  - A diagram marker is a grip: Given a marker bound to the field "placements[id=p1].x" When an arrow key is pressed or "350 cm" is typed into its table row Then the same single commit as a pointer drag is produced within the limits of the axis
