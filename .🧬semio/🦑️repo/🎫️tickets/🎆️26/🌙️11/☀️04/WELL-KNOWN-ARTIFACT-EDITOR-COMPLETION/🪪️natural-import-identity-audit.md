# Natural Import Identity Audit

Current source proves identity collisions between two newly created stdio apps of the same surface. This audit is read only; it does not claim browser execution.

## Current reference birth

- Plugin declaration surface/editor-app Default sets id to surface_app_id(dialect, role). ArtifactApp::instance_id returns this controller surface ID. ViewerApp has the equivalent override.
- Plugin instance/staging/ingress VcsArtifactApp::with_registry_on_bus reads app.instance_id into app_id and passes it directly to create_document_envelope.
- Store/document create_document_envelope_from_genesis_owners copies the provided id unchanged into ArtifactEnvelopeOwners.id. It does not mint another identity.
- Plugin instance/plugin-app-impl document_identity returns the live envelope.id as Some when its character length is 1 through512.

Therefore separate TXT editor instances currently share s.stdio.txt@utf-8/*#editor as actual parent_document_id. Numeric runtime instance IDs are allocated separately, after app construction, and do not repair this. A SHA256 path based on this ID is also identical and must not be written twice.

## Natural import

Natural-file-and-media-artifacts document_reference_load_emit encodes the decoded snapshot and an empty-identity SPR into LoadDocument. InstanceStores::stamp_load_document_identity stamps the live store envelope.id, schema, dialect and owner onto it. Both ordinary and retained publication lanes use this stamp. The import preserves genesis identity; it does not mint another identity. read_document_identity returns snake_case parent_document_id from this store.

## Native original birth

Plugin instance/construction/factory retains definition/actor/registration/instance_id source identity, but passes source.definition.id to OriginalArtifactStoreSet::admit. The store set uses this as document lane label and suffixes config/draft/interaction labels. OriginalStoreGenesis::admit constructs the ID by concatenating O::identity_parts(label). Stdio currently has no authored OriginalArtifactAppBirth implementations found by source search, so its current reference path above is decisive. Native generic original birth also needs a distinct document seed when extended.

## Existing authoring seam

Framework os identity/time_ordered_id is an existing own-code UUIDv7-shaped generator using platform entropy and clock implementations for native Linux/macOS/Windows, browser and P2. It is independent of controller IDs and has no external runtime library contract. Use this or an explicitly fallible entropy-backed document birth authority at creation, before an archive exists. Keep app_id for action/controller ownership and use a separate minted document_id for envelope genesis. Never alias in the host or rekey only catalog metadata. Native original construction must retain its exact minted seed across yields and cancellation, and document lane birth must consume that seed independently of controller identity.

Required regression: two same-format apps have different nonempty parent IDs; each exported SPR agrees with its own ID; importing natural bytes preserves each identity; archive reload restores the exported identity; controller IDs remain canonical surface IDs. Validate generated identities using an independent UUID parser/schema.

## Full archive reopen identity

The full archive loader has different identity semantics from natural import. In instance/docs/staging/maintenance-ladder HydrateParent, expected.artifact_id comes from decoded history.doc_id, not the current store envelope. RetainedPersistedDocumentHydration::from_pack receives that archived expected reference, original parent pack and history; its returned envelope goes directly into begin_persisted_document_store_replacement. Thus full archive reopening adopts the saved identity. Existing real test a_whole_document_media_import_is_a_stepped_archive_load_and_never_folds_inside_the_call asserts consumer identity equals source after begin/poll/ack. Added a distinct source/consumer pre-load assertion so the adoption proof cannot pass vacuously when genesis IDs collide. Rust execution pending shared compile queue.

## Reference Test Dependency Repair

The real reference identity compile reached os-store-testing-fixtures and failed on two imports of physical_test_close_grant, gated by the value crate testkit feature. Enabled testkit explicitly on that test-only fixture crate normal first-party value dependency and removed its redundant dev-only duplicate declaration. This prevents reliance on incidental feature unification when the fixture crate is a dependency. Editor agent owns the queued retry; no passing reference Rust result is claimed here.
