# Weather Host Integration

The production canonical browser actor path is Store worker ReservationTurnEffects.routeTurnEffects. Ordinary background turns currently reject all effects except send-message, publish-event and spawn-job, including LinkResolve. The older shared ShardClient has no production onHostEffect handler in PluginRuntime runtime-singletons. Both paths need actual request completion, cancellation and actor-lifetime custody.

ArtifactState.currentPack is an immutable bootstrap baseline, not the live current head. It must never answer head resolution. The live target actor must publish the requested snapshot through its own document read protocol. AppChannel DocumentReads provides ReadDocument and ReadDocumentArchive, but the newer reservation path currently has no equivalent read facade. Checkpoint pins must use exact history content; snapshot pins must fetch and validate their addressed blob.

Existing native MemberLinkResolver resolves directory head/checkpoint and BlobStore snapshot pins. SpaceHost implements MemberDirectory. The resolver now returns Result<LinkState,VcsError>, preserving head/checkpoint/blob read failures rather than collapsing them into missing content. Its production-source native regression passes all three failure paths and actual absence. The resolver still trusts the link-declared dialect; an authoritative service must validate actual target identity.

No fallback to synthetic weather for a configured but unresolved link is acceptable. The energy request receiver is being connected independently to the real original completion registry.

Native check: ticket `🧪️link-resolver` harness compiles the exact production resolver source with its real first-party dependency types (no source copies or stand-ins). One regression passed. Existing Store history law now also asserts an unknown checkpoint remains UnknownChange; full Store history law still awaits its dependency graph.

## Native Snapshot Read Transport

The lifecycle agent is adding an explicit `semio.snapshot.pack` native media-export capability for the EPW editor, using the existing SubmitMediaExport/PollMediaExport/TakeMediaExportChunk/CancelMediaExport protocol and actual retained snapshot source. AppDocumentIdentity now reports its actual base_revision. No new WIT API is needed. The host must read that actual metadata and submit against the exact revision; a stale admission must refuse or recapture explicitly.

Added the bounded host payload consumer at Store worker reservation `📖️snapshot-read/🟦️.ts`. It validates original handle identity on status and every chunk, expected MIME/length/capacity, truncation, cancellation and progress. It zeroes original transport chunks after copying or refusal; a foreign admission handle is never cancelled. Its schema and shared hostile transcript fixture have an independent Ajv/Buffer test: seven transcripts pass. This consumer is not wired to the target reservation yet.

Next host work: add a reservation native command query lane using the existing command page admission and AppFrame codec. Private query sequence numbers can use the upper u64 range, separate from user action safe-number sequences. Do not steal unrelated target background frames: route only matching DocumentIdentity/MediaExport replies to a pending query sink and route unrelated publications normally. Target queries must preserve original actor/lifetime and cancellation until chunk EOF or exact cancel receipt. Existing ordinary routeTurnEffects rejects LinkResolve; add one ledger-backed request branch returning the actual Event::Completed bytes on original request id. Actual target dialect comes from its verified execution lease, not authored link fields. Scope target lookup to the requesting space and handle exact checkpoint/blob pins without reading or changing bootstrap currentPack.


## Snapshot Pin Transport Verification

The browser link broker now reads exact snapshot pins through the real authenticated `GET /blobs/{hash}` server route. It keeps the original source actor and same-space target lease checks, enforces the declared Pack media type and 32 MiB limit, uses the existing bounded streaming body reader, and independently verifies the exact BLAKE3 content address before native delivery. Hashing yields every 64 KiB and checks cancellation, deadline and original owner throughout. No currentPack baseline supplies a head.

The native completion ledger and late in-flight source retirement tests pass. The snapshot verifier passes a valid original body and five refusal transcripts (hash, length, declared media type, cancellation and stale owner). The neutral `abc` content fixture is independently verified with test-only Python blake3 1.0.11 through `🧪️snapshot-pin-oracle.py`. Duplicate JSON member refusal and exact maximum-u64 blob sizes also pass through the real first-party JSON decoder.

Remaining application verification: native producer-to-host reader integration waits for the full editor compile; checkpoint selection is under implementation; closed targets still require an independently admitted read actor instead of fabricating an execution lease. These tests do not establish those remaining paths.
