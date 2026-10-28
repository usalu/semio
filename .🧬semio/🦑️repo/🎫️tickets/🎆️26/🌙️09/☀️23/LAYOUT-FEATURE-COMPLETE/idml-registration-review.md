# IDML Registration Review

## Scope And Evidence

Read-only primary-source audit on 2026-10-21 of canonical stdio IDML native model, all twenty descriptor payloads, declaration and current stdio definition validator. Root and applicable s/stdio AGENTS were read. No native compilation or tests were executed; this report is a source audit. No source files were changed by this audit.

## Repaired Finding

- P1, repaired by main during audit: IDML declaration native_codec line22 called artifact_codec although IdmlSnapshot does not implement ArtifactSqliteSnapshotHost. It now calls bare_artifact_codec. Framework store codec factory/📥️lifted/🏪️store/🦀️.rs lines49-55 requires the relational host for artifact_codec; lines66-69 omit it for bare_artifact_codec, whose line210 obtains the actual optional snapshot_sqlite capability. IDML IO line54 implements ArtifactStoreSnapshot with its default absent optional SQLite capability. This correctly models current capability ownership without fabricating relational storage.

## Remaining Concrete Boundary Gap

- P2: IDML currently has no original retained semantic preparation implementation or issuer. Mutation schema Rust lines38-75 supplies Mutation and ColdMutation only; diff Rust lines69-94 supplies cold algebra/application only. Thus these types cannot be used by the retained semantic path requiring RetainedMutation and RetainedDiff, and an owner catalog made using funded_bounded_artifact_store_owners installs no one-item or replay semantic producer. Framework initialization/🏠️owners/🦀️.rs lines290-304 explicitly initializes those slots to None; lines716-723 and796-801 preserve that state. A retained batch submitted through store-cursor-operations/🦀️.rs lines324-325 is refused with the explicit app-owned ArtifactStoreOneItemPreparationFactory requirement; grant-based admission lines238-239 does the same. This is a retained publication capability gap, not proven loss of ordinary native codec edits.

The earlier hypothesis that this necessarily rejects codec apply was disproved by tracing actual primary code: codec factory line170 calls dispatch_apply_exact; cursor-admission-dispatch-and-apply-batch line218 invokes cold apply_command; live/history-lane-commands-and-outbound line87 invokes cold replay_mutations; store-owners-batch-fold-and-history lines393-447 performs the ColdMutation inverse and fold. It does not enter the retained batch gate. No codec edit refusal is claimed solely from missing retained issuers.

## Artifact-Owned Remedy Hooks

ArtifactStoreSnapshot (store/🚪️io/🦀️.rs lines36-45) exposes native_snapshot_registration and sqlite_snapshot_codec only; there is no semantic preparation provider hook there. Correct retained ownership needs an actual IDML RetainedDiff::Apply implementing RetainedDiffApply (replication/🎮️mutation/🧳️retained/🔺️diff/🦀️.rs lines8-9,28-38), and RetainedMutation::Preparation implementing RetainedMutationPreparation (retained/🦀️.rs lines38-52), with exact original source binding, demands, diagnostics, inverse and funded closure. Then supply a concrete original issuer implementing ArtifactStoreOneItemPreparationFactory plus ArtifactReplayPreparationFactory via the initialization owners APIs: bounded_artifact_store_owners_with_shared_semantic_preparation lines752-758 or funded_bounded_artifact_store_owners_with_semantic_preparation lines762-765. Those are explicit source-injection APIs; adding a global cold fallback would bypass their contract. An existing artifact exemplar was not established in this bounded audit.

## Declaration And Facet Checks

The descriptor Rust lines6-33 includes twenty real existing facet payloads: five each for root/snapshot/diff/mutations. Snapshot native wire and TS/JSON/GraphQL/Proto facets retain domVersion and ordered parts with byte payloads. Diff retains ordered steps, optional version/order and nullable removal bytes; Protobuf uses explicit RemovedPart and oneof for bytes/removal. Mutation facets agree on replacePackage/replacePart payloads. Protobuf/GraphQL are structural contracts; no generated conversion/runtime claims were verified. TypeScript package parsing explicitly delegates XML/package relationship admission to the codec; it does not validate XML/DOMVersion equality itself.

The current artifact definition matches the visible validator rules: standards, profile/source/representation identities, two exact locales, executable codec with nonzero authored hash and native factory receipt, direct resource/conformance leaves, runtime codec/representation claim sets and complete ledger references. Opaque support ledger states intentionally do not claim normative implementation conformance (assembly validator lines583-603). No concrete new manifest refusal was identified.

The factory/schema receipt path verifies actual native schema, extension and record hash (assembly lines836-854 and895-908). IDML native IO lines18-22 exposes the real Value record spec; pack lines42-50 checks package admission on output/input and envelope identity. Snapshot controlled decoding lines35-40 performs controlled wire decode and package admission with checkpoints. Package admission's internal ZIP/XML allocations remain synchronous; this source audit does not establish bounded cancellation within a single XML parse.

## Native Verification Limit

Framework bare_artifact_codec invokes cold dispatch while its displayed generic bounds do not spell ColdMutation (factory lines66-69 versus dispatch lines209-212). This is a generic framework type-check risk, not a confirmed IDML-specific compile result; no build was run. Native retained publication/replay/cleanup must be exercised after any IDML producer work. No data-loss defect was established from inspected code.
