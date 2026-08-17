# Audit (2026-05-12)

- Reverted mistaken renames: `*Delta`, `DesignBodyDelta`, `*Scalars` (where they replaced `DesignScalarDiff` / `TypeScalarDiff`), `PieceTransform`, `*InsertPayload`, `*UpdateEntry` — restored metabolism kit-diff names (`TagsCollectionDiff`, `TagPatch`, `DesignDiff`, `operation::Diff` envelope, etc.).
- SDL `Diff` lives in `compose/graphql/target.schema.graphql` as `interface Diff` + concrete `*Diff` types; `operation::Diff` is a separate in-memory operation payload — doc on `operation::Diff` updated to state that explicitly.
- Rule going forward: do not invent wire vocabulary (`Delta`, …) for types that are **not** in `target.schema.graphql`; metabolism JSON shapes keep their existing Rust names until a planned mapping to SDL `Modification` / `Diff` ladders exists.
