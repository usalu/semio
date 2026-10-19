# Framework and Hub Source Merge Results

Base `1bae6115bc56bf5e1b01212774dd76877a5f64fe`, current `36fbe9da786b71ec7b6946a1fc1cbb4e9d4d25ec`, incoming `a09b7afdff001185403f0decb7cb0714a7bb839e`.

Applied2082 uncontested source writes in assigned framework/shared/OS/hub/other-products scope, excluding Course/Proctor, repository product, root manifests/locks and later-delegated renderer/plugin overlaps. Applied88 disjoint three-way source merges before renderer/plugin ownership handoff. Resolved shared script/UI/job test conflicts1–5 and Store composers6–16. Placed263 incoming edits into current extracted Store owners, preserving the current monolith dissolutions. Placed the new weaker-bound ArtifactStore impl into the current engine owner and retained current three-store composer ownership while carrying new decision/planning fields and Debug implementations.

Wired the actor typed-operation result page through `actor::typed_operation_page`. Bound diagnostic staging is owned by one-item-publication::diagnostics; its ArtifactStore publisher impl lives in the current engine owner. The new durable planning module belongs to the stage owner and its shared public planners are reused by the durable composer; the wire owner includes only the budget source.

Added direct local manifest edges: engine-stage → store-canonical-json and dsl-record; engine → value-snapshot-read. Root owns lock edge synchronization.

The current deletion of the obsolete hub shipped-fleet SQLite test is retained. Incoming only corrected two fixture import paths in that removed source owner.

Static source audit:1269 changed source/configuration paths in remaining assigned scope, zero merge marker matches, zero paths exceeding256 UTF-16 code units, and zero changed JSON syntax failures. No builds, compiler invocations, tests, runtime execution, installations, or dependency resolution were performed. These checks establish source preservation constraints only. Incoming source inconsistencies and remaining compiler/pipeline behavior are outside the task’s requested preservation check.

See `framework-static-audit-repeat-691.md` and `store-relocations-repeat-691.md` for details. Independent audit requested before final ticket closure.

## Explicit Current-Only Path Dispositions

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📦️packages/🦀️rust/Cargo.toml`: Added direct value-snapshot-read dependency for the new SnapshotReadRegistryHandle import in the relocated weaker-bound engine methods.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🗄️stage/📦️packages/🦀️rust/Cargo.toml`: Added direct store-canonical-json and dsl-record dependencies for the new durable planning module placed under the current stage owner.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs`: Ported incoming codec-bound changes from the removed Store monolith to the current codec factory owner; retained current crate namespaces and visibility.
