# DAG Document Contract Ownership

## Native contract

The native DAG durable DTOs already define one shared child identity:

- DagArtifact has schema and content fields.
- DagSnapshot has schema and content fields.
- DagDiff has optional schema and optional content fields.

The child is an ArtifactChild of SemioGraphSnapshot classified as s.stdio.semio.graph. Its concrete reference uses artifact kind s.stdio.semio, standard v1, and subset graph. Nodes and edges live behind that child owner. They are not artifact/snapshot fields or parent-level structured diff fields. Editor selection and camera state are outside the durable document DTO.

## Corrected facets

The JSON Schema, TypeScript, GraphQL, and Proto artifact/snapshot facets now expose only required schema and content. Their diff facets expose the exact native sparse carriers schema and content; JSON and TypeScript require both serialized carrier keys and accept null for unchanged Option values. TypeScript production parsers now enforce closed field sets and delegate the child structure to the shared parseArtifactChild.

Removed non-native shapes include embedded nodes/edges, DagNodesDelta, DagEdgesDelta, set-node/set-edge wrappers, replacement artifact, editor selectedNodeIds, and editor camera.

## Oracle

bun nx run workspace:test-dag-document-contract is green:

> [DEBUG] DAG exact document contracts matched 28 native snapshots, 0 committed diffs and independent owner/child rejection vectors

The test reuses the shared assertDocumentContractOracle with the exact DAG mutation-fixture root. Ajv and the production parsers independently accept all 28 committed before/after snapshots. Independent vectors reject embedded graphs, malformed child references, editor fields, replacement artifacts, structured graph deltas, malformed child diffs, and missing native diff carrier keys. The production parser also checks the concrete s.stdio.semio plus graph child address.

git diff --check is green across the DAG facet, oracle, ticket registration, root Nx registration, and launch registration scopes.

## Exact file ledger

The twelve non-Rust facets changed are:

- ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}
- .../🧬️schema/📸️snapshot/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}
- .../🧬️schema/🔺️diff/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}

Here, ... expands to ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any.

The neutral fixture and oracle added are:

- .../🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json
- .../🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts

Validation registration changed only the DAG document-contract entries in:

- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/project.json
- 📜️script.ts
- 📋️project.json
- .vscode/launch.json
- .vscode/🧩️launch.seed.jsonc

The native Rust DTOs, shared document oracle, DAG fixture taxonomy, and the concurrently moved framework/plugin demo asset were read as authority and left to their existing owners.
