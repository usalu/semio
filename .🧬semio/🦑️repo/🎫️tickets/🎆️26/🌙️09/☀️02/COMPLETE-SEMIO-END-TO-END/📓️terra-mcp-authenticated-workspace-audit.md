# Authenticated Hub Workspace / MCP — P4 Refresh Audit

Observed 2026-09-03 from the shared working tree. This is a read-only source audit: no build or test command was run, and concurrent changes were re-read immediately before this report. The P2-C rebootstrap path is now mounted in the hub process and closes lagged WebSockets with a verified rebootstrap control; that improves recovery but does **not** provide an authenticated MCP cold-artifact read endpoint.

## Outcome

P4 is not yet a production authenticated hub workspace. `semio-os-mcp --hub` currently establishes a `PersistenceBinding::Hub`, but it neither authenticates the hub identity nor enumerates or reads cold hub artifacts. The sharp blocker is the absence of an authenticated, `DocumentScope`-bound, descriptor-and-verified-checkpoint endpoint. The existing generic blob route is deliberately only a space-wide content-addressed store and cannot be promoted to this role.

The smallest independently landable production slice is an **authenticated, bounded metadata binding actor**: validate the token with the directory, require current space membership, maintain a bounded descriptor index, and make MCP resources consume only that ready snapshot. It does not need the unfinished plugin loader. It must not claim bytes, schema, validation, or GIS execution until the paired-read slice is present.

## Current authority and cold-read matrix

| Surface | Current source-backed behavior | P4 conclusion | Severity |
| --- | --- | --- | --- |
| MCP process identity | `mcp/📦️bin.rs:48-107` takes `--principal` and `--scopes` from the local launcher. HTTP has a distinct process bearer; its `--token` for `--hub` is forwarded separately. `mcp/🛡️policy/🦀️.rs:22-97` maps those local scopes to local capabilities. | CLI policy is a local admission/presentation policy, not hub authorization. A launcher can name an arbitrary principal/scopes, so they must never authorize remote data. | High |
| MCP session isolation | `mcp/🦀️.rs:297-300` has `DEFAULT_SESSION_ID = "sess_default"`; `mcp/🚚️transport/🦀️.rs:196-241,1354-1407` keeps one server/process bearer and checks that bearer per HTTP request. | There is no per-MCP-caller binding to a remote hub subject or revocation generation. Do not share a remote catalog cache across callers. | High |
| Hub workspace origin | `mcp/🏠️workspace/🦀️.rs:419-450,1182-1184` turns `Hub { base_url, space_id, token }` only into `PersistenceBinding::Hub`. | Startup does not call `DirectoryClient::me`, resolve a role, or bind a remote session. Token optionality is unsafe for an authenticated origin. | High |
| Cold enumeration/read | `workspace/🦀️.rs:1207-1215` gets persisted IDs only for folders; `:1223-1234` returns `Ok(None)` for `WorkspaceOrigin::Hub`. `/workspace` and `/workspace/artifacts` consume this (`:1489-1497`), and `list_resources` hides errors with `unwrap_or_default` (`:1518`). | A hub workspace is empty from a cold MCP process. It cannot truthfully advertise a catalog. | High |
| Directory client | `directory/🔌️client/🦀️.rs:1-20,83-98,242-318` supports async authenticated `me`, spaces, a space detail, command/events, and a resumable WS stream; it has no public typed binary artifact-pair method. | It is the right authentication/discovery seam, but needs a bounded canonical-pair client method only after a hub endpoint exists. | High |
| Directory metadata | `directory/🧬️schema/🦀️.rs:60-71,277-306,370-399,485-489` gives structural `DocumentScope`, owner plugin/package identity, descriptor, and canonical descriptor digest. `hub/📇️directory/🦀️.rs:972-1011` separates public checkpoint from internal verified checkpoint. | Descriptor metadata can form a remote index. Never expose an internal blob locator or use `document_id` alone as authority/cache identity. | High |
| Hub REST authorization | `hub/📦️bin.rs:326-377` resolves current session role, share, public, or denied. `/directory/spaces` and `/{id}` are routed at `:1854-1857`; `/spaces/{space}/blobs/{hash}` at `:1875`. | An authenticated P4 origin must require a non-expired session and current membership even when a space/document projection is public. Public/share behavior belongs to a separately explicit read-only origin, not this one. | High |
| Generic blob read | `hub/📦️bin.rs:376-377,517-550` permits a current space authorization to put/get/head a hash. No descriptor, active checkpoint, pair aggregate, document scope, or revision is checked. | It is not a canonical artifact authority and must not become MCP's fallback. A hash-only URL leaks cross-document correlation within a space and has no atomic pair snapshot. | High |
| P2-C recovery source | `hub/🛰️lag-rebootstrap/🦀️.rs:1-80,187+` verifies descriptor/public/internal checkpoint identity, limits/cancels transfer, and checks bytes. It is instantiated at `hub/📦️bin.rs:1994-2009`; lag closes with control at `:616-637,1011-1014,1466-1468`. | Reuse this server-internal selection/verification seam for a future P4 endpoint. It currently serves WS recovery controls, not an HTTP MCP artifact read. | Medium |
| Resource/schema/validation | `mcp/🏠️workspace/🦀️.rs:1579+` base resource only packs bytes locally; non-probe `/schema` and `/validation` report retryable `PLUGIN_UNAVAILABLE`. | P4-A may expose descriptor metadata, but must keep schema and validation unavailable until a declared codec and verified bytes are selected. | Medium |
| GIS inference | `mcp/💡️inference/🦀️.rs:1-20` says the `ArtifactInferenceRouter` is a separate process and the channel has no infer command; local discovery reads generated registry/descriptors. The GIS plugin implements an inferrer in `plugins/🌍️gis/…/🧬️schema/💡️inferences/🦀️.rs:58-69`. | P4 must not execute GIS inference from a local static roster or received bytes. Its handoff is a later revision-bound, authorized plugin-host job. | Medium |

## Required trust boundaries

An authenticated hub workspace has two unrelated gates that must both pass:

1. The MCP listener verifies its local HTTP bearer and local capability policy. This protects access to the MCP process only.
2. The hub verifies a non-expired authenticated session, re-resolves current membership for the requested `space_id`, and authorizes the exact `DocumentScope`. This protects remote content.

The second gate must run at initial binding and on every networked metadata/pair read. A 401/403, membership event, reconnect, rebootstrap control, digest/checkpoint mismatch, or stream lag invalidates cached authority. Cache keys must include base URL, authenticated subject/session generation (never the raw bearer), `space_id`, `document_id`, descriptor digest, verified checkpoint ID, and frontier/epoch. A same document ID in another space is a distinct object.

Do not cache raw bytes as an authorization substitute. Either reauthorize before returning a cache hit or discard it. Do not expose private checkpoint storage keys, inferred codec choices, or blob hashes as a means to bypass the descriptor/checkpoint association. Cancellation, response-size bounds, and deadlines apply to every remote operation.

## Dependency-ordered implementation packet

### P4-A — authenticated descriptor index (parallel, no loader dependency)

1. In `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs` and `🏠️workspace/🦀️.rs`, make a hub origin require a hub token and introduce an explicit remote binding state: `Unbound`, `Refreshing`, `Ready(authorized descriptor snapshot)`, and `Revoked`. Do not infer authority from `AgentPrincipal`.
2. Use the existing `DirectoryClient` in `📇️directory/🔌️client/🦀️.rs` to perform `me → space` before publishing a ready workspace. Require a live authenticated session plus membership; reject the public/share fallback for this origin. Use an operation context with a fixed deadline and cancellation.
3. Feed an actor-owned, bounded `DocumentScope → DocumentDescriptor/DocumentView` index from the directory stream. Treat stream lag/reconnect as stale and refresh before ready. Keep network I/O out of the synchronous `ResourceRegistry`/`GatewayBackend` traits in `🌉️mcp/🧭️protocol/🦀️.rs:621-624,773-781`; resource calls only read an atomically published snapshot and return retryable unavailable while stale.
4. Make `semio://workspace`, `/artifacts`, and a new descriptor-only scoped resource enumerate that snapshot. Preserve current `PLUGIN_UNAVAILABLE` behavior for `/schema`, `/validation`, and raw artifact bodies. Never use `unwrap_or_default` to turn an authorization/network error into an empty catalog.
5. In `🌉️mcp/📦️packages/🦀️rust/Cargo.toml`, enable the existing OS-kernel native transport feature rather than adding a runtime HTTP library. Keep all remote DTOs schema-first at the directory boundary.

This slice validates endpoint/session/membership handling and gives an honest authenticated workspace index without waiting for trusted-loader composition or artifact bytes.

### P4-B — descriptor-bound canonical pair endpoint (depends on P2-B/P2-C, not plugin loading)

1. Add a dedicated hub route, for example `GET /spaces/{space_id}/documents/{document_id}/artifact-bootstrap`, rather than extending `/blobs/{hash}`. The handler must authenticate and recheck membership, construct `DocumentScope`, reread the descriptor and active public/internal verified checkpoint, require their exact digest/identity agreement, and reject unavailable or changed state.
2. Adapt `VerifiedRebootstrapSource` in `🌎️hub/🛰️lag-rebootstrap/🦀️.rs` for the endpoint's transfer result. Its output must be revision-pinned metadata plus bounded pack/SPR chunks and their individual/aggregate hashes; private locator values never cross the boundary. Preserve its cancellation/progress/deadline and return a deterministic resource-limit result.
3. Add a typed, bounded binary/client method to `DirectoryClient` (or a narrowly named hub artifact client next to it) rather than exposing its private `request_json` internals. Recheck authorization for metadata and every chunk; revocation must terminate the transfer and invalidate P4-A state.
4. Only then allow `HeadlessWorkspace::read_artifact_bytes` and artifact resources to consume a verified pair. The read contract must pin the descriptor digest/checkpoint ID through the entire response and fail closed on a change.

### P4-C — consume, validate, and hand off

1. Decode only through a codec registered for the verified descriptor's declared artifact kind/schema. Surface schema/validation only after this check; never substitute the local generated plugin registry for remote authority.
2. Define a plugin-host handoff carrying structural scope, descriptor digest, checkpoint ID, authorized operation context, deadline, and cancellation. Revalidate membership before dispatch and before publishing an inference result.
3. Let the GIS inference router run only in its owning plugin-host process against that pinned reference. P4 itself must return an unavailable/in-progress job state, not locally run GIS code.

## Capacity / integrity blocker

P2-A1 permits a 64 MiB artifact pair (`🌎️hub/🗿️artifact-authority/🦀️.rs:16`), while `DbImmutableArtifactBlobStore` independently caps each durable blob at 496 KiB (`…/🗿️artifact-authority/🔌️adapters/🦀️.rs:21,214-240`). `VerifiedRebootstrapSource` reads that durable substrate. This makes the P4-B endpoint **High severity blocked for the advertised authority envelope**: it may not imply that a 64 MiB trusted pair is supported.

Choose and verify one coherent design before exposing the endpoint:

- a bounded chunk-manifest CAS with chunk and aggregate hashes, atomic manifest publication, descriptor/checkpoint binding, retention/garbage-collection rules, and all backend parity; or
- a coherent payload-ceiling redesign across every durable backend, transport limit, and budget.

The generic hub blob page ceiling needs the same review. A small-pair-only implementation must state and enforce its smaller maximum as an interim contract; it cannot silently inherit the 64 MiB authority claim.

## Required oracle tests

Add a language-neutral fixture for: no token; expired token; public projection without membership; member then revoked; same `document_id` in two spaces; descriptor/checkpoint rotation between index and read; invalid pack/SPR/aggregate hash; chunk-size and deadline limits; stream lag/reconnect; and P2-C rebootstrap followed by fresh index. Expected status/error, resource list, cache action, and no-leak assertion belong in the fixture.

Rust unit/integration tests should use a recording `DirectoryTransport` plus a deterministic hub store to prove path, bearer omission from logs/cache, cancellation, reauthorization, atomic descriptor/checkpoint pairing, and cross-space denial. A real hub HTTP test must revoke the session or role during an attempted pair read. The MCP TypeScript suite should use the MCP SDK as an independent protocol oracle; use Node `crypto.createHash` to independently recompute individual and aggregate identities, and an HTTP client to compare route responses with the neutral fixture. These are separate implementations, not self-confirming serializers.

After the named tests land, keep runs focused while shared Cargo work is active:

```sh
bun nx run @semio-tech/framework-os-mcp-rs:test --skip-nx-cache --verbose -- authenticated_hub_workspace
SEMIO_OS_MCP_BIN="<verified absolute MCP binary>" bun nx run @semio-tech/framework-os-mcp:test --skip-nx-cache --verbose --testNamePattern='authenticated hub workspace'
bun nx run os-hub-ts:test --skip-nx-cache --verbose --testNamePattern='authenticated MCP workspace|revocation|cross-space'
HUB_E2E=1 bun nx run os-hub-ts:test --skip-nx-cache --verbose --testNamePattern='authenticated MCP workspace'
```

Those commands are a future focused verification packet, not evidence of a passing test in this audit.

## Blocking decision

Do P4-A now, in parallel with loader work. Do not expose hub artifact bytes until P4-B's authenticated descriptor/checkpoint route and the 64 MiB-vs-496 KiB substrate decision are complete. The existing P2-C source is reusable for verified selection and WS lag recovery, but it is not authorization for MCP nor a substitute for a canonical-pair read surface.
