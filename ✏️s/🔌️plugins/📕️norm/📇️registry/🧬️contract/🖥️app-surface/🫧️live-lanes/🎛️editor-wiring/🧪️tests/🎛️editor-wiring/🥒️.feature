@capability-direct-manipulation
@oracle-numpy-snap
Feature: Editor gestures preview in the transient lane and commit as one setFields
  Executed by `🔬️unit/🦀️.rs`; the snapped values are the numpy-validated snap rules of `🕹️direct-manipulation`.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - A drag previews and the release commits one bundle: Given a length handle with 50 mm increments When a stream phase at 4.27 m and a commit phase at 4.27 m arrive Then the transient previews members[id=b1].length = 4.25 and the commit carries exactly that one write
  - Escape or a lost capture leaves no trace: Given a drag in progress When an abort phase or the Escape key arrives Then the transient holds no drag and no commit is raised
  - A keyboard nudge equals the equivalent drag: Given a focused handle at 4 m When ArrowRight and Enter arrive Then the commit equals the commit of a drag released at the nudged value
  - Typed entry overrides the value: Given a focused handle When the text 7,3 is entered and Enter arrives Then the commit writes 7.3
  - A placement drag of an entity without a row commits one insert: Given an entity standing in its derived layout pose without a placements row When a stream phase and a commit phase on its move grip arrive Then the preview document already carries the complete row, the commit is exactly one insertItem with the standing pose and the dragged leaf, and replaying it on the document gives the previewed pose
