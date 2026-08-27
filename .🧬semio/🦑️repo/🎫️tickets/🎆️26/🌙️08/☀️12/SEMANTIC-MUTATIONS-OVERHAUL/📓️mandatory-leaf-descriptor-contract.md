# Mandatory Leaf Descriptor Contract

## Coordinator Decision

The full leaf metadata type is `MutationLeafDescriptor`. Its serialized representation exactly matches the14 required fields in the existing language-neutral descriptor schema. There is no `Default`, omitted-field fallback, optional opt-in trait or compatibility descriptor.

| JSON field | Rust field and type |
| --- | --- |
| schemaVersion | `schema_version: u32`, exactly1 |
| owner | `owner: &'static str` |
| semanticKind | `semantic_kind: &'static str` |
| displayName | `display_name: &'static str` |
| emoji | `emoji: &'static str` |
| aggregateVariant | `aggregate_variant: &'static str` |
| payloadSchema | `payload_schema: &'static str` |
| textOpcode | `text_opcode: Option<&'static str>` |
| binaryTag | `binary_tag: Option<u32>` |
| invertibility | typed four-value `MutationInvertibility` |
| diffParticipation | typed four-value `MutationDiffParticipation` |
| outcomeClasses | `&'static [MutationOutcomeClass]` |
| composition | typed two-value `MutationComposition` |
| requiredLanguageSurfaces | `&'static [MutationLanguageSurface]` |

The enum wire values remain exactly those in the schema. `textOpcode` and `binaryTag` are required nullable properties; null does not mean an omitted field. The schema's binary-tag upper bound becomes4294967295, matching `u32`. Source/provenance is separate from the14-field serialized object.

`SemanticDescriptor` remains the distinct semantic/history vocabulary also used by inference contracts. It is not replaced by, or confused with, the full mutation leaf metadata. The end-state `MutationKind` requires both `DESCRIPTOR` and `SEMANTICS`, with no defaults; their semantic kinds must match. Shared identity should reference the descriptor value rather than introduce a second independent literal. The aggregate derive must expose full descriptor iteration and `From<Leaf>` conversions, reject invalid/duplicate metadata, retain mechanical behavior delegation, and never infer missing metadata from a variant name.

## Ownership Proof and Remaining Freeze Boundary

An owner string that merely looks like a mutation path is insufficient. The mandatory derive/leaf contract still needs compile-time source provenance or an explicit validated metadata contribution to compare the real direct owner with the descriptor. Repository source-path verification remains an independent gate, not a substitute for the requested derive rejection. No claim is made that Rust alone automatically provides this proof.

The current runtime `MutationDescriptor` is a registry envelope containing schema/state/fingerprint information, not this14-field object. Full leaf metadata must become mandatory for mutation registration without assigning mutation-only fields to inference metadata. Constructor/registry propagation and duplicate-registration behavior require an explicit consumer transaction; a nullable compatibility metadata field is rejected.

## Independent Read-Only Evidence

The Luna audit located339 direct descriptor files with the exact14-key shape;182 declared text/binary identities and157 declared null wire identities. Observed binary tags were0–17. It reported339 direct Rust leaves and145 observed mutation roots plus31 files mentioning registry-related names. These are the audit's current coverage counts, not an exhaustive acceptance census or the fan-out of all remaining legacy `MutationKind` implementations. The earlier global inventory covered additional legacy and central-only records.

Authoritative source paths:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` and its source component mirror

## First Implementation Packet

FND-METADATA-TYPES-03 implements the exact complete type/enums, non-default validation helpers, matching schema upper bound, explicit facade reexports and neutral/compiler-backed tests. It does not silently wire a partial metadata contract into `MutationKind`, mutate artifact leaves or claim the full foundation complete. The following mandatory trait/derive/registry cutover must be separately assigned with its complete source-owner and consumer fan-out, then every remaining leaf must converge before the goal can close.

Current type implementation state: the exact types and const-validation extension are implemented and independently accepted as a bounded foundation packet. Five unchanged actual-source Rust tests pass,20 neutral roster vectors are covered, and valid/invalid compile-time assertions produce the expected compiler results. The roster scope is the normalized aggregate mutation-root; each descriptor's owner remains the full distinct direct-leaf path. Core source ownership is released after the final path-safety correction. Existing `MutationKind`, `CompositeMutationKind`, `SemanticDescriptor`, runtime registry and artifact leaves are not changed in this bounded stage. A registered kernel selection is running separately; no full trait/registry cutover is claimed.

The coordinator independently completed the narrow schema boundary preflight while execution capacity was unavailable. The eight language-neutral vectors in `🧪️descriptor-contract-preflight/🔣️vectors.json` first produced one real mismatch: JSON Schema accepted4294967296 while Rust's `Option<u32>` rejected it (`🧪️red.log`). Adding the frozen4294967295 maximum to the authoritative schema corrected this. The replay passed all8 expected results against Ajv, the repository schema-subset validator and Rust's type checker (`🧪️green.log`). Null, zero and maximum are accepted; overflow, negative, fractional, string and omitted fields are rejected. This does not implement the full metadata type or mandatory trait cutover. The permanent descriptor-type test packet should reuse/promote these vectors into its registered test suite.

## Source Provenance Preflight

The follow-up Luna review identified `proc_macro::Span::local_file()` as a source-path authority available since the declared Rust1.88 baseline. The coordinator independently compiled a minimal metadata-only proc macro and executed two consumers: ordinary compilation and `--remap-path-prefix`. Both returned the actual physical direct-leaf file from `local_file()`; only the diagnostic `Span::file()` value changed under remapping. Evidence: `🧪️source-provenance-preflight/🧪️root-runtime-retry.log`, retained fixture `🧫️run-RltgxY`. The first harness attempt used a noncanonical dynamic-library filename and failed linkage; correcting the test artifact prefix produced the passing result. No production derive or ownership check has yet changed.

The intended source proof belongs to the existing mutation-kind boundary, not a second semantic descriptor. A metadata-only leaf derive can validate the real source location and sibling descriptor, then expose provenance required by the aggregate derive. Aggregate const checks must compare actual owner provenance with full metadata and variant identity. Manual provenance implementations remain technically spoofable; source policy must reject them independently. `file!()` is diagnostic, not provenance authority.

Filename and root authority must come from the governing taxonomy, with lexical no-follow checks before canonicalization and opaque paths rejected before traversal. The current taxonomy names Rust primary leaves `🦀️.rs` and descriptors `🔣️.json`; the pasted plan and existing production owners use `🦀️component.rs` and `🔣️component.json`. This is an explicit pending full-owner source/mount/descriptor cutover, not permission to accept both forms as compatibility aliases. The metadata-only type stage does not rename any production owner.

The existing TypeScript loader locates its authoritative taxonomy relative to the discovery module; its workspace marker is the ancestor containing `nx.json` and `📋️project.json`. A generic workspace-wide search for any similarly named taxonomy is not accepted as the final Rust authority discovery mechanism. Dependency tracking for descriptor/taxonomy bytes should use visible metadata-only `include_str!` dependencies or another verified compiler mechanism, never hidden mutation implementation. The source-authority resolution and final required marker API remain to be frozen before the mandatory derive cutover.

## Workspace Taxonomy Locator Decision

The coordinator freezes the workspace-to-taxonomy locator as `metadata.semio.taxonomy` in the root `📋️project.json`. Its value is one normalized repository-relative path to the governing taxonomy, currently `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. Root discovery requires the exact `nx.json` and `📋️project.json` pair. Both TypeScript discovery and Rust compile-time tooling must consume this locator without a hardcoded fallback or recursive filename search.

FND-TAXONOMY-AUTHORITY-06 owns the root metadata field, a narrow loader/locator region, dedicated language-neutral schema/fixtures and tests. It must reject lexical excluded/escaping/opaque paths before reading target metadata and reject symlink components. This packet changes no taxonomy vocabulary or production leaf filenames. The metadata-only source-proof derive follows only after this authority is independently verified.

The proposed metadata boundary is `MutationLeaf` with full `DESCRIPTOR` and `PROVENANCE`, and an end-state mandatory `MutationKind: MutationLeaf` retaining explicit `MutationKind::DESCRIPTOR` and `SEMANTICS`. The kind descriptor must exactly equal the derive-owned descriptor; it must not become a second handwritten identity source. Provenance contains mutation-root, full owner and source-path facts. No behavior belongs in the metadata trait/derive. This is the next contract direction, not an implemented optional compatibility path. Aggregate derive must independently validate its own source/root and every wrapped leaf connection; a handwritten provenance literal alone is not ownership proof.

The registered kernel selection now passed5 metadata tests, with869 filtered, after an actual2m55s compile and69 warnings. Evidence: `🧪️metadata-kernel-registered.log`. This validates the core type/const stage in its real crate; no other kernel tests or mandatory metadata consumer cutover are claimed.

## Source Path Preflight Extension

The extended compiler preflight passed all four cases, including relative compiler entry paths and parent-mounted direct leaf sources, under ordinary and remapped compilation. Evidence: `🧪️source-provenance-preflight/🧪️parent-mount-runtime.log`, fixture `🧫️run-GXTYrQ`. For `rustc consumer/main.rs`, `Span::local_file()` returns `consumer/../✏️s/🧪️probe/🧬️mutations/➕️insert-page/🦀️.rs`, not an absolute normalized path. Canonicalizing against the compiler working directory locates the actual leaf. The preflight itself is not a security implementation.

Source proof must distinguish compiler source paths from strict repository-relative manifest locators. Legitimate compiler parent segments must resolve correctly; locator parent segments remain forbidden. Excluded source components must be rejected before I/O, and every traversed component must be checked without following symlinks before canonicalization can erase evidence. FND-SOURCE-AUTHORITY-07 implements and tests this private compile-time boundary before public derive integration.

## Registry Fan-Out Decision

The bounded read-only audit found four textual `MutationDescriptor::new` calls: one in each existing derive mirror and two core tests. The only production registration construction is generated by those derives; no handwritten production constructor was found. The scan found86 production derive declarations,26 explicit generated-registration invocations (22 plugin test sites, one core fixture, three config runtime calls). These are qualified source-scan counts, not an exhaustive acceptance census.

Inference is already separate: `InferenceSpec` is in the core command module and its registry is in framework replication wire, forwarded by the schema facade. Inference leaves do not construct `MutationDescriptor`. Preserve this separation. The mandatory mutation registry envelope will contain the full leaf metadata; no optional metadata, `with_semantics` patch-up, or invented inference mutation descriptor is needed. The two existing constructor tests must receive real explicit metadata and the synthetic aggregate must satisfy the same source ownership rules as production. Registration collision behavior remains to be specified and tested before implementation.

The follow-up field audit found no persisted descriptor decoder: the current runtime descriptor derives Serialize only, its optional semantic fields are asserted only in core tests, and contributor/artifact-kind builder methods have no observed callsites. These qualified findings support removing compatibility-only optional semantic identity. The frozen collision rule is idempotent equal re-registration, explicit error on conflicting same-id metadata, and no overwrite. Binary tags/opcodes remain unique within the owning mutation root, not globally across unrelated roots. The new fingerprint must include the complete immutable schema/state/leaf/semantic identity; its old schema-only golden hash is not a compatibility constraint. Registry result propagation must include transitive startup callers, not only the three immediate config calls.

Legacy Note/FEM/DAG payloads under `🦠️mutation` are unconverted triads, not evidence for widening the optional-facet authority. They must be migrated. Do not restore that facet, replace the exact workspace marker pair with arbitrary project discovery, substitute unverified call-site spans for declaration spans, or demote actual descriptor JSON to fixture-only validation. Public source proof for genuinely authority-listed child facets is being checked separately.

The follow-up bounded scan found459 immediate directories with direct historical Rust/descriptor primary siblings, but no converted payload declaration under an authority-listed optional facet. Those facets currently contain codec/helper logic referencing the direct payload. This is qualified census evidence, not proof that such a future layout is forbidden. The first private source-authority packet explicitly proves direct declarations; any later optional-facet payload support must also prove the direct source's public reachability and ownership, never accept legacy triads.

The exact config registry symbol scan found no callers of `register_os_config_mutation_descriptors`; its only observed fan-out is the three generated registries inside that function. Result propagation therefore currently stops at its return type, subject to rechecking concurrent changes at implementation time.

## Lower Contract Placement Review

The live OS command source reexports `protocol::mutation::*`; the actual `Mutation<P>` trait is in `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs`, below OS kernel. The replication Cargo manifest has no OS dependency. A mandatory base mutation metadata requirement cannot reference the currently OS-owned descriptor by adding a reverse dependency. The single-source descriptor type/validators must move to the lower mutation contract, with explicit public facade reexports, if that base trait uses them. No duplicate descriptor type or metadata opt-in bypass is accepted.

The exact base trait/aggregate/generic roster API is under bounded read-only review before implementation. Direct runtime-state mutations and generic collection families remain included. FND-LEAF-JSON-08 proceeds independently with a private compile-time full JSON parser and typed token emitter; its OS facade target remains valid through deliberate reexport if the defining location moves. It does not implement the public trait transaction.
