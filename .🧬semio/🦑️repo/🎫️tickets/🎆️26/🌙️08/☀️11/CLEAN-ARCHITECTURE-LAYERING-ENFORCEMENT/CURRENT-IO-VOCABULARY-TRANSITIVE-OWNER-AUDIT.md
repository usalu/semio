# Current I/O Vocabulary Transitive Owner Audit

Read-only manifest/source audit, 2026-10-03. No Cargo, compiler, production action, deletion, or Git modification occurred.

## Actual Cargo Closure

The root Cargo.toml contains 115 explicit workspace members. Bun.TOML.parse and independent development-only @iarna/toml parsed every member identically. Starting at 🧰️framework/🔨️modules/🚪️io/🧬️schema/📦️packages/🦀️rust/Cargo.toml, recursively resolving normal, build, and target normal/build path dependencies yields exactly six local packages and seven edges. All six manifests are declared workspace members; no missing local manifests occurred.

| Package | Actual manifest |
| --- | --- |
| semio-framework-io-schema | 🧰️framework/🔨️modules/🚪️io/🧬️schema/📦️packages/🦀️rust/Cargo.toml |
| semio-framework-value | 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml |
| semio-framework-value-derive | 🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml |
| semio-framework-io-base64 | 🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust/Cargo.toml |
| semio-framework-diagnostic | 🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/Cargo.toml |
| semio-framework-schema-registry | 🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust/Cargo.toml |

Edges: IO-schema → Value, ValueDerive, Diagnostic, SchemaRegistry; Value → ValueDerive, IOBase64; Diagnostic → Value. No OS, S, or Hub path occurs in this closure. No target/build local edge occurs in this closure. Conservative traversal includes optional declared dependencies and does not resolve feature activation. External dependency names in the closure are syn, quote, proc-macro2, serde, serde_json. Direct IO-schema serde_json is development-only; transitive manifests expose serde/serde_json elsewhere. External registry source and lockfile transitive dependencies were not traversed.

## Ownership Laws and Routing

The ownership JSON schema closes the outer object with additionalProperties:false, requires exactly four unique admitted normal dependencies, exactly nineteen unique admitted exports, and exactly three ArtifactRef samples. The fixture and TypeScript source test compare manifest dependency maps using Bun and @iarna, validate with Ajv, independently round-trip literal identity through SQLite, and reject an injected OS dependency. Package entry explicitly reexports nineteen names and mounts two native ownership tests under cfg(test). No wildcard export appears in that entry. Package scripts route via Bun/Nx into the local 📜️script.ts for test-ownership and test-native; launch.json exposes both project selector entries. The native runner selects only semio-framework-io-schema with --lib --no-fail-fast and explicit caller-owned Cargo policy.

The portable tree-sitter law follows physical module files, including cfg(test) module paths. It does not macro-expand Rust or recursively traverse Cargo manifests. This audit supplies the missing manifest closure evidence; it does not convert that scanner into a compiler or prove macro-produced APIs. Source vocabulary documentation still describes its previous OS mounting, and root integration still needs the parent task migration. Neither schema admission nor literal round-trips establishes exclusive definition ownership across the entire repository.

## Limits and Actionable Follow-Up

Package runtime/path independence and repository preparation are separate claims. Cargo workspace discovery/preparation can require other members even when this six-package runtime closure contains no product edge. Deleting OS/S/Hub directories from this actual workspace has not been attempted and is not proven to compile. A temporary reduced workspace prepared by a repository-wide helper could also pull unrelated infrastructure dependencies; that would be a tool/preparer dependency, not an IO-schema runtime manifest edge.

Nx project test inputs currently cover IO schema, Value, Diagnostic, and process tooling, but omit the actual direct SchemaRegistry owner subtree. Include that subtree in cache inputs when editing the project metadata. Verify launch GUI tasks and native package execution in the parent task; neither was executed in this read-only audit.
