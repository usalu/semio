# Note Document Contract Ownership

Native Note owns a `linked_artifact: Option<Store::ArtifactLink>`; snapshots and diffs encode `linkedArtifact`. Non-Rust facets omit it, and the Protobuf diff still includes editor camera, selection and engagement fields. The root parser also loses block payloads and leaves assets unvalidated; the diff's added collection has the obsolete direct-node shape instead of native `{parentId,index,block}` entries.

The correction will reuse shared Store link and blob identity contracts across languages, preserving IO's shared ArtifactRef. The existing native types define the link as `target`, `pin`, `role`; pin is `head`, `checkpoint` with `id`, or `snapshot` with a BlobRef (`hash`, `size`, `mediaType`). Native fixtures and independent Ajv checks will determine exact optional-field and nested-payload behavior before changing Note projections. Window camera and draft migration remains separately tracked in the continuation inventory.

The shared addressing neutral vectors now include all three pin variants and nine malformed/foreign-field cases, with TypeScript/Ajv and native first-party codec tests written before the shared facets are implemented. Validation is pending.

## Shared Link Boundary Checkpoint

`shared-link-addressing-red-1.log` failed on the expected missing shared link schema import. Added Store-owned ArtifactLink/LinkPin and BlobRef schema facets (Rust, JSON, TypeScript, GraphQL, Protobuf), importing the existing shared IO reference. Native type declarations moved into their owner schema directories and remain reexported through Store. Unknown fields are refused at the link, pin and blob boundary. `shared-link-addressing-green-1.log` has passed the TypeScript/Ajv oracle for 3 pin variants, 9 malformed/foreign links, 3 child identities and 5 foreign child records; native compilation is still running.

## Current Contract and Fixture Result

`note-document-contract-red-1.log` reproduced Ajv rejecting a valid `linkedArtifact`. The 12 non-Rust document facets now import shared Store link/blob/child definitions, describe all six block variants, retain native text child records, use identified `{parentId,index,block}` additions, and omit obsolete editor-only Protobuf diff fields. TypeScript snapshot and diff parsers reuse the artifact's domain validators.

`note-document-contract-green-1.log` exposed that a broad fixture scan also picked up schema files. The shared testkit now accepts explicit `mutationRoots`; Note passes only each subset's current fixture root, and Forms, Playbook and Norm were updated to the same API and current fixture locations. GREEN-2 reached the expected stale document diff fixtures.

All 99 Note fixture corrections are listed in `note-fixture-file-ledger.md`. The source explicitly establishes the original empty paragraph record and the edited `Hello, note.` record; existing child identities are preserved. Seven removed editor fields were asserted null before removal. `note-document-contract-green-3.log` is terminal GREEN through Nx: production TypeScript parsers agree with Ajv on 66 native snapshot files, 33 diff files, all authored block variants, shared links and three additional sparse delta vectors; strict TypeScript also passes. Full native Note fixture tests are running in `note-document-contract-native-1.log` and remain unclaimed until completion.

Shared link addressing is separately terminal GREEN in `shared-link-addressing-green-1.log`, including two native tests and independent Ajv validation. All three pin variants preserve wire identity and all nine foreign/malformed link records are rejected.
