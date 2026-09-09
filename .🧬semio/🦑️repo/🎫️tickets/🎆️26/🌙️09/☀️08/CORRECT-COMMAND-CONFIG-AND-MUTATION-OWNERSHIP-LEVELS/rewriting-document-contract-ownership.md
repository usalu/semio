# Rewriting Document Contract Ownership

Rewriting artifact and snapshot schemas now consume the framework dynamic value contract for parameter bindings. JSON Schema references `framework/value/schema.json#/$defs/DslValue`; TypeScript imports the shared parser and type; Protobuf uses the shared `DslValue` message; GraphQL uses the shared scalar, including null-valued entries. Layout points remain owned by Rewriting, and snapshot representations reuse the artifact-owned layout declaration. The stale document-level Camera declarations were removed; camera belongs to the concrete window configuration.

Native PropertyValue imports now point directly to `semio_framework_graph::manifest`, the existing owner, with the internal graph crate declared as a production dependency. This removes the accidental route through Jack without introducing another value implementation.

The exact document parser checks nested values, layout coordinates and keys, and rejects extra window fields. Language-neutral vectors exercise the six JSON value kinds, malformed layout records, and non-JSON nested values. Ajv independently validates the same inputs.

## Validation

- `rewriting-document-contract-red-1.log`: Nx failed on the expected regression, `Missing expected exception: parser accepted invalid camera`.
- `rewriting-document-contract-green-1.log`: Nx passed in 3.9 seconds, with the runtime receipt confirming both document and snapshot parsers matched the independent schema oracle.
- Native import changes await the next Rewriting native run; no native pass is claimed for those import changes yet.
- Protobuf and GraphQL source contracts were corrected, but no IDL compiler is available in this workspace and compilation is not claimed.

The command is registered through the root script, Nx, the ticket validation project, and both launch catalogs as `rewriting-document-contract`.
