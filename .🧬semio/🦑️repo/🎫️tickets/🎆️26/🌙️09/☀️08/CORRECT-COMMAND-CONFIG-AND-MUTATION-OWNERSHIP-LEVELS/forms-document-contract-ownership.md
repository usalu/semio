# Forms Document Contract Ownership

The live Rust artifact and snapshot own schema, id, version, optional title, structure and results. Structure/results are exact composed child identities. Artifact JSON/TypeScript/GraphQL/Protobuf still declare inline steps; snapshot and diff JSON/Protobuf expose an unrelated generic value wrapper. Snapshot/diff TypeScript additionally redefine OS child and IO addressing contracts. These stale projections contradict the committed native fixtures.

The correction uses the existing OS Store ArtifactChild and framework IO contract in all non-Rust representations, while retaining the actual six native document fields. The neutral regression checks committed snapshots/diffs and rejects editor-owned step/try/camera state at the document boundary. Native mutation semantics are unchanged. Validation is pending.

## Validation

`forms-document-contract-red-1.log` reproduced the stale artifact schema requiring inline steps and rejecting both actual child slots. All twelve non-Rust artifact/snapshot/diff projections now declare the six native fields and import the shared child contract. Unused local child addressing, generic value wrappers and duplicated form helper types were removed from these document facets.

`forms-document-contract-green-1.log` reached all parser/schema checks but failed the test's assumed fixture count: the owner contains twenty committed snapshots and six committed diffs, not ten diffs. Correcting that assertion to the observed fixture inventory produced `forms-document-contract-green-2.log`: Nx passed in 1.6 seconds. Ajv and the production TypeScript parsers agree on every committed input, optional-title normalization, and rejection of five editor/obsolete fields and three malformed child shapes. This run does not claim a GraphQL/Protobuf compiler check or a new native mutation test run.

The check is registered as `forms-document-contract` in the root script, Nx project, isolated ticket validation project and both launch catalogs. Forms next/previous-step and try-value runtime ownership still require their separate exact-window migration.
