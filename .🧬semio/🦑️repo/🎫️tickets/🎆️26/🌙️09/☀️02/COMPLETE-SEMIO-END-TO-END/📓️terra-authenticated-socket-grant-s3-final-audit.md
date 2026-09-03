# SocketGrant S3 Client Migration And Removal Audit

Date: 2026-09-03  
Scope: read-only implementation audit of the current S1+S2 source.  This report
did not modify production/test/plan/acceptance source and did not run a build,
test, or runtime probe.

## Decision

**REJECT S3 release today; ACCEPT the following one-breaking-change packet as
the required implementation order.**  S1+S2 offers a sound server destination,
but every real consumer still uses a legacy carrier.  In particular, both old
routes are mounted, tag 0 remains executable, normal directory reconnects put a
reusable bearer in the URL, and no concrete browser/native/MCP protected-session
broker currently exists.  A partial client conversion would be worse than an
atomic cutover because the old authority path would remain reachable.

The authority distinction is non-negotiable:

```text
protected upstream session/share delivery     per-dial socket receipt
local bootstrap / host relay / MCP relay  ->  POST protected issue endpoint
private broker memory only                 <-  { grant, actorId, expiry }
                                               -> exact WS offer, then discard grant
```

The upstream session is not a SocketGrant and is never an actor binding.  The
grant is not durable, is not logged or URL-encoded, and is never reused after an
upgrade failure, close, rebootstrap, or cancellation.  Current server source
intentionally keeps the ledger issuer-process-local; the issue request and its
upgrade therefore require explicit same-process affinity until a different,
durable grant design exists.

## Final-Source Contract To Target

The current accepted server packet mounts protected issuers and v1 upgrades in
`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1319-1388,1614-1636,2740-2778,3479-3506`:

```text
POST /spaces/{space}/documents/{document}/socket-grants
POST /directory/socket-grants
GET  /spaces/{space}/documents/{document}/socket/v1?surface=
GET  /directory/socket/v1?since=&spaceId=&documentId=
```

For each dial, obtain and strictly parse the bounded receipt
`{ schema: "semio.hub.socket-grant/v1", protocol: "semio.socket.v1", grant,
actorId, expiresAtMs }`.  Offer exactly two subprotocols in this order:

```text
Sec-WebSocket-Protocol: semio.socket.v1, socket.v1.<32 lower-hex>.<64 lower-hex>
```

The current server requires one raw header, no `Authorization`, no third
protocol, and a 256-byte ceiling (`bin.rs:1533-1569`).  It selects only
`semio.socket.v1`.  The URL holds only the listed routing values.  Immediately
after upgrade send one binary `ClientFrame::SocketHelloV1` (tag 7) with the
existing wire/protocol/schema/pack hash/resume/frontier fields, never actor or
credential.  This is also required for directory v1, not only document v1:
`handle_directory_ws_v1` requires it before replay (`bin.rs:2914-2928`).

Session receipts intentionally give the same `hub.v1.<64 hex>` actor on fresh
grants from one session; share receipts get a per-grant actor
(`bin.rs:1257-1316,1319-1388`).  A client must install the receipt actor before
constructing a command envelope.  If a new receipt actor differs from an active
hub actor, it must close the old epoch and refuse/reconcile unsent semantic
intent; it must never relabel an already signed/causal envelope or send it under
the new receipt.  This is needed even though session actor stability is intended.

## Exact Caller And Carrier Census

| Surface | Current carrier/source | Required S3 replacement |
| --- | --- | --- |
| React document backbone | `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:540-607` dials document `/ws` then tag-0 `Hello` with `state.config.actor` and `binding.token`. | A bounded host-broker RPC per connection attempt; after receipt validation set the ephemeral hub actor, create `WebSocket(v1Url, ["semio.socket.v1", grant])`, send binary `SocketHelloV1`, erase the receipt grant, and restart that full sequence after close/1013/rebootstrap. |
| React normal directory | `🧰️framework/🛍️products/💻️os/🟦️.ts:3975-4138` stores a bearer and redials `/directory/ws?token=…&since=…`; worker `directory-open` also carries `token` (`🟦️.ts:676-681`, `🟦️backbone-worker.ts:1373-1375`). | Split REST authority from a `DirectorySocketGrantSource`; each redial calls the protected directory issuer, opens `/directory/socket/v1?since=…` with the two protocols, sends binary hello, then resumes only from the tracked sequence.  Preserve member filtering; do not create an admin mode. |
| Shell identity/dev bootstrap | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️.tsx:839-874,1268-1290,1517-1592,3374-3389,5440-5442` persists/mints `sessionToken`, exports it into bindings and derives `client-*` actor.  Vite and four dev launches export `VITE_S_USER`/`S_USER` (`…/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts:160-166`, `.vscode/launch.json:2510,2534,2558,2582`). | Remove public email mint and token-bearing worker/config messages.  Implement a main-thread `BrowserCredentialRelay` backed by verified local bootstrap, then a request-id/cancel-bound worker bridge which returns one parsed receipt only.  The relay owns the long session in memory; the worker never sees it.  A hub-bound document remains `authorizing` and cannot construct causal commands until an actor is installed. |
| Native document actor | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:78-111,756-772,1808-1840,3199-3223,3580-3605` serializes `PersistenceBinding::Hub.token`, forms `/ws`, and sends tag-0 on native and wasm paths. | Delete `Hub.token`; split a local-only actor from the ephemeral receipt actor.  Inject a non-serializable `HubSocketGrantSource` into `ArtifactHost`/actor.  It issues a document receipt under the caller's `OperationContext`, validates/installs actor before mutations, builds a tungstenite request with the exact raw header, and uses `WebSocket::new_with_str_sequence` on browser wasm.  No grant can enter `ToValue`/`FromValue`, a plugin boundary, or a persisted binding. |
| Rust directory client | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:111-115,274-298,364-385,394-505` owns `RwLock<Option<String>>`, computes query-token URLs, and exposes a text-only WS transport. | Add a private grant source and a non-Debug `DirectoryWsDial { url, protocols, helloBytes }`; remove token from stream state/URL.  A dial obtains exactly one receipt, native `IntoClientRequest` sets the protocol header, browser `new_with_str_sequence` receives the protocol array, and both transports send `SocketHelloV1` binary before text directory data is visible.  Expired/replayed/failed grants are discarded and a later scheduled dial gets another receipt. |
| MCP hub upstream | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:446-450,1217-1223` and `…/🏠️workspace/🔗️remote/🦀️.rs:319-321,446-479` require `WorkspaceOrigin::Hub { token }`, copy it into the store binding, and use the legacy directory client. `📦️bin.rs:2-15,80-108` accepts a second hub `--token`. | Replace the origin token and CLI option with an `McpUpstreamCredentialSource` supplied solely by verified MCP local-bootstrap delivery.  It may issue document/directory grants but cannot expose/session-serialize the parent.  Wire that source through both native directory and ArtifactHost grant interfaces.  No trusted delivery implementation exists now: `NativeCredentialEnvelopeDelivery`, `McpCredentialEnvelopeDelivery`, and `BrowserCredentialRelay` are declarations only (`🌎️hub/📇️directory/🦀️.rs:754-763`).  This is a hard S3 prerequisite. |
| Hub legacy ingress | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1598-1606,1898-1917,2758-2767,3482-3506` mounts document `/ws`, directory `/directory/ws`, and accepts `ClientFrame::Hello`. | Delete both handlers/query structs and routes, the `None` legacy branch in `handle_ws`, and every tag-0 fallback.  v1 must be the only public hub socket contract. |
| Frame/schema/generated worker | Tag-0 resides in `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`, `…/📡️replication/🟦️.ts`, TS/Rust hub and OS tests, and generated `…/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🟨️frame-worker.js`. | Delete tag 0 and its actor/token encode/decode/vector cases in one schema break, retain tag 7, then regenerate/check the checked worker from its owner.  WASI plugins do not link store sync/worker; host/worker bundles—not plugin ABI—are the affected binaries. |
| Admin connections | `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔴️ConnectionsPage/🟦️.tsx:9-76` seeds authenticated REST but opens a credential-free normal directory WS. Its fake test asserts that unauthenticated URL (`…/📦️packages/🟦️typescript/🧪️admin.test.tsx:168-205`). | Remove `DirectoryClient` and its fake WS.  Use the already authenticated `AdminClient.connections()` as an abortable, single-in-flight bounded snapshot poll while the page is mounted/visible; render an explicit fresh/stale state and keep EN/DE messages.  Do **not** issue a new unaudited admin socket audience. |
| Separate MCP local bridge | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs:2571-2573,3164-3210`, `…/🚚️transport/🦀️.rs:126-140`, and MCP argv persist/accept `/bridge?token=` and a bridge-token file. | This is not a hub session or SocketGrant audience.  It needs a separate local process credential protocol (credential-free `/bridge`, independently scoped local grant/protocol, and a credential-free bridge hello) or S3 cannot claim URL-secret removal for MCP.  Never make it accept `socket.v1` or a hub receipt. |

## Smallest Safe S3 Packet

1. **Freeze the wire deletion first.**  Make tag 7 the only first client hello;
   delete actor/token tag 0, legacy doc and directory handlers/routes, URL-token
   helpers, persistence/config token fields, and tests that encode those
   carriers.  A route/match arm/token field left behind is a release failure;
   do not retain a compatibility flag.

2. **Add the private credential delivery layer before any caller conversion.**
   Implement native/MCP/browser concrete local-bootstrap delivery plus
   `HubSocketGrantSource` and `DirectorySocketGrantSource`.  Each source takes
   a bounded `OperationContext`/abort signal, an exact typed audience/scope and
   client class, performs the protected `POST`, parses the receipt strictly, and
   exposes neither parent bearer nor grant in `Debug`, error, progress, event,
   persisted state, or plugin API.  Permit at most one outstanding issue per
   document/directory stream; cancellation before/after issue drops the receipt
   and never starts an upgrade.  Do not fabricate public `/auth/sessions` or
   `VITE_S_USER` fallback.

3. **Refactor the common directory transport once.**  Replace text-only
   `open_ws(url)` and `DirectoryStreamTurn::Dial { url }` with a token-free dial
   descriptor carrying the ordered offer plus pre-encoded binary hello.  Send
   hello during dial completion before changing to replay/live.  Keep existing
   finite polling/eight-invalid-frame/backoff behaviour, cancellation and
   `since` convergence; schedule a new issue on every retry.

4. **Migrate document actors.**  Receipt acquisition precedes `connectHubOnce`,
   `start_connect_hub`, and wasm `connect`.  A hub-bound actor must not flush its
   envelope outbox or create new remote envelopes before the receipt actor is
   active.  After `1013`, auth close, error, revoke, or ordinary redial, clear
   the active actor/receipt and begin a new issue→upgrade→hello epoch.  Retain
   unsent semantic intent only where it has not formed a causal envelope; a
   prebuilt different-actor envelope is a terminal reconcile error, not a retry.

5. **Migrate React, native and MCP callers in the same change.**  Update worker
   request schemas, ShellHost, local `ArtifactHost`, directory client, MCP
   workspace/remote binding, fixtures and dev launch seed together.  Remove raw
   hub `--token`, session fields in identity/persistence state and synthetic hub
   actor plumbing.  Keep local-only folder identity separate where needed; it
   must not leak onto a hub socket.

6. **Make the admin deliberately non-streaming.**  Implement the bounded REST
   poll and remove ordinary directory subscription before the old directory
   route is deleted.  Members continue to use directory v1 and server-side
   member filtering; admin gets no accidental cross-space directory grant.

7. **Complete the independent bridge cutover or keep S3 unreleased.**  It may be
   a sibling commit in the same release atom, but cannot be a `socket.v1`
   compatibility mode.  Remove bridge query-secret/secret-URL/file tests only
   after its distinct local authority protocol is complete.

## Required Laws And Gates

Use a neutral fixture with a receipt grammar, ordered-protocol header, URL
without credential, tag-7 binary hello, and a redacted rejection corpus.  Have
the Rust and TypeScript implementations consume it; a browser/WebCrypto parser
is the independent receipt/grammar oracle.

Focused tests must prove at least:

1. Every React/native/MCP document and directory dial issues a fresh receipt,
   offers exactly `["semio.socket.v1", grant]`, sends tag 7 first, and never
   sends `Authorization`, actor or token over the upgrade/hello.  Assert no
   grant in URL, serialized config, local storage, logs, queued request or error.
2. Each reconnect, P2-C rebootstrap and cancellation gets no reuse: a consumed,
   expired or issue-cancelled receipt cannot reach a second dial.  The client
   preserves `since`/frontier only, not a receipt.
3. Receipt actor is installed before command construction; forged/mismatched or
   post-receipt-change envelopes never cross client transport.  A stable same-
   session reissue preserves valid pending work only when its actor matches.
4. The directory transport sends binary hello before it reads/replays any text;
   normal events remain member-filtered.  A share cannot issue directory; an
   ordinary member cannot obtain admin connection data.
5. Deleting old routes yields 404/upgrade failure for `/ws` and `/directory/ws`;
   tag 0, `Hello.actor`, `Hello.token`, query `token`, a third/reordered
   subprotocol and `Authorization` are rejected.  Regenerated worker and Rust/
   TypeScript decoder no longer contain tag-0 vectors.
6. Native/MCP protected delivery has a fake verified envelope and an independent
   oracle showing it can issue a grant without serializing the parent session.
   No-delivery cases fail closed.  Local bridge tests separately reject a hub
   `socket.v1` and prove no bridge URL secret.
7. Admin page tests assert authenticated `GET /admin/api/connections`, one
   bounded in-flight poll, abort/cleanup, a visible stale state and EN/DE
   labels—never a directory socket.

Register focused `bun nx` targets only through each package's `📜️script.ts`:
the hub socket-grant gate, replication wire vectors, framework OS worker/
directory tests, native store sync test subset, MCP workspace/bridge subset,
admin component tests, generated-worker check, and the independent Bun oracle.
Run feature-backed PostgreSQL/Neo tests only when configured and report a skip
as a prerequisite, never as a pass.  The existing attributed S1+S2 server gate
does not validate S3 callers and must not be presented as S3 evidence.

## Release Atomicity And Blocking Order

**Do not merge/release S3 until all of these are true:** concrete protected
browser/native/MCP upstream delivery exists; every actual caller dials v1; tag 0
and both legacy hub routes are gone; the local MCP bridge no longer carries a
query secret; generated worker artifacts match; and focused/independent gates
exist.  The server's voluntary issue/upgrade affinity must be documented in the
deployment target.

1. **Blocker A — credential delivery:** only the delivery traits exist today;
   browser/native/MCP cannot safely call the protected issuers.
2. **Blocker B — actor epoch:** present configs build envelopes from a synthetic
   actor before a receipt exists.  The authoring gate and mismatch disposition
   must precede client migration.
3. **Blocker C — directory transport framing:** current interfaces can neither
   offer subprotocols nor transmit the mandatory binary hello.
4. **Blocker D — atomic removal:** old document and directory routes, tag 0,
   URL/query tokens, hub persistence token fields, raw MCP `--token`, dev
   minting, fixtures and checked worker all remain.
5. **Blocker E — local bridge:** it is separate authority, but its live
   `/bridge?token=` keeps a no-secret-URL release claim false.

## Evidence Qualification

This audit reread live source after the accepted S1+S2 report.  It did not run
commands.  The prior attributed server-only gate (`bun nx run
os-hub:socket-grant-check --skip-nx-cache`, all 12 isolated laws, plus the
Bun/AJV/WebCrypto oracle) supports the v1 target only.  It is not evidence that
any S3 client, delivery relay, bridge conversion, generated worker, or admin
poll is implemented.  PostgreSQL/Neo4j S1+S2 runtime parity remains unproven.

## Live S3 Re-Read (In Progress)

The initial TypeScript patch now adds strict receipt parsing,
`SocketGrantIssuerV1`, a worker request/result shape, directory `/socket/v1`
redial with the ordered offer, and binary directory hello
(`🧰️framework/🛍️products/💻️os/🟦️.ts:3979-4029,4115-4190`; worker
`🟦️backbone-worker.ts:224-262,600-650,1055-1064`).  These are useful partial
closures, not an acceptance:

- `ShellHost` currently has no `socket-grant-request` branch in its worker
  `onmessage`, so the real worker has no issuer and its bounded request expires.
  It needs a trusted, origin/path/audience-validated broker with a host-side
  abort registry; the worker's local abort currently does not cancel the host
  POST.
- The worker assigns `state.actor` after receipt but accepts and relays caller-
  constructed envelopes unchanged (`backbone-worker.ts:714-727,1360-1374`).
  ShellHost also currently drops `socket-actor`.  Therefore pre-receipt
  synthetic envelopes can still reach the transport.  Install the returned actor
  in the command constructor/PluginRuntime before authoring, reject mismatches
  at the worker boundary, and fail/reconcile—not re-label—the old epoch.
- Both new browser dial sites must verify the negotiated `ws.protocol` is
  exactly `semio.socket.v1` before sending hello.  `DirectoryClient` still owns
  mutable raw-session/mint API, so it is not yet the required clean split.
- No native store/directory/MCP/bridge/admin/legacy-route/tag-0 cutover is
  present in this reread.  No gate has been attributed for this partial source.

The immediately subsequent live patch wires the request/cancel branches in
`ShellHost` and checks `ws.protocol` before either browser hello
(`ShellHost:1371-1408`, `backbone-worker.ts:600-650`, `🟦️.ts:4170-4190`).
Those partial closures expose two more release blockers:

- The proposed trusted browser route is hard-coded as `/_semio/hub${path}` with
  cookie credentials.  The dev Vite proxy forwards `/_semio` only when
  `S_LOCAL_RELAY_URL` exists (`…/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts:108-116`),
  and no concrete relay endpoint/delivery registration was found in this
  reread.  Existing `Identity.sessionToken`, `VITE_S_USER`, public mint and
  token-bearing bindings are still live.  This is not yet a verified protected
  upstream delivery boundary.
- `socket-actor` overwrites one global `shellActorIdRef` and PluginRuntime actor.
  That cannot model concurrent documents: especially a share receipt has a
  per-grant actor.  It can cross-wire a later document's actor into another
  document's command constructor.  Actor state and its authoring fence must be
  keyed by document/actor URI, with no usable authoring path before that
  document's receipt and no dispatch of an envelope that does not exactly match
  it.

## Live S3 Re-Read — Per-Document Stamping Revision

The preceding global-actor finding is superseded by the next live worker
revision.  This audit re-read it rather than carrying the earlier snapshot
forward.  `ArtifactState` owns `actor` and `hubActorReady` per document
(`🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:158-161`); a parsed,
unexpired receipt sets both before notifying the shell
(`:614-620`).  Before then, a hub-bound local mutation is rejected
(`:1369-1376`).  At the authority-bearing boundary, `relayMutationsToHub`
passes `state.actor` into `toWireEnvelope` (`:718-732`), whose explicit
argument replaces the caller envelope actor (`:408-421`).  Therefore the
outgoing `Commands` frame is per-document receipt-stamped, and two documents
with distinct share receipts do not share an actor variable.  The synchronous
`touchSpaceIndexArtifact` path still attaches without awaiting the shell's
`socket-actor` promise (`ShellHost/🟦️.tsx:5489-5502`), but its worker state
rejects pre-receipt mutations and stamps any later wire command; this is not
evidence of a forged actor reaching the hub.

This changes the required test, not the release result: add a two-document,
two-share fixture with deliberately different receipts and deliberately forged
local actors; assert each emitted `Commands` frame has only its document's
receipt actor, that a pre-receipt command is rejected, and that the direct
space-index path has the same result.  No such isolation law was present in
this reread.

The worker now also sends a cancel on abort *and* deadline
(`🟦️backbone-worker.ts:231-258`), and the host aborts the corresponding fetch
(`ShellHost/🟦️.tsx:1371-1400`).  These are partial progress/cancellation
closures.  Both pending registries remain unbounded (`socketGrantRequests` at
`🟦️backbone-worker.ts:226-229`; `socketGrantFetchesRef` at
`ShellHost/🟦️.tsx:1337,1382-1383`), so a malicious or runaway document-open
loop can retain arbitrary request state.  Bound the worker and host sides to
the same small fixed limit, reject before issuing, and add overflow plus
cancel-before/after-result laws.

The release remains **REJECTED**.  Browser code now removes the public
`VITE_S_USER` mint and binding token and uses cookie-bearing
`POST /_semio/hub…` for directory identity and grants
(`ShellHost/🟦️.tsx:836-870,1386-1394,1588-1595`), but no concrete local relay
or bootstrap delivery is present in this tree.  The only forwarding evidence
is the conditional Vite proxy controlled by `S_LOCAL_RELAY_URL`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts:108-115`).
The protected upstream session/relay boundary consequently cannot be audited
or exercised fail-closed.

All cross-lane atomic-removal blockers remain live in this re-read: Rust hub
still mounts `/directory/ws` and document `/ws` and accepts tag-0 `Hello`
(`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1898-1917,3484,3506`); native sync and
the Rust directory client retain token/tag-0 paths; MCP remote workspace still
requires and installs a token (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:319-325,440-462`);
the admin Connections page still uses its ordinary directory socket; and the
separate MCP bridge still has URL-token authority.  No S3 gate has been
attributed for this evolving source; prior S1/S2 server gates remain
insufficient evidence for it.

## Live S3 Re-Read — Admin And Bootstrap Provenance

The admin boundary has now made the intended narrow cutover.  `ConnectionsPage`
does not import `DirectoryClient` or create a WebSocket: it performs a
sequential, abortable authenticated `AdminClient.connections()` snapshot,
with a 2-second deadline and next poll only from `finally`
(`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔴️ConnectionsPage/🟦️.tsx:35-73`).
Unmount cancels the active request and scheduled timer; the rendered
`role=status` reports Fresh/ Stale (`:79-83`) with English and German strings
in `📚️I18n/🟦️.tsx:103-104,211-212`.  This is the safe S3 choice and removes
the unaudited admin socket audience.  It is only a **source-level partial
acceptance** until its laws prove stale-on-failure, unmount cancellation with
no next poll, single in-flight behavior, and absence of a WebSocket.  The
current test asserts only one fresh authenticated REST row
(`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️admin.test.tsx:132-150`);
no run has been attributed to this audit.
`TabsContent` renders only the selected tab, so its effect is also correctly
limited to the visible Connections panel
(`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🟦️.tsx:233-241`).

The repository does contain a real fd-3 inherited transport, but it is not
the requested native/MCP consumer cutover.  The hub itself opens inherited
descriptor 3 through `InheritedLocalBootstrapTransport`
(`🌎️hub/🔐️local-bootstrap/🦀️.rs:267-287,875-928`) and hub startup consumes it
only in development mode (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3646-3649`).
`🌎️hub/📦️packages/🦀️rust/📜️script.ts` contains a Node child proof of envelope
delivery, not a native OS or MCP workspace consumer.  The actual clients still
carry the old authority: native sync opens `/ws` and emits tag-0
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:757-767,1826,3215`),
and MCP validates and installs a raw token into `DirectoryClient`
(`…/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:319-325,440-462`).  fd-3 therefore
cannot be counted as a native/MCP SocketGrant delivery closure.  The required
next boundary is a one-time native/MCP consumer that authenticates its bounded
envelope over the inherited handle, retains the parent only in a private,
non-serializable issuer, and proves that neither the parent nor a derived
grant reaches a binding, `Debug`, argv, URL, persisted state, or error.

## Live S3 Re-Read — Secure Local Browser Relay

The newly landed `dev secure-suite` topology is an improvement but is **not an
S3 authority closure**. `DevScript` obtains a `react-relay` local-bootstrap
envelope, gives it to `startLocalBrowserRelay`, which takes the bearer into a
closure and blanks the envelope field; it then starts Vite as a direct child
with only a random relay channel secret
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:58-106,658-687`). The relay binds
`127.0.0.1`, requires the UI Host, loopback peer, its secret and
non-mismatching Origin/Referer, has a 64-request in-flight cap, streams a
1-MiB request cap, applies a two-second upstream deadline, returns only
content type, clears the bearer on upstream 401, and its `stop` method clears
both bearer and secret. `S_LOCAL_RELAY_SECRET` appears only in the direct
child environment and Vite server proxy configuration, not a browser `define`
or other bundle-facing source in this reread. These are source observations,
not a runtime oracle.

Three blockers prevent accepting that boundary.

1. Vite's proxy injects `x-semio-local-relay` unconditionally for every
   same-origin `/_semio` request
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts:108-115`).
   A request to Vite with no browser Origin or Referer is forwarded with the
   channel secret; the relay explicitly treats those headers as optional
   (`📜️script.ts:68-75`). Thus the proxy itself is an ambient same-origin
   bearer: any local/same-origin executable content that can request Vite can
   use the relay without holding a per-worker capability. Host and loopback
   peer checks do not identify that caller. Replace this blanket proxy with a
   bounded authenticated broker channel tied to the intended worker/renderer,
   or prove that every executable served at that origin (including plugin and
   dev assets) is in the same trusted authority domain. Add a negative
   no-Origin/no-Referer Vite request law and a cross-origin/local-process law;
   both must receive no upstream effect.
2. The upstream selection is not exact-scope. `localRelayUpstreamPath` admits
   broad `/directory/spaces/` and `/directory/events` prefixes and the relay
   admits every GET, POST or DELETE on every admitted route while forwarding
   arbitrary query (`📜️script.ts:26-32,76-84`). Bind method, canonical path
   parameters and allowable query fields per actually needed operation;
   reject an extension suffix, method confusion and unexpected query before
   contacting the hub.
3. Teardown is incomplete. Signal handling calls `relay.stop()` and kills the
   UI, but `DevScript`'s normal `finally` only calls `finishLocalHub(run)`
   (`📜️script.ts:664-667,689-696`). A normal hub/UI exit can leave the
   relay's bearer-bearing listener and/or sibling alive. Make one idempotent
   owner teardown run from every exit path: abort in-flight proxy work, stop
   the relay, zero secret/bearer, terminate/wait for the direct child, then
   finish the hub. Prove both child-first and hub-first exits close the port
   and make a former channel request fail 401/connection refusal.

The comparison is also early-exit bytewise rather than a constant-time local
secret comparison, and upstream responses are materialized with `arrayBuffer()`
without a response bound (`📜️script.ts:73,94`). These are additional
hardening/bounded-resource laws for the repair. No dev-secure-suite runtime
result has been attributed, so none is counted here.

There is also a canonical-origin mismatch to close before an oracle can be
trusted. The relay hard-codes `http://127.0.0.1:${S_OS_PORT}` and requires its
Host exactly (`📜️script.ts:67-75,677`), but the Vite config sets only `port`
and `strictPort`, not a loopback `host`
(`…/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts:105-117`), while the
launch ready-pattern accepts either `127.0.0.1` or `localhost`
(`.vscode/launch.json:4392-4410`). A normal localhost Vite proxy can therefore
fail the relay Host law, and the Vite bind/allowed-host posture is not proved.
Pin both servers and launch opening to one loopback canonical origin; prove the
canonical address works and localhost/cross-origin requests are rejected with
no relay or hub effect.

### Secure-Relay Revision Re-Read

The preceding second and third blockers, optional-header observation, and
canonical-origin mismatch are superseded by the next source revision. The
current relay requires **all** of exact Host, loopback peer, exact Origin,
Referer under the canonical UI origin, and `Sec-Fetch-Site: same-origin`
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:67-77`); it now carries a
method/path/query matrix rather than forwarding the original arbitrary query
(`:26-36,78-85`). `DevScript.finally` stops/zeroes the relay and kills the UI
before finishing the hub (`:689-696`), Vite pins `host: "127.0.0.1"`, and the
launch target opens only that canonical origin
(`…/⚙️vite.config.ts:105-118`, `.vscode/launch.json:4392-4410`). These are
proper source closures, subject to the requested runtime oracle.

The ambient-authority finding remains. The Vite proxy still adds the secret to
every `/_semio` request, and same-origin loaded plugin/extension code is a real
executable surface: Vite serves `/plugin-modules` and `/extensions`
(`⚙️vite.config.ts:99-100,138-154`), and the shard worker dynamically imports
the supplied module URL (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:195-203`).
Such code naturally produces the newly required browser headers and can make
authority-bearing relay requests without holding an authority assigned by the
ShellHost broker. This is not repaired by loopback, Host, Origin, Referer or
Fetch-Site checks because all describe the shared origin, not the invoking
plugin/worker. The safe boundary is a broker with a capability unobservable to
plugin/extension code, such as a private `MessagePort` held only by the
backbone worker plus a request/audience allowlist, rather than a Vite-wide
header injection. A negative plugin/extension same-origin request law must
prove it cannot mint a grant, issue a command, or read directory data.

More fundamentally, those headers are not an authenticated client-to-relay
channel. A local process can send a raw request to loopback Vite with the
canonical Host, Origin, Referer and `Sec-Fetch-Site: same-origin`; Vite injects
the channel secret, and the relay sees Vite—not that process—as its loopback
peer. Therefore the new header checks close accidental cross-origin browser
calls but do not establish provenance against a local caller. This is a
material **REJECT** for the claimed BFF authority boundary, not merely a
plugin-isolation preference. The runtime oracle must include a raw local Vite
request with all spoofed browser headers and prove it cannot cause a hub
request; the present proxy architecture cannot satisfy that law without moving
authority off the Vite-wide header injection path.

The required repair is architectural, not another HTTP-header filter. A Unix
socket only between Vite and relay does not help because the untrusted caller
still calls Vite. Either deliver the audience issuer through an OS/native
trusted IPC or preload boundary with a private `MessagePort` held only by the
ShellHost, or classify this external-browser dev relay as non-authoritative and
do not let it satisfy S3's protected browser delivery prerequisite.

### Browser-Fragment Broker Assessment

The proposed one-use 256-bit fragment plus private broker port is a viable
replacement for the ambient Vite header authority **for the stated threat
model**, but only if its proof is genuinely confined and replay-safe. The
current tree supports that boundary: the shell synchronously parses only an
exact lower-hex fragment and removes it from the visible history URL
(`ShellHost/🟦️.tsx:154-160`); it transfers a `MessageChannel` endpoint to the
backbone worker (`:1378-1381`); plugin shard code executes in its own worker
and dynamically imports its plugin module there
(`…/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:195-203`). The plugin-to-backbone
route accepts only decoded mutation or snapshot messages and constructs a
document `send`; it does not expose a worker handle or a socket-grant request
variant (`ShellHost/🟦️.tsx:2378-2410`). Thus an ordinary malicious shard module
cannot read `window.location` or the private port. This conclusion does not
extend to arbitrary untrusted JavaScript in the main browser realm, browser
extensions, or an operating-system adversary able to inspect the browser
process; those must remain outside the explicitly tested boundary.

The live partial patch is **not yet that design**:

1. The relay retains the raw `browserProof` and accepts it indefinitely
   (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:94,114-115,157`), not a digest-only
   one-use/rotating proof with replay rejection.
2. `ShellHost` retains a module-scoped raw proof and emits it from a direct
   main-thread fetch (`ShellHost/🟦️.tsx:154-160,1402`); sending a copy through
   `MessageChannel` does not make the port its sole holder. Move all proof use
   into the broker worker and do not retain it in the main shell after transfer.
3. The Vite proxy still fabricates its own relay-secret header. It may forward
   the browser proof, but needs an explicit source and runtime law that it
   never supplies, rewrites, logs, or reflects `x-semio-browser-broker`.
4. The supervisor opens the browser immediately after spawning Vite
   (`📜️script.ts:744-751`) with no Vite readiness probe, so the first and only
   fragment can reach a closed server. Wait for a bounded non-authoritative
   readiness endpoint before opening, and never print the fragment.

The bounded repair is: retain only `SHA-256(domain || proof)` at the relay;
atomically consume a proof before any upstream call, generate and return one
next proof only over the private broker response path, and persist the next
digest plus bounded request-id/replay state. A lost response must become an
explicit fail-closed rebootstrap rather than reuse the old proof or repeat an
authority action. The broker serializes/limits concurrent requests, rejects
cancelled/expired work, and owns the proof until teardown. The independent
oracle must prove: a raw local Vite caller with every spoofed browser header
but no proof has no hub effect; a replayed old proof has no effect; a malicious
shard module cannot read the fragment/port or issue a grant/command; the
canonical fragment is removed before plugin activation; no proof appears in
bundle, logs, errors, readiness, persistence, or a second UI child; and
loss/rebootstrap/teardown rejects further calls.

#### In-Progress Rotation Re-Read

The first live rotation attempt is not executable and must not be counted. In
`🟦️backbone-worker.ts:257-266`, `browserBrokerFetch` fills `current` with
zeroes before serializing it into `x-semio-browser-broker`; it therefore sends
64 zero hex characters rather than the current proof. The relay still compares
only a fixed raw proof and does not consume or interpret the advertised next
digest (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:94,114-115`). Separately,
ShellHost posts a raw string over the new port
(`ShellHost/🟦️.tsx:1378-1381`), whereas `attachLocalBrokerPort` accepts only
`{ kind: "initialize", proof }` (`🟦️backbone-worker.ts:281-289`), so the
worker ignores initial proof delivery. The shell also still has its direct raw
proof fetch path. These are immediate source blockers; preserve request bytes
until the network handoff completes, establish a single typed port protocol,
then remove the parallel shell carrier before any gate is meaningful.

The next re-read closes those first wiring defects: the relay derives then
retains only a SHA-256 proof digest and zeroes the initial raw buffer
(`📜️script.ts:94-100`); the worker serializes the current proof before
zeroing it and retains the generated next proof (`🟦️backbone-worker.ts:257-269`);
and its own `requestSocketGrant` and `DirectoryClient` now call
`browserBrokerFetch` (`:306-311,1100-1106`). This is meaningful source
progress, but still not acceptance. The old ShellHost grant request/cancel
map and response variants remain as dead parallel carriers
(`ShellHost/🟦️.tsx:1345,1386-1413,2433-2434`; `💻️os/🟦️.ts:719-720`) and must
be deleted. The worker retains a rotated proof after an HTTP 401 instead of
failing closed into explicit rebootstrap; the initial digest has no expiry;
the browser is still opened before a Vite readiness probe; and the port RPC
accepts generic `/_semio/hub/` GET/POST plus unbounded string body rather than
the exact, bounded broker operation union. No gate or runtime oracle has been
attributed for this evolving source.

The current rotation transition also lacks a commit acknowledgement. The
worker advances locally before sending; the relay may return capacity, path,
length, stopping, or other status before its digest replacement
(`📜️script.ts:115-127`), while `browserBrokerFetch` keeps its next proof for
ordinary non-throwing HTTP responses. The two sides then hold different
digests. Return a non-secret “broker proof advanced” marker only after the
relay atomically installs the advertised next digest, and retain the next proof
only on that marker; otherwise zero it and require rebootstrap. Cover full
capacity, malformed route/body, 401, cancellation after send, upstream 503,
and a replayed old proof with independent laws.

Finally, retain the response cap and constant-time comparison requirements;
the present `upstream.arrayBuffer()` is unbounded and `Buffer.every` exits on
the first mismatch (`📜️script.ts:74,95`).

The claimed method matrix still includes `DELETE /auth/sessions/me`
(`📜️script.ts:30`) although the S3 browser grant/directory callers do not need
that mutation. Remove it unless a separately audited UI operation owns it; a
relay allowlist must be the exact union of current broker operations, not a
convenience superset. `stop()` also clears values and stops the listener but
does not own/cancel or await in-flight upstream fetches (`:84-105,664-667`),
and may be invoked by both signal and `finally`. Make shutdown idempotent and
give each admitted proxy request a relay-owned abort signal; prove signal,
hub-first and UI-first shutdown leaves no pending authorized request and no
surviving child/listener.
