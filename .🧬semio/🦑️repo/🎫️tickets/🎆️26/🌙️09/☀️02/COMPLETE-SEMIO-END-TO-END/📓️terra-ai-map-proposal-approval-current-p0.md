# Current AI-via-MAP Proposal/Approval Loop — P0 Execution Packet

## Verdict

**RED: no user-visible MAP proposal/approval loop exists.** The new static native-provider work can verify and instantiate the GIS Map/Terrain codec closure, and the hub has a bounded private inference ledger, authorization predicate, canonical command decoder, and committed-WAL witness. None is connected to an authenticated hub route, running job, typed Map receiver, normal document commit, or browser/native Shell action.

This is a source audit on 2026-09-05. No command or native/browser/process test was run here. Earlier ledger, codec, and provider test evidence remains evidence for those components only, not for this loop.

## Current evidence

| Reusable current source | Exact anchor | What it proves | Boundary still missing |
|---|---|---|---|
| Static server codec closure | `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs` (`NativeCodecProviderSetV1::linked`, `preview_gis_bindings`) | The linked stdio+GIS set has package-owned GIS codec receipts/factories subject to receipt identity checks. | It is not a job binding, a trusted running profile, or an execution route. |
| Private selection identity | `🌎️hub/💡️inference/📇️catalog/🦀️.rs` (`identity_from_verified_catalog`) | A verified catalog can be constrained to GIS Map selection. | `HubState` retains only `Arc<dyn DocumentOpenCatalogAuthorityV1>` at [`🚀️bin.rs:1418`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1418) and startup erases the concrete catalog at [`🚀️bin.rs:5708`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5708); no executable binding survives. |
| Author/session predicate | [`🌎️hub/💡️inference/🛂️authorization/🦀️.rs:7`](../../../../../../🌎️hub/💡️inference/🛂️authorization/🦀️.rs:7) | It checks exact session/user/auth-generation/scope and current `Author` role before and after async directory access. | It returns `()`, not a retained submission/commit grant; it has no caller in a public inference route. |
| Private ledger/outbox and WAL proof | `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs` (`prepare_approval`, `reconcile_committed_approval`) and `🧾️wal/🦀️.rs` | Proposal bytes can be held private and reconciled only against an exact committed-WAL witness. | There is no worker claim/lease, endpoint, socket event, typed execution, or approval caller. |
| Deterministic proposal primitive | [`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:255`](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:255), [`…/🧬️schema/💡️inferences/🦀️.rs:38`](../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:38) | The Map service can produce exactly one bounded `CreateRegion` proposal and inverse from a retained base snapshot. | It is a local function callback; no hub authority or document application is implied. |
| Existing collaborator relay | [`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3045`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3045), [`…:3088`](../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3088) | Normal accepted commands Fsync through `ArtifactHandle` and then broadcast `ServerFrame::Commands`. | The route deliberately does not interpret mutation schemas. It cannot turn a GIS inference result into a valid Map mutation. |

The current `HubState` has no inference runtime/ledger field, route, worker, cancellation registry, or job fanout. Its readiness is still published with inference disabled. `handle_client_frame` recognizes ordinary `Commands`, frontier, preview, presence, credit, and bye—not jobs or approvals. The GIS Map editor and Shell transports have no inference request/progress/cancel/approval action or DTO.

Two facts prohibit a shortcut:

1. `db_artifact::diff_entries` and `inverse_entries` intentionally return an empty set for every foreign schema ([`🧰️…/db/🗿️artifact/🦀️.rs:179`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:179)). A GIS diff sent through generic DB path-map machinery could be durably relayed without applying its typed meaning.
2. Map snapshots own composed children. Existing `CreateRegion` publication therefore depends on the unresolved atomic parent+existing-child visibility and global composition-history routing packets. A normal Fsync or generic path-map replacement is not evidence of an atomic GIS change.

## Smallest honest P0: one deterministic Map bounds proposal, explicit approval, one durable two-user commit

This is server-first. The UI portion is deliberately last; adding a Shell button before the server route/typed committer would create a false loop.

### Hard prerequisites (remain separate REDs)

1. Server owns a selected, verified GIS Map target from a trusted profile and public `MemberFactory::Open` can materialize the selected Map snapshot with retained ownership. Browser/WGPU execution-target leasing is needed for a browser-visible Map UI, but not for the server-only core.
2. `ArtifactGroupVisibilityOwner` must commit the Map parent and already-materialized child effects in one visibility flip, then global composition history must route that mixed group durably. This P0 must use that receipt; it must not call generic `db_artifact` directly.

### Slice A — bind execution to a concrete trusted Map selection

Keep the concrete `Arc<VerifiedTrustedCatalog>` in `HubState` beside the existing erased `openable_catalog`, and introduce a private `VerifiedGisMapProposalBindingV1`. Its constructor takes the verified selection plus the GIS package receipt and freezes:

- descriptor SHA-256; component SHA-256 **and** BLAKE3; descriptor canonical kind/schema;
- package id/version/hash, catalog generation, target/surface and granted mode;
- exact parent dialect, service id/version/algorithm version; and
- the server-local GIS Map inference function pointer/factory.

The current `InferenceIdentityV1` at [`🌎️hub/💡️inference/🧬️schema/🦀️.rs:43`](../../../../../../🌎️hub/💡️inference/🧬️schema/🦀️.rs:43) has an ambiguous `package_hash` and lacks component BLAKE3, package version, surface/grant, full parent dialect, and binding identity. Change this schema/fixture coherently now—there is no reason to preserve an ambiguous pre-runtime wire.

The binding materializes the current Map only through the selected retained member opener. It neither accepts a client Map pack nor uses an untyped global registry.

### Slice B — owned hub job runtime and owner-private events

Add one `HubInferenceRuntime` to `HubState`, owning the ledger, Map binding, a fixed-capacity operation set, one per-`DocumentScope` async gate, and retained cancellation controllers. Expose only authenticated HTTP endpoints (not the document socket):

- `POST /inference/gis-map/jobs`: closed client intent;
- `GET /inference/gis-map/jobs/{id}/events?after=<cursor>`: owner-private bounded progress page;
- `POST /inference/gis-map/jobs/{id}/cancel`; and
- `POST /inference/gis-map/jobs/{id}/approval`: `InferenceApprovalRequestV1 { jobId, proposalHash }` only.

At accept, claim, every expensive checkpoint, offer/read/cancel, and approval, re-run `check_live_inference_author` and compare the complete frozen binding plus scope/document/frontier/base-pack digest under the per-document gate. An `Admin` is not implicitly allowed: the present predicate permits only `Author`, so any broader policy needs an explicit separate capability law.

Extend the ledger with scoped idempotency (not globally unique client `request_id`), appendable bounded progress cursor, run epoch/claim lease, and durable cancel-request state. A restart may reclaim only an expired owned epoch after revalidating authorization and the binding; it may never re-execute a job already witnessed as applied. Progress/proposal reads are private to the original live owner; a collaborator observes nothing until normal committed document fanout.

### Slice C — approval is a typed, server-stamped Map commit

On work success, retain the Map base and call the local deterministic `infer_gis_map_controlled`, then `bounds_proposal`; canonicalize and store its proposal hash server-side. On approval, reload private bytes/base, re-check original identity, current author/session, catalog binding, and unchanged frontier. Build the sole `CreateRegion` and inverse on the server.

Introduce a private `GisMapApprovalCommitter` which hands that typed effect to the atomic parent+child composition transaction. Only its immutable committed receipt may create the normal command/event and only after its actual committed-WAL proof may the ledger outbox reconcile. On a crash after typed commit but before reconciliation, witness-based recovery reconciles; it does not run inference again.

Do **not** use `ArtifactHandle::submit`/`db.pathmap.v1` as the mutation receiver. The ordinary socket’s `submit_commands` is only the downstream relay once this typed committer has successfully emitted the durable canonical composition event.

### Slice D — side-effect-free client port, after A–C

Add a host-owned ephemeral GIS Map inference port, not a persisted Map field or generic document command. It carries only closed request/receipt DTOs and renders `idle`, `submitting`, `running`, `offered`, `approving`, `applied`, `cancelled`, `stale`, and `failed`. It needs semantic progress, Cancel/Approve controls, and explicit English/German terminology without a default locale. Browser and native ports must refuse to start until their immutable execution-target lease is verified; installation/lease verification and WGPU rendering remain separate REDs.

## State and visibility law

`accepted → claimed(epoch) → running(progress*) → offered(proposalHash) → approval-prepared → applied(reconciled)`

Terminal branches are `cancelled`, `stale`, and `failed`. The origin can observe its own ordered job events; a second user can observe **zero** job/proposal events. The second user receives exactly one ordinary committed document event only after the typed group transaction is committed. Duplicate approval and replayed requests cannot yield a second Map commit.

## Required test packet

1. **Schema-first neutral fixture** `gis-map-proposal-approval-v1`: canonical selected-binding fields, base Map identity/frontier, deterministic result/proposal/inverse hashes, lifecycle events/cursors, and two-user visibility expectations. Include exact byte/item/lease limits.
2. **Independent Bun/AJV/Geo oracle**: canonical framing/state-transition/hash verification; no Rust codec used as the oracle.
3. **Hub native laws** (new ticket-owned hub target calling only `📜️script.ts`):
   - authorized owner claims a bound Map job, streams monotonic cursor/progress, and boundedly retires on cancellation;
   - peer/cross-space/viewer/admin/stale-session/cancelled job cannot read, approve, or cause a fanout;
   - approval creates only server-stamped `CreateRegion` plus inverse, rejects changed frontier/base/binding/digest/dialect/surface/grant, and reconciles exactly one committed witness;
   - concurrent duplicate approval and restart around claim/commit reconcile to one event and no re-execution.
4. **Framework native law** before the hub acceptance: parent+existing-child atomic visibility receipt rejects/cancels without visibility; it gives one durable mixed-group history record on success.
5. **Two-user process law** after the trusted profile and member opener: two real authenticated clients; A requests then approves, B receives no private job data and one post-commit document update; restart between typed commit and ledger reconciliation proves recovery. Test cross-space, wrong-document, wrong proposal hash, stale authorization generation, provider failure, disconnect/reconnect, and cancellation before/during/after offer.
6. **Browser/native UI laws only after the execution lease P0**: actual `DirectoryClient`/native port uses the closed endpoints, cancellation closes retained work, and EN/DE accessibility semantics are asserted. Do not call this a WGPU rendering proof.

## Nonclaims

This packet does not claim a real model/MCP provider, autonomous mutation, generic DB schema execution, browser Map rendering, all GIS artifact kinds, or full composition recovery. It is intentionally one deterministic Map proposal with explicit approval. The existing ledger, provider, and local codec tests cannot substitute for the above native and two-user process laws.
