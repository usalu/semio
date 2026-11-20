# Implicit worksheet coordinates

Workbook projection and cell addressing read rows and cells in SpreadsheetML document order. Unqualified explicit references establish the row or column, and omitted references advance from the preceding SpreadsheetML element. Comments retain their physical XML indices and do not advance either coordinate.

Cell removal inserts a reference on the first following implicit cell so it keeps its coordinate. The inverse records the original physical child index and clears that reference when it restores the cell. A row stays in the worksheet when removing it would renumber a following implicit row. Undoing an insertion into a pre-existing empty row keeps that row. SetCell preserves the addressed cell's original reference form. The details pane can also resolve the coordinate of a verbatim cell inserted without a reference.

The language-neutral fixture lists seven edits and every expected worksheet cell after each edit. The Rust native regression exercises the retained editor reducer for SetCell and details-pane insertion, checks save and reopen against Calamine, round-trips text and binary inverses, and compares exact workbook snapshots after undo and redo.

The repository history schema audit passed. The TypeScript vacancy address parser passes strict ES2020 type checking. It accepted regular and restoration addresses, and rejected malformed placement fields. The XLSX base whitespace check and pinned Rust formatter syntax parse passed. The native regression build is in progress.
