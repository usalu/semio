@capability-shortcut-dispatch
@oracle-handle-drag-tool-key-vocabulary
Feature: Keyboard shortcuts drive handles through the same statechart as the pointer
  Scoped chords become handle keys or host actions; a nudge by keyboard is a nudge of the drag tool.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Nudge chords agree with the drag tool: Given the twelve nudge chords of the handle scope When each is dispatched and read back through the drag tool's key vocabulary Then its direction and its modifier scale (fine 0.1, normal 1, large 10) equal the shortcut's
  - Scopes decide the meaning of a key: Given the same arrow key in the viewport scope and the handle scope Then it orbits in the viewport and nudges a focused handle, Tab walks the handles only with a handle focused, and undo is reachable in every scope
