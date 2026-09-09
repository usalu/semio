# Shared Artifact Addressing Ownership

Artifact coordinates and references now have one authored TypeScript, JSON Schema, GraphQL, and Protobuf owner beside their native IO schema: `🧰️framework/🔨️modules/🚪️io/🧬️schema/`. JSON definitions use `https://semio.tech/schema/framework/io/schema.json#/$defs/ArtifactDialect` and `#/$defs/ArtifactRef`. The TypeScript module owns `ArtifactDialect`, `ArtifactRef`, their exact record parsers, dialect coordinate codecs, and artifact URI codecs.

Cross-plugin app roles and app references belong to Manifest: `🧰️framework/🔨️modules/🛂️manifest/🧬️schema/`. Its existing JSON module now owns `AppRole` and `AppRef`; corresponding TypeScript, GraphQL, and Protobuf definitions and surface-ID codecs live beside it. Kernel consumes these owners instead of declaring duplicate types or codecs. The framework package barrel exposes IO directly and Manifest exposes its own app-addressing contract.

Persisted child handles belong to OS Store: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/`. The JSON ID is `https://semio.tech/schema/os/store/child.json`; the shape is exactly `childId` plus `target`, referencing IO's `ArtifactRef`. No local materialization or native phantom type appears on the wire. Native child/address decoding now rejects unknown fields rather than silently accepting hidden state.

OS opening preferences and their set/clear mutations reference the shared IO and Manifest definitions. Their TypeScript imports and GraphQL/Protobuf dependencies point at those owners; OS config no longer declares generic addressing types.

## Validation

The new neutral fixture and independent Ajv oracle were registered before implementation. The initial Nx oracle failed because the shared IO schema was absent. After implementation, `shared-artifact-addressing` passed through Nx in 44.3 seconds: the TypeScript oracle checked three canonical identities, five rejected child shapes, coordinate/URI parsing, and exact serialization; one native Rust test passed with zero failures and verified that attached local materialization never appeared in the encoded child. The follow-up oracle also passed after adding opening preference and set/clear mutation references and surface-ID codec round trips. The ownership gate passed after this extraction.

Permanent execution route: root `📜️script.ts verify shared-artifact-addressing [oracle]`, `workspace:test-shared-artifact-addressing[-oracle]`, and matching launch configurations in the authored seed. Native verification selects `shared_artifact_addressing` in `semio-framework-os-kernel`.

Wires now consumes these canonical references in its artifact, snapshot, and diff schemas. Remaining application ownership migrations are tracked separately.
