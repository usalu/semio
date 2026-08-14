# IFC2X3 Structural Roundtrip Implementation

## Outcome

IFC2X3 now persists an ordered `Part21PhysicalFile` token stream beside the semantic `Part21Document` and structured `Ifc2x3EdmPreamble`. Native export concatenates physical token lexemes and rejects the snapshot unless reparsing those lexemes produces the identical semantic document. There is no whole-file `ArtifactSource`, byte replay shortcut, or semantic-fingerprint bypass in the IFC2X3 tree.

The supplied `temp/wellness-center-sama.ifc` remains the exact fixture: 21,282,588 bytes, 409,102 instances, CRLF, IFC2X3, SHA-256 `f4dbc661d555bbf92fb80a40443f6b6b540fa0a833b85d78487930368147b593`.

## Structural Contract

- `Part21PhysicalToken` classifies ordered whitespace, comment, string, word, and symbol lexemes.
- `Part21PhysicalFile::render` is the normal native writer; `decode_ifc2x3` tokenizes every input character.
- `encode_ifc2x3` requires `parse_part21(physical.render()) == snapshot.document` before emitting bytes.
- Header and instance mutations synchronize only changed physical records, preserving untouched comments, whitespace, CRLF, lexemes, and record order.
- Existing-instance upserts replace in place. Diffs carry explicit `instance_order` when membership/order changes.
- Mutation diffs carry the resulting physical state. Set-snapshot validates physical/semantic synchronization before application.
- Mutation inverse uses a complete set-snapshot when positional restoration is required.
- DSL, pack, raw binary/text IO, analyzer, composer, artifact conversion, diff text/binary, and set-snapshot text/binary retain or reconstruct the physical token model.
- Diff/op binary decoders validate the format byte and reject trailing bytes.
- Infallible DSL printing no longer silently substitutes an empty body; invalid physical state fails explicitly.

## Laws Added Or Extended

- direct, pack, DSL, raw IO, analyzer, and composer exact fixture export;
- self-diff/no-op/inverse/absorb exact restoration;
- semantic-bypass rejection when document state changes without physical synchronization;
- interior-instance upsert position preservation and exact mutation+inverse restoration;
- physical state through diff and set-snapshot codecs;
- strict op trailing-byte rejection.

## Schema Mirrors

Rust, TypeScript, GraphQL, JSON Schema, and Protobuf artifact/snapshot/diff mirrors expose `Part21PhysicalFile`; diffs additionally expose `instanceOrder` and optional physical replacement. Mutation and diff grammars include the physical payload.

## Validation Evidence

- `rustfmt --edition 2021 --check` parsed all modified IFC2X3 Rust implementation files; it exited `1` only for the tree's existing compact formatting differences.
- `git diff --check -- <IFC2X3 tree>` exited `0`.
- `jq empty` succeeded for the three modified IFC2X3 JSON schemas.
- `rg 'ArtifactSource|unwrap_or_default' <IFC2X3 tree>` found no relevant implementation match.
- No Cargo or Nx command was run during this structural finalization, per coordinator instruction. Central compilation/runtime fixture execution remains the coordinator's gate.

## Concurrency

Concurrent structured EDM preamble and shared exact-decimal work was preserved. IFC4 routing remains unchanged because the fixture declares `FILE_SCHEMA(('IFC2X3'))`. Ticket closure remains with the primary coordinator.
