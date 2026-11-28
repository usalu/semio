# Details Xml parts and bad-path refusal

## Defect 1: xmlParts children never render
Root cause: `default_open` in `detail-model/🦀️.rs` returned true for every collection, so first paint opened the whole recursive XML tree. `tree_window_indexed_rows` (window-kits/tree) prices each row with `surface_subtree_items` and ends a window on `cost > items` (measured: row 0 of the part window cost 921 > 812 remaining). The first part row was dropped, so `xmlParts` showed no children.
Fix: `default_open` opens top-level collections and direct items whose own fields are all scalars; items holding nested collections (XML documents) stay closed and open on host request (windowed). Generic, no XLSX code.
Test: `xml_parts_collection_renders_every_part_row_and_opens_each_node_tree_on_request` in details `🧱️lazy-and-bounded-laws` (XLSX-shaped Vec of parts with recursive element trees, 3x4 and 8x300). Details crate: 27 passed before the final edit round.

## Defect 2: bad path gives interactive-job.undecodable-fault
Root cause (probe printed the page text): the retained command job settled with plain text "retained command work refused the command before any capacity was measured" (`retained-command-job/🦀️.rs:571`), because `snapshot_edit_extent` returned None for an unaddressable path. Every static job fault (9 sites) was plain text, which the renderer cannot decode.
Fixes:
- `tool-job-factories/🦀️.rs` `snapshot_edit_extent`: an admitted payload with an unaddressable path still gets its capacity, so the reducer refuses with `snapshot-edit.index-out-of-bounds` etc.
- `retained-command/🎟️admission/🦀️.rs`: `static_fault!` canonical Fault wire; all 9 static faults in `retained-command-job` use it (codes `interactive-job.command-*`).
Tests: admission `every_static_refusal_is_a_canonical_fault_the_host_decodes_by_code` (2 passed, exit 0); editing `an_unaddressable_path_with_an_admitted_payload_is_left_to_the_typed_reducer_refusal`.
Wasm rebuild: required (stdio-office). Localized notice text for `snapshot-edit.*` codes: not verified by me; the renderer falls back to the generic localized "operation was refused (code)" notice, which now names the real code.
Debug instrumentation removed (tree window eprintln, TS console.warn).
