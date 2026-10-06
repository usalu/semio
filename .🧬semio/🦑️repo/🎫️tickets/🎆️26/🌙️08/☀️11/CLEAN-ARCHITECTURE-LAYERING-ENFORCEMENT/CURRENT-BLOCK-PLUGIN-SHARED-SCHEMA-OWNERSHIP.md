# Block Plugin Shared Schema Ownership

Original owning TypeScript direction2 finds the Block plugin public entry exporting its API from the removable 2D artifact. The same physical shared schema has six source files: JSON schema, TypeScript records, TypeScript scalar admission, JSON transport, SQLite transport and native Rust records. Artifact 3D/5D also import these files through Artifact 2D.

The canonical owner is Block plugin schema. The public TypeScript API will name its exports explicitly. A plugin-root native package will own shared records and their eight retirement implementations, depending only on General Framework contracts. All three native artifacts will consume that package; the existing 2D retirement module will retain its own artifact records and mutations.

Artifact-local producer laws, parsers, fixtures, snapshots and conformance metadata remain under their current artifact owners. Rust serde admission is a test feature; no runtime external library is added. The actual shared JSON schema, independent Ajv and SQLite controls and owning native tests define the behavior to preserve.

Whole plugin composition also currently has a bridge that imports concrete artifact descriptors. The shared schema lift does not claim that bridge or every plugin/application has already passed deletion checks. Those remaining dependencies must be addressed and enforced separately.

