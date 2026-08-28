# Immutable Mutation Registry Contract

## Frozen Identity

The registry envelope owns mandatory immutable schema id, schema version, state class, the complete fourteen-field `MutationLeafDescriptor`, and the four-field `SemanticDescriptor`. All fields are private, exposed through read-only getters. There are no partial constructors, compatibility optional semantic fields, contributor/artifact-kind patch-up builders, or mutable fingerprint fields.

`MutationDescriptor::new(SchemaId, SchemaVersion, StateClass, MutationLeafDescriptor, SemanticDescriptor) -> Result<MutationDescriptor, MutationDescriptorError>` validates nonblank id, positive schema version, the existing full leaf schema, approved semantic verb, nonblank entity/record and exact leaf/semantic kind agreement. The constructor does not invent descriptors or perform filesystem I/O. Source ownership remains the existing compiler-derived provenance and aggregate validation boundary; machine-local provenance tokens are not runtime registry identity.

Getters are `id() -> &SchemaId`, `schema_version() -> SchemaVersion`, `state_class() -> StateClass`, `leaf() -> &MutationLeafDescriptor`, `semantics() -> &SemanticDescriptor`, and `fingerprint() -> &[u8; 32]`. Serialization uses camelCase envelope fields `id`, `schemaVersion`, `stateClass`, `leaf`, `semantics`, `fingerprint`.

## Fingerprint Encoding

The exact byte preimage is UTF-8 `semio.mutation-descriptor/v1` followed by one zero byte, then compact JSON with keys in this fixed order: `id`, `schemaVersion`, `stateClass`, `leaf`, `semantics`. The leaf retains the published fourteen-field order and enum spellings; semantics order is `verb`, `entity`, `kind`, `record`. State class uses its actual serde spelling (`Artifact`, `Config`, `Presence`, `Transient`). Arrays preserve declared order. No sorting, physical path, workspace token, source timestamp, or fingerprint is part of the preimage. Every immutable identity field contributes; no schema-only golden value is preserved.

Use the existing repository-owned `semio_framework_hash::Sha256` implementation. An independent Node crypto SHA-256 oracle and JSON Schema/Ajv fixture validation verify the exact bytes and field sensitivity. This introduces no runtime dependency.

## Collision And Batch Semantics

`MutationDescriptorRegistry` owns a private map and exposes `new`, `len`, `is_empty`, `get(&str) -> Option<&MutationDescriptor>`, `register(MutationDescriptor) -> Result<(), MutationDescriptorError>` and `register_all(impl IntoIterator<Item = MutationDescriptor>) -> Result<(), MutationDescriptorError>`.

Equal same-id registration succeeds without replacing the established entry. Unequal same-id registration returns an explicit `Conflict` with id and existing/incoming fingerprints; equality compares the complete immutable envelope, not only its hash. A batch fully preflights both existing entries and same-batch duplicates before committing any new entry. A conflict leaves every existing value and registry cardinality unchanged. Empty batches succeed. Binary tags and opcodes are unique per owning mutation root, never globally across unrelated owners; compile-time roster validation remains responsible for that constraint.

The single global registry wraps this same owned registry in the existing lock. Public `register_mutation_descriptor` and new `register_mutation_descriptors` return `Result<(), MutationDescriptorError>`; `mutation_descriptor` continues returning an optional cloned immutable envelope. `MutationDescriptorError` is an owned repository type implementing Display/Error, with `InvalidField { field, requirement }` and `Conflict { id, existing_fingerprint, incoming_fingerprint }` cases.

## Generated Registration And Callers

The existing aggregate derive emits `register_<aggregate>_descriptors(state_class: StateClass) -> Result<(), MutationDescriptorError>` (with existing generics and where clauses preserved). It forces the validated full leaf roster, constructs every complete envelope with `?` before registration, then invokes the single atomic batch API once. State class is an explicit caller decision, not silently Artifact for config owners. Both existing derive mirrors change together; no competing derive or generated implementation is introduced.

Existing test callers pass Artifact explicitly and assert success. The OS config wrapper returns Result, passes Config explicitly, and propagates failures. Recheck all transitive startup consumers before acceptance. Separate config aggregate calls are not claimed to form one transaction until the wrapper collects them into one batch or a separate owner-level all-or-nothing API is established.

## Tests And Ownership

Root owns the shared command registry, public reexports, derive emission and immediate caller propagation. The registry review lane owns a new ticket-only neutral matrix/reference oracle and genuine compiled-client probes in `🧪️registry-immutable-32`; it does not recreate the vanished `🧪️registry-contract-review`. Pre-change failures and post-change results must be separately recorded. Root owns Cargo serialization and reuses the demonstrator's explicitly loaned existing target without deleting any output.

Mandatory cases: every identity field changes its fingerprint; invalid required metadata rejects; equal idempotence; conflict without replacement; same-batch conflict; conflict against an existing entry; batch success; empty batch; distinct owner wire tag reuse; no partial commit; all fields survive lookup; generated caller propagates failure. The stale core miniature inline mutation fixture must become a real canonical direct leaf before full kernel test acceptance. This packet does not accept the full monorepo goal, Workflow/Run behavior, or all startup reachability.
