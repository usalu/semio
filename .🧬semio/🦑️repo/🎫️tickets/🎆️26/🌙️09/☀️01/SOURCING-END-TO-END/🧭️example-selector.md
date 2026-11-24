# Example selector and duplicate key (journey 2026-11-24, step 16 and console error)

## Defect 1: no empty example in the navbar selector

Root causes
- `ShellHost/🔎️command-bar/🔶️scope/🟦️.tsx` never passed `includeNoExample` to `NavbarExampleSelect`, so no app offered "No example".
- The canonical empty id is already `""` on the wire: Rust `EMPTY_EXAMPLE_ID = ""` (sourcing `editor_vocabulary`), the puzzle handlers match `""`. The selector's `__none__` is only the Radix Select item value (an item cannot be `""`); it is normalised back to `""`. There is no schema-generated constant (the id is a Rust const), so the TS mirror is `EMPTY_EXAMPLE_ID` in `ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts`.
- The shell treated `""` as "load the default document": `exampleSelectionAfterInputV1` published the boot example (Demo) as the visible id and remembered the previous one, and `rememberedExampleIdFromDispatchV1` ignored `""`. Offering the row alone would have snapped the picker back to Demo. Redo/undo recovery had the same blind spot (`"" || undefined`).

Fix
- `includeNoExample` passed in the shared scope (every app); label from `ui.common.noExample` (en "No example", de "Kein Beispiel").
- `EMPTY_EXAMPLE_ID`, `requestedExampleIdV1`; empty id is explicit memory and visible id; `buildActiveExampleAction` passes the id through unchanged; `navbarExampleIdFromHistoryUpserts` takes `string | undefined` (undefined = nothing remembered), so a redone empty switch relabels the picker.
- `shell-core-state` passes the remembered id without the `""` fallback.

Other apps using this scope: all editors with a registered catalogue. Guest handling of `""` verified by reading each `set-active-example`: this was WRONG: trinity rewriting, jack, remodeling, raster and block 2d/3d/5d were inert (`Emit::default()`) and note faulted; fem 3d falls to its default snapshot (fine). All fixed below, see "Empty example in every app". Wasm-side follow-up (not done): trinity `rewriting` and `jack` `set-active-example` should treat `""` as their blank document, and the wgpu shell picker (Rust) needs the same "No example" row.

## Defect 2: "Encountered two children with the same key `beam-kvh-c24`"

Not the Curated list. It fires when a curated count exceeds 1 (set count, undo/redo), from `World3dHost/🧱️instance-layers` (`key={instance.id}`) of the Grid window. Root cause: the sourcing Grid (`🪟️windows/🔢️grid/🦀️.rs` `push_kind_copies`) emits `count` copies of a kind in line-behind mode, all with instance id = `kind.id`.

Fix (Rust, needs wasm rebuild, not built): `kind_copy_instances_json(kind, copy, ...)` in `🧬️schema/🧾️domain/🦀️.rs`; copy 0 keeps the kind id, later copies get `~{copy}`; `kind_instances_json` delegates with copy 0 (Preview unchanged). Grid calls the copy variant. Rust test `grid_instance_ids_stay_unique_across_curated_copies` added (not run: no cargo build).

## Tests
- `bun test` over `example-offers-and-switching` (8 pass, including the new empty-document round trip and a lock that the scope passes `includeNoExample`); settlement fixture row "default" became "empty document".

## Empty example in every app

Every `set-active-example` handler, behaviour for the canonical empty id `""` (before the fix):
- Empty or default document already: drawing, fem 2d, fem 3d (default snapshot), flow, procedural 2d and 3d (editor and viewer), forms, writer, playbook, dag, procedure, wires, playground, equation, presentation (animate), wfc bitmap (boot example), puzzle 2d (`_` empty), puzzle 3d, sourcing curation (`EMPTY_EXAMPLE_ID`).
- Inert (`Emit::default()`): trinity `rewriting`, trinity `jack`, remodeling, raster, block 2d, block 3d, block 5d, hub space (navigation, no document; left alone).
- Fault: note (`note.example.unknown`).

Fixes (each loads the app's own empty document for `""`)
- rewriting: `""` joins the `default`/`blank` arm (`default_rule_state()`).
- jack: `""` resolves to `C::initial_viewer_snapshot()` (the composition's empty graph) with the default query.
- remodeling: `RemodelingSnapshot::default()`; raster: `empty_raster_document()`; block 2d/3d/5d: `BlockNdSnapshot::default()`; note: `empty_note_snapshot()`.

Laws: one per fixed plugin (`set_active_example_empty_id_*` / `*empty_document*` in each crate's unit tests).
