# GIS Approval To Authenticated Command Commit — Current-Source Audit

## Verdict

RED — the durable GIS ledger and committed-WAL witness are not connected to a
production command executor. The smallest honest vertical slice is one
server-owned, catalog-bound CreateRegion approval committer. It must use the
existing authorisation and Fsync submission authority for document WebSocket
writes. No command, route, provider, or runtime gate was run in this audit.

## Current Reusable Authority

| Boundary | Evidence | Required reuse |
| --- | --- | --- |
| Immutable job identity | 🌎️hub/💡️inference/🧬️schema/🦀️.rs:10-79 bounds the request and records user, session, authorisation generation, document scope, descriptor/catalog/package digests, and ordinal/edit/commit/chain frontier. | It is a job precondition, never current authorisation. |
| Exact catalog metadata | 🌎️hub/💡️inference/📇️catalog/🦀️.rs:12-106 matches GIS scope/package/version/hash/service and a verified open target, while its module docs explicitly deny execution authority. | Pair it with a compiled, verified GIS execution binding. |
| Durable ledger | 🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:219-289 provides bounded private outbox bytes, deterministic mutation ID, SQLite Immediate transactions, restart paging, and private witness reconciliation. | Retain its first-terminal state machine; reconcile after Fsync, not as a claimed cross-store transaction. |
| Committed proof | 🌎️hub/💡️inference/🧾️wal/🦀️.rs:91-244 produces a fenced private witness from a committed target. | Use it only after the same real submit succeeds, to reconcile a prepared row after crash. |
| Current membership | 🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1044-1072 validates session, user, generation, revocation, expiry, and current space role. The trait is in 🌎️hub/📇️directory/🦀️.rs:2038-2043. | Require Active Author when committing; only bounded Unavailable is retryable. |
| Real command path | 🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2809-2846 uses CommandBatch, SecurityGate admission, and ArtifactHandle submit with Fsync. Its WebSocket client path rejects actor mismatch at 2867-2881. | Share or extract a private typed helper. Never write a second WAL path. |
| GIS semantics | ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:36-56 yields typed CreateRegion; the mutation aggregate and binary codec are under its schema mutations and binary modules. | P0 permits only the typed CreateRegion proposal and its exact inverse. |

## Current RED Boundaries

### No running-hub consumer

HubState, router, and the binary have neither inference state nor an inference
route. The router at 🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5129-5161 has no
inference endpoint and readiness hard-codes inference false at 1683-1724. A
current source search found no production caller of InferenceJobLedgerV1,
InferenceWalVerifierV1, or identity_from_verified_catalog outside the inference
module tests.

The registered gis-inference-ledger-check is correctly limited:
🌎️hub/📦️packages/🦀️rust/📜️script.ts:3450-3474 explicitly prints that it makes
no route, provider, or GIS-approval acceptance claim. It is foundation evidence,
not the requested approval-to-command proof.

### GIS is declared but not linked in the hub

The hub depends on semio-s-plugin-stdio but not the GIS plugin
(🌎️hub/📦️packages/🦀️rust/Cargo.toml:39). NativeCodecProviderSetV1 linked at
🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:24-28 contains
only semio:stdio, and startup uses it at bin.rs:5311. VerifiedTrustedCatalog
holds packages, codecs, and open targets only
(🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:250-255), not an
executable inference-service binding.

GIS itself has a real local service and codec at
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:196-211 and 359-365, but this
does not select, link, or execute it in the hub. A direct route call would
bypass the exact trusted package closure.

### Raw command bytes are not a submit-safe authority

prepare_approval accepts private raw bytes and checks only the first three
protocol strings: mutation ID, document ID, and actor
(🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:219-253). On a matching command hash the
WAL verifier reads only mutation ID and document ID
(🌎️hub/💡️inference/🧾️wal/🦀️.rs:213-221). Neither proves dependencies,
diff/inverse schema, GIS payload, HLC, or exact EOF.

The generic protocol decoder at
🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:828-879 takes a
caller-owned offset. The slice decoder in db sync is test-only and omits its
advertised EOF check at
🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:46-56. In contrast,
the retained decoder at 58-101 bounds each field and rejects trailing data.

P0 needs one production exact command decoder, with the stricter 8 KiB approval
limit applied before allocation, retained-decoder field bounds, exact EOF, and
canonical re-encoding equal to stored bytes. Preparation, dispatch, and WAL
target formation must all use it. A prefix scanner never authorises submission.

### Security and frontier are currently incomplete for a background committer

The live socket path revalidates its grant per frame, checks envelope actor
equality, then calls admit_writes. SecurityGate itself intentionally permits a
delegated envelope actor while applying tenant, role, budget, and replay policy
(🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔒️security/🦀️.rs:601-615).
Therefore a background committer must independently require exact equality of
the decoded actor with the identity-derived server actor before SecurityGate;
it must not synthesize an inference super-principal.

The identity captures all frontier values, and ArtifactHandle exposes frontier
through bin.rs:2747-2750, but no current code compares them at commit time. A
generation-only fence cannot prevent a different valid author from changing the
document between inference and approval.

## Smallest Complete Vertical Slice

### P0a: Catalog-bound GIS executor prerequisite

Extend catalog activation with a private VerifiedInferenceBinding for exactly
one selected semio:gis package. Construct it atomically from that package's
exact receipt closure. It exposes only the exact package hash, artifact and
inference identities, GIS codec/service factory, and typed conversion of the
bound proposal to CreateRegion. Do not use a process-global registry, generated
marketplace row, descriptor string, or a direct plugin call.

### P0b: Server-built approval candidate

Expose a strict bounded approval request containing job ID and offered proposal
digest only. It does not accept command bytes, actor, document, mutation,
package, or frontier from browser or MCP callers.

The hub reads held identity and typed proposal, verifies P0a binding, produces
exactly one CreateRegion against the retained base snapshot, computes its
inverse, and creates one canonical MutationEnvelope. Deterministic mutation ID,
scope, actor, diff schema, inverse schema, and byte payload all derive server
side. Persist only this exact-decoded canonical private command. Reject every
other GIS mutation and any generic JSON patch/envelope input.

### P0c: Private InferenceApprovalCommitter

For each bounded prepared outbox entry:

1. Acquire a bounded per-full-document submission guard.
2. Exact-decode and canonical-byte-check the stored command, then verify job,
   proposal, deterministic mutation ID, scope, actor, GIS schemas, and sole
   CreateRegion variant.
3. Call socket_session_binding for the original session/user/generation/space;
   require current Author. Revoked, expired, lost membership, and spectator
   deny or stale. Only bounded directory unavailability may retry without
   mutating the entry.
4. Read exact live frontier and require equality with all identity frontier
   fields. A changed frontier stales the proposal before any write.
5. Build the same tenant, principal, role policy, and SecurityGate decision as
   document WebSocket writes; explicitly bind actor equality; then submit via
   existing CommandBatch to ArtifactHandle Fsync.
6. After Fsync only, obtain an exact witness for this deterministic command and
   reconcile. If reconciliation fails, retain the row and retry proof plus
   reconciliation only; never resubmit. Before Fsync, private typed abandonment
   zeroes command/proposal bytes.

The guard extends from frontier comparison through accepted submit, not provider
execution or long WAL replay. This is deliberately submit-then-idempotent-
reconcile, not a false cross-SQLite transaction claim.

### P0d: Route, cancellation, fanout, readiness

After P0a-P0c, hold ledger, verifier, binding, and a cancellation/recovery
supervisor in HubState. Add normal authenticated request and approval handlers.
Resume jobs using durable identity plus current directory revalidation, never a
stored bearer. Reuse accepted-command fanout from handle_client_frame. Return
only bounded owner-authorised state. Advertise inference readiness only when
binding, ledger, committer, and supervisor are all live.

## Required Acceptance

Create a neutral corpus with one canonical CreateRegion envelope and hashes,
then hostile rows for malformed, trailing, and over-limit bytes; every identity,
actor, scope, proposal, schema, inverse, HLC, dependency, package, catalog, and
frontier mismatch; unlinked GIS; non-author/revoked/expired/generation-rotated/
membership-lost/unavailable session; duplicate/concurrent approval and replay;
cancel before and after Fsync; ledger restart after Fsync; and a two-author
race where ordinary socket mutation advances frontier and the queued approval
emits no command fanout.

The Rust proof must use real SQLite directory/session, ArtifactHandle Fsync,
actual WAL witness, and a subscriber observing exactly one accepted command. A
Bun/AJV/WebCrypto oracle independently checks request/proposal schema,
canonical bytes, hashes, and all hostile outcomes. Only then extend the current
gis-inference-ledger-check with exact laws, an Nx target, and a seed-first launch
entry. The current foundation gate cannot be credited for that result.

## Nonclaims

This P0 does not enable a remote provider, arbitrary GIS actions, generic
plugin commands, browser/MCP raw command submission, automatic application,
or full catalog activation. It proves one server-built, human-approved GIS
CreateRegion through existing authenticated Fsync command authority.

## Coordination

The ledger/WAL lane should retain its bounded private semantics. The next work
belongs at catalog binding and the hub command-owner seam. Do not expose
pending approvals, raw abandonment identifiers, or command bytes as route
contracts; use typed internal preparation and abandonment, and consume the
private witness only after real submit.

