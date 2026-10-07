# Independent OS Retained Caller Compiler Prerequisite Audit

Original MissingBefore41153 is compiler-only: 28 errors, no runtime. Immutable owns 14 actual retained transport demands. The other 14 errors reduce to retirement namespaces, ValueRefusalKind import, three Sync bounds, one genesis capability bound and one unit fixture authority issue. Attribution here is by error locus, not by assumed author of concurrent changes.

Canonical retirement functions are public semio_framework_value::retirement::{owned_retirement,shared_retirement}, not value root exports. ArtifactGenesisRetirement is defined in os_store::component. ValueRefusalKind is exported by semio_framework_value. Actual ArtifactGenesis<P> stores Arc<P>, making P:Sync necessary wherever retained targets must satisfy Send; do not weaken retirement or target Send supertraits. This explains FreshRecordTarget, MemberStoreOpenRetained retirement and owned_test_envelope bounds.

parse_document_text calls replay_ops, whose exact requirement is ArtifactGenesisCodec. Add that capability rather than the broader ArtifactPack requirement: ArtifactPack supplies a blanket implementation, but the lower-layer capability is the actual authority.

During exact region capture, concurrent fixes superseded the original source. Readback already shows retirement::shared_retirement, retirement::owned_retirement, FreshRecordTarget P:Send+Sync, and parse_document_text P:Clone+ArtifactDsl+ArtifactGenesisCodec. An obsolete exact before anchor was rejected, so no stale narrowed guard artifact was saved. Rebase other regions against current source before mounting.

The unit fixture completed_record_owner returns ArtifactEnvelopeCompletedRecord<(),()> and previously called create_document_envelope with (). That constructor requires ArtifactGenesisCodec; unit has no current implementation. The actual create_document_envelope_from_genesis requires Clone and genuine ArtifactGenesis, enabling test-only construction with canonical unit Pack bytes and verified digest. Alternatively change to an actually encodable fixture and corresponding factory/types. Do not invent an empty-byte codec or bypass digest authority merely to compile. This remains a constructor authority question distinct from transport APIs.

No production edits, compiler reruns, or runtime success credit from this audit.
