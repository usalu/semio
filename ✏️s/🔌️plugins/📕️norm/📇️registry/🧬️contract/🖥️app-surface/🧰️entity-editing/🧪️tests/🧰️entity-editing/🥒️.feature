@capability-entity-editing
@oracle-framework-value-path-reducer
Feature: Every norm editor copies, duplicates, bulk edits and snaps through the same shared events
  Editing capabilities are pure events over the document value; the oracle is the framework's own value-path reducer.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Duplicate: Given rows b1, b2 and b3 where b3 refers to b2 When b2 and b3 are duplicated with a 0.5 m offset Then the copies b2-copy and b3-copy follow b3 in order, b3-copy refers to b2-copy, and the whole paste is one emit and so one undo step
  - Fresh ids: Given a document that already contains b1-copy When b1 is pasted again Then the copy is b1-copy-2 and a copy of a copy keeps the stem of the original
  - Bulk edit: Given a multi-selection with different values in one field Then the properties view shows mixed, one write per selected row sets them all, and an unknown id fails the whole bundle
  - Snapping: Given member ends and midpoints of the scene When a translating handle is dragged within its tolerance of one of them Then it lands exactly on it, and Ctrl frees the drag
  - Typed entry: Given a handle bound to a field in metres When 35 cm or 1,5m is typed during the drag Then the parameter is the value in the field unit, and a unit of another dimension is rejected
  - Shortcuts: Given the shortcut overlay Then copy, paste, duplicate and select all are listed with English and German labels
  - Select all: Given a viewport window with a scene and an outliner filter When Ctrl+A is pressed Then the interaction selection of the artifact domain is replaced by the entities the filter admits (all entities without a filter), it is ephemeral view state without a document edit or an undo step, and a window without domain or an empty scene selects nothing
