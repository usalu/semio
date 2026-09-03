# MCP P4-B Current CAS Implementation Audit

## Scope and evidence

This is a source-only, read-only reassessment of the smallest P4-B packet after
P2-D. No production or test source was changed, and no build, test, or runtime
command was run.

The requested `📓️sol-artifact-chunk-cas-reference-retention.md` does not occur
in the current hidden workspace. This assessment therefore uses the current
sources below, the initial P2-D note
`📓️sol-artifact-chunk-cas-retention.md`, and the earlier P4-B audit
`📓️terra-mcp-canonical-pair-p4b-audit.md`.

| Seam | Current source evidence | Consequence |
| --- | --- | --- |
| P2-D CAS | `🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs:11-30,99-163,220-353,388-499,992-1047`; `🌎️hub/🗿️artifact-authority/🦀️.rs:19,119-178,275-280,435-492` | Scoped chunk and manifest CAS, private locators, ownership plans, reservations, exact raw verification, cancellation/deadline context, and 64 MiB aggregate publication admission now exist. |
| CAS ledger and retention | `🌎️hub/📇️directory/🦀️.rs:280-329,1424-1446,1497-1618,1658-1687,2049-2100`; `🪶️sqlite/🦀️.rs:200-268,1022-1195`; `🐘️postgres/🦀️.rs:1192-1213`; `🌐️neo4j/🦀️.rs:1037-1105` | The directory has atomic reservation/publication/reference replacement and fenced sweeping in all current backend implementations. |
| Public projection boundary | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:174-176,426-475`; `🌎️hub/📇️directory/🦀️.rs:907-1003,1300-1337,1040-1090` | Public active checkpoint and retention events intentionally omit private `storage_key` locators; a reader must use the private verified projection internally. |
| P2-C reader | `🌎️hub/🛰️lag-rebootstrap/🦀️.rs:12-60,187-280` | It selects public-active plus private-verified checkpoint correctly, but reconstructs both `Vec<u8>` values before enforcing the aggregate pair limit and has no reusable record-emission interface. |
| Hub authorization and routes | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:409-500,682-789,1277-1295,1625-1638,1721-1754,2090-2123` | `authorized()` admits the public-space fallback, generic blob fetch is client-selected and unsuitable, and no exact canonical-pair route exists. |
| P4-A descriptor index | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:19-42,148-264`; `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1272-1290,1558-1666,1715-1730` | The descriptor index is metadata-only, generation-fenced, and clears its snapshot on refresh, revocation, or stream loss. It has no pair cache, pair transport, or mounted artifact bytes. |
| Current client credential carrier | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs:1-115`; `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:431-463`; `.../🔗️remote/🦀️.rs:440-463` | MCP still accepts and carries a raw `--token`; local hub bootstrap does not make that a safe upstream capability boundary. This is SocketGrant/client work, not this packet. |
| Catalog and native codec bindings | `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:212-246`; `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:205-207,545-585` | There is no stable catalog-generation digest and linked native bindings are empty. Raw verified snapshot transfer must not pretend to be decoded or mounted. |
| Tail/barrier | `🌎️hub/🛰️lag-rebootstrap/🦀️.rs:226-280`; `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:682-694` | P2-C checks only a caller-provided frontier is not before baseline. No server transaction binds active-checkpoint selection to a post-baseline document-tail cursor. |

## Decision

P2-D is no longer the first implementation blocker. The immediate Sol packet is
a **server-only, bounded, verified immutable active-checkpoint-pair snapshot**.
It shares the verified selection and integrity logic with P2-C, admits only an
exact authenticated scope, and has one neutral fixture plus an independent
oracle. It does not create an MCP raw resource, client cache, mount, codec
catalog dependency, SocketGrant, or tail subscription.

The snapshot means “the active checkpoint selected by the server for this
request,” not “the document is current until the body ends.” The selected
checkpoint and its baseline are self-binding in the header. Do not include a
client-supplied checkpoint, hash, scope, frontier, cursor, or catalog selector.

## Immediate implementation packet

### 1. Shared verified reader

Add a server-internal `VerifiedActiveCheckpointPairReader` alongside
`VerifiedRebootstrapSource` in `🌎️hub/🛰️lag-rebootstrap/🦀️.rs`, factoring the
selection proof currently embedded in `verified()`:

1. Accept one exact `DocumentScope` from the server route and a bounded
   `OperationContext`.
2. Read the descriptor, public active checkpoint, and private verified
   checkpoint; require exact equality of their public projections, descriptor
   digest, and canonical checkpoint identity.
3. Before any blob allocation or CAS read, require nonzero blob lengths and
   `pack.byte_length.checked_add(spr.byte_length) <= AUTHORITY_MAX_PAIR_BYTES`.
   This is a required defensive read-side guard even though normal publication
   already admits the same limit.
4. Read only the selected private locators through the dedicated manifest CAS;
   verify raw SHA-256 and length for each part and the public aggregate. Never
   fall back to a newer active checkpoint after selection.
5. Return a public immutable selection record containing only scope, descriptor
   digest, active checkpoint id, baseline frontier, pack/spr public integrity,
   and aggregate integrity. Locators, manifest ids, chunk ids, ownership plan,
   reservation, and delete fence stay private.

The present `ArtifactChunkBlobStore::read` materializes a complete blob. P2-C
also reports only `PackRead`/`SprRead`; its result must not become the P4-B
transport directly. Give the shared reader a bounded internal record-emitter or
validated read-plan interface so the route can emit 4 KiB records and surface
its own monotonic progress. The P2-C `load()` façade may remain its existing
materialized compatibility-free internal consumer, but it must share source
selection and read-side guards.

This packet may initially use bounded materialization behind that internal
interface, because the aggregate preflight cap is 64 MiB. A follow-on can make
the CAS read itself record-streaming. Neither implementation may expose a
manifest/chunk fetch API to clients.

### 2. Exact server-selected route and admission

In `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, add only:

```text
GET /spaces/{space_id}/documents/{document_id}/active-checkpoint/pair
Accept: application/vnd.semio.canonical-checkpoint-pair.v1
```

There are no query parameters, range requests, checkpoint selectors, blob hash
selectors, client tail frontiers, or alternative blob routes. Set
`Cache-Control: private, no-store` and `Vary: Authorization`. Reject an
unexpected `Range` or media type before body work.

Use a private `authorize_checkpoint_pair` helper, not `authorized()` and not
`authorized_for_blob()`. It must admit only:

- a current session whose user is still a member of the path space; or
- an exact `DocumentScope` share capability, if share access is retained for
  this endpoint.

Reject `AuthOutcome::Public` and `Denied`, and do not add an administrative
static bypass. The helper constructs the scope solely from the route path. It
must accept bearer credentials only in the `Authorization` header; do not copy
the `/directory/ws` query-token compatibility path. A rate/concurrency key, if
needed for the 64 MiB body, is an opaque session id or capability selector, not
capability text or a secret digest in an externally visible diagnostic.

Revalidate authorization and exact scope after metadata selection and before
the first record. While streaming, check cancellation, deadline, and revocation
at every record turn or a no-less-frequent bounded turn. A selected checkpoint
may become non-active after the header; that does not alter this immutable
snapshot. Revocation must stop the response and prevent the receiver from
accepting a partial body.

### 3. Version-one framing and limits

Use an owned binary body, not JSON and not a generic blob response. It has one
bounded header, ordered records, and one terminal record:

```text
Header CanonicalCheckpointPairReadV1
  format = 1
  scope = (space id, document id)
  descriptor_digest = [u8; 32]
  active_checkpoint_id = [u8; 32]
  baseline_frontier = (document id, head edit ordinal, head edit id,
                       last commit sequence, chain hash)
  pack = (sha256[32], byte_length u64)
  spr  = (sha256[32], byte_length u64)
  aggregate_sha256 = [u8; 32]

Data record
  part = pack | spr
  ordinal = u32
  byte_offset = u64
  byte_length = u32
  bytes

Terminal record = complete | cancelled | unavailable | integrity | deadline
```

Use the repository’s owned canonical big-endian length-field conventions. Cap
the header at 16 KiB, each scope component at the existing 256-byte protocol
limit, each data record at 4 KiB, and the whole pair at 64 MiB / 16,384 data
records. Emit pack fully before SPR; ordinals and offsets are contiguous; no
empty data record is valid. The receiver accepts data only after the bounded
header and accepts the transfer only after `complete`, exact contiguity, both
part hashes/lengths, and aggregate verification. It drops all partial data for
any terminal failure or malformed/truncated/reordered/oversized body.

Derive the ETag from a domain-separated encoding of scope, descriptor digest,
checkpoint id, baseline, public part hashes/lengths, and aggregate hash. Do not
include a catalog generation in this v1 response: none is presently defined.

The reader and route use a 15-second bounded operation context consistent with
`REBOOTSTRAP_DEADLINE_MS`. Report monotonic bounded stages:
`authorize`, `metadata`, `verify-pack`, `verify-spr`, `stream-pack`,
`stream-spr`, `ready`. Check cancellation/deadline before selection, every CAS
chunk, and every emitted record. Public failures before the body are bounded
HTTP failures without backend, locator, manifest, chunk, or authorization
secret detail; after a header, use only the bounded terminal code.

### 4. Neutral fixture and oracle

Add a language-neutral fixture and schema adjacent to the P2-C reader, for
example:

```text
🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🧬️canonical-checkpoint-pair/🧬️schema/🔣️.json
🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🧬️canonical-checkpoint-pair/🔣️.json
```

The fixture describes canonical header fields, deterministic part generators,
record boundaries, expected hashes, aggregate, ETag input, and expected terminal
outcome. It must not encode private locators or storage implementation details.
Use a Rust reader/route test plus an independent TypeScript/Node oracle near
`🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts`, using JSON Schema/AJV and
Node built-in crypto. The oracle independently generates bytes, decodes the
framing, and recomputes part hashes, aggregate, and ETag; it must not invoke the
Rust serializer or verifier.

Required cases are member access; exact share access; public/non-member denial;
cross-space equal document ids; `496 KiB + 1` part; full 64 MiB aggregate;
malformed canonical identity/hash/aggregate; foreign selector/query/range
rejection; cancellation and deadline; missing or deleted CAS after metadata;
retention-sweep race; checkpoint replacement during a stream;
truncated/reordered/oversized records; and revocation between metadata and first
record. The resulting assertion is that no receiver accepts a partial or
mixed-checkpoint pair.

## Race and retention law

The P2-D sweep mints a fence under the directory writer lock and performs the
physical delete under that lock. The P4-B reader currently has no equivalent
read lease: it can select an active private checkpoint, then lose a concurrent
new-publication plus retention-release plus sweep race before its CAS read.
That is an availability race, not permission to substitute data. The immediate
packet must map it to bounded `unavailable` or `integrity`, discard partial data,
and never retry against a new active checkpoint inside the same response.

A future reader lease/reference pin can eliminate the transient failure, but it
is not needed to make this minimal packet truthful and safe. The route must not
call its result a continuously current document snapshot. The header binds the
selected immutable checkpoint; a later SocketGrant/open-plan operation must
atomically bind a server-issued post-baseline tail cursor.

## Explicitly deferred work

1. SocketGrant and replacement of the MCP upstream raw `--token` carrier.
2. A P4-A MCP pair transport, pair cache, artifact resource, catalog selection,
   codec activation, mount, or descriptor-to-runtime bridge. P4-A’s current
   refresh/revocation state already clears metadata snapshots, but there is no
   pair state to invalidate yet.
3. Trusted catalog immutable generation, native bindings, and catalog-bound
   open-plan semantics. Do not insert a zero, empty, or invented catalog field
   into this raw-snapshot frame.
4. Server-issued tail cursor/barrier, gap-free replay composition, and a claim
   that the response represents the newest state after body completion.
5. CAS read-lease/pin availability improvement and fully streaming CAS internals.

## Difference from the earlier P4-B audit

The earlier audit correctly identified the missing canonical route, exact
authorization boundary, neutral oracle, no raw MCP resource, and absent tail
barrier. Its first blocker is now obsolete: P2-D source includes dedicated CAS
stores plus reservation, reference, release, and fenced sweeping rather than a
single generic 496 KiB database blob limit. It also proposed catalog generation
in the immediate pair header. Current source has no stable catalog generation
and no native codec bindings, so catalog must remain a later, separately bound
open-plan concern.

This reassessment adds two source-derived constraints not explicit in the older
packet: P2-C currently checks the aggregate size after reconstructing both
blobs, and the reader lacks a lease against a concurrent retention sweep. The
immediate implementation must preflight before allocation and fail closed on
the latter race without checkpoint substitution.

## Blocker order

1. Implement the shared verified active-pair reader with preflight aggregate
   admission, exact public/private projection proof, raw/aggregate verification,
   bounded emission, and failure-without-fallback law.
2. Add the exact path-only server route and dedicated member/exact-share
   authorization boundary with cancellation, progress, revocation, and framing
   enforcement.
3. Land the neutral fixture, Rust server tests, and independent TypeScript/Node
   framing and cryptographic oracle.
4. Only then design SocketGrant/client transport, MCP state/cache invalidation,
   catalog-bound activation/mount, and the atomic checkpoint-to-tail barrier.

