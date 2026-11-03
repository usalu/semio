@capability-norm-outliner-selection-sync
Feature: The outliner, the checks window and the 3D viewport share one selection
  A row activation writes the artifact's interaction domain; the selection mirror resolves the scoped row address to the entity,
  its governing check and its field paths, so every window shows the same thing.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Rows are pick targets with collection-scoped addresses: Given the collections "members" and "loads" that both contain an entity "b1" When the outliner renders with an interaction domain Then each row id is the scoped address "collection/id" and carries the granularity "entity" And the two addresses differ
  - Activating a row selects the entity everywhere: Given the outliner row "members/b1" When the row is activated Then the entity "b1" is highlighted in the viewport, its governing check is selected in the checks window and the diagrams focus it
  - Without an interaction domain rows keep path ids: Given an outliner state without an interaction domain When the outliner renders Then each row id is "norm-outliner.<path>" and no row carries a granularity
