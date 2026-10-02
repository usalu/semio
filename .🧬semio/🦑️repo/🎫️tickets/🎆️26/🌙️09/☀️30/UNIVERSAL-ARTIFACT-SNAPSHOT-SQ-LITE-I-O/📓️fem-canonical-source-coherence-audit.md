# FEM Canonical Source Coherence

The two exact SQLite Source implementations currently expose `Fem2dSqliteSnapshot` and `Fem3dSqliteSnapshot`, whose binary64 values are owned IEEE bit objects and whose native machine-sized counts are bigint. Their physical/native/public selected laws passed. The pre-existing canonical Source Snapshot interfaces still declare the equivalent fields as ordinary JavaScript numbers, including counts. Root facades currently export both the exact relational state and those numeric wire interfaces.

This is an unfinished source-model boundary: selected SQL roundtrips do not establish exact canonical artifact/diff/schema consumer coherence. A clean completion must make the actual owned Source Snapshot use the exact primitives and explicitly author numeric JSON/render/solver projection boundaries, then update all real consumers and handcrafted facets. No compatibility union or implicit number coercion is appropriate. The parent was notified; this repair follows the currently assigned Puzzle5d native owner work. Whole native FEM suites are also pending and independent of this Source boundary.

## Canonical Source repair contract

The two existing Snapshot/Artifact/Diff/Mutation Source facets duplicate persisted entity definitions. Their finite `number` scalars and safe-number counters contradict the actual Native `f64`/`u64` fields. The earlier independently verified SQLite readers expose exact words, but through a second SQLite-prefixed model. This is a real owned model disagreement, not a query projection.

The new authored law reuses the language-neutral IEEE/count corpus and sends its exact typed state through Snapshot and Artifact parsers, Diff additions, and physical SQLite. The first registered invocation remains pending; no parser production repair is mounted before genuine assertion failure. The intended clean authority is Snapshot's exact entity types and parsers. Artifact/Diff/Mutation reuse these real types; SQLite signatures consume the same Snapshot. IEEE query numbers stay derived at actual rendering/solver boundaries. No number/word compatibility union or JSON carrier is planned.

Read-only consumers inspected: the three Model Source windows only declare literal view identities and do not read scalar fields; the developer story helper owns a separate explicitly numeric DSL-to-scene projection. It does not import the persisted FEM entity types. Existing Native solver/render paths remain unchanged.
