# Authenticated Checkpoint Publication Command

## Current outcome

The schema-first public command and the complete source-level Hub→MCP process harness are implemented. Native and direct-process receipts remain pending; the first current warm native retry stopped during the shared React plugin reactor build before any publication law ran.

The production route is `POST /spaces/{spaceId}/documents/{documentId}/checkpoint-publications`. It accepts only an exact canonical `semio.hub.checkpoint-publication-command/v1` naming already-uploaded Pack/SPR hashes and lengths. Scope, actor, descriptor, active checkpoint, actor generation, storage owners, checkpoint identity, and publication event remain server-owned.

## Production authority

- Only an authenticated current `Author` may claim a command. Share and spectator callers are rejected before a receipt is consulted.
- The DB document actor returns one retained `CheckpointPublicationSnapshot` with its exact committed frontier, nonzero authority generation, and last committed mutation id. Reopen reconstruction retains the tip id even below a materialized snapshot floor.
- The command's descriptor, document frontier, baseline frontier, and required `none | active` current-checkpoint value are expectations. The trusted materializer receives only the actor-derived frontier and current parent.
- Normal websocket writes and final checkpoint publication use the same exact `DocumentWrite(DocumentScope)` gate. Blob reads, codec work, reserve, stage, and readback occur outside the gate; the final publisher reacquires live authorization and the gate, then rechecks descriptor, current checkpoint, and actor snapshot before the atomic directory decision.
- `(author user id, correlation id, SHA-256(exact canonical command))` is durably claimed. Success atomically completes that claim with the checkpoint event. A returned failure now awaits claim release before returning; disconnect/drop retains a best-effort async release owner.
- The HTTP operation has a 30-second deadline, request-local cancellation, a 50-ms revocation monitor, an 8-KiB command ceiling, and a 1-MiB pair ceiling.

## Runtime laws and process design

Registered native laws:

1. `checkpoint_publication_route_is_author_owned_actor_fenced_idempotent_and_cancellation_safe`
2. `checkpoint_publication_route_rejects_stale_or_cross_scope_inputs_before_publication`

The second law includes queued-write drift, descriptor replacement after materialization, revocation cancellation, no checkpoint publication, and synchronous failed-claim reclaim.

The registered process phase adds `checkpoint_publication_process_fixture_emits_verified_gis_pair_and_catalog`. That exact Rust law emits:

- a real `semio_s_plugin_gis::GisMapSnapshot` `ArtifactPack`;
- a real first-party empty SPR;
- real `db.pathmap.v1` mutation/inverse bytes; and
- a production-loader-verified GIS Map descriptor/native-codec catalog generation.

The Bun process path then starts an actual Hub, creates and announces a private GIS Map document, obtains an authenticated open plan and one-use socket grant, commits one actor-stamped binary websocket mutation, uploads the exact pair, calls the public publication route, proves identical lost-response replay, starts the credential-FD MCP binary, and verifies `resources/list`/`resources/read`, exact bytes and SHA-256, same-document cross-space denial, and no checkpoint body after a descriptor rotation reaches the MCP binding.

The test-support catalog deliberately has synthetic component/browser-actor bytes, which are never executed. This process law is a cold canonical-pair publication/read law, not GIS actor activation, inference, rendering, Store mounting, or durable Map approval.

## Evidence

Source gate:

```text
checkpoint-publication-command-oracle: valid=2 rejected=12 ajv=1 typescript=1 sha256=2 actor-snapshot=1 final-writer-fence=1 durable-idempotency=1 process-route=1
checkpoint-publication-check: checks=22 phase=source
NX Successfully ran target checkpoint-publication-check for project os-hub
```

First current native attempt:

```text
checkpoint-publication-exact/exact-cargo-laws-ZBduvw/00
build: status=101
shared dependency error: plugin reactor turn/line 601 lifetime escaped PATCHES.with closure
publication laws run: 0
```

This is an unqualified build failure, not a publication-law failure. Root has repaired the shared reactor owner and is compiling that cohort before this lane retries its exclusive warm target.

Registry generation refreshed 59 plugin crates, 60 playgrounds, and 45 framework packages. The isolated immediate freshness retry completed successfully with `plugin registry generated catalog and launch bytes are fresh.` Generated launch command lines are 6778/6786/6804 for source/native/process and 5027/5043 for guest lifecycle source/native.

