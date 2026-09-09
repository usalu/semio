# Puzzle Ownership Implementation

## Implemented boundary

Puzzle 2D now composes its command/render runtime from three owners:

- `Puzzle2dConfig` keeps only the deliberately shared node and handle kind weights.
- `Puzzle2dWindowConfig` owns camera, LOD, grid, suggestion distance, and fill count for the exact framework window instance.
- `Puzzle2dWindowTransient` owns engagement input and brush candidate state for the exact framework window instance.

The retained Puzzle command payload captures the exact window configuration snapshot, window transient snapshot, and request-context identity. Checkpoints include that context identity. A resumed job refuses a different context, restores the captured owners, and publishes its real ephemeral output through the framework completion lane.

Fill work freezes the exact window controls and shared generator weights when the operation starts. Its lifecycle, checkpoint cursor, accepted/search counts, and fault state stay inside the retained job. Completion publishes document mutations and the exact window's fill count; no fill checkpoint is copied into app configuration.

The two authored Puzzle 2D fixture cameras remain document data. They seed a generation-zero default window config and never receive live camera updates.

## Schema parity correction

Puzzle 5D artifact and diff surfaces now expose `kindCatalogsExtra` in JSON Schema, TypeScript, Protocol Buffers, and GraphQL. Rust already carried the field. The type is the Puzzle-owned overflow sibling of the composed kind-catalog child.

## Validation

- `bun nx run workspace:artifact-field-parity-test --output-style=static`: passed. The target verified TypeScript AST discovery against the independent Ajv exact-record oracle.
- Puzzle native quick tests: pending completion in the ticket-isolated `cargo-trinity` target after the first compile pass exposed and the implementation fixed seven local errors (duplicate import and qualified text/binary operation conversions).
- Artifact field parity enforcement: pending.

## Remaining implementation

Puzzle 3D still needs its keyed `window_options` and flat materialized window fields replaced by an exact `WindowConfig`, its popup/input/candidate data moved to `WindowTransient`, and its fill checkpoint restored only from retained operation state. Puzzle 5D still needs its camera/display controls and engagement/candidate state redirected from app configuration to exact window owners.
