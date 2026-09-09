# Stdio Object and Kit Mutation Facet Audit

## Scope and evidence boundary

This is a bounded, read-only audit of the Stdio Semio v1 Object and Kit mutation
trees on 2026-09-09. It covers the aggregate Rust, TypeScript, GraphQL, JSON
Schema, and Proto facets; every direct mutation leaf, descriptor, diff, inverse,
and committed mutation vector; and the shared geometry, artifact reference,
child, link, pin, and blob definitions those facets should consume. Object and
Kit production sources were not changed by this audit. Root was editing the
Object document contract concurrently, so the Object observations below name
the exact mutation files observed rather than claiming anything about root's
subsequent edits. No Nx or Cargo command was run.

The audit counted 80 files below Object `🧬️schema/🧬️mutations` and 116 below Kit
`🧬️schema/🧬️mutations`. Object has 9 leaves and Kit has 15. For each family the
count includes 5 aggregate language facets, one Rust payload, one JSON payload
schema, one descriptor, one diff, one inverse, and one Rust leaf test per leaf.
The separately stored neutral corpus has exactly 9 Object and 15 Kit
`🦠️mutation/🔣️.json` vectors.

## Canonical shared owners

| Value | Canonical owner | Available facets |
| --- | --- | --- |
| `ArtifactDialect`, `ArtifactRef` | `🧰️framework/🔨️modules/🚪️io/🧬️schema` | Rust, TypeScript parser/types, GraphQL, JSON Schema, Proto |
| `ArtifactChild` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema` | Rust, TypeScript parser/type, JSON Schema; Stdio `✉️base/…/🪆️child` adds exact `s.stdio.semio@v1/<subset>` identity policy |
| `ArtifactLink`, `LinkPin` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema` | Rust, TypeScript parser/types, GraphQL, JSON Schema, Proto |
| `BlobRef` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️blob/🧬️schema` | shared link facets already refer to it |
| `SemioPoint3`, `SemioQuaternion`, `SemioTransform` | Stdio `✉️base/🧬️schema/🧮️geometry` | Rust, TypeScript parser/types, GraphQL, JSON Schema, Proto |

The Stdio child module is a policy layer over the Store-owned `ArtifactChild`,
not another data model. Its TypeScript facet reexports the Store type and calls
the Store parser before checking exact Stdio kind, standard, subset, and equal
child/target identifiers.

## Object findings

The Rust mutation enum and all 9 neutral vectors agree on the native wire shape:
one externally tagged PascalCase variant and a direct payload record. The 9 leaf
JSON Schemas use the same literal Rust payload names. The only snake-case key is
`child_id`, present in three payloads: `CreateBrep`, `CreateMesh`, and
`CreateProperties`. Their nested shared `ArtifactRef` remains camelCase
(`artifactId`, `artifactKind`) by its own owner. This mixed casing is deliberate
in the current native derivation, but it differs from the camelCase parent,
snapshot, diff, child, and link contracts and must be kept explicit if the
native payload structs remain without `#[value(rename_all = "camelCase")]`.

The Object aggregate JSON Schema is exact: it has 9 closed one-branch wrappers
whose payloads reference the 9 leaf schemas. The TypeScript union also has 9
correct externally tagged branches, but its shared imports are not canonical:

- `🧬️schema/🧬️mutations/🟦️.ts` imports `ArtifactRef` from the Object artifact
  facet, even though the observed Object artifact TypeScript facet does not
  reexport it. The import should target the framework IO owner directly.
- The same file restates two `SemioPoint3` records and one `SemioQuaternion`
  record inline in `MoveObject`, `ScaleObject`, and `RotateObject` instead of
  importing the Stdio base geometry types.
- Six Object leaf schemas carry duplicated shared `$defs`: three child-create
  files each restate `ArtifactRef` and `ArtifactDialect`; move and scale each
  restate `SemioPoint3`; rotate restates `SemioQuaternion`. That is 9 duplicated
  `$defs` occurrences across 6 files.

The GraphQL and Proto aggregate facets do not mirror the native mutation at all.
GraphQL invents `{ kind, payloadWire }`; Proto invents `{ mutation,
payload_wire }`. Neither field pair exists in Rust, the aggregate JSON Schema,
TypeScript, or any committed vector. Both need a typed payload per mutation
variant and an exact union/`oneof` dispatch while referring to the shared IO and
geometry owners.

The Object correction surface is therefore exactly 9 mutation files: the 3
aggregate facets `🟦️.ts`, `🔗️.graphql`, and `🛰️.proto`, plus the 6 leaf JSON
Schemas named above. The aggregate Rust and JSON Schema, 9 Rust payloads, 9
descriptors, and 9 committed mutation vectors currently agree and should not be
rewritten merely to normalize style.

## Kit findings

The Rust enum, all 15 leaf JSON Schemas, and all 15 neutral vectors agree on the
same externally tagged dispatch. Six payload contracts expose snake-case keys:
`child_id` in `CreateObject`, `DeleteObject`, `CreateModel`, `DeleteModel`, and
`CreateProperties`, plus `new_name` in `RenameType`. Nested domain records remain
camelCase (`typeId`, `connectingPieceId`, and related connection keys), and
shared reference records remain camelCase. There is no mismatch among Rust,
leaf JSON Schema, TypeScript, and committed vectors today; the mismatch is
between this leaf convention and the surrounding canonical document facets.
A future casing change must update the Rust attrs, 6 leaf schemas, 6 neutral
vectors, aggregate TypeScript, text/binary codecs, and tests together.

The aggregate Kit JSON Schema is exact: 15 closed one-branch wrappers reference
the 15 leaf schemas. The aggregate TypeScript union has the right branch and
payload structure, but its import source is wrong:

- `🧬️schema/🧬️mutations/🟦️.ts` imports `ArtifactRef` and `ArtifactLinkRef` from
  the Kit snapshot. That snapshot copy models `ArtifactLinkRef.target` as a
  string and its snapshot-pin payload as flattened `{ hash, size, mediaType }`,
  while the Store owner requires a structured `ArtifactRef` target and
  `{ kind: "snapshot", blob: BlobRef }`.
- The same import obtains Kit-owned `SemioKitPiece` and
  `SemioKitConnection` from the snapshot, which is appropriate for those domain
  records, but their `SemioTransform` member must resolve through the Stdio base
  geometry owner rather than a snapshot-local geometry copy.
- Six Kit leaf schemas duplicate shared definitions. Create-object,
  create-model, and create-properties each duplicate `ArtifactRef` and
  `ArtifactDialect`; bind-representation duplicates those two plus `LinkPin` and
  `BlobRef`; change-representation-pin duplicates `LinkPin` and `BlobRef`;
  edit-design duplicates `SemioTransform`, `SemioPoint3`, and
  `SemioQuaternion` alongside its legitimate Kit-owned `SemioKitPiece` and
  `SemioKitConnection`. This is 15 duplicated shared `$defs` occurrences across
  6 files; the two Kit domain `$defs` in edit-design remain Kit-owned.

Kit GraphQL and Proto have the same invented opaque envelopes as Object:
GraphQL `{ kind, payloadWire }` and Proto `{ mutation, payload_wire }`. They omit
all 15 concrete payload contracts and cannot express or validate a child target,
history pin, design piece, connection, or geometry value. They need typed
variants using shared IO/link/geometry facets and Kit-owned catalog/design
records.

The immediate Kit mutation correction surface is exactly 9 files: aggregate
`🟦️.ts`, `🔗️.graphql`, and `🛰️.proto`, plus the 6 leaf JSON Schemas listed
above. If Kit chooses the repository-wide camelCase payload convention during
its document-contract correction, 6 more leaf JSON Schemas and 6 neutral
vectors contain the affected keys; some overlap the 6 shared-definition files,
so the union is exactly 18 files before codec/test source changes are counted.

## Mutation ownership and metadata

All 24 leaf descriptors identify their exact mutation-leaf taxonomy directory,
declare `composition: "atomic"`, and match the Rust aggregate variant and
kebab-case semantic kind. The 24 mutations change authored document state:
Object transform/child slots and Kit catalog/design/child/link collections.
None is window configuration, window transient state, host contribution data,
or retained operation progress. The audit found no mutation that should move to
a window/app/operation owner and no descriptor `owner` mismatch.

The two aggregate Rust `#[mutations(... schema = ...)]` values are also schema
identifiers, not artifact kinds: `s.stdio.semio.object` and
`s.stdio.semio.kit` match their `#[artifact_schema(id = ...)]` owners and are
used to form per-kind payload schema ids. They must not be changed to the
canonical artifact kind `s.stdio.semio`. Artifact references use the separate
dialect tuple `{ artifactKind: "s.stdio.semio", standard: "v1", subset:
"object" | "kit" }`.

The concrete metadata defect is confined to the aggregate GraphQL and Proto
facets: their invented `kind`/`mutation` plus opaque payload fields claim a
transport-owned envelope that native mutations do not have. The 24 descriptor
JSON files are source metadata for mutation registration and are not persisted
application data.

## Execution order

Kit's parent/snapshot/diff correction should first make the catalog/design
records consume Stdio base geometry and make children/links consume their Store
owners. The mutation TypeScript, GraphQL, Proto, and six shared-definition leaf
schemas can then refer to those same canonical records without snapshot-owned
copies. The native aggregate shape and committed mutation vectors should remain
the oracle unless a separately tested casing decision changes the Rust wire
contract. Object remains root-owned and was not edited here.
