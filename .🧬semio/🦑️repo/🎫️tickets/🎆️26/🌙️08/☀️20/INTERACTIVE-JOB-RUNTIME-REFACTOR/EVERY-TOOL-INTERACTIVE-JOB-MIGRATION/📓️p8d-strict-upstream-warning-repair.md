# P8 Strict Upstream Warning Repair

## Scope

The native warning-denied Phase 8 gate originally stopped before `semio-framework-plugin` on 24
owned dependency lints. Repairing those exposed seven further leaf lints and one pack-contract lint.
The fixes preserve behavior and remove the warnings at their source rather than adding broad lint
allowances.

## Repairs

- SHA-256, mesh, and base64 fixed-width chunks now use fixed-size slice views.
- Mesh GLB traversal borrows handles, uses direct option combinators, and preserves every importer
  and exporter result.
- Replication derives trivial defaults, removes redundant scalar module nesting, and modernizes
  equivalent option/iterator expressions.
- UI runtime names listener/deferred callback types; UI scene and test fixtures use equivalent
  non-redundant expressions.
- `AsyncPackSource` now exposes `is_empty` consistently with its synchronous known length.

## Evidence

The focused warning-denied cohort passes for hash, mesh-engine, replication, OS-kernel DSL derive,
UI runtime, UI scene, and pack with `--all-targets -D warnings`.

The focused behavioral suite passes 436 tests:

- hash: 6/6
- mesh-engine: 20/20
- pack: 66/66
- replication: 188/188
- UI runtime: 72/72
- UI scene: 84/84

The full plugin warning gate now reaches `semio-framework-os-kernel` and exposes a distinct 225-lint
cohort. That cohort is being repaired separately; it is no longer hidden behind the Phase 8 leaf
dependencies documented here.
