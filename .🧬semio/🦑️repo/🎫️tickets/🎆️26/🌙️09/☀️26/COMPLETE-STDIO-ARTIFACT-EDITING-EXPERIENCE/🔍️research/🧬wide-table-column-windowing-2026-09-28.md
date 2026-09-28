# Wide Table Column Windowing

The current `TableWindowKit::render_indexed_rows_with_id` windows rows through `TreeWindows` but inserts every column into `UiFixedList<Label>` before constructing the table. CSV/TSV and the new WAV natural editor build every visible row's cells across all columns. A large column count therefore fails admission for the whole main surface instead of exposing a bounded editable slice.

Current shared schema `TableProps` has `label`, `columns`, `actions_label`, and one row `window`. The corresponding builder, typed property visitor, scene projection, and React/WGPU renderer transports require one coherent schema-first update for a second axis. Root has not changed them in this checkpoint.

Required behavior: independent bounded column and row windows; logical column indices in action arguments even when a slice starts after zero; accessible column navigation with English/German labels; no full-width temporary header/cell allocations; exact snapshot revision on every cell action; selected-cell/keyboard handling that remains correct across both window changes. Shared toolbar labels must be supplied by the artifact domain so WAV says frame/channel while CSV says row/column.

Acceptance should include columns beyond the fixed-list capacity, tiny row/column slices, nonzero offsets, insertion/removal at both slice boundaries, stale revisions, keyboard navigation, and the same independent matrix oracle in native and TypeScript fixtures. React and WGPU must receive the same canonical transport. A single small table or rows-only virtualization law does not cover this requirement.

Text execution will own the common table slice after its Office schema audit corrections. WAV may use a bounded selected-channel view while that work is in flight; no broad shared change is currently assigned to the media worker.
