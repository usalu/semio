@capability-norm-viewport-config
Feature: The viewport view is persisted per window and local-only
  Camera preset, hidden layers, colour map and unit system belong to the local user's window, not to the shared document.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Changing the view replaces all four members and is undoable: Given a viewport window with the iso camera, hidden loads, the viridis colour map and SI units When the view is changed to the top camera with nothing hidden Then the config holds exactly the new view and the inverse restores the previous one
  - An unchanged view is a no-op: Given a viewport window config When the same view is applied again Then the mutation reports a no-op warning and the config is unchanged
  - The wire forms round-trip: Given the mutation "change-viewport-view" Then its text and binary forms decode to the same mutation And unknown operations and truncated binary frames are refused
