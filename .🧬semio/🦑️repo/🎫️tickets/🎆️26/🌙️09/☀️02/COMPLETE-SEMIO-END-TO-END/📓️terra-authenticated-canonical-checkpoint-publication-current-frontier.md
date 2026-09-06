# Authenticated Canonical Checkpoint Publication Frontier

## Current Boundary

The proposed `POST /spaces/{space}/documents/{document}/checkpoint-publications` is the right shape: route-owned scope, a bounded canonical command naming only existing content hashes/lengths, and a server-owned authority/publisher. It must be a new document route, not `POST /directory/commands` and not a socket `ClientFrame`. The generic event append seam rejects `ArtifactCheckpointPublished` specifically (`🌎️hub/📇️directory/🦀️.rs:1877-1880`); only `DirectoryService::publish_reserved_artifact_checkpoint` can atomically consume a CAS reservation, private locators, public event, and projection (`🌎️hub/📇️directory/🦀️.rs:1974-1985`).

The schema and Rust/TS parser are now useful request grammars:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📣️checkpoint-publication-command-v1/{🧬️.schema.json,🔣️.json}`
- `.../🧬️schema/{🦀️.rs:363-480,🟦️.ts:328-432}`

They correctly keep space/document out of the body, cap the command at 8 KiB and the named pair at 1 MiB, reject unknown/noncanonical JSON, and carry descriptor, expected DB frontier, active-checkpoint expectation, baseline frontier, and Pack/SPR content identities. At review time they are not yet routed or consumed by Hub production code.

## Existing Production Pieces to Reuse

| Required operation | Existing owner | Constraint |
| --- | --- | --- |
| Route session+membership authentication | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2478-2485` (`authenticate_document_socket_subject`, `SocketSubject::revalidate`) | Publication must accept only `Session { role: Some(Author) }`; do not reuse canonical-pair authorization, which deliberately permits share tokens (`bin.rs:1858-1868`). |
| Current durable descriptor | `bin.rs:2490-2497` | Reload it, compute its digest, and require command equality before materialization and again before publication. |
| Current document actor | `bin.rs:2936-2947`, `db/🗿️artifact/🦀️.rs:5042-5053` | Obtain it with `ensure_document` and use an actor-owned checkpoint-publication snapshot query; do not race raw WAL/storage against the actor. |
| Bounded content read | `bin.rs:2951-3001` | Use the private payload-store read/copy/retire pattern, bounded by the smaller route/authority limit. Verify each actual byte length and SHA-256 after read; a URL/path hash is not proof. |
| Codec materialization | `🌎️hub/🗿️artifact-authority/🦀️.rs:498-555` | `ValidatingCanonicalArtifactAuthority` validates Pack/SPR through the trusted descriptor-selected codec and derives checkpoint identity. |
| CAS reserve/stage/readback/publish | `artifact-authority/🦀️.rs:435-490`; `directory/🦀️.rs:2213-2239` | Use `CheckpointPublicationOrchestrator` and `HubVerifiedCheckpointPublisher`; callers must never construct an `ArtifactCheckpoint` with a storage locator. |

## P0: Client Baseline Is Not Authoritative

`CheckpointPublicationCommandV1` validation proves only grammar. `ValidatingCanonicalArtifactAuthority::materialize_checkpoint` validates a caller-built `CheckpointRequest`, then derives the candidate from that request (`artifact-authority/🦀️.rs:342-390,498-555`). In particular, an empty operation list is legal and leaves `request.base_frontier` as the published baseline. Therefore a handler that copies `baselineFrontier` or `expectedDocumentFrontier` from the HTTP command into `CheckpointRequest` without a server comparison can publish a valid codec pair under a forged/stale lineage.

There is a second exact issue: `ArtifactHandle::frontier()` exposes `{ document, head_seq, commit_seq, chain_hash, epoch }` (`db/⚙️engine/🦀️.rs:3658-3664`) but not `head_edit_id`. Hub’s generic conversion presently inserts an empty id (`bin.rs:3214-3215`). The database sync model can derive a full `RuntimeFrontierSummary` only from its committed command sequence (`db/🔄️sync/🦀️.rs:201-206`), but direct `replay_sync_state` is a raw storage/WAL API and must not run concurrently behind the live per-document actor.

### Required correction

Before adding the route, add one narrow actor-owned read operation, for example `ArtifactHandle::checkpoint_publication_snapshot()`, returning an opaque-to-callers committed snapshot containing:

- exact runtime/document frontier including last committed mutation id;
- the corresponding full `ArtifactFrontier` projection;
- a monotonic actor version/fence suitable for a final equality check.

The route must compare all of the following against that snapshot before materialization and again in its publisher final fence:

1. route scope and current descriptor/digest;
2. `expectedDocumentFrontier` including epoch, head sequence, commit sequence and chain;
3. `baselineFrontier` including document, ordinal, last commit sequence, chain and tip edit id;
4. `expectedCurrent` against the active directory checkpoint (including its full baseline) or exact `None`;
5. copied Pack/SPR bytes against the command identities and the configured pair limit.

Only then may the route construct the private `CheckpointRequest` with `parent_checkpoint_id` from the active checkpoint and `base_frontier` from the actor snapshot. It must not accept a request-built operation list in this initial checkpoint-publication route: the named pair is already canonical input, so use zero operations after trusted codec validation.

## Final-Fence and Multi-User Contract

Home’s proposed request-local `VerifiedCheckpointPublisher` is the correct place for terminal authority. It should retain the authenticated subject, route scope, descriptor digest, actor snapshot fence, and active-checkpoint expectation; it must revalidate after every external await, particularly:

1. after both payload reads and before `materialize_checkpoint`;
2. in `reserve`, before the directory write/physical epoch advance;
3. after each stage/readback and immediately before `publish_reserved`.

The final revalidation must call the same session/membership authority as document opens and require `Author` again. A share, spectator, revoked session, changed role, descriptor rotation, changed active checkpoint, or changed actor frontier returns no receipt and performs no `publish_reserved`. Do not rely on an initial `authorized`, a public blob GET authorization, or the 30-second deadline alone.

Use a request-local cancellation/deadline control passed through both `materialize_checkpoint` and `publish_candidate`; cancellation before the CAS decision prevents publication. A post-reserve cancellation may leave unreferenced staged blobs/reservation to existing bounded CAS cleanup, but cannot emit a directory checkpoint event.

The `correlationId` needs a durable authenticated-user idempotency owner before a public success receipt is claimed. Existing `DirectoryCommandReceipt` state covers `/directory/commands`, not this new route. The minimal law is lost-response/retry with the same authenticated author+correlation: exactly one authoritative checkpoint event and identical public receipt; a different command digest under that key is conflict and another user cannot read the result.

## First Runtime Laws

1. Author posts exact bytes already present in the scoped payload store; the route reads/hash-checks both, compares the live actor frontier/descriptor/current checkpoint, materializes, reserves, stages/readbacks and emits exactly one active checkpoint receipt.
2. Spectator, share token, revoked/replaced session, different space, descriptor digest mutation, active-checkpoint replacement, actor command committed during a controlled stage, and full frontier/tip-id substitution all deny with no new directory event or CAS reference.
3. Same blob hashes from a different document in the same space are treated as untrusted input and can only publish if codec validation plus exact live frontier/descriptor checks succeed; never infer document authority from payload-store addressability.
4. Pack/Spr missing, length/hash mismatch, aggregate over limit, noncanonical request, cancellation/deadline during read or stage, and readback mismatch result in no receipt/no published checkpoint.
5. SQLite restart after reserve/stage before publish either has no active new checkpoint or one fully valid projected checkpoint; retry under the same correlation is deterministic and does not issue a second event.

## Nonclaims

The existing read-only canonical pair route (`bin.rs:2847-2933`) is an authenticated projection of the active checkpoint, not a publication authority. The existing `/spaces/{space}/blobs/{hash}` API (`bin.rs:2972-3014`) is a scoped payload transport, not a checkpoint-ownership proof. The checkpoint schema and test fixtures alone do not establish a user-visible publication until this route and its retained actor/final-fence path are wired and exercised.
