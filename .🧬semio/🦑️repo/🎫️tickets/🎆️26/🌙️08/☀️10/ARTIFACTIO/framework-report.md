# Framework Artifact I/O Boundaries

Framework artifacts now own native text and binary implementations under their representation-first I/O facets. Playbook, Workflow, Run, Space, Collection, Flow, DAG, and the snapshot-refusal fixture have separate operation/document codec modules. Flow's DSL mirror types, conversion helpers, and controlled record lowering live with its text snapshot codec; binary and SQLite implementations consume that owner rather than retaining serialization code in VCS semantics.

The framework SQLite relocation moved 82 source, schema, fixture, and test files across 20 owned branches. Run, DAG, and snapshot-refusal schema assemblies no longer mount SQLite implementations. SQLite projection, reconstruction, preflight, and native helpers are assembled through artifact I/O. Workflow and Run TypeScript schemas no longer reexport persistence functions; consumers import their I/O owner directly.

Exact Binary32/Binary64 identities and ordinary numeric capture now belong to framework Value. SQLite keeps column companions and row reconstruction. This removes the semantic snapshot type dependency on a SQLite adapter. Named imports in 257 consumers were redirected along with the two scalar implementation owners.

Validation is ongoing. Registered Nx native checks for the seven production artifact crates and their SQLite source suites are running. The framework source suite also covers exact IEEE words against an independent SQLite engine. Final outcomes will be added after completion.

File inventories: [relocations](framework-relocation-files.md), [codec sources](framework-codec-files.md), [assembly and source imports](framework-mount-files.md), [native scalar owners](native-scalar-files.md).
