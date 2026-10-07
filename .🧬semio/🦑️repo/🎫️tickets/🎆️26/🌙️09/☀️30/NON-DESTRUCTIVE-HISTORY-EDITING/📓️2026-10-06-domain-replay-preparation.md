# Domain Replay Preparation Capabilities

Current shared contract: Store/🔁️replay/🎮️operation/🦀️.rs exposes ArtifactReplayPreparationFactory/P/M and a domain advance borrowing the original mutation and optional InputReplacement. DocumentStoreOwners::with_replay_preparation retains the factory. Its grant permits at most one semantic item and charges real payload/scaffold bytes; cancellation transfers retained owners into bounded close. Store integration is being authored by Core.

## Concrete Missing Producers

| Family | Current Authority | Missing Controlled Semantic Units |
|---|---|---|
| Puzzle2d | Editor Puzzle2dArtifactStorePreparation::advance phase 0 calls inverse, diff and apply synchronously | Input replacement; node/edge/catalog search; inverse row construction; sparse delta construction/application; mutation/next retirement |
| Writer | Editor prepare_writer_artifact calls inverse, clones entire base, applies diff synchronously; live factory additionally restricts vocabulary to EditText/SpliceText | All five semantic mutation variants, scalar UTF8 reconstruction and splice search, independent inverse refusal, next ownership, input replacement, bounded close |
| Generic document owners | Plugin bounded_document_store_owners forwards bounded_config_store_owners | No replay preparation factory installed by this generic wrapper; its live publication authority must not become a false cooperative semantic producer |

These are source findings, not runtime failures or execution claims. Existing final-serialization cancellation and scalar reconstruction are insufficient proof that arbitrary semantic inverse/diff/apply yields. Reusing a synchronous body behind a per-tick wrapper would violate the required controlled work. Drawing is Core's actual producer and will supply the first concrete retained semantic template; follow-up family adoption must retain source-neutral and independent expected-output laws.

## Writer Primitive Authority

Writer has five semantic variants: RenameWriter, ChangeUri, ChangeLanguage, EditText and SpliceText. The first three publish one sparse scalar; EditText and SpliceText additionally derive a content-addressed Semio Document child. That identity is currently produced by document_snapshot_from_text, JSON projection, content_id hashing and an immutable local Arc text owner. SpliceText::diff and inverse synchronously call TextSplice::apply, which relocates context and constructs both the new text and inverse. A controlled Writer producer needs incremental child snapshot/JSON/hash ownership and the retained splice relocation/apply/inverse primitive; a scalar-copy-only cursor would not cover those semantic owners.

WriterDiff intentionally owns custom physical JSON text/binary codecs rather than the shared diff_text record macro. Its absence of DslRecord alone is not a demonstrated compiler defect, so no speculative derive/codec rewrite was made. Puzzle2d actually calls the generic typed-record text/binary macros, which establishes its different metadata requirement.
