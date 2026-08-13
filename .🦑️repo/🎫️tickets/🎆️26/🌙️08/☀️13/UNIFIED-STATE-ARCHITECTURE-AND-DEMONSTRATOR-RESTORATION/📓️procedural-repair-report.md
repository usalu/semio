# Procedural + demonstrator crate repair — 105 → 0 errors

Required because the user ruled out running on stale code: a fresh WASM build needs every pane
plugin to compile, and `semio-s-plugin-procedural` had not compiled all day.

## What was actually wrong

A peer session's semantic-mutation migration had landed partially. None of the defects were design
questions — every one had exactly one correct answer, which is why I finished rather than deferred.

| # | Defect | Fix |
|---|---|---|
| 1 | **Value-namespace collision (procedural2d).** glue does `pub use component::*`, so the file's own re-exported builder *functions* land back in `super`; `use super::{change_schema, …}` then imported both the module and the same-named function → E0252 ×7. | Dropped the import, qualified all 28 references with `super::`. |
| 2 | **Stale slot names (procedural3d).** Eight `use super::remove_*/set_*` named modules glue no longer declares; glue declares the semantic names the file already referenced everywhere else. | Rewired the 8 imports (old names existed nowhere else but one stale comment). |
| 3 | **Missing path segment (procedural2d).** Six builders imported as `<slug>::<fn>`; they live at `<slug>::mutation::<fn>`. | Corrected the paths. |
| 4 | **`payload` used inside `&self` methods** — 15 sites across 6 triad leaves (`label`/`target` bodies written against a free-function signature). | `payload.` → `self.` |
| 5 | **`widget_index`/`synapse_index` imported from `procedural2d`** — they live in `procedural2d::mutations`. | Split the imports across 12 leaves. |
| 6 | **`SynapseSpec` never imported.** | `use flow::SynapseSpec`. |
| 7 | **Missing builder/helper imports** in 5 leaves; two generation inverses used `<slug>::<fn>` again. | Added imports, corrected paths. |
| 8 | **`Mutation::Generation(GenerationMutation)` deleted** but still called from 3 app command sites. procedural3d had a replacement bridge (`generation_mutation_to_procedural3d`); **procedural2d had none**. | Wrote `generation_mutation_to_procedural2d` mirroring 3d exactly (the two facets' payloads differ only in field naming: `name`/`value` vs `new_name`/`new_value`), routed all 3 call sites through the bridges, exported it via `mutations::text` like its twin. |
| 9 | **`Mutation::SetWidget { index, widget }` deleted** but still called from 3 sites. | Migrated to the semantic variants — 2d → `replace_widget(widget)`, 3d → `UpdateWidget { widget }`. Both address by the widget's own stable id, so the positional `index` is dropped; that is exactly the point of the migration, and each call site had *already located the widget by id*, using the index only incidentally. |

## Demonstrator crate (14 → 0)

- Same `super::` namespace fix for `🎪️playground`'s `change_schema` triad mount — with the extra
  wrinkle that references inside `mod tests` need `super::super::` (one more hop).
- **`CsvSnapshot` migrated under it.** stdio's shape changed `{headers, rows}` → `{has_header,
  records}`. The playground's CSV serializer/deserializer were vestigial scaffolding that probed the
  JSON for `"headers"`/`"rows"` keys a `PlaygroundSnapshot` never had, fell through to
  `unwrap_or_default()`, and so silently emitted an **empty** table — the schema was lost on every
  CSV round trip. Rewritten as a real single-column table (header record + data record) that
  actually carries `schema` both ways. `CsvField`/`CsvRecord` added to stdio's `artifacts::csv`
  re-export.

## A real bug the migrated bundle test caught

`every_pane_declares_a_document_schema` — one of the three tests moved into `🎛️apps` during the pane
dissolution, and never run-verified until now — **failed**: `puzzle3d-play declares no document
schema`.

Cause: `Puzzle3dPlayApp::io()` declared a proper `AppIo` for the runtime, but
`create_puzzle3d_app()` never called `.io(..)` on the `AppBuilder` — and the builder is what puts
`document_schema` into the published `AppDefinition`. So the manifest's `io` was empty and a host
reading the manifest could not route a document to that surface. Every other pane calls `.io(..)`.

Fixed by extracting the definition into a free `puzzle3d_io()` used by **both** the trait method and
the builder, so the two cannot drift again.

`semio-s-plugin-demonstrator`: **19 passed / 0 failed**, including all three migrated bundle tests.

## Known-failing, not mine

`semio-s-plugin-procedural`: **503 passed / 3 failed** —
`assembly::…::empty_assembly_solves_trivially_with_no_assignments` (solver returns `Unsolved` for an
empty spec) and `procedural2d`/`procedural3d` `inference_default_law`. All three are in `💡️inferences`
facets I never touched. They were **unrunnable** before today because the crate did not compile, so
they are pre-existing defects in the peer's in-flight work now made visible — not regressions.
