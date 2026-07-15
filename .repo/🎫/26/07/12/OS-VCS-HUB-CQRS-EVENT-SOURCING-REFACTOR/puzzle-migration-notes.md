# Puzzle family migration (WS-F Wave 2a)

## Domain findings

- Plugin is ONE crate `puzzle-plugin` with 3 modules d2/d3/d5 in `puzzle/plugin/rs/lib.rs` (10.3k lines).
- Documents:
  - d2: untyped `fixture: serde_json::Value` (keys: schema, camera{x,y,zoom}, nodes[], edges[], wires[], meta.kindCatalogs). Stateful `BoardHost` engine mirror.
  - d3: typed `Puzzle3dFixture` (camera, meta, objects[] w/ nested vortices[], attractions[], targetVolumes[], references[]). `Puzzle3dPrecomputeSession` engine.
  - d5: typed `Puzzle5dDocument` (parts[] w/ grips). `Puzzle5dPrecomputeSession` engine (wraps 3d).
- All persisted as `{ fixture|document, runtime }` envelope (`Puzzle2dPlayEnvelope` / `Puzzle3dEnvelope` / `Puzzle5dEnvelope`) → snapshot via `setDocument` op. Runtime is view state.

## Design (uniform)

- Projection = `serde_json::Value` for all three (the bare fixture/document json). Runtime moves into app struct.
- ONE generic Op per core crate (replaces placeholder SetRevision op):
  - `UpsertItem { collection, item }` (item has string "id")
  - `RemoveItem { collection, id }`
  - `SetField { key, value }`
  - `ReplaceDocument { document }`
- `Diff { ops: Vec<Op> }`, apply = fold, absorb = extend.
- `backwards(projection)` computes true inverse per variant from pre-state.
- `document_delta_ops(before, after)`: per top-level key: id-keyed arrays -> Upsert/Remove per id; else SetField; **safety check**: replay ops on before; if != after -> single ReplaceDocument. Bulletproof correctness + granular convergence for the common case (add/remove/patch id'd items, camera).
- Plugin keeps typed fixtures: parse projection Value -> typed for render; mutate typed -> serialize -> delta ops.
- Transient view bundle replaces `*PlayEnvelope` (rename to `*Scene`) so the large render surface is untouched.

## Verification: cargo build/test -p puzzle_2d puzzle_3d puzzle_5d puzzle-plugin
