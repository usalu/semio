# Derived Artifact Facets Research

## Scope

Remove every explicit `🏗️builder`, `🧐️analyzer`, and `🎹️composer` directory below `✏️s/🔌️plugins/**/🗿️artifacts` and derive their lifecycle behavior from artifact schemas, dialect coordinates, subset validation, and IO conversion registrations.

## Inventory

- 321 Rust builder facets and 321 TypeScript metadata twins.
- 321 Rust analyzer facets and 321 TypeScript metadata twins.
- 321 Rust composer facets and 321 TypeScript metadata twins.
- Facets repeat at artifact, standard, and subset levels.
- Artifact and standard facets are almost entirely delegation/aggregation.
- Base-subset builders repeat snapshot storage, text/pack decoding, semantic mutation diffing, diff application, and final materialization.
- Base-subset analyzers repeat text/pack decoding into an optional snapshot plus confidence and diagnostics.
- Base-subset composers repeat native decoding plus IO-leaf dispatch.
- Real stdio subsets additionally contain conformance checks and validators. Those checks are schema/standard rules, not analyzer ownership.
- Standard composers additionally contain export-entry wiring. Those entries are IO topology, not composer ownership.

## Existing Mechanisms

- `ArtifactBuilder`, `ArtifactAnalyzer`, and `ArtifactComposer` are framework traits in the OS plugin module.
- `ArtifactSerializer`, `ArtifactDeserializer`, `ComposerEntry`, `SubsetValidator`, and the IO registry already provide the lower-level conversion/validation seams.
- `ArtifactDsl`, `ArtifactPack`, `Mutation`, and `MutationDiff` already provide every generic operation needed to derive the common builder/analyzer lifecycle.
- Artifact dialect coordinates already have a shared `Dialect` value type.
- Artifact schemas already have a derive crate and structural descriptors.

## Target Ownership

- Schema owns snapshot, mutation, diff, and conformance rules.
- IO owns directed serializers/deserializers and their registry entries.
- Framework derives builder/analyzer/composer lifecycle from those inputs.
- Package glue only wires schema, engine, IO, and generated/derived facet declarations; it does not point at capability files.
- Taxonomy and policy must forbid explicit capability directories instead of requiring them.

## Concurrent Work

The working tree contains an unrelated in-progress semantic-mutations overhaul with many deletions and new mutation triads. This ticket must preserve those changes and avoid rewriting their semantic mutation implementations.
