# Artifact Bootstrap Owner Current Audit

Read-only source audit on 2026-09-06. I did not run the active `91713` fixture gate.

## Confirmed repair

`captureArtifactBootstrapOwner` now captures the socket, client, target-open attempt, private lease, Hub binding, installed target, schema and pack hash at [backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1926). Both chunk and done handlers retain that exact owner and pass it to `rejectArtifactBootstrap` ([lines 2136-2164](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2136)); an old done catch therefore cannot clear a successor assembler or close its replacement socket. The transfer pair is copied into `currentPack/currentSpr` before its source views are wiped in `finally` ([lines 2043-2066](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2043)). That is the right returned-byte ownership boundary.

The fixture now covers owner replacement after hashing and progress, plus the intended lease vectors ([corpus](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧵️artifact-bootstrap-owner-v1.json)).

## Resolved in the current in-flight source: leased cold-pair binding

The initial audit snapshot found the `lease-*` corpus ahead of implementation. The current source now performs the needed relation in `validateArtifactBootstrapIdentity` ([backbone-worker.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2014)): it checks lease scope and normalized Hub origin, the artifact triple, `descriptorDigestV1`, required checkpoint, aggregate hash, full baseline frontier, and the existing Welcome-tail fence. If a browser grant is present, it also invokes its private current-owner assertion. This is the right field mapping. The current fixture run was still active when reviewed, so this is source confirmation rather than a qualification claim.

## Historical P0 repaired by that binding

Before the current patch, the assembler's expected descriptor was `bootstrap.descriptor_hash` itself, an internally consistent but non-authoritative comparison. The required relation is:

- `bootstrap.descriptor_hash` is the lower-hex bytes of `leaseFields.descriptorDigestV1` — not `leaseFields.descriptor.sha256`;
- `bootstrap.artifact_schema`, `artifact_kind`, and `pack_schema_hash` equal `leaseFields.artifact`;
- lease scope equals the captured Hub space and document;
- a cold bootstrap under that lease has `checkpoint`; `bootstrap.baseline_frontier` equals its full `baselineFrontier`, and `bootstrap.aggregate_hash` equals `checkpoint.aggregateSha256`;
- the existing required-tail versus Welcome frontier check remains independent.

`lease-valid`, descriptor, aggregate, baseline, missing-checkpoint, scope, kind and dropped-lease rows form the correct first acceptance corpus. The active law should also assert that an invalid row neither creates an assembler nor mutates the retained prior pair. A non-lease Hub replication client can retain the config-only path.

## P1: stale folder persistence has no final owner fence

After hashing, `installArtifactBootstrap` awaits `writeFolder` with only `state.docAbort.signal` ([lines 2048-2052](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2048)); `writeFolder` immediately PUTs its reconstructed pair ([lines 1648-1657](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1648)). A client/lease/socket replacement during that await is detected only afterward. Thus an old authenticated pair can still be persisted to the folder even though in-memory installation is rejected.

Do not describe a post-PUT `assertCurrent` as a fence: it cannot retract a completed PUT. Either keep this authenticated bootstrap out of the generic folder writer, or give the folder protocol an owner-scoped stage/readback/final-publish compare fence and abort it when the captured owner is retired. Add a delayed-folder row that changes client or lease during PUT and proves the final folder generation was not advanced by the stale transfer.

## P1: socket-only stale failure leaves an aborted assembler mounted

When a captured owner differs only because `state.socket` changed, `rejectArtifactBootstrap` aborts the old assembler and closes its old socket, but returns without clearing `state.artifactBootstrap` or `state.artifactBootstrapOwner` ([lines 1996-2001](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1996)). This avoids damaging a successor, correctly, but the no-successor socket-replacement case remains mounted with an aborted assembler until a later Welcome happens to call `startArtifactBootstrap`.

Introduce an exact-owner retirement helper: clear bootstrap, owner, deadline, pending token and required tail only when both mounted identities still equal the captured owner/assembler; otherwise touch nothing. Abort and close the captured socket outside that conditional. The `socket-after-hash` row should additionally require null owner/assembler and zero retained bytes; the `assembler-after-hash` rows must continue to preserve the successor and never close its socket.

## Event reentrancy and pair memory

Progress is already safe against the synchronous test sink: a `client-during-progress` mutation is observed after constructor return, before mounting the assembler. Browser `WorkerGlobalScope.postMessage` itself is non-reentrant, so a synchronous `snapshotReplaced` observer is not a demonstrated browser attack. Still, the test sink can synchronously alter the owner while `installArtifactBootstrap` has already written `currentPack/currentSpr/frontier` and before its next assertion ([lines 2053-2057](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2053)). Prefer one final owner assertion before the synchronous state commit, clear the bootstrap owner as part of that commit, and only then emit snapshot/outbox events. This makes the state transition atomic under both the test seam and future internal callbacks; it is robustness work, not evidence of a browser message reentrancy exploit.

The current `ArtifactBootstrapAssembler.finish` returns subviews of its staging array then drops its private storage ([replication.ts](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts:984)). The current copied-state-then-wipe pattern is correct. Keep the existing valid-row assertion that state bytes survive while every captured returned pair is zeroed; it proves the two ownership regions are distinct.
