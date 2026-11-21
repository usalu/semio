# XLSX Tool Proof Ingress

The XLSX history suites stopped at instance ingress when a generated edit proof used the last path segment of the monomorphized factory type as its display label. The actual generic editor path ended in `XlsxEditor>`. The live registration independently identified the outer factory and rejected the mislabeled proof.

Both shared editing proof producers now supply the outer factory identifier with `stringify!` and retain the full concretized type name and TypeId from `with_factory_type`. The registry validator keeps its existing owner, controller, schema, factory, payload wire and execution-contract checks.

A base XLSX editor regression checks both `setSnapshotValue` and the native `set-cell` proof against the existing sheet-grid JSON fixture. Both Rust cases read the existing include path, keeping the Base crate within its source-file budget. It also checks the exact concrete factory TypeId and full type name. The independent SQLite law traverses the qualified declaration path before its generic argument and agrees with both fixture labels.

The SQLite law passed one test with two assertions. The pinned Rust parser accepted the runtime and test modules. The live four-package Cargo run will verify instance allocation and the previously blocked history paths.
