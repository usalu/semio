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

## Live S3 Re-Read — Current Native Mid-Migration State

This source-only read was made after the browser proof-rotation revision. No
build, test, process, browser, or runtime probe was run by this audit.

The earlier absence-of-UI-readiness finding is superseded. The secure launcher
now polls the canonical UI origin before it opens the fragment-bearing browser
URL (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:451-461,794-795`). Its relay also
uses a constant-time digest comparison, bounded response reading, an explicit
proof-advanced header, and worker-side fail-closed clearing when that header
is absent or fetch throws (`📜️script.ts:62-91,124-149`; `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:248-282`). These are useful source
closures only. The same implementation gives a deadline only to the initial
proof (`📜️script.ts:97-100,124-131`); every successfully rotated proof has no
expiry. There is no focused or runtime law in the current source census for a
replayed proof, lost/uncertain response, cancellation after send, capacity,
worker-only possession before plugin activation, or a zero-hub-effect
malicious-shard case. Consequently neither proof rotation nor relay delivery
is accepted as an S3 runtime boundary.

The native migration is currently internally incomplete. `PersistenceBinding::Hub`
no longer declares a serializable token (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:82-96`), but the browser-wasm actor still destructures and
stores it (`:3177-3224,3581-3607`) and sends legacy
`ClientFrame::Hello`; the native actor also sends that tag-0 frame
(`:1825-1834`). The WGPU shell test still destructures and expects that removed
field (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs:653-667`). This is a live
mid-patch compile/contract blocker, not evidence of removal.

`ArtifactHost` has a new mutable `set_local_hub_credential` setter
(`…/🏪️store/🔄️sync/🦀️.rs:1046-1084`), but the current tree has no caller.
The WGPU shell does call `restore_inherited` and receives
`IdentityOutcome.credential` (`…/Shell/🎯️targets/🧊️wgpu/🦀️.rs:4022-4052`),
yet does not pass it into the host before opening a document and calling
`plugin.attach_backbone` (`:3530-3533,3573-3575`). The current launcher
therefore has no demonstrated native delivery path, no pre-activation
consume-and-seal property, and no platform inheritance proof (CLOEXEC or
Windows no-inherit). A setter without a privileged, single-consumer caller is
not a credential-delivery boundary.

Atomic S3 removal remains false in the same current source: store sync still
constructs legacy document `/ws` URLs and tag-0 hello; the Rust directory
client still documents/uses `/directory/ws` and a token field
(`…/📇️directory/🔌️client/🦀️.rs:263,514-538`); MCP workspace still requires
raw `--token` and invokes `set_token` (`…/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:319-325,462`);
MCP still logs and tests `/bridge?token=` (`…/🌉️mcp/🦀️.rs:626-739`,
`…/🌉️mcp/🧵️bridge/🦀️.rs:3166-3206`); and four configured launch profiles
still export `S_USER` (`.vscode/launch.json:2510,2534,2558,2582`). The earlier
safe admin REST-poll observation remains source-only and does not offset these
cross-lane blockers.

## Live Browser Ratchet Re-Read — 2026-09-03

This is a source and test-census audit only. It does not treat an implementation
report, a compile, or a server-side SocketGrant suite as proof that the browser
authority boundary ran.

### Superseded Source Findings

The previous missing-per-generation-TTL finding is superseded. Both relay and
worker now bind every installed proof generation to the same fifteen-second
deadline: the relay initializes and renews `browserProofExpiresAtMs`
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:105,136-137`) and the worker does the
same at installation and only after a commit acknowledgement
(`🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:272-273,300-302`). The
earlier absence of a commit marker is also superseded: the relay replaces its
digest before the protected upstream call and only then returns
`x-semio-browser-broker-advanced: 1` (`📜️script.ts:132-148`); the worker keeps
the next raw proof only when that exact marker is returned and status is not
401 (`🟦️backbone-worker.ts:292-312`). A lost response, bad marker, exception,
or 401 therefore discards the candidate next proof rather than retrying an
ambiguous authority operation.

The digest construction is now domain separated on both ends:
`SHA-256("semio/browser-broker-proof/v1\\0" || proof)` at
`📜️script.ts:25-29` and `🟦️backbone-worker.ts:232,251-257`. Relay comparison
uses Node's `timingSafeEqual` after fixed-format validation
(`📜️script.ts:129-136`). The worker serializes broker requests and caps its
queue at 64 (`🟦️backbone-worker.ts:277-314`), while the relay independently
caps in-flight work and bounds request/response bodies (`📜️script.ts:43-91,
126-148`). These are meaningful source closures, not yet an acceptance proof.

### Remaining Browser S3 Blockers

1. **No focused browser-boundary law exists.** The live worker test inventory
   has no broker/relay test group (its suites begin at
   `🟦️backbone-worker.ts:1597,1698,1846,1953,2022`), and the registered
   `socket-grant-check` executes Rust hub/lib tests plus `cargo check` only
   (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:727-753`). It cannot demonstrate the
   browser relay. There is no evidence-backed negative for replay, uncertain
   response after admission, cancellation after send, expiry after a rotation,
   capacity exhaustion, raw-local Vite use, or a malicious plugin/shard. Add
   focused execution of those laws before accepting this boundary.

2. **The broker transport is not an exact typed capability protocol.**
   ShellHost posts a raw `initialize` object carrying a raw proof string
   (`…/ShellHost/🟦️.tsx:1422-1427`); worker dispatch casts each port payload to
   `Record<string, unknown>` and switches untyped string fields
   (`🟦️backbone-worker.ts:319-346`). The narrow `LocalBrowserBrokerPort.me()`
   façade is useful, but it does not turn this runtime message grammar into a
   defined discriminated request/response union with strict reject/close laws.
   Define and decode that exact boundary, including initialization, request,
   cancel, response, and terminal/teardown messages.

3. **Worker-only possession/pre-activation are unproved.** The fragment is
   removed synchronously and the main-shell variable is set to `undefined`
   after transfer (`ShellHost/🟦️.tsx:153-159,1425-1428`), but the raw proof is
   nevertheless a main-thread JavaScript string during that handoff. The
   source census contains no executable order law that transfer and removal
   finish before any plugin/shard activation, nor a malicious-shard runtime
   proof that it cannot obtain the fragment, port, grant, command, or directory
   response. The desired authority property cannot be inferred from module
   scope alone.

4. **Zeroing/redaction needs a focused check.** Initial buffer and retained
   worker bytes are cleared (`📜️script.ts:103-105`; `🟦️backbone-worker.ts:260-263`),
   but the relay leaves the transient `currentDigest` and the buffer decoded
   from the raw request proof uncleared after comparison (`📜️script.ts:132-133`).
   JavaScript headers/strings cannot be reliably scrubbed, so the boundary must
   explicitly document that limitation and demonstrate no proof in logs,
   errors, readiness, persistence, bundle output, or a second child rather
   than overclaim memory zeroization.

The rest of S3 remains blocked independently by the native and MCP migration
residues documented above. These browser findings do not supersede those
cross-lane blockers.

### Live Amendment — Typed Broker and Focused Laws Landed

The immediately preceding claims that no focused broker tests exist and that
the port grammar is untyped are superseded by the next live revision. The
shared `BrowserBrokerPortRequestV1`/`BrowserBrokerPortResponseV1` unions now
admit only exact-key initialized, `me`, cancel, and bounded response shapes
(`🧰️framework/🛍️products/💻️os/🟦️.ts:3984-4009`); ShellHost and the worker both
decode these forms rather than casting arbitrary records
(`…/ShellHost/🟦️.tsx:169-200,1430-1434`; `🟦️backbone-worker.ts:325-351`). The
relay also now clears the decoded current-proof and derived-digest buffers
after comparison (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:134-140`).

A noncached `os-hub:browser-broker-check` target is registered
(`🌎️hub/📦️packages/🦀️rust/📋️project.json:71-77`). Its Bun loopback oracle
exercises raw-local/no-proof denial even with the relay secret, same-origin
shard/no-proof denial, admitted ratchet acknowledgement, old-proof replay
denial, upstream-401 closure, and cancellation after upstream admission
(`📜️script.ts:720-789`). The worker Vitest block separately covers explicit
ack/digest binding, lost acknowledgement, 401, cancellation, expiry,
capacity, and a source-order/shard census (`🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:1583-1680`). This audit did not execute either target; their
presence is source evidence, not a passing result.

Two scope-qualified gaps remain before browser acceptance. First, the oracle
named as rotated-TTL coverage starts a new five-millisecond relay and waits;
it does **not** first successfully ratchet and then prove expiry of that next
generation (`📜️script.ts:755-761`). Add that exact sequence. Second, the
malicious-shard check reads source text and relative order; it does not run a
hostile shard against an actual browser worker, Vite proxy, and private port
(`🟦️backbone-worker.ts:1670-1678`). The new browser-broker target runs the Bun
relay oracle but not that worker Vitest block, so a single release gate does
not yet execute every law it claims. Keep the native/MCP/tag-0 migration
blockers independently open.

### Live Amendment — Consolidated Browser Gate Independently Executed

The two immediately preceding browser gaps are superseded by the current live
revision. The relay oracle now first admits and acknowledges a proof rotation,
then waits beyond the per-generation TTL and proves that the rotated proof is
rejected without a second upstream effect
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:784-793`). Its hostile-shard law now
starts a real isolated `Worker`, gives it the relay endpoint and ambient relay
secret plus spoofable same-origin headers, and proves it has neither a proof,
private port, nor fragment and receives `401` with zero upstream effect
(`📜️script.ts:717-743,774-775`). This is materially stronger than the prior
static source census; it is still a Bun worker/loopback oracle, not a browser
engine/Vite end-to-end run.

I independently ran, uncached, on 2026-09-03:

```
bun nx run os-hub:browser-broker-check --skip-nx-cache
```

It exited `0`. The relay oracle completed, and its consolidated child command
ran `@semio-tech/framework-os:test-quick` with the focused selection
`browser broker proof ratchet|queues a directory command while the hub is
unreachable`. Vitest reported **1 passed / 2 skipped files** and **5 passed /
225 skipped tests** in 7.13s; Nx reported both `test-quick` and
`browser-broker-check` successful. The target's final assertion summary named
raw-local and shard denial, one-use ratchet, replay rejection, rotated TTL,
upstream-401 epoch closure, cancel-after-send, and redaction. The only output
warnings were that `NO_COLOR` was ignored because `FORCE_COLOR` was set; there
were no test or runtime failures.

On source re-read, the scoped browser boundary now has domain-separated
SHA-256 proof digests, fixed-shape timing-safe comparison, destructive buffer
clearing where bytes are available, exact typed MessagePort grammar, explicit
advance acknowledgement, per-generation expiry, serialized/capped requests,
and fail-closed loss/401/cancellation paths
(`📜️script.ts:25-29,102-166,774-827,892-896`; `🟦️.ts:3984-4009`;
`🟦️backbone-worker.ts:232-351,1583-1681`). I accept that **browser-only
sub-boundary** at this evidence level. This does not make S3 release-ready:
the live native credential-consumer/launch ordering and inherited-handle work,
legacy tag-0/socket routes, raw MCP token and bridge-query paths, and launcher
environment residue documented above remain independent release blockers.

### Live Native Consumer Re-Read — Inherited Handle Still Not Sealed

The native source now has a real one-shot credential reader, which is progress
over a setter-only design: `IdentityEnv` requires `S_LOCAL_CREDENTIAL_FD=3`,
`restore_inherited` obtains a `LocalHubCredential` before `/me`, and the
credential type redacts `Debug` and zeroes its owned capability on final drop
(`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️identity/🦀️.rs:54-77`;
`…/📇️directory/🔌️client/🦀️.rs:315-328`). The JavaScript launcher also writes
a bounded, class-bound, framed envelope over child fd 3 and clears the source
envelope capability (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:513-545`).

It is not yet an acceptable native delivery boundary. On Unix the reader
immediately adopts fd 3 with `File::from_raw_fd(3)` and reads it
(`…/📇️directory/🔌️client/🦀️.rs:334-366`), but does not set `FD_CLOEXEC` before
the credential can coexist with plugin activation; no equivalent Unix sealing
or descendant-inheritance rejection law is present. The Windows duplication
requests a non-inheritable duplicate (`DuplicateHandle(..., 0, 2)` at
`:384-405`), but that does not close the Unix gap. The JavaScript delivery
check uses a synthetic Node child, not the actual WGPU launcher/reader, so it
does not prove consume-before-plugin ordering, direct fd sealing, or that a
child plugin cannot retain the endpoint.

Moreover, `poll_identity_bootstrap` uses the restored credential to create a
directory client but never calls `document_host.set_local_hub_credential`;
the only setter remains at `…/🏪️store/🔄️sync/🦀️.rs:1070-1072`, while WGPU's
success path is `…/Shell/🎯️targets/🧊️wgpu/🦀️.rs:4044-4054`. Document actors
therefore still have no demonstrated authenticated native path before
`open`/`attach_backbone`. Combined with the live token-bearing tag-0 actors,
legacy WebSocket constructors, and stale WGPU `PersistenceBinding::Hub`
token field shown above, this remains an S3 release blocker. No native runtime
command was run for this amendment; it is source evidence only.

The launcher helper is also not wired into a real consumer launch: current
references to `deliverNativeCredentialEnvelope` and
`deliverMcpCredentialEnvelope` are their declarations and the synthetic
security-smoke calls only (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:543-547,
634-646`). The normal `dev` path issues only the React relay envelope
(`📜️script.ts:919-949`). Thus even if the synthetic Node fd-3 reader succeeds,
there is no source-supported claim that the actual WGPU executable or the MCP
remote process receives a capability through the helper. This is a separate
runtime wiring block, not merely missing test coverage.

### Live Amendment — Native Handle-Sealing Source Change (Fanout Incomplete)

The prior statement that the Unix reader lacks `FD_CLOEXEC` is superseded.
The live reader now calls `fcntl(F_GETFD)` then `fcntl(F_SETFD,
FD_CLOEXEC)` before adopting fd 3; its Windows path duplicates a
non-inheritable handle and closes the inherited CRT fd before use
(`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:338-423`).
The new identity module also declares a process-global `OnceLock` claim API
which consumes the fd before returning an `Arc<LocalHubCredential>`
(`…/📇️directory/🪪️identity/🦀️.rs:63-94`). Those are source-level native
sealing improvements.

They do not close the release finding in this live revision. WGPU still
imports and calls the old `restore_inherited` path rather than the new claim
and restore-claimed APIs (`…/Shell/🎯️targets/🧊️wgpu/🦀️.rs:34-38,4022-4034`),
creates plugin apps before starting identity bootstrap (`:2269-2319`), and
still does not inject the restored credential or a grant source into
`ArtifactHost` (`:4043-4052`). Native document dialling still derives the
legacy `/ws` URL (`…/🏪️store/🔄️sync/🦀️.rs:754-766`) despite a partially added
grant-source seam. The new claim is untested for competing callers or a
descendant attempt, and normal launcher references remain synthetic rather
than actual WGPU/MCP direct-child provisioning. This amendment is a live
mid-fanout source audit; no runtime acceptance is claimed.

### Live Amendment — WGPU Claim Ordering and Remaining Actual-Launch Block

The immediately preceding claim that WGPU still calls the old inherited-restore
path and injects no credential/grant source is superseded. The actual WGPU
binary now calls `claim_inherited_local_hub_credential("native")` before any
argument lookup, scale/smoke branch, or call to `run_native`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs:5-47`).
The binary package includes that exact entrypoint
(`…/📦️packages/🦀️rust/💾️binary/🦀️.rs:1-3`). `LocalHubCredential` is no
longer `Clone`, redacts its `Debug` representation, and zeroes its owned
capability on final drop (`…/📇️directory/🔌️client/🦀️.rs:323-338`). Shell
construction installs the claimed credential in `ArtifactHost`, and successful
identity bootstrap then installs both that credential and a typed
`HubSocketGrantSource` backed by an authenticated `DirectoryClient`
(`…/Shell/🎯️targets/🧊️wgpu/🦀️.rs:2043-2048,4052-4057`). The typed source
issues the path-bound document receipt with the private capability, validates
the v1 receipt, and does not expose a raw token in its trait
(`…/📇️directory/🔌️client/🦀️.rs:277-279,514-556`).

This is still not an acceptable native launch proof. The only calls to
`deliverNativeCredentialEnvelope` are the synthetic Node child delivery proof;
no local-dev or production WGPU invocation calls it
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:513-579,625-646`). There is no
native-binary direct-child test showing: fd3 is provided to the real WGPU
process; it is consumed exactly once before any plugin activation; a descendant
cannot inherit/read it; racing claims fail closed; or a debug/error path cannot
serialize a capability. The `OnceLock` claim code is source progress, but its
only identity test continues to test a nonsecret `Identity`, not the inherited
credential or claim law (`…/📇️directory/🪪️identity/🦀️.rs:63-99`).

The document cutover itself also remains incomplete: the native actor still
constructs the retired `/spaces/{space}/documents/{document}/ws` URL (and an
optional query `surface`) (`…/🏪️store/🔄️sync/🦀️.rs:754-770`); the wasm actor
still retains `hub_token` and sends tag-0 `ClientFrame::Hello`
(`…/🏪️store/🔄️sync/🦀️.rs:3242-3290,3648-3672`). Separate MCP paths still
accept raw hub `--token`, invoke `DirectoryClient::set_token`, and log a bridge
URL containing `?token=` (`…/🌉️mcp/📦️bin.rs:28-103`;
`…/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:323-325,459-463`;
`…/🌉️mcp/🦀️.rs:736-741`). This amendment is a live source read only; no
native direct-child runtime, race, descendant, or actor-epoch acceptance is
claimed.

### Live Amendment — Native V1 Route Source Closure, Gate Still Unwired

The previous finding that the native actor dials the legacy document route is
superseded by the next live change. `hub_ws_url` now derives the exact
`/spaces/{space}/documents/{document}/socket/v1` route and the native actor
offers its receipt as the second `Sec-WebSocket-Protocol` value, requires the
v1 negotiated response, then sends `SocketHelloV1`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:754-766,1842-1895`).
The receipt actor is retained only after that successful upgrade; outbound
mutation envelopes are rewritten to it and a mismatched server `Session`
actor fails closed (`:1876-1895,2169-2173,2227-2246`). This is a meaningful
native actor-epoch source closure, not runtime proof.

The same live revision has added a process-backed WGPU probe which launches
the actual native executable through fd3 delivery, asserts the descendant no
longer sees fd3, caps output, and rejects capability reflection
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:551-572`; WGPU entrypoint
`…/⌨️native-entrypoint/🦀️.rs:19-50`). At this read it is **unwired**:
`runSecureLocalSmoke` does not call either the real probe or its source-order
check (`📜️script.ts:625-692`), and no `dev-secure-native` launch registration
exists yet even though that source-order check requires one. It consequently
cannot be treated as an executed direct-child, descendant, redaction, or
normal-launch test.

There is also a currently stale test assertion: `hub_ws_url` returns `/socket/v1`
but its unit test still expects `/ws` (`…/🏪️store/🔄️sync/🦀️.rs:4715-4723`).
The actor path therefore lacks even a green focused compilation/test result at
this point. A remaining exhaustive search continues to find the wasm
`hub_token`/tag-0 actor, hub `/ws` and `/directory/ws` routes, raw MCP hub
`--token`/`set_token`, and MCP bridge query token paths. This amendment does
not supersede those atomic S3 migration blockers.

### Live Amendment — Real WGPU Probe Is Wired, But Does Not Exercise Sync

The immediately preceding statement that the WGPU probe and secure-native
registration are unwired is superseded. `secure-local-smoke` now source-checks
claim order, builds the WGPU native binary, and calls the actual fd3 WGPU
`--credential-probe`; the binary path honors `CARGO_TARGET_DIR`
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:551-575,645-674,1010-1017`). The
normal secure-native dev route also builds then launches the direct WGPU binary
with a native envelope, and the generated launch configuration registers
`os-hub:dev-secure-native` (`📜️script.ts:942-975`; `.vscode/🧩️launch.seed.jsonc:3091-3097`).
This restores a real process-backed fd3/descendant-sealing probe to the native
gate rather than the former Node-only proxy.

It remains insufficient for native SocketGrant acceptance. The probe exits
before Shell, plugin, identity bootstrap, or any document actor runs. It can
prove owned fd consumption, descriptor sealing, bounded nonreflective output,
and static pre-plugin source order, but it does **not** prove that a real WGPU
process: issues a protected document receipt; dials the v1 route; receives and
uses the server-issued actor; rewrites a mutation; or handles a reconnect by
obtaining a fresh actor/receipt. `dev-secure-native` starts such a process but
has no automated assertion for those facts. The source’s stale `/ws` URL unit
expectation and wasm/tag-0/MCP legacy residues remain live at this audit point.
No final native gate result is claimed here.

### Live Amendment — V1 Client Is Present; Legacy Server and MCP Paths Still Block

The prior native-actor route finding remains superseded: the current native
`hub_ws_url` implementation derives the v1 document route, and the native
actor source uses the receipt-backed v1 negotiation.  However, the immediately
adjacent focused unit test is still stale: it expects `/documents/{id}/ws`
although the function returns `/documents/{id}/socket/v1`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:754-766,4716-4723`).
It is therefore not a passing regression law and will fail when compiled.

More importantly, the hub still registers the retired runtime endpoints
`/directory/ws` and `/spaces/{space_id}/documents/{id}/ws`
(`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3761,3784`), and the same server still
admits token-bearing tag-0 `ClientFrame::Hello` and resolves its token
(`:2179,2192-2194`).  Its live test suite continues to execute those `/ws` and
`/directory/ws?token=` shapes.  This is an active legacy credential path, not
just stale comments or fixtures; v1 client fanout does not remove it.

MCP is likewise still pre-cutover: remote workspace validation demands raw
`--token` and calls `DirectoryClient::set_token`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:323-325,462`),
while the bridge runtime/tests retain `?token=`.  The `set_token` call also
appears inconsistent with the current typed directory-client direction and
must be removed/migrated rather than ignored.  These legacy server and MCP
findings block S3 acceptance even if the final secure-local native probe is
green.  No final native runtime command is recorded in this amendment.

### Live Amendment — Process-Backed Actor Probe Added; Claim Is Still Racy

The previous narrow statement that the real WGPU probe stops before document
sync is superseded by the newest live source. `secure-local-smoke` now runs a
real fd3-provisioned WGPU `--socket-grant-probe` after the secure hub smoke
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:1074-1099`). Its in-process test hub
requires an authenticated receipt POST and a one-use v1 subprotocol grant,
then asserts three grants, three v1 upgrades, three tag-7 hellos, two
actor-stamped mutations, a deliberately wrong first `Session` actor, and a
forced reconnect before accepting the second mutation (`:579-654`). The WGPU
probe itself constructs `ArtifactHost` with the claimed credential and typed
receipt source, waits for server `Session`, sends mutations, and requires two
accepted outcomes (`…/🧊️renderer/🦀️.rs:15861-15932`). The surrounding driver
caps stdout/stderr and rejects a local capability reflection. This is now a
process-backed receipt/actor/reconnect law in source, rather than a mere fd
probe. It has not yet been independently executed by this audit.

The advertised process-global one-shot claim is nevertheless not race-safe.
`claim_inherited_local_hub_credential` first calls
`LocalHubCredential::read_inherited` and only then invokes `OnceLock::set`
(`…/📇️directory/🪪️identity/🦀️.rs:68-72`). Concurrent callers can therefore
both reach `from_raw_fd(3)`/the read before either setter wins, yielding
competing ownership and reads of the same descriptor; the losing caller fails
late, after the secret boundary has already been entered. There is no
concurrent-claimer regression test. A lock/claim state must be acquired
before opening fd3, with a process-backed or equivalent race assertion, before
the one-shot/descendant law is accepted.

### Live Amendment — Concurrent fd Claim Is Serialized, Class Recheck Is Missing

The prior pre-`OnceLock::set` double-owner race is superseded. The live claim
cell is now `OnceLock<Result<Arc<LocalHubCredential>, IdentityError>>`, and
`get_or_init` runs `read_inherited` only in its one initializer while other
callers wait (`…/📇️directory/🪪️identity/🦀️.rs:63-74`). At source level this
prevents simultaneous callers from each constructing a `File` from fd3; a
stored failure also fails closed rather than retrying a consumed descriptor.

It introduces/retains a class-confusion hole: `expected_class` is consulted
only by that winning initializer. After a successful
`claim_inherited_local_hub_credential("native")`, a later
`claim_inherited_local_hub_credential("mcp")` clones the initialized `Ok(Arc)`
without comparing the requested class (`:72-78`). Thus a wrong-class caller can
receive the native credential. The stored claim must carry its bound class (or
the function must check it on every call) and reject mismatches. There is also
still no regression that races simultaneous claims or proves the
wrong-class-after-success rejection, so neither source serialization nor
class isolation is accepted as runtime evidence yet.

### Live Amendment — Class-Bound Once Claim and Deterministic Race Law

The preceding wrong-class claim finding is superseded by the live repair.
`ClaimedLocalHubCredential` now stores both `client_class` and the terminal
result in the single `OnceLock`; the helper initializes that cell once, compares
the stored class with every later `expected_class`, and returns an error before
cloning a mismatched credential
(`…/📇️directory/🪪️identity/🦀️.rs:63-100`). Its deterministic law races two
same-class callers through a barrier and asserts exactly one reader execution
and `Arc` identity, then rejects `mcp` after `native` and proves a failed first
read is terminal (`:131-165`). This closes the source-level simultaneous
double-read/double-close and direct wrong-class-claim findings.

The public `claimed_local_hub_credential()` accessor is intentionally only
valid after the process entry claim, but has no class argument; current callers
are native restore/probe paths. Future non-native callers must continue to use
the class-bound claim function rather than that unrestricted getter. Independent
runtime evidence for the repaired native boundary is still pending; the legacy
hub/MCP cutover blockers in the preceding amendment are unaffected.

### Live Amendment — Class-Bound Accessor Fanout Closed in Source

The accessor residual in the preceding amendment is superseded. The live
`claimed_local_hub_credential` now requires `expected_class`, filters the
stored class before returning an `Arc`, and `restore_claimed`, Shell, and the
WGPU socket-grant probe all pass `"native"`
(`…/📇️directory/🪪️identity/🦀️.rs:99-110`; `…/Shell/🎯️targets/🧊️wgpu/🦀️.rs:2043-2047`;
`…/🧊️renderer/🦀️.rs:15868-15872`). This closes the current source fanout
route by which a future wrong-class caller could have used the no-argument
getter. The one-read, class-bound, terminal-failure unit law remains in place.
Final native acceptance still requires the independently executed
process-backed gate; it cannot be inferred from the source repair.

### Live Amendment — Store V1 URL Regression Law Is Source-Closed

The earlier claim that the store URL law still expected the retired `/ws`
endpoint is superseded. The current helper derives
`/spaces/{space}/documents/{document}/socket/v1` (with only its non-secret
surface selector when present), and every local assertion now expects that v1
path (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:759-766,4716-4723`).
The native actor also obtains a fresh typed `HubSocketGrantSource` receipt,
offers the v1 subprotocol and grant, verifies the selected protocol, and sends
tag-7 `SocketHelloV1` (`:1846-1894`). This removes the specific stale-test
observation; it is source evidence only, not a completed native runtime gate.

The S3 cutover itself remains blocked independently: the hub still registers
the retired directory/document websocket routes and accepts/resolves token
tag-0 Hello (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2175-2194,3759-3784`), while MCP
still takes raw hub `--token`, invokes the retired `DirectoryClient::set_token`,
and exposes the bridge token in a query URL/log/test
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs:28-103;
🏠️workspace/🔗️remote/🦀️.rs:323-325,462,635; 🦀️.rs:739;
🧵️bridge/🦀️.rs:2525-2529,3164-3206`).

### Live Finding — Failed Native Socket Keeps Its Prior Receipt Actor

The native actor stamps outgoing envelopes with the receipt actor and refuses a
wrong `ServerFrame::Session`, which is correct as far as it goes
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2169-2175,2227-2247`).
But the failure paths do not consistently end that actor epoch. In particular,
`fail_artifact_bootstrap` tears down the websocket and schedules reconnect
without setting `socket_actor = None` (`:1801-1814`), and the EOF/error reader
arm does the same (`:1908-1924`). A failed connection and a failed write clear
it (`:1893-1895,2261-2267`), demonstrating that this is an incomplete fanout,
not an intentional immutable identity.

Consequently a forged initial `Session` or forced remote close retains the
prior receipt actor in process memory until a later successful connect happens
to overwrite it. The new process-backed oracle checks that three grants,
three sockets, three tag-7 hellos and two actor-stamped mutations occur, but
does not observe `socket_actor` between the terminal failure and replacement;
it can therefore pass without proving the required clear-on-failure actor
epoch (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:579-651`). Clear that field in
every terminal teardown path and add an explicit wrong-Session/EOF assertion
before treating native actor recovery as accepted.

### Live Amendment — Native Actor Epoch Is Source-Closed; Runtime Pending

The preceding actor-epoch finding is superseded by the live repair.
`clear_socket_epoch` clears the actor, its confirmation bit, and session color;
it is now called by bootstrap failure, EOF/transport failure, failed connect,
replay failure, and failed write paths
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1803-1821,1904-1906,
1930-1935,2024-2030,2302-2307`). A new receipt is explicitly unconfirmed;
outbound local operations queue without being stamped until a matching
`ServerFrame::Session` confirms it (`:1885-1903,2208-2215,2263-2288`).

The focused native law directly exercises bootstrap failure and EOF, proves
the old actor/confirmation/color are absent, verifies locally-authored
envelopes stay queued with their original local actor, and then proves an
unconfirmed fresh receipt still cannot stamp outbound work before its matching
Session (`:4563-4613`). A full source census of every `semio_hub = None` path
now finds the epoch clear adjacent to each one. This is source/focused-law
closure only: the separately process-backed secure-local rerun remains pending
and cannot be substituted by this unit law.

The secure native source launch seed and generated launch no longer contain
`S_USER` and retain the direct `os-hub:dev-secure-native` command
(`.vscode/🧩️launch.seed.jsonc:3092-3104; .vscode/launch.json:4440-4452`).
The implementation report records generator session `71854` and freshness
session `90231` green, including an S_USER/VITE_S_USER census of both files.
This audit independently confirms the live absence but did not itself rerun
the generator; that reported freshness is supporting evidence, not a substitute
for the remaining native process gate or S3-wide carrier removal.

### Live Finding — Receipt Confirmation Strands the Reconnect Outbox

The epoch repair correctly refuses to stamp outbound mutations before an exact
Session confirms the freshly issued receipt. It also creates a live reconnect
liveness failure. On a `Bootstrap::None` Welcome, the actor marks itself live
and calls `flush_outbox` before the Session frame arrives
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2103-2115`).
The new confirmation condition consequently returns those envelopes to the
outbox (`:2263-2276`). The later matching `ServerFrame::Session` only sets the
confirmation bit/emits an event; it never flushes that requeued outbox
(`:2208-2216`).

Any mutation already pending from an offline period or a closed prior socket is
therefore stranded after reconnect, despite a successful new receipt and
matching Session, until an unrelated later trigger happens to flush it. The
new focused law proves clearing and unconfirmed queueing but not confirmation-
driven delivery, and the process probe creates mutations only after Session;
neither detects this path. Flush the outbox after matching confirmation and add
a process-backed or focused law that queues before Welcome/Session and requires
the subsequent accepted actor-stamped command after confirmation. This blocks
native S3 acceptance.

### Live Amendment — Confirmation Flush Is Source-Closed; Delivery Proof Is Still Missing

The immediate liveness defect above is superseded in current source:
after an exact matching `ServerFrame::Session`, the native actor marks the
receipt confirmed, emits the session event, and calls `flush_outbox`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2208-2216`).
With a live hub connection this reaches the existing actor-stamping relay only
after confirmation (`:2271-2288`), so its source ordering is correct.

The new focused test does not prove that delivery. Its
`install_test_socket_actor` only sets the receipt/confirmation fields and
does not install `semio_hub` (`:2065-2068`); its state helper exposes the
outbox and pending-batch counts (`:2071-2072`). The test queues an envelope
before a fresh Session and, after injecting that Session, asserts only receipt
identity and the confirmation bit (`:4600-4607`). Because `flush_outbox`
returns when no hub is installed (`:1824-1826`), this test cannot observe an
actor-stamped command becoming a pending batch or reaching a peer.

Likewise the process probe sends each mutation from an `ArtifactEvent::Session`
handler and expects only two post-Session mutations (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:619-649`);
it never queues a local mutation across a forced close and then requires the
fresh Session to flush it. Retain the source repair as provisional and add a
focused transport harness or process oracle that queues before the new Session,
then requires one accepted envelope with the fresh actor after confirmation.
Until that observable law and the independent process-backed gate run, native
S3 acceptance remains blocked.

### Live Finding — Native Envelope Decode Leaves a Non-Zeroed Capability Copy

`LocalHubCredential` correctly redacts `Debug` and zeroes its final boxed
capability on drop (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:319-333`).
The inherited-envelope parser also clears its framed input `Vec<u8>` immediately
after `decode_json_bytes` (`:358-372`). That is not complete transient-secret
zeroization: the JSON decode returns a `DslValue` holding the capability as an
owned string; the parser borrows it and then copies it into the final
`Box<[u8]>` (`:373-388`). When `DslValue` drops, its string allocation is
deallocated without a wipe, leaving a second parsed capability copy in process
memory for the allocator to reuse. The same nonzeroed temporary occurs on a
post-decode validation error.

This does not expose the capability through `Debug`, argv, URL, or durable
state, but it contradicts a stronger claim that parsed secret bytes are fully
zeroed or never cloned. Move the capability out of a zeroizable decoded
envelope (or explicitly wipe its owned string on every success/error path),
and add a focused ownership/zeroization law. Native source/runtime acceptance
must treat the existing raw-buffer wipe and final-drop wipe as partial rather
than complete secret erasure.

### Live Amendment — Native Decode Wipe Is Source-Closed; Focused Runtime Pending

The preceding decode-copy finding is superseded in current source. Raw envelope
bytes are owned by `WipeBytes`, which wipes on every read, trailing-byte, decode,
and validation exit (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:324-329,404-437`).
After a successful JSON decode, `WipeDslValue` recursively zeroes all decoded
strings and object keys during unwinding (`:332-359`). A valid capability is
moved, rather than copied, out of its decoded `String` through
`take(...).into_bytes().into_boxed_slice()` into the final zero-on-drop
credential (`:361-399`); invalid field/class/expiry paths retain the guard and
therefore wipe the decoded allocation.

The new serde-json-backed focused law observes nonzero decoded-field/key wiping
on success and at least the complete capability allocation on an invalid
expected-class path (`:1363-1384`). Source review finds no ordinary `Result`
early return after the decoded guard is constructed that bypasses its `Drop`.
As with all Rust drop-based erasure, process abort/allocator-failure is outside
that guarantee; the 16 KiB bounded envelope also remains the relevant input
cap. Session `14348` was red before any native law ran on the unrelated
`ProgramContributionEntry: serde::Serialize` compile error, so this amendment
is source/focused-law evidence only until an independently observed rerun
terminates.

### Live Amendment — Receipt-Confirmation Delivery Law Is Source-Closed; Runtime Pending

The previous delivery-proof gap is superseded by the current focused native
law. It now opens an observable loopback WebSocket through the real native
connect path, observes `SocketHelloV1`, queues three local envelopes, and
asserts no frame crosses before a matching Session. That Session must produce
one three-envelope `Commands` batch whose every actor is the fresh receipt
actor, no duplicate frame, and an empty outbox after acknowledgement
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:4564-4658`).
It then forces EOF, queues another local envelope, and repeats the zero-before-
Session / exactly-one-fresh-actor-after-Session assertion (`:4659-4670`). This
directly observes the previously untested confirmation-triggered flush.

The WGPU process probe likewise now queues its first mutation immediately after
opening the actor, before any welcome/session event. Its deliberately wrong
first receipt Session must not emit that mutation; the second fresh Session
causes its accepted first command, forced close, third fresh receipt, and a
second accepted command (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:579-651`;
`…/🧊️renderer/🦀️.rs:15860-15951`). The trace still requires exactly three
grants, sockets, tag-7 hellos, and two accepted mutations.

This closes the source/proof-design finding, not runtime evidence. The first
native process session `14348` terminated red before any law on an external
`ProgramContributionEntry: serde::Serialize` compile failure. A subsequent
independent process-backed secure-local run must terminate green before this
native leg can be accepted.

### Live Residual — Process Probe Does Not Count a Rejected Pre-Session Command

The loopback focused law above directly observes no pre-Session command, but
the new process probe does not yet make that property terminal. Its server
closes a `Commands` frame on connection one (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:628-635`),
while the final oracle counts only accepted mutations (`:648`). A regression
that emits the queued command before the deliberately wrong first Session can
therefore be rejected/closed, requeued by the client, and still later produce
the expected two accepted commands on connections two and three with the same
three grants/sockets/hellos. Add a total or connection-one command counter and
require zero before treating the process trace as proof of the no-pre-Session
law. This does not negate the focused transport evidence, but it is required
for the advertised process-backed counterpart.

### Live Amendment — Process Pre-Session Counter Is Source-Closed; Runtime Pending

The preceding process-oracle residual is superseded in current source. A
connection-one `Commands` frame increments `preSessionCommands`, is closed, and
the final child assertion requires `preSessionCommands === 0` alongside three
grants/sockets/tag-7 hellos and two accepted mutations
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:579-657`). Thus a rejected early send can
no longer be hidden by later reconnect success. The process trace now matches
the focused transport law's no-pre-Session requirement. Its independent runtime
execution remains pending a green secure-local terminal.

### Live Infrastructure Note — Active Native Target Was Externally Removed

While the subsequent focused native rerun was compiling, concurrent cleanup
externally removed the ticket-local `🗑️generated/native-target` (and the
already-finished P4-B target). No report was removed. Any terminal failure from
that in-flight native invocation must be classified as target invalidation,
not as a SocketGrant law failure; no native pass is inferred. A later
independent native gate requires a fresh, explicitly retained target or normal
uncached execution after the shared build state stabilizes.

### Live S3 Re-Read — Admin Connections Is REST-Only, But Not Yet A Bounded Admin Boundary

The current connections panel no longer opens a directory WebSocket. Its only
connection-data operation is `AdminClient.connections(signal)`, a `GET
/admin/api/connections`; kicking is the separate REST `POST
/admin/api/connections/{id}/close`
(`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx:46-106`). A source
census of the admin application finds neither `WebSocket` nor `EventSource`.
The active Connections tab owns one recursive poll: it creates one
`AbortController`, aborts it after 2 seconds, waits for completion before
scheduling the next 2-second poll, retains the prior `Map` while marking
freshness false on error, and on effect cleanup marks itself cancelled, clears
the timer and aborts the active request
(`…/🔴️ConnectionsPage/🟦️.tsx:35-71`). Thus the source has single-flight,
stale-on-failure, unmount-abort and no-next-poll-after-cancellation ordering;
those are source conclusions, not independently executed lifecycle evidence.

The authority is direct browser bearer, not a BFF: the provider reads/writes
the pasted `session.v1` capability in `sessionStorage`, constructs an
`AdminClient` with it, and each request sends `Authorization: Bearer …`
(`…/🔑️AdminSession/🟦️.tsx:46-62,112-166`). The production entry supplies
`window.location.origin`; the development Vite proxy is transparent and merely
forwards `/admin/api` (`📦️index.tsx:17-21`; `⚙️vite.config.ts:13-18,34-39`).
Hub authority is nevertheless server-side: `is_admin` parses/authenticates the
session then constant-time compares the verified identity digest to configured
`OS_HUB_ADMIN_SUBJECTS`, before the connections list or kick route runs
(`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1173-1188,1222-1231,3484-3493,3562-3572`).
There is no per-profile BFF relay, httpOnly session carrier, or brokered
admin-snapshot authority. This is the acceptance-matrix's existing direct
bearer/BFF blocker, not a SocketGrant audience and not a substitute for one.

The advertised snapshot is also unbounded. The handler calls
`list_active_sync_sessions(None)`, allocates to every returned row, and awaits
one user lookup per row (`bin.rs:2732-2751,3484-3493`). SQLite, PostgreSQL and
Neo4j each issue an unpaged all-active-session query and materialize the entire
result (`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1073-1094`,
`🐘️postgres/🦀️.rs:1159-1184`, `🌐️neo4j/🦀️.rs:1046-1063`). The client then
builds another `Map` and nested grouping over the full response
(`…/🔴️ConnectionsPage/🟦️.tsx:20-32,42-75`). A 2-second abort limits a caller's
wait, not server work, JSON body size, response cardinality, browser heap, or
the N+1 lookup fanout. A typed bounded page/snapshot identity and a server-side
deadline/cancellation boundary remain required before calling this safe at
scale.

The current component tests prove a successful rendered REST snapshot and that
only the active tab makes the fast mocked request
(`…/📦️packages/🟦️typescript/🧪️admin.test.tsx:132-150,178-229`). They do not
hold a request pending to observe one in-flight call, inspect the connections
request's `Authorization` header, reject a completed snapshot and require its
rows to remain stale, unmount/retab during a pending request and observe
`AbortSignal.aborted`, or advance timers to prove no subsequent poll. Those
are required runtime/component laws, not claims supported by the current
successful-snapshot test.

EN/DE structural key parity is compile-checked and tested, including the
connections freshness keys (`…/📚️I18n/🟦️.tsx:91-105,199-213,284-303`). However
the connection renderer interpolates `row.role` directly rather than a
localized role key (`…/🔴️ConnectionsPage/🟦️.tsx:103-110`), so the backend enum
remains raw in both locales; the DE UI test only flips the global selector and
does not mount a Connections snapshot or assert `Aktueller Stand`/`Veralteter
Stand` (`…/🧪️admin.test.tsx:154-176`). The labels exist, but the full EN/DE
operator lifecycle remains unproved.

### Live S3 Re-Read — MCP Stdio Migration Is Source-Positive, Not Yet a Complete Security Proof

The new authenticated stdio path removes the hub credential from the MCP
workspace selector itself. `HubArgs` and `StdioOptions` carry only hub origin
and space id; the stdio parser rejects `--token`. At process entry, an inherited
fd3 credential is claimed for client class `mcp` before `parse_args` and before
the server/workspace/plugin path can run
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs:26-62,115-131`). The
shared reader marks fd3 close-on-exec before consuming it on POSIX and makes a
non-inheritable duplicate before closing fd3 on Windows
(`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:403-488`).
`WorkspaceOrigin::Hub` and its persistence binding now retain only
`base_url`, `space_id`, and `surface`, not a bearer
(`🌉️mcp/🏠️workspace/🦀️.rs:432-450`), while `open_hub` injects the protected
credential and typed `HubSocketGrantSource` into `ArtifactHost` before exposing
the workspace (`:1217-1227`).

The remote binding uses `DirectoryClient::authenticated`, issues a fresh
directory receipt for each `DirectoryStreamTurn::Dial`, connects only to
`/directory/socket/v1`, offers the two receipt protocols, verifies the selected
protocol natively, and sends tag-7 before reading the stream
(`📇️directory/🔌️client/🦀️.rs:557-582,980-1056`; `🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:478-548`).
The same injected source issues document-specific receipts for `ArtifactHost`;
the actor installs an unconfirmed receipt epoch, sends tag 7, and waits for its
matching Session before relay delivery (`🏪️store/🔄️sync/🦀️.rs:1803-1906,
2208-2288`). The driver has a cancellation token and joins its thread from the
workspace drop path (`🌉️mcp/🏠️workspace/🦀️.rs:1178-1187`; `…/remote/🦀️.rs:492-562`).
This is meaningful source-level progress and supersedes the prior MCP hub
`--token`/workspace-state carrier finding for the protected **stdio hub
upstream** only. It is not a runtime result.

Three independent acceptance gaps remain live:

1. The direct-child supervisor copies the full ambient environment into the MCP
   child: `env: { ...process.env, S_LOCAL_CREDENTIAL_FD: "3" }`
   (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:514-540`). The local-hub child is
   separately sanitized (`delete env.S_USER` and `S_*TOKEN` keys at :348-357),
   but that sanitization is not applied to the MCP child. Therefore removal from
   the launch seed is not an enforced no-raw-session-in-environment boundary:
   an ambient `S_USER` or matching token is propagated to the MCP process. The
   current oracle neither poisons that ambient input nor proves its absence in
   the child.
2. The claimed-fd reader gives a credible universal source mechanism, but there
   is no MCP-specific actual-child/descendant seal law. The MCP source-order
   checker verifies claim-before-argv but does not assert `FD_CLOEXEC`, Windows
   closure, or launch a descendant (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:562-580`).
   The only actual `--assert-no-local-credential-fd` descendant probe is WGPU
   (`:633-652`). An MCP process or plugin/child invocation must prove it cannot
   read fd3 after the MCP entrypoint has claimed and closed it.
3. `proveMcpWorkspaceProcess` does start the real hub-bound direct binary and
   checks a bounded stderr diagnostic, zero stdout bytes, exit status, and no
   capability in retained output (`:605-631`). It sends no JSON-RPC request,
   closes stdin immediately after startup, and observes no real protocol
   response. It is consequently an EOF/startup byte-clean check, not the claimed
   actual JSON-RPC framing/redaction oracle. It also exercises cancellation only
   while the binding is idle; it does not close stdin during a pending dial or
   refresh and prove the driver cancellation/join is bounded.

The remaining independent MCP HTTP/bridge surface is still a global S3
release blocker rather than an exception to this review. Its `http` parser
accepts a raw `--token` into `HttpOptions.token`
(`🌉️mcp/📦️bin.rs:64-97`; `🌉️mcp/🦀️.rs:701-742`), while `run_http` mints and
writes a bridge secret and logs the complete
`ws://…/bridge?token=…` URL to stderr (`🌉️mcp/🦀️.rs:737-742`). That bearer and
bridge secret are distinct from the protected fd3 session, but both disprove an
S3-wide claim of no raw CLI/file/log/URL secret and leave bridge authority
uncut over.

No MCP build, gate, real stdio JSON-RPC exchange, descriptor-inheritance probe,
or process-cancellation probe was run by this audit. The currently reported
generation failure before any such runtime evidence is external root
pytest/eslint taxonomy validation, so it is neither a pass nor a refutation of
these laws.

### Live S3 Cutover Census — Tag-0 And Legacy Routes Are Still Mounted

**Current source verdict: REJECT.** This is a deletion/cutover census, not a
runtime result. The authenticated v1 paths are present, but the old bearer-in-
frame and bearer-in-URL routes remain executable in the same hub. There must
be no compatibility period: all following deletions and replacements belong to
one atomic source cutover, with v1 as the only accepted public contract.

| Surface | Current executable dependency | Atomic cutover action |
| --- | --- | --- |
| Document hub route | `document_ws` upgrades `/spaces/{space_id}/documents/{id}/ws` with no admission (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1880-1883,3782-3784`). `handle_ws` accepts `ClientFrame::Hello` and calls `resolve_auth` with its `token` (`:2173-2180,2191-2195`). | Delete `document_ws`, its old query type if then unused, the `None + Hello` authentication arm, and the old route. Retain only the issuer and `/socket/v1` route with `SocketGrantAdmissionV1` (`:1889-1907,3782-3783`). |
| Directory hub route | `DirectoryWsQuery` carries `token`; `/directory/ws` forwards it to `handle_directory_ws`, which resolves that bearer before streaming (`bin.rs:3033-3045,3120-3123,3760-3761`). | Delete the query type, handler and route. Retain only `/directory/socket/v1`, its receipt admission and tag-7 first-frame boundary (`:3015-3030,3191-3194,3760`). |
| Rust wire authority | `ClientFrame` still declares `Hello` with `actor` and `token`; the serializer writes client tag `0`, the decoder accepts it, and two round-trip tests preserve it (`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:48-50,855-867,920-941,1230-1255`). | Delete the variant, its tag-0 encoder/decoder branch and all typed fixtures. Keep `SocketHelloV1` as the only client bootstrap type. A negative raw-byte law may construct tag `0` bytes directly and require rejection; it must not keep a typed legacy frame merely to test rejection. |
| TypeScript wire authority and fixtures | The TS union/encoder/decoder still recognizes `Hello`, emits tag `0`, and checks the old `client-hello.bin` fixture (`🧰️framework/🔨️modules/📡️replication/🟦️.ts:1088-1104,1138-1157,1561-1565`). | Delete the `Hello` shape, codec arms and fixture assertion; replace the fixture with tag-7 `SocketHelloV1` and an independent byte-level tag-0 rejection assertion matching Rust. |
| Hub Rust and cross-language tests | The hub test helper builds `ClientFrame::Hello` (`bin.rs:4744-4746`); legacy document cases still dial `/ws` and send it (`:5456-5471,5718-5722`); directory replay/isolation/presence cases still own legacy `/directory/ws` coverage (`:5738-6040`). The Bun e2e defines `helloFrame` with a token and dials `…/documents/index/ws` (`🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:793-794,918-940`). | Rebuild each behaviour law through issuer → receipt subprotocols → `/socket/v1` → tag-7. Add terminal negatives for both removed URLs (no upgrade) and raw tag `0` (no session/auth); migrate replay, membership, presence, conflict and restart assertions rather than deleting their behaviour coverage. |
| React/browser source | The live document worker already requests a receipt, opens `/socket/v1`, validates the selected protocol and sends tag 7 (`🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:720-775`). The directory client likewise issues a receipt, opens `/directory/socket/v1` and sends tag 7 (`🧰️framework/🛍️products/💻️os/🟦️.ts:4063-4160`). Stale prose/test names still say `ClientFrame::Hello` (`🟦️.ts:564-568`; `🟦️backbone-worker.ts:1724-1727,2445-2463`). | Preserve the real v1 flows; rewrite stale test/docs to the receipt/tag-7 contract and ensure their fake `WebSocket` captures URL **and** offered protocols. Do not reintroduce a browser bearer to make an old test convenient. |
| Native store and MCP caller | Native document sync sends `SocketHelloV1` and has current no-pre-Session/reconnect laws (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1893-1906,4629-4658`). MCP's remote directory binding dials v1 through the authenticated client and sends tag 7 (`🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:478-548`). These sources have only stale old-route/Hello prose in the cited store and directory-client docs (`🏪️store/🔄️sync/🦀️.rs:87,109,1384`; `📇️directory/🔌️client/🦀️.rs:3-5,118,625-650`). | Preserve the typed credential/grant injection and v1 callers; revise all stale contract docs in the same cutover so no future caller treats an obsolete bearer route as an option. This does not close the separate MCP fd3/environment/JSON-RPC proof gaps recorded above. |
| WGPU shipped generated worker | The checked-in artifact copied by the WGPU HTML still contains a `DirectoryClient.token` and constructs `/directory/ws?token=…` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🟨️frame-worker.js:241414-241478`); the same stale intermediate generated file has the matching legacy code (`…/🧵️frame-worker/🤖️generated/🟨️.js:12757-12821`). The deploy HTML copies the former (`🌐️.html:8-11`). The present worker source has no such dial (`…/🧵️frame-worker/🟦️.ts:1-260`), and the owner script defines both generation and byte-for-byte freshness checking (`📜️script.ts:151-165,327-345,392-394`). | Treat this as a deployed stale artifact, not harmless test debris. Regenerate only from the current source with the registered owner target, delete/refresh any obsolete intermediate, and require the owner freshness check plus a post-generation scan proving neither tokenized directory URL nor client tag-0 bootstrap ships. |
| Admin connections | This is REST-only: `connections()` is `GET /admin/api/connections` and kick is a REST POST (`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx:85-106`); the panel owns recursive polling and abort cleanup, not a websocket (`…/🔴️ConnectionsPage/🟦️.tsx:45-71`). | Do **not** move admin observations to a SocketGrant stream as part of this deletion. Preserve REST-only admission while independently resolving its direct-browser-bearer and unbounded-snapshot blockers documented above. |
| MCP HTTP/bridge | This is not a document/directory legacy compatibility route. It independently retains raw `--token`, writes a bridge token file, logs `ws://…/bridge?token=…`, and authorizes the bridge query secret (`🌉️mcp/📦️bin.rs:64-97`; `🌉️mcp/🦀️.rs:737-742`; `🌉️mcp/🧵️bridge/🦀️.rs:2524-2573`). The renderer discovers `VITE_SEMIO_BRIDGE_TOKEN`, appends it to a URL and dials it (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/AgentBridge/🟦️.tsx:103-140,431-433`). | Cut this separate raw-secret protocol over at the same release boundary or explicitly block S3 on it; it must never be relabelled a SocketGrant. Replace its UI/tests and remove secret URLs/files/logs with a distinct authenticated local bridge authority before making an S3-wide no-secret-carrier claim. |

#### Safe Atomic Execution Order

1. Write/adjust the v1 behaviour laws first: issuer/receipt/subprotocol/tag-7
   for document and directory replay, presence, membership, conflict and
   restart; negative old-route and raw-tag-0 probes; then replace the Bun
   oracle's legacy `helloFrame`. This establishes coverage before removal.
2. Delete `ClientFrame::Hello` and tag `0` from both wire implementations and
   fixtures in the same change. Compile breakage now exposes every remaining
   native, hub and browser call site; do not retain a compatibility decoder.
3. Delete both hub legacy handlers/routes and their bearer query/frame
   authentication arms. Keep v1 issuer and admission routes in the same router
   transaction, and make the new negative tests terminal rather than merely
   checking a later close.
4. Convert/remove remaining React, hub test, native/MCP documentation and
   fake-socket assumptions exposed by step 2. Preserve the already-v1 native
   store and MCP calls; do not migrate them back through an adapter.
5. Regenerate the WGPU copied worker using its registered `📜️script.ts`
   target, reconcile/delete the stale intermediate generated asset, then run
   the owner freshness check. The artefact scan is required after generation,
   because the HTML publishes `🟨️frame-worker.js` directly.
6. In the same release decision, resolve or retain a **blocking** status for
   the unrelated MCP HTTP/bridge raw-secret surface and retain Admin as
   REST-only. Neither may silently become a fallback path around the deleted
   document/directory contract.
7. Only after source ownership is quiet, run the registered uncached
   language-specific gates and the process-backed secure-local oracle. This
   audit intentionally ran none while shared unique Rust targets were active;
   a compile-only outcome still would not prove the receipt, fd3, redaction or
   no-pre-Session runtime laws.

### Live S3 Regression — Current Producer/Consumer Contract Is Split

**Current source verdict: RED before any runtime gate.** A concurrent literal
rename has changed producers and the hub-side schema to emoji-bearing protocol
identifiers while several protected consumers retain the original identifiers.
This is an executable contract split, not a stale comment or an audit-only
checker defect:

* The local bootstrap schema and hub supervisor now use client class
  `"🔌️mcp"` (`🌎️hub/🔐️local-bootstrap/🧬️schema/📨️credential-envelope-v1/🔣️.json:16`; `🌎️hub/📦️packages/🦀️rust/📜️script.ts:18,514-549`), but MCP claims fd3 as
  `"mcp"` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️bin.rs:115-119`).
  The shared claim binds and requires exact class equality
  (`📇️directory/🪪️identity/🦀️.rs:83-103`), and the decoder compares the
  envelope class to that requested value (`📇️directory/🔌️client/🦀️.rs:361-372`).
  The newly added source guard independently looks for `"🔌️mcp"`
  (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:569-570`). Therefore the real MCP
  child rejects the delivered credential and the source gate fails before a
  workspace can open.
* The supervisor writes fd3 frames with schema
  `semio.local.consumer-credential/🐼️v1` (`📜️script.ts:525-531`), whereas
  `LocalHubCredential::read_inherited` accepts only
  `semio.local.consumer-credential/v1` (`📇️directory/🔌️client/🦀️.rs:361-436`).
  This rejects the actual native delivery as well as MCP; a mocked delivery
  consumer accepting the new string cannot establish native reader success.
* Hub document/directory grant issuance emits
  `semio.hub.socket-grant/🐼️v1` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1335-1336`),
  but `DirectoryClient` validates only
  `semio.hub.socket-grant/v1` (`📇️directory/🔌️client/🦀️.rs:588-591`). The
  actual v1 receipt is consequently rejected before WebSocket dial.
* The new MCP source-order guard contains its own impossible expectations: it
  reads a non-existent duplicate `📦️📦️📦️packages` runner path and requires
  `/directory/socket/🐼️v1` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:567,577`),
  while the real runner is `🌉️mcp/📦️packages/🦀️rust/📜️script.ts:43-44` and the
  directory client creates `/directory/socket/v1`
  (`📇️directory/🔌️client/🦀️.rs:630-632`). It must fail independently even
  after a child-class repair.

Resolve these as one schema-first contract decision: choose the canonical wire
literals, update every hub producer, JSON schema, direct-child frame writer,
Rust/TS/native/MCP reader, wire/source oracle and fixtures together, then
prove a real direct child consumes the exact bytes. Do not weaken the reader
into accepting both spellings; that would reintroduce a legacy protocol
compatibility path the cutover explicitly forbids. All preceding native/MCP
source-positive statements are superseded by this current-tree RED finding;
no earlier compile or process result establishes the present contract.

### Live S3 Re-Read — fd3 Literal Split Is Closed; v1 Route Split Is Still Terminal

The preceding consumer-frame/class and receipt-schema mismatches are
**superseded in the current tree**: the fd3 reader now accepts
`semio.local.consumer-credential/🐼️v1`, MCP claims `"🔌️mcp"`, and the native
receipt reader accepts `semio.hub.socket-grant/🐼️v1`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:361-372,588-591`;
`🌉️mcp/📦️bin.rs:115-119`). The repaired MCP source guard also uses the
canonical client class and non-duplicated runner location. This is
source-only; no process evidence is inferred.

**A different current-tree RED is live:** the hub mounts its only v1 upgrades
at `/directory/socket/🐼️v1` and
`/spaces/{space_id}/documents/{id}/socket/🐼️v1`
(`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3759-3760,3782-3784`), but the native
directory client still derives `/directory/socket/v1`
(`📇️directory/🔌️client/🦀️.rs:630-632,1410-1412`) and the React directory
client still dials the same old path (`🧰️framework/🛍️products/💻️os/🟦️.ts:4163-4167,4343-4377`).
The hub's own two v1 directory tests also construct that absent old URL
(`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5168-5191`). Document sync is internally
split: its no-surface branch emits `/socket/🐼️v1`, while its surface branch
emits `/socket/v1` (`🏪️store/🔄️sync/🦀️.rs:754-765,4862-4868`). These paths
cannot all dial the present server; an aliased route would violate the
no-legacy cutover requirement.

Pick exactly one canonical path and update router, document/directory native
and browser callers, MCP binding, server process oracle and all fixtures in
one source transaction. Add an actual receipt/subprotocol/tag-7 upgrade law
for every caller shape (including document `?surface=`), require the other
path to fail before upgrade, and regenerate the shipped worker afterwards.
Until then the SocketGrant native/MCP/browser lane is source RED regardless of
its focused compile status.

### Live S3 Re-Read — Route-Wide Claim Superseded; Current Native/MCP Splits Remain RED

The immediately preceding route-wide conclusion is **superseded** by the next
current-tree read: hub routes are again `/directory/socket/v1` and document
`/socket/v1` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3759-3760,3782-3784`), matching
the native directory client and React directory/browser document callers
(`📇️directory/🔌️client/🦀️.rs:630-632`; `💻️os/🟦️.ts:4163-4167`;
`💻️os/🟦️🧭️backbone-worker.ts:734-735`). It does **not** establish a stable
runtime contract, because two exact producer/consumer failures remain now:

1. Native document sync uses `/socket/🐼️v1` in its no-surface branch but
   `/socket/v1?surface=…` in its surface branch
   (`🏪️store/🔄️sync/🦀️.rs:754-765,4862-4868`). The server mounts only the
   latter base path. A normal native document without `surface` cannot dial;
   its own unit law presently preserves this wrong result.
2. The hub supervisor currently issues/delivers plain client class `"mcp"`
   and `semio.local.consumer-credential/v1`
   (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:18,540-575,595`), while the MCP
   entry claims `"🔌️mcp"` (`🌉️mcp/📦️bin.rs:115-119`) and the protected reader
   accepts `semio.local.consumer-credential/🐼️v1`
   (`📇️directory/🔌️client/🦀️.rs:361-372`). Exact class/schema equality makes
   the actual delivered MCP frame fail; the source-order checker itself still
   expects plain `"mcp"`, so it fails before process evidence too.

The source has changed between successive reads, including direction of these
literals. Treat all prior process/compile results as history only. Do not start
or accept an S3 runtime gate until one declared stable snapshot has one
canonical literal per field/path across bootstrap schema, hub producer,
direct-child frame writer, native/MCP/browser consumer, server route, fixture,
and source/process oracle. That repair must delete the alternate spelling,
not accept both.

### Live S3 Re-Read — Browser Receipt Parser And Direct-Child Environment

The latest hub and native reader agree on
`semio.hub.socket-grant/v1` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1335-1336`; `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:588-591`),
but the React receipt type/parser and backbone worker fixtures require
`semio.hub.socket-grant/🐼️v1`
(`💻️os/🟦️.ts:3975-4047,4310-4317`; `💻️os/🟦️🧭️backbone-worker.ts:1573-1582,2189-2198`).
Consequently the browser rejects a real current hub grant before its v1
WebSocket attempt. This is another live exact-literal producer/consumer RED,
in addition to the native no-surface path and MCP class/schema split above.

The direct-child environment residual from the earlier MCP re-read is
**source-closed provisionally**. `directChildEnvironment` constructs a fresh
environment, removes `S_USER`, `VITE_S_USER`, `S_HUB_URL` and every name
containing `TOKEN`, `SESSION`, `CREDENTIAL`, `BEARER`, `CAPABILITY`,
`AUTHORIZATION`, or `COOKIE`, then adds only the fd3 selector
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:517-543`). Both native and MCP delivery
now call it. This prevents the earlier full-`process.env` propagation in
source. It is not a runtime closure: require the actual direct MCP child and
an actual malicious descendant to receive poisoned ambient keys, assert no
raw key/value appears in environment/stdout/stderr, and still prove fd3 is
sealed after the claim. The source rule's broad substring filter should also
be exercised with a benign required environment variable so the security
filter does not silently break child launch prerequisites.

### Live S3 Stable-Literal Re-Read — Core Socket/fd3 Contract Is Source-Uniform

After a further stable interval, the immediately preceding literal/path RED
entries are **superseded**. The current source consistently uses plain ASCII
`v1` and client class `mcp` at every checked core boundary:

* Hub issues `semio.hub.socket-grant/v1` and mounts document/directory
  `/socket/v1` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1335-1336,3759-3760,3782-3784`).
* The direct-child writer and MCP source-order guard use
  `semio.local.consumer-credential/v1`, `mcp`, and `/directory/socket/v1`
  (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:18,540-575,595-604`); MCP claims
  exactly `mcp` before parsing (`🌉️mcp/📦️bin.rs:136-143`).
* The shared fd3 reader and native directory binding accept those same schemas
  and URL (`📇️directory/🔌️client/🦀️.rs:361-372,588-591,630-632`); both native
  document URL branches use `/socket/v1` (`🏪️store/🔄️sync/🦀️.rs:754-765,
  4862-4868`).
* React's receipt parser, directory WebSocket and backbone-worker receipt
  fixtures/document WebSocket match (`💻️os/🟦️.ts:3975-4047,4163-4167,
  4310-4377`; `💻️os/🟦️🧭️backbone-worker.ts:734-735,1573-1582,2189-2198`).

`proveMcpWorkspaceProcess` has also materially strengthened source coverage:
it poisons inherited environment keys, starts the real hub-bound MCP child,
sends a bounded JSON-RPC `initialize`, requires one byte-bounded parsed result
with the matching id/protocol version, and rejects capability/poison text in
retained stdout/stderr (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:631-671`). This
supersedes the earlier source finding that it closed stdin without a request.
It remains runtime-pending.

Current remaining S3 blockers after this source closure are narrower and
independent:

1. The tag-0 `ClientFrame::Hello`, document `/ws`, directory `/directory/ws`,
   and their bearer paths are still mounted, as recorded in the cutover census
   above.
2. The actual WGPU-published generated worker still contains a legacy
   `DirectoryClient.token` and tokenized `/directory/ws` dial
   (`…/🟨️frame-worker.js:241414-241478,241610,241637`); generation/freshness
   must be run and proved after source ownership quiets.
3. MCP has no actual MCP child/descendant fd3-seal assertion, and its new
   process probe does not hold a pending directory dial/refresh while closing
   stdin to prove bounded driver cancellation/join. The poisoned parent
   environment is checked for source filtering and output redaction, but the
   real MCP child does not itself report/assert each absent poisoned key.
4. The independent MCP HTTP/bridge raw `--token`, bridge-token file and
   secret-bearing URL logging remain global no-raw-carrier blockers.

No runtime gate was run in this audit. A future terminal must be from this
stable literal snapshot or a fresh re-read is required before it can support
an acceptance claim.

### Live S3 Re-Read — MCP Descendant Source Law Closed; Published Worker Is Still Red

The third remaining-blocker item in the preceding stable-literal section
(missing MCP descendant assertion) is **superseded in source**, after a
further stable re-read. The supervisor's fresh environment copies only
non-protected variables, adds the benign sentinel and fd3 selector
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:517-544`), and its real MCP process
oracle injects both poisoned parent keys and `SEMIO_DIRECT_CHILD_PROBE=1`
(`:632-645`). Before parsing argv, the MCP entry rejects each protected-name
class, requires the benign sentinel when fd3 is present, and claims the `mcp`
credential (`🌉️mcp/📦️bin.rs:115-167`). The probe branch then spawns the same
binary as a real descendant with `--assert-no-local-credential-state`, removes
the fd3 selector/probe trigger, and rejects a nonzero result (`:168-183`). That
descendant checks descriptor 3 is closed using `fcntl(F_GETFD)` on Unix or
`_get_osfhandle(3)` on Windows (`:132-146`). A successful process terminal
would therefore establish the requested direct-child poison filtering,
benign-environment preservation, one-time fd3 claim, and descendant seal
together; none has been inferred here before such a terminal.

The generated WGPU lane is now a separately concrete **RED** source blocker,
not merely a missing test run. The published HTML copies
`🟦️typescript/🟨️frame-worker.js` (`…/📦️packages/🦀️rust/🌐️.html:8-11`), whose
checked-in bytes still define the old bearer-bearing `DirectoryClient`
(`…/🟨️frame-worker.js:241414-241478`) and retain tokenized legacy directory
websocket expectations (`:241610,241637`). Its owner now resolves the missing
`../../🧵️frame-worker/🟦️.ts` (`…/📦️packages/🦀️rust/📜️script.ts:151-165`),
whereas the live source is
`…/wgpu/🧵️🚀️frame-worker/🟦️.ts`. Thus the registered renderer cannot render
the current source for its byte-equality freshness check, and the shipped
artifact continues to publish a raw bearer carrier. The actual current shared
directory client is v1/receipt based (`💻️os/🟦️.ts:4025-4029,4130-4202`), so
this is stale generated deployment output rather than authority for a fallback
protocol.

I ran the isolated registered command
`bun nx run framework-renderer-wgpu:check-frame-worker --skip-nx-cache` on
2026-09-03. It exited **1 before Nx reached the target**: the root Bun wrapper
rejected the external taxonomy with invalid semantic-member and path-projection
entries. That terminal does not validate or refute the worker bytes; the
missing-owner-path and published-artifact findings above are direct source
evidence. Once taxonomy preflight is repaired, rerun the exact uncached target
and require it to render the moved source, match the shipped bytes, and reject
the legacy bearer/tag-0 scan.

The independent MCP `http`/bridge raw `--token`, bridge-token file and
secret-bearing bridge URL/log remain unchanged blockers; the new MCP fd3 proof
does not apply to or authorize that separate surface.

### Live S3 Re-Read — Worker Owner Path Repair Is Source-Closed; Artifact Remains Stale

The preceding statement that the worker owner still names a missing source
path is **superseded by the next stable reread**. The owner now resolves the
live `../../🧵️🚀️frame-worker/🟦️.ts` source
(`…/wgpu/📦️packages/🦀️rust/📜️script.ts:151-165`), so the taxonomy move no
longer prevents generation at that source boundary. The checked-in published
artifact is nevertheless unchanged: it still carries the old bearer
`DirectoryClient`, raw tag-0 `Hello` decode and legacy
`/directory/ws?token=` expectations (`…/🟨️frame-worker.js:238237-238242,
241414-241637`), while HTML still copies those bytes (`🌐️.html:8-11`). The
owner repair is source-positive only; it does not make the shipped worker
fresh. The previously recorded uncached Nx invocation remains an external
taxonomy-preflight failure before the owner check, so rerun that exact check
after the preflight repair and do not mark this artifact fresh from the path
fix alone.

### Live S3 Re-Read — MCP HTTP/Bridge Raw Carrier Is Source-Closed, Runtime-Pending

The raw-MCP-HTTP/bridge blocker in the preceding sections is **superseded in
the current tree**. `parse_http_args` now rejects both `--token` and
`--bridge-token-file` rather than carrying either into `HttpOptions`
(`🌉️mcp/📦️bin.rs:64-110,199-208`). `run_http` obtains only the already-claimed
protected `mcp` credential, logs a credential-free `/bridge` URL and passes
the credential directly into `HttpTransportOptions` (`🌉️mcp/🦀️.rs:701-744`).
The production bridge admission accepts exactly two ordered subprotocols,
`semio.mcp.bridge.v1` plus a capability checked through the protected
credential, before upgrade (`🌉️mcp/🚚️transport/🦀️.rs:127-165,1492-1498`);
it no longer obtains authority from a query parameter, argv, environment or
disk file. The React component has no Vite discovery path (it always returns
`null`) and only uses a supervisor-supplied in-memory `admissionProof` as the
second subprotocol (`📺️renderer/…/AgentBridge/🟦️.tsx:103-115,276-277,403-408`).

Literal `?token`/`bridge-token` remnants found under the MCP tree are now
test fixtures, negative fixtures, or stale prose (for example the transport
helper comment at `🚚️transport/🦀️.rs:1740`), not a production route/query
parser. They still warrant documentation cleanup, but do not establish a live
raw carrier. This is a source-only classification: the external taxonomy
preflight prevented the registered gate from running, so the real HTTP
handshake, supervisor-only proof transfer and rejected query/argv/file paths
remain runtime-pending. The published WGPU worker and mounted tag-0/legacy hub
routes remain independent RED blockers.

### Live S3 Re-Read — Late Directory Dial Has A Production-Runner Ownership Hole

The new generic `DirectoryStream::complete_dial` correctly owns a *delivered*
late success: it clears `dialing`, sees `closed`, invokes
`DirectoryWsConnection::close` on `Ok(connection)`, and remains terminal
(`📇️directory/🔌️client/🦀️.rs:679-695`). Its focused law cancels after a
`Dial`, supplies a late observer-backed `FakeWs`, observes exactly one close,
and observes a further terminal turn (`:1623-1639`). This is a real
source-positive for the stream state machine, but it does not prove the actual
native producer delivers every late result to that state machine.

In the WGPU Shell's production `ShellDirectoryRunner`, the I/O closure obtains
the `Result` first and hands it to `complete_dial` only when
`weak.upgrade()` succeeds (`📺️renderer/…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1296-1308`).
`ShellState::drop` cancels and removes the runner (`:1774-1782`), so a shell
drop/cancel while that I/O dial is in flight can make the weak upgrade fail.
That branch just drops an `Ok(TungsteniteConnection)`; it does **not** call the
connection's explicit close operation, which is the only operation here that
sends a WebSocket close frame (`📇️directory/🔌️client/🦀️.rs:943-945`). The
new `DirectoryStream` unit law cannot reach this weak-failure ownership path.
This is a live **RED**: retain a close-owning runner through the submitted
dial, or explicitly close an `Ok(mut connection)` when upgrade fails, then
exercise actual Shell cancellation/drop racing a successful dial and assert
one close plus no reschedule/reconnect.

There is a related grant-refresh race. `open_stream_ws` checks cancellation
only before issuing a new directory socket grant (`📇️directory/🔌️client/🦀️.rs:568-576`);
it has no check after grant issuance or after `open_ws` returns before sending
the `SocketHello`. A cancellation in either interval can spend a freshly
minted proof and send its authenticated greeting before the later
`complete_dial` cleanup. Native `open_ws` does perform pre-call and
per-address checks (`:1018-1053`), but that does not close the post-upgrade
interval. Add post-grant/post-open cancellation checks that explicitly close a
new connection before returning cancellation, and a production-path law that
pauses a reconnect after grant/upgrade, cancels it, proves no greeting or
reconnect timer and exactly one close. No runtime terminal has been run for
this lane.

### Live S3 Re-Read — Refresh-boundary Check Is Source-Closed; Runner Ownership RED Persists

The post-grant/post-upgrade portion of the preceding RED is **superseded in
current source**. `open_stream_ws` now rechecks cancellation immediately after
grant issue, after `open_ws` (explicitly closes before it can greet), after a
send failure (also explicitly closes), and after the v1 greeting
(`📇️directory/🔌️client/🦀️.rs:568-589`). The dedicated laws deterministically
cancel inside the fake grant issue and assert no dial/no greeting, then cancel
inside fake `open_ws` and assert one close/no greeting
(`:1660-1688`; the cancellation hooks are `:1278-1379`). This is adequate
source evidence that a refreshed proof cannot progress from either tested
boundary to an unaudited greeting.

It does **not** close the WGPU producer ownership RED. The actual submitted
I/O closure still retains only a `Weak<ShellDirectoryRunner>` and does nothing
with its `Ok(connection)` if the runner disappears before completion
(`📺️renderer/…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1296-1308`). The direct
stream tests do not execute that closure. Retain the runner through the dial or
explicitly close the successful connection in the weak-failure branch, with a
real runner cancel/drop race law. No runtime terminal supports this lane.

### Live S3 Re-Read — Credential Wiping Still Permits A Malformed-UTF-8 Panic

The new ownership is source-positive: `WipeBytes` covers the allocated
fd payload on full and partial `read_exact` failure, `WipeDslValue` recursively
wipes decoded values and object keys on ordinary post-decode returns, and the
accepted capability is moved from its decoded `String` into the credential's
boxed bytes rather than copied (`📇️directory/🔌️client/🦀️.rs:324-387,430-436`).
The capability field is removed from the decode tree before its guard drops;
the retained credential redacts `Debug` and wipes the owned bytes on final
drop (`:390-400`).

There is nevertheless a live **RED** panic path inside this trusted-boundary
parser. `valid_session_capability` slices UTF-8 strings at fixed byte offsets
`value[11..43]` and `value[44..]` (`:464-470`). An envelope can have exactly
108 bytes and the required ASCII prefix, while placing a two-byte UTF-8 scalar
beginning at byte 42; byte 43 is then not a character boundary and this
validation panics instead of rejecting it. The decoded-value destructor runs
only during unwinding; a panic-abort build skips it altogether, and either
behavior violates a one-shot credential parser's bounded `Unauthorized`
failure law. Validate a fixed `value.as_bytes()` layout without `str` slicing,
and add a malformed non-ASCII boundary law that proves no panic, rejection and
the observer's wipe.

The single unexpected trailing raw byte is also kept in an unguarded local
`[u8; 1]` before its error return (`:432-435`). The main declared payload is
wiped, but this does not literally meet the claim that every raw partial/full
read is zeroed. Include that byte in an explicit wipe guard (and make the
observer law exact for valid, invalid and trailing/partial paths). No runtime
terminal is claimed.

### Live S3 Re-Read — WGPU Late-Dial Ownership Is Source-Closed, Runtime-Pending

The WGPU producer ownership RED above is **superseded in current source**.
The submitted production closure still receives the dial result before trying
the weak runner reference, but its no-runner branch now passes that exact
`Result` to `close_unowned_directory_dial`
(`📺️renderer/🧑️‍🎨️engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1319-1335`).
That helper takes an `Ok(mut DirectoryWsConnection)` and invokes its explicit
`close` operation (`:1259-1263`), rather than relying on drop. The focused
`dropped_shell_runner_explicitly_closes_a_late_dial_result` law supplies a
close-observed real trait implementation and proves exactly one close
(`:646-656`). Together these establish the formerly missing ownership branch
without retaining the runner beyond shell teardown. The law calls the helper
directly rather than arranging a scheduler/weak-reference race, so it is
source evidence for the branch and not process-backed WGPU proof. The native
run remains runtime-pending.

### Live S3 Re-Read — MCP Remote Pending Dial Has No Separate Ownership Gap

The MCP remote driver's synchronous initial dial and single background actor
both deliver every returned result into the same finite stream state machine:
initially at `🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:478-486`, and on each
reconnect at `:507-523`. `DirectoryClient::open_stream_ws` fences cancellation
before grant, after grant, after opening the socket (where it explicitly
closes), and after v1 hello (`📇️directory/🔌️client/🦀️.rs:568-591`), while
`DirectoryStream::complete_dial` explicitly closes an `Ok` supplied after the
stream has been closed (`:692-709`). `NativeHubBindingDriver::drop` cancels and
joins the single actor thread (`🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:556-562`),
and that thread eventually calls `stream.close()` (`:494-549`); it cannot
abandon an in-flight successful result. A revocation is observed on that same
actor after message delivery, breaks its loop and closes the stream; a
continuity failure invalidates the binding before the next dial (`:507-549`).
No separate thread can observe authority loss halfway through that blocking
dial. Therefore this caller introduces no additional source-level late-Ok,
post-cancel grant, or post-cancel hello path beyond the shared client law.
It has no dedicated process-backed MCP cancellation terminal, so this remains
runtime-pending rather than accepted runtime behavior.

### Live S3 Re-Read — Malformed-UTF-8 And Trailing-Byte Code Paths Are Closed; Direct Frame Laws Pending

The malformed-UTF-8 panic and unguarded trailing-byte **code-path** findings
above are superseded after the next reread. Capability layout validation now
uses only `value.as_bytes()` and bounded `get` slices, so a non-character-byte
offset cannot panic (`📇️directory/🔌️client/🦀️.rs:482-489`). The fd payload and
the one-byte trailing probe now each live in `WipeBytes`, whose drop fills the
buffer before returning (`:330-358`); `read_inherited` obtains this owned frame
before decode (`:431-455`). This covers ordinary full, partial, error and
nonempty-trailing exits by RAII rather than trusting a happy-path branch.

The direct proving law is not yet present in the current test module. Its sole
wipe-observer test drives `decode_local_hub_credential` for valid and
wrong-class JSON (`:1440-1461`); it does not call
`read_local_hub_credential_frame` with a partial payload, a nonempty trailing
byte, or a malformed multibyte capability. Therefore classify the mechanisms
as source-closed but retain the requested focused raw-frame/malformed-input
proof as pending. No compile or process terminal is inferred from this reread.

### Live S3 Re-Read — Credential-Frame Wipe Laws Are Now Source-Closed

The preceding missing-law qualification is superseded in the current tree.
The valid/wrong-class JSON law now asserts exact key/non-secret/capability wipe
counts rather than merely a positive count
(`📇️directory/🔌️client/🦀️.rs:1440-1464`). The crafted 108-byte capability
puts a two-byte `é` exactly across the prior fixed slice boundary, frames it
through the raw reader, proves denial without a panic and verifies both the
decoded and raw/probe exact wipe counts (`:1466-1496`). Finally, the direct
frame-reader law verifies the allocated payload is wiped after partial data and
verifies payload-plus-probe wiping for both normal EOF and a nonempty trailing
byte (`:1498-1516`). This completes the focused source proof for the former
malformed input and trailing-read paths. As always, it is not a compiled or
process-backed terminal; the broader native gate remains independently
pending.

### Live S3 Re-Read — Server Cutover Is Mounted, But A Real Hub E2E Test Retains Tag-0 And Legacy Paths

The current hub router itself has completed the narrow mount cutover: it
registers only `/directory/socket/v1` and
`/spaces/{space_id}/documents/{id}/socket/v1`
(`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3592,3614`), with the corresponding v1
handlers at `:1877,2938,3023`. No old server route mount or
`ClientFrame::Hello` arm appeared in the current server reread. The shared
TypeScript replication neutral law loads the committed legacy tag-0 fixture and
requires `decodeClientFrame` to reject it as `unknown tag 0`
(`🧰️framework/🔨️modules/📡️replication/🟦️.ts:1526-1528`).

This is not yet an atomic caller/test cutover. The live gated hub E2E test
still defines a token-bearing old `Hello` object
(`🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:793-795`), opens the deleted
`/spaces/{space}/documents/index/ws?surface=...` endpoint (`:918`), and sends
that frame (`:922,927,939`). It is an actual test implementation, not a
negative fixture; it is also structurally incompatible with the current
`ClientFrame` union's credential-free `SocketHelloV1` tag-7 type. Remove it or
rewrite it to obtain a v1 receipt, use the two ordered subprotocols and send
the tag-7 hello, while retaining explicit assertions that legacy path/tag-0
are rejected. Old `/directory/ws` references in the E2E header, directory
schema prose and ShellHost comments are stale documentation rather than route
authority, but should be cleaned as part of the same atomic deletion. The
stale shipped WGPU worker remains an independent deployment RED.

The ordinary store caller itself is not another legacy carrier: it derives
`/socket/v1`, issues a document grant, supplies the receipt as ordered
subprotocols and sends `SocketHelloV1`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:754-765,1864-1900`).
The remaining old endpoint phrases under the live tree are comments/prose in
the directory service/schema/client, DB-sync and store docs, ShellHost, and
the admin Vite comment (for example `🌎️hub/📇️directory/🦀️.rs:1425,1582,1959`
and `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/⚙️vite.config.ts:13-18`).
The Vite proxy entries are generic `/directory` and `/spaces` forwarding, not
an old WebSocket mount. Treat that prose as cleanup work, while the E2E test
is the actual compatibility/dead-code blocker.

### Live S3 Re-Read — Legacy Hub E2E Is Now a Negative-Only Cutover Law

The preceding E2E-caller finding is superseded in the current source. The hub
TypeScript test no longer constructs a typed bearer `Hello`, dials a legacy
document URL as a positive flow, or sends a token-bearing frame. Instead it
requires the byte-level tag-zero input `[0, 0]` to reject with `unknown tag 0`
(`🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:693-696`) and, when `HUB_E2E`
is enabled, boots the actual hub and requires both `/directory/ws` and
`/spaces/legacy/documents/index/ws` to return 404 (`:698-709`). The neutral
TypeScript replication fixture independently retains the tag-zero rejection
law (`🧰️framework/🔨️modules/📡️replication/🟦️.ts:1526-1528`).

This closes the source-level legacy test/caller residual and makes the
document/directory cutover atomic in the reviewed source. It is not a runtime
closure: the real-hub 404 probe is intentionally skipped without `HUB_E2E`,
and this audit has not run it while the native/shared build lanes are active.
The WGPU published worker freshness and the protected native/MCP process proof
remain separate S3 acceptance conditions.

### Live S3 Re-Read — Secure MCP Source Guard Has a Runner-Path RED

The direct-child environment source boundary is positive: it normalizes every
parent key, removes `S_USER`, `VITE_S_USER`, `S_HUB_URL` and names containing
`TOKEN`, `SESSION`, `CREDENTIAL`, `BEARER`, `CAPABILITY`, `AUTHORIZATION` or
`COOKIE`, then admits only the benign sentinel and fd3 selector
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:517-538`). This is still source-only.

However, the registered source-order proof cannot run to its assertions in the
current tree. `proveMcpCredentialSourceOrder` reads
`…/🌉️mcp/📦️📦️packages/🦀️rust/📜️script.ts`, with a duplicated `📦️`
path component (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:589-605`). The only live
MCP runner is at `…/🌉️mcp/📦️packages/🦀️rust/📜️script.ts`. That `readFileSync`
therefore throws `ENOENT` before it can verify the direct-binary supervisor,
and before the secure-local smoke can reach its direct-child JSON-RPC or native
actor checks (`:1247-1258`). Correct that exact path, then rerun the owned
uncached gate; no process proof may be credited from the source guard alone.

### Live S3 Re-Read — MCP Child Probe Does Not Exercise Its Document Grant Path

Even after the runner-path repair, the current direct MCP process oracle has a
strictly narrower scope than a document SocketGrant claim. It creates only a
space, starts `semio-os-mcp stdio --hub --space`, waits for the authenticated
directory binding, and sends JSON-RPC `initialize`
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:624-670`). It creates/announces no
document and invokes no MCP operation that opens the workspace's
`ArtifactHost`. `HeadlessWorkspace::open_hub` does inject both the protected
credential and typed socket-grant source
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1217-1227`),
but this process probe never consumes that injection.

Consequently, the existing MCP child path can establish fd3/environment seal,
directory v1 binding and byte-clean initialize output only. It cannot establish
MCP document receipt issuance, `/socket/v1` protocol/tag-7 admission, matching
Session actor confirmation, post-confirmation delivery, or reconnect/fresh
grant behavior. The separate WGPU fake-hub actor probe is not MCP evidence.
Add a real MCP document operation against an announced document (or a focused
state-machine/process law) that observes each of those events before making a
runtime MCP document-transport claim.

### Successor Re-read — Secure-MCP Runner Path And Document Announcement Are Source-Closed

The preceding runner-path RED is superseded. The current source guard reads
`🌉️mcp/📦️📦️packages/🦀️rust/📜️script.ts`, and that exact doubled-`📦️` directory
is now the live MCP package location (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:589-605`).
The source-order guard can therefore reach its direct-binary, early fd3 claim,
typed credential/grant injection, v1 receipt/tag-7 and launch-registration
assertions. This corrects a stale-tree path finding; it is source proof only,
not a secure-process gate terminal.

The statement that the process oracle creates no document is also superseded.
`createMcpProbeWorkspace` creates a private space and announces the exact
`mcp-socket-grant-probe` descriptor before stdio launch (`:612-653`). The
remaining scope gap is narrower but real: `proveMcpWorkspaceProcess` discards
that returned `documentId` at `:669`, waits only for the directory binding and
sends only JSON-RPC `initialize` (`:677-694`). It invokes no document or
`ArtifactHost` operation. Thus it still cannot evidence MCP document grant
issuance, `/socket/v1` receipt/tag-7/session confirmation, post-confirmed
delivery, or reconnect/fresh-grant behavior. Retain that residual until a
process-backed document operation observes those events.

### Live S3 Re-Read — Shipped WGPU Worker Is Fresh And Carrier-Clean

The prior stale-worker deployment hold is **superseded** for the current
artifact. The committed
`📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🟨️frame-worker.js`
is 965,257 bytes and is regenerated from
`🧵️🚀️frame-worker/🟦️.ts`. A direct fixed-string census found none of the
deleted carrier/protocol markers (`/directory/ws?token=`, legacy Bearer header
assignment, `this.token = token`, `hub_token`, `set_token`,
`ClientFrame::Hello`, `Hello`, `/socket/v1`, or `Bearer `). This is a
carrier-specific census: unrelated UI scheduling uses of the ordinary word
`token` remain and are not credentials.

The artifact guard itself rejects the three legacy emitted carrier fragments
and requires actual production bootstrap markers
(`📺️renderer/…/📦️packages/🦀️rust/📜️script.ts:164-179`). I independently ran
the registered uncached guard:

```text
bun nx run @semio-tech/framework-renderer-wgpu:check-frame-worker --skip-nx-cache
framework-renderer-wgpu: 🟨️frame-worker.js is fresh
NX Successfully ran target check-frame-worker for project @semio-tech/framework-renderer-wgpu
exit 0
```

This closes the browser generated-artifact freshness/census boundary only. It
does not replace the pending native process-backed SocketGrant, direct-child,
or MCP document-delivery evidence.

### Successor Re-read — MCP Process Probe Now Reaches a Real Document Path, But Its Reconnect Assertion Contradicts the Stable-Session Actor Contract

The preceding statement that the probe does not open an `ArtifactHost` document
is superseded. The live probe creates a private space, announces
`mcp-socket-grant-probe` as `os.agent.probe/v1` with the Rust workspace's exact
`PROBE_PACK_SCHEMA_HASH`, then sends `artifact_open` for that exact id
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:612-655,723-728`). The actual MCP
handler accepts that authenticated descriptor only after the schema/hash check,
opens a real `ProbeStore` through `ArtifactHost::open`, and reads its pack/spr
bytes (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗿️artifact/🦀️.rs:337-367`; `🏠️workspace/🦀️.rs:1249-1258,1332-1378`). The source proof then uses
the authenticated admin connection list to isolate the exact space/document,
requires a tag-7-derived `hub.v1.*` actor, posts the admin close, waits for a
different `syncSessionId`, and calls `artifact_snapshot` (`📜️script.ts:677-737`).

The close operation is authentic but asynchronous: the admin route performs
admin authentication and signals only the `Notify` registered by that live
WebSocket (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3401-3411`); the owning loop
selects the notify, then records the close and removes the same map entry
(`:2534-2548`). The probe's subsequent bounded poll for exactly one new,
document-scoped session is therefore the required observation of completion;
the `204` alone is not a synchronous closure acknowledgement.

**RED — the actor-change assertion is impossible under the reviewed live
contract.** Document grants belonging to an authenticated user pass that
session's stable `secret_digest` as actor material (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1339-1389`). `issue_socket_grant` deliberately hashes that stable
material when present (`:1308-1336`), so reconnects of the same session retain
the same `hub.v1.*` actor (`:1277-1283`). Yet the new probe throws if the
replacement actor equals the first actor (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:728-734`). A real run which reaches that point must therefore fail even
though it received a fresh one-use grant and a new sync-session id. Replace
that assertion with the actual invariant (different session id plus fresh
receipt/connection, while the stable session actor remains equal), or make
actors intentionally per-grant and revise the server contract and its laws.
Until resolved and process-executed, this is a source RED; session `87074`
stopped before compiling the relevant target and supplies no runtime evidence.

The prior path-positive sentence saying the doubled `📦️` MCP runner directory
exists is also superseded by the current filesystem reread: only
`…/🌉️mcp/📦️packages/🦀️rust` exists; the doubled form is absent. The live hub
source guard currently reads the single-package runner successfully, but the
registered MCP Nx target still names the absent doubled path, as recorded in
the P4-C audit below.

### Successor Re-read — MCP Document Process Proof Is Source-Consistent; Runtime Is Still Held

The preceding actor-assertion RED is superseded in the current tree. The probe
now asserts the server's intended **stable** actor equality, a different
`syncSessionId`, and different first/reconnect one-use grant selector digests
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:759-774`). The diagnostic does not emit
a raw receipt: it is enabled only for the direct-child probe and SHA-256 hashes
the 32-byte public selector segment before writing its narrowly parsed stderr
line (`🧰️framework/🔨️modules/📇️directory/🔌️client/🦀️.rs:307-314`). The raw
grant's 64-hex secret segment is neither logged nor inspected by the proof.

The delivered receipt actor is not merely inferred from the admin view. The
real store connection holds the receipt actor, sends the tag-7 hello, refuses
to mark an epoch confirmed until the received `ServerFrame::Session` actor
matches it, and fails/clears the epoch on mismatch
(`🏪️store/🔄️sync/🦀️.rs:1885-1902,2214-2223`). Outbound mutations remain
queued until that Session confirmation (`:2273-2284`). Together with the
document-specific admin observation and post-reconnect snapshot/persisted-head
checks in the process source, this closes the **source** path for receipt,
tag-7, matching Session, initial document persistence, admin kick/reconnect,
fresh-grant identity, and byte-clean JSON-RPC.

It remains deliberately unaccepted as runtime evidence. The current native
attempt stopped before the relevant target compiled (reported session `87074`),
and this audit did not start a competing Cargo run. A successful owned process
terminal must still demonstrate those assertions against the real hub/child.

### Live S3 Re-Read — Secure-MCP Process Gate Still Cannot Reach Its Child Build

The P4-C-specific `canonical-pair-check` target has been repaired to use the
real MCP package directory, but this does not repair the S3 secure-local path.
The native/MCP security gate invokes
`bun nx run @semio-tech/framework-os-mcp-rs:build` before it starts the hub or
the MCP child (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:1366-1372`). The MCP
project's `build` target (and its neighboring check/test/dev targets) still
sets `cwd` to the nonexistent doubled-`📦️` path
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📋️project.json:8-50,60-66`). Only the P4-C
canonical target at `:52-58` uses the actual single-package directory.

**RED — after the external Cargo graph is repaired, the S3 process gate will
still terminate at this Nx working-directory error before it can exercise the
otherwise source-consistent MCP child proof.** Correct the remaining MCP
target directories atomically (or use a direct valid MCP build route from the
S3 gate) and retain a fresh uncached process terminal; the P4-C target repair
cannot be substituted for it.

### Successor Re-read — MCP Package Move Makes the Project CWD Valid; Workspace Membership Is RED

The preceding single-package filesystem statement is superseded by a fresh
current-tree reread. Only
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️📦️packages/🦀️rust`
and the corresponding doubled plugin directory now exist; both single-package
directories are absent. Every MCP project target, including `build` and the
secure-process prerequisite, consistently uses the doubled MCP cwd
(`🌉️mcp/📦️📦️packages/🦀️rust/📋️project.json:8-66`). That project-local cwd
is therefore no longer a gate-entry blocker.

The workspace root remains inconsistent: its member/dependency declarations
still name the absent single directories (`Cargo.toml:13,205,207`). Cargo can
therefore stop before the S3 child build and process proof despite the
project-local correction. This is an external workspace-path RED, not runtime
evidence and not a reason to credit the source-consistent MCP proof. A stable
tree repair followed by an owned uncached secure-MCP terminal remains required.

### Live S3 Re-Read — Current Admin Boundary (Backend and Browser Kept Separate)

The backend has superseded static-token/loopback authorization: production
requires a configured verifier and nonempty administrator-subject policy
(`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1198-1214`), while `is_admin` parses a
live `SessionCapability`, authenticates it, and constant-time compares its
verified provider/subject digest (`:1224-1231`). All current admin routes are
bounded REST mounts rather than an admin WebSocket (`:3600-3610`). The
Connections route caps its active-session snapshot at 1,024 (`:3320-3332`),
and user-session revocation serializes its subject binding, durably revokes,
invalidates each grant, then issues bounded live-session kicks (`:3416-3451`).
The focused socket law proves a late same-user grant waits behind the revoke
and then receives 401 (`:5036-5061`). These points close the older static
authority and missing revoke-before-kick findings at source level.

Two backend REDs remain. First, `is_admin` returns only a boolean; the command
route constructs the fixed `DirectoryActor { kind: Admin, id: "admin" }`
instead of retaining the verified principal (`:3374-3390`). Resulting command
events cannot attribute a mutation to its actual administrator. The same route
continues to reject `create-space` for that Admin actor by design (`:3374-3378`),
so the acceptance-matrix create-space failure is live rather than superseded.
Second, the read surface is unbounded: overview and users request
`list_users(i64::MAX, 0)` (`:3267-3275,3312-3317`), spaces iterates the whole
projection (`:3287-3297`), all-documents requests `list_spaces(i64::MAX, 0)`
and aggregates every document (`:3340-3358`), and the caller-provided event
limit has no hard cap before `events_since` (`:2837-2840,3361-3371`).

The browser delivery is independently RED in the current source. Its
`AdminSessionProvider` reads/writes `sessionStorage`, retains a JS token, and
the `AdminClient` sends it as `Authorization: Bearer` on every call
(`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx:46-62,136-166`);
the component test expressly asserts this storage and header behavior
(`📦️packages/🟦️typescript/🧪️admin.test.tsx:68-95`). Thus no one-use URL
fragment exchange, HttpOnly cookie/BFF boundary, or no-JS-bearer law exists
yet. This remains separate from the REST-only polling result: Connections
does use one recursive, abortable two-second snapshot poll, preserves prior
rows and marks them stale on failure, and clears both timer and controller on
unmount (`🔴️ConnectionsPage/🟦️.tsx:45-71`). Existing tests do not hold an
in-flight request to prove abort/no-next-poll-after-cancellation. The EN/DE
key-set law is present (`📚️I18n/🟦️.tsx:287-294`), but `useAdminT` still falls
back to English (`:268-279`), which is incompatible with a strict
no-default-language assertion.

### Live S3 Re-Read — Admin Mutation Audit Attribution Is Also Missing

The durable revoke ordering does not make the administrator action auditable.
`admin_revoke_user_sessions` passes `None` as `actor_user_id` to the auth
revocation store (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3430-3434`). SQLite and
PostgreSQL do write a `session-revoked` record, but use that supplied field
(`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:376-381`; `🌎️hub/📇️directory/🐘️postgres/🦀️.rs:469-475`),
so the record cannot identify which administrator revoked the user. The
separate close and rebuild mutations authenticate then notify/rebuild without
recording an administrator action (`📦️bin.rs:3393-3411`). This corroborates
the boolean-only `is_admin` finding: resolve and retain a bounded/opaque
verified principal across command, revoke, kick, and rebuild paths, then make
the write/audit outcome durable. No runtime claim is made from the existing
same-user revoke race law.

### Live S3 Re-Read — Admin Relay Carrier Has Safe Pieces but No Production Owner Path

The prior direct-browser-bearer finding is source-superseded only at the SPA
component boundary. `AdminClient` now sends no Authorization header and uses
same-origin credentials (`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx:38-61`);
the provider accepts only a 32-byte fragment nonce, removes the fragment before
probing, and exchanges it at the local relay (`:124-166`). The relay source
uses separately domain-separated SHA-256 digests and timing-safe comparison,
consumes a valid bootstrap proof before minting an opaque host-only
`HttpOnly; SameSite=Strict; Path=/` cookie, and sends the retained capability
only upstream (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:62-145`). Its source
oracle covers raw-local denial, proof replay, opaque-cookie admission,
same-origin mutation fencing, and expiry (`:1353-1396`).

**RED — this carrier is not launched by a production command.**
`startLocalAdminRelay` is referenced only by that self-contained oracle
(`📜️script.ts:62,1358`); `DevScript` recognises only `secure-suite`,
`secure-native`, and `secure-mcp` (`:1486-1569`), never requests an
`admin-relay` credential, starts an admin relay, or opens a relay URL with the
one-use fragment. The router similarly registers no dedicated admin command
(`:1587`). The UI's displayed `os-hub:dev-secure-admin` remedy is therefore
currently nonfunctional (`AdminSession:203`). Direct hub `/admin` is now
fail-closed because its SPA has no bearer, but the secure administrative
journey is absent.

The browser laws are stale as well: the current test still imports deleted
`AdminTokenForm` and asserts `sessionStorage` plus `Bearer secret-token`
(`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️admin.test.tsx:12,65-96`).
It cannot evidence fragment clearing, one-use rejection, HttpOnly/no-JS bearer,
or the abort/no-next-poll lifecycle. Replace it and obtain an owned terminal
only after the relay's actual command/launch owner is wired.

The relay's otherwise useful direct Bun oracle is likewise not currently a
gate: `proveAdminRelayBoundary` is defined but no registered script calls it
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:1335-1396,1587`). It would fail its own
source-census assertion now, because it requires the Vite `/admin/api` proxy
to be absent (`:1391-1392`) while the live config still installs that direct
hub proxy (`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/⚙️vite.config.ts:32`).
This is a concrete pre-runtime RED, not a browser implementation detail:
remove the bypass and register the oracle only with the actual relay launch
path.

The backend packet is also preparatory at this exact reread. The shared
`AdminIntentV1`, page, receipt, and operation-audit DTOs now exist
(`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:216-355`),
but hub `📦️bin.rs` has not yet installed an `AdminPrincipal`, typed intent
handler, operation-audit projection, or replacement routes. The live boolean
principal/generic command and unbounded read findings above remain in force
until that server-side landing is complete and independently executed.

### Successor Re-read — Admin Relay Browser Boundary Is Source-Closed; Runtime Is Pending

The preceding “no production owner”, “unregistered gate”, direct-Vite-proxy,
stale-test, and English-fallback statements are historical. After a twelve
second stable reread, the current Vite, SPA, test, and hub-script hashes were
respectively `5c190c3b…`, `b6dd4ec7…`, `2dfea517…`, and `f6c53d60…`.

`DevScript` now owns the secure admin journey: it admits only the
`admin-relay` class for its administrator profile, configures that exact
verified subject, mints the class-bound envelope, starts the loopback relay,
and opens the one-use fragment URL (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:1493-1509,1543-1549`).
The Nx/launch owners exist at `:1603-1611` and
`.vscode/🧩️launch.seed.jsonc:3044,3163`. The current Vite config has no proxy
block at all (`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/⚙️vite.config.ts:13-32`),
so its historical direct `/admin/api` bypass no longer exists.

At source level the relay accepts the exact 64-hex fragment once, under its
separate proof digest and a timing-safe comparison; it zeroes the raw proof,
replaces it with an opaque separately-domain-separated host-only
`HttpOnly; SameSite=Strict` cookie, and keeps the capability in the local
relay alone (`📜️script.ts:62-105`). It restricts peer, Host, URL origin,
allowlisted API paths, body/response sizes, request deadline, and mutation
Origin/Referer/fetch-site before forwarding the relay-owned bearer
(`:43-60,85,113-145,178-225`). The registered Bun oracle exercises raw-local
denial, cookie attributes, replay rejection, authenticated forwarding without
response disclosure, forged and same-origin mutation behavior, expiry, and
the SPA/Vite/launch source census (`:1335-1395`).

The current React laws have also replaced the old bearer-positive fixture.
They assert exact bootstrap, fragment clearing, same-origin credentials, no
storage/cookie session secret/Authorization carrier, malformed-fragment
fail-closed behavior, and an actually held Connections request which aborts
on unmount and never schedules its successor
(`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️admin.test.tsx:89-138,194-219`).
The locale provider now presents an explicit bilingual chooser for unsupported
or absent locales, throws if no locale is selected, directly indexes the
selected locale without English fallback, and checks EN/DE key parity
(`🧱️elements/📚️I18n/🟦️.tsx:243-294,297-317`).

### Independent Browser Relay Gate — Green

After the preceding stable reread, I independently ran
`bun nx run os-hub:admin-relay-check --skip-nx-cache`. It terminated exit 0
in 24.8 seconds. The uncached target ran the direct admin entry graph (four
laws), the stylesheet graph (five laws), and
`nx run os-hub-admin:test long --run 🧪️admin.test.tsx`; Vitest reported one
file and all ten tests passed in 14.17 seconds. Its terminal assertion covers
one-use fragment bootstrap, host-only `HttpOnly; SameSite=Strict` cookie,
expiry/replay/CSRF/raw-local denial, bearer redaction, and EN/DE UI laws.

The terminal emitted only Node's `NO_COLOR`/`FORCE_COLOR` compatibility
warnings; those are advisory and did not affect either target result. This is
an independently terminal-green acceptance for the *browser relay boundary*.
It is not a backend acceptance: the typed-principal/CQRS, bounded-operation,
revocation-audit, and backend parity findings remain separately RED below.

### Live Re-read — Backend Audit Store Has Partial Durable Parity, but the Hub Has Not Adopted It

The prior preparatory description needs one qualification. The schema DTOs are
now backed by bounded SQLite and PostgreSQL operation-audit tables and store
methods: validation constrains text, phase, principal generation, and event
ranges (`🌎️hub/📇️directory/🦀️.rs:903-929`); both schemas enforce exactly one
accepted and one terminal fact per `request_id` with
`UNIQUE (request_id, terminal)` (`🪶️sqlite/🦀️.rs:182-201`,
`🐘️postgres/🦀️.rs:183-202`); and each read page is capped at 100
(`📇️directory/🦀️.rs:334-339`, `🪶️sqlite/🦀️.rs:1026-1078`,
`🐘️postgres/🦀️.rs:1060-1132`). The old claim that the audit work was DTO-only
is therefore superseded.

This does not close the backend boundary. The live hub still has no
`AdminPrincipal`/typed-intent adoption and still mounts the boolean-only,
generic/unbounded handlers documented above. Moreover its all-feature
directory dispatch calls all three new audit methods for Neo4j
(`🌎️hub/📇️directory/🦀️.rs:2544-2574`) while the current Neo4j implementation
contains none (its only `HubDirectory` implementation begins at
`🌐️neo4j/🦀️.rs:494`). That is an all-feature source RED until Neo4j parity
lands.

For a future typed handler, the database invariant prevents two durable
accepted rows, but PostgreSQL's first-row `SELECT … FOR UPDATE` does not lock
an absent request key (`🐘️postgres/🦀️.rs:1061-1080`). Simultaneous first
requests therefore race to the unique insert; one must re-read the established
accepted receipt rather than turn the expected unique conflict into a 5xx.
Require a direct concurrent same-request law when the hub integration lands.
No runtime result is claimed.

### Live S3 Re-read — Ordinary WGPU Direct Runner Still Admits Protected Environment State

The secure hub-owned direct-child path is source-positive: it strips protected
environment keys before passing only fd3 plus a benign marker
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:650-701`), and the real child probe
checks descendant fd3 closure and output redaction. That does not cover the
ordinary native owner. `NativeRunScript` builds the real WGPU binary and then
launches it with `env: { ...process.env, SEMIO_PLUGIN_MODULES }`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:276-293`).

The native entrypoint defines the protected-key census, but only evaluates it
for `--assert-no-local-credential-state` and `--credential-probe`; ordinary
startup claims fd3 and continues to plugin selection / `run_native` without
rejecting a protected inherited environment
(`⌨️native-entrypoint/🦀️.rs:35-75,83-108`). Consequently an ordinary direct
runner invocation can carry `S_USER`, `S_HUB_URL`, or TOKEN/SESSION/CREDENTIAL/
BEARER/AUTHORIZATION/COOKIE-named state into process/plugin activation. The
secure child probe is not evidence for this separate normal launch path.

**RED if the S3 no-raw-carrier contract covers every native launcher, as its
cutover census states.** Make the ordinary runner use the same allowlist (or
fail closed in normal entry before plugins) and add an actual ordinary-runner
poisoned-environment negative. This is source evidence only; no competing
native execution was started.

### Successor Re-read — Ordinary WGPU Environment Carrier Is Source-Closed

The preceding ordinary-runner finding is superseded in the current source.
`NativeRunScript` now reaches the real native executable only through
`runNativeBinary`, whose environment constructor removes `S_USER`,
`VITE_S_USER`, `S_HUB_URL`, and every case-insensitive
TOKEN/SESSION/CREDENTIAL/BEARER/CAPABILITY/AUTHORIZATION/COOKIE key before
retaining only benign state (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:31-59,323-341`).
It applies on both the ordinary and `--scale` native paths. The entrypoint
also fail-closes its protected-key census before fd3 claim, argument parsing,
or plugin selection (`⌨️native-entrypoint/🦀️.rs:35-62,87-112`), while allowing
only the fd3 marker needed by the protected direct-child mechanism.

I independently ran the smallest registered source/child gate,
`bun …/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts native-environment-check`.
It exited 0 and printed exactly: `native-environment-check: poisoned ordinary
runner sanitized and binary fail-closed guard precedes credential/plugin
activation`. Its Node direct child receives the real `runNativeBinary`
environment transform under poisoned raw values; its entrypoint part is a
source-order assertion. This is terminal evidence for that transform and
source-order contract. It is not a compiled/started WGPU binary, so it does
not supersede the separately pending native process-backed SocketGrant/actor
runtime evidence. Launch generation is also independently RED on its external
zero-host-metadata condition and is not credited here.

I then ran the exact registered uncached target,
`bun nx run @semio-tech/framework-renderer-wgpu:native-environment-check
--skip-nx-cache`. It also exited 0 and reported `NX Successfully ran target
native-environment-check`. Its sole `NO_COLOR`/`FORCE_COLOR` diagnostic was
advisory. The registered launch seed references this target at
`.vscode/🧩️launch.seed.jsonc:1532-1539`; that registration is therefore
terminal-green independently of the still-red generated launch catalog.

### Successor Re-read — MCP Post-Return Directory Dial Fence Is Source-Closed

The MCP native driver now captures a nonzero authority generation immediately
before its initial `open_stream_ws` and applies the same discipline on every
reconnect (refreshing first if the authority generation is zero)
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:514-519,553-573`).
Both sites call `complete_authorized_directory_dial` (`:662-676`), which
checks cancellation, a zero generation, and generation turnover *after* the
open returns. A failed fence closes the stream before forwarding the late
connection to `complete_dial`.

This has an exact once-close terminal path: `DirectoryStream::complete_dial`
closes a successful late connection if its stream is already closed and
returns `Closed` (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:706-720`);
the initial activation maps that turn to cancelled/unavailable and the worker
breaks (`🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:519-526,573-576`). Its focused
native-driver law turns each fixture stream to `Dial`, then independently
cancels or invalidates its authority before providing an observable socket;
it asserts `Closed`, exactly one close, no request increase, and no
cancel-case binding mutation (`:841-893`). Kernel laws separately prove that a
cancellation after grant prevents both socket open and hello and that a
cancellation immediately after open closes before hello (`📇️directory/🔌️client/🦀️.rs:1779-1807`).

This supersedes the prior post-return cancellation/generation source finding.
The driver fixture supplies an already-open observable socket, so it is joined
to rather than substituted for the kernel's actual grant/open/hello laws. No
Cargo runtime terminal was run in this shared-build interval.

### Live RED — Admin Intent Cutover Is Not Atomic Across the Shipped SPA

The current hub mounts only the typed mutation endpoint,
`POST /admin/api/intents` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3877-3884`),
but the shipped `AdminClient` still offers the removed generic command,
directory rebuild, per-connection close, and per-user session-revocation URLs
(`🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx:93-106`). It has
no typed intent method. Those user-visible controls therefore resolve to no
route, rather than to the new principal-bound/idempotent intent handler.

The read side is likewise only partial: the client and its active test fixture
still decode legacy unpaged `ConnectionView[]` (`🔑️AdminSession/🟦️.tsx:81-82`,
`📦️packages/🟦️typescript/🧪️admin.test.tsx:175-179`) while the schema's bounded
`AdminConnectionSnapshotV1`, `AdminPageV1`, and redacted operation-audit types
are not yet used by the hub. This is a source-level atomic-cutover RED; the
otherwise green relay/browser test only proves the bearer boundary and does
not exercise these mutations. No runtime terminal is claimed.

The mismatch also reaches the authority owner: the loopback relay currently
allowlists those four old POST forms but not `POST /admin/api/intents`
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:47-52`), and its positive unsafe-request
law still calls the removed rebuild route (`:1372-1375`). The MCP process
probe similarly tries to force its document reconnect through the removed
per-connection close route (`:897-899`). Thus the earlier relay terminal only
establishes bearer/cookie handling against its fixture; it cannot be credited
as a live typed-admin mutation result, and the current MCP reconnect proof
cannot be credited until the route/relay/probe change lands atomically and a
fresh process terminal is obtained.

### Successor Re-read — Typed Admin Mutation Adoption Is Partial, Not Absent

The earlier statements that the hub had no typed principal or intent handler
are superseded. `authenticate_admin_principal` parses one exact bearer,
performs a two-second durable-session lookup, and rejects zero-generation,
expired, or unconfigured provider/subject identities using the configured
digest comparison (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1252-1297`).
`admin_intents` admits only exact JSON no larger than 8 KiB, derives the
intent digest, writes an accepted operation fact before dispatch, and writes
a terminal fact afterwards (`:3574-3628`). SQLite/PostgreSQL use the
`(request_id, terminal)` uniqueness invariant and re-read the winning record;
Neo4j now constrains `requestTerminalKey` as unique
(`🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:169-193,901-966`). A session-derived User
actor drives directory operations, so the previous static-Admin/create-space
source failure is also closed (`📦️bin.rs:1091-1094,3457-3463`; `📇️directory/🦀️.rs:1324-1329`).

Credential lifecycle attribution is source-present across all three
backends: shares and session revocation receive the verified user id and
correlation, while directory invite operations derive that user id from the
session-derived actor (`📦️bin.rs:3468-3537`; `📇️directory/🦀️.rs:1390-1399`).
This is source-only: no focused concurrent first-writer, principal-rejection,
cross-backend parity, durable-revoke/kick, or terminal handler law has run.

The remaining backend REDs are concrete. Legacy reads are still boolean,
unpaged, and frequently unbounded/N+1 (`list_users(i64::MAX)`, all-space
loops and per-space view/document lookup); connections merely avoids the
per-row user lookup through one map but still returns legacy `ConnectionView`
instead of the recorded-binding snapshot (`📦️bin.rs:3628-3739`).
`AdminPageV1`, `AdminConnectionSnapshotV1`, `AdminOperationAuditV1`,
`ADMIN_PAGE_MAX`, and `ADMIN_RESPONSE_MAX_BYTES` remain imported/declared but
unwired. An accepted-only crash record has neither operation-status/recovery
route nor a reconciliation actor, and the 10-second rebuild control publishes
neither progress nor cancellation externally. The outer
`AdminIntentV1::Directory { command }` also still carries a generic directory
command, even though the old `/admin/api/commands` route is gone. These facts
block a backend acceptance independently of the client cutover RED above.

## Durable Current-Tree Supplement — Browser Artifact, Native Epoch, and Exact FD Marker (2026-09-03)

This supplement records the later live re-reads that were initially written under a variation-selector ticket path; the other file is preserved, and this no-VS ticket path is the durable audit record.

The native receipt epoch source is closed. `clear_socket_epoch()` clears receipt actor, explicit Session confirmation, and color; artifact-bootstrap failure, EOF/transport failure, failed replay, failed write, and unsuccessful connection all use it (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1803-1821,1885-1936,2023-2029,2302-2315`). Delivery queues until a matching `ServerFrame::Session`; the direct-socket law exercises bootstrap failure, EOF, zero pre-Session delivery, exactly one fresh-actor batch after Session, and reconnect (`:2214-2222,2273-2294,4570-4665`). This is source/law evidence, not an independent Cargo terminal.

The prior non-fd3 marker escape is source-superseded for both native and MCP. Each predicate allows `S_LOCAL_CREDENTIAL_FD` only when its value is exactly `"3"`, before claim/argv/plugin or renderer activation (`🌉️mcp/📦️bin.rs:107-160`; `🧊️wgpu/⌨️native-entrypoint/🦀️.rs:35-89`). The compound secure-local smoke is wired to run actual native and MCP non-fd3 children with bounded, redaction-checked stdout/stderr (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:1115-1159,1356-1363`). Runtime behavior remains pending that compound terminal.

I independently ran `bun nx run @semio-tech/framework-renderer-wgpu:check-frame-worker --skip-nx-cache` three times during the import/census repair cycle. The first two current-tree attempts correctly failed before rendering on stale styling then interactive-registry imports; both causes were repaired. The final run, after the normalized carrier policy, exited **0** and printed `framework-renderer-wgpu: 🟨️frame-worker.js is fresh`. The current census lower-cases the deployed artifact and rejects authorization/bearer/credential, token/access-token query forms, token setter/hub token, raw environment, storage, and cookie residues (`🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:220-229`). Direct inspection found no such raw carrier or legacy Hello; generic `token` occurrences are internal UI-operation tokens. The current browser artifact freshness boundary is accepted.

### D1 Receipt-to-SocketGrant Exchange — Current Source Reread

The private open-plan authority is now carried on `SocketGrantRecordV1` and is
part of every consume, live-registration, and live-authority equality check
(`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:625-819`).  The plan ledger keeps its mutex
through the prospective socket-grant insertion, marks a receipt consumed only
after that insertion succeeds, and leaves it `Issued` with no selector if the
bounded socket ledger rejects capacity (`:1120-1283`).  The concurrent law
uses eight simultaneous exchanges and proves one grant, seven consumed
failures, exact authority carriage, and capacity retry (`:6402-6466`).

The route authenticates and revalidates before the exchange, caps body and
whole operation duration, rechecks descriptor/catalog/revision, and exposes
only the grant receipt.  The historical readiness statement in the preceding
paragraph is superseded: both `open_plan` and `open_plan_exchange` derive
from the one production `open_plan_ready` predicate
(`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1710-1744,5278-5283`). Binding revocations acquire
their subject admission and make the socket- then plan-ledger invalidations as
separate, sequential locks, avoiding a reverse nested ledger lock.  The
late-invalid receipt decoder law observes all 31 candidate bytes wiped and is
exact-selected by both the all-feature and default-server gates
(`:6762-6777`; `📜️script.ts:2317-2359`).  No material D1 source defect was
found.  This is source/law evidence only; this audit did not run Cargo and
does not credit a runtime terminal.

## Live Document-Open Transport Cutover (2026-09-04) — Browser Source/Test Present; Native And MCP RED

This is a current-byte end-to-end map, not a runtime claim. The server has no
production direct document-grant issuer: its sole mounted document paths are
`POST /open-plan`, receipt exchange at `POST /socket-grants`, and
credential-free `/socket/v1` (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5066-5102`).
The test-only direct issuer is behind `#[cfg(test)]` (`:2215-`), so it is not
a production bypass.

| Surface | Shipped entry and authority path | Current classification |
| --- | --- | --- |
| Hub issuer and socket admission | `issue_document_open_plan` rejects query, malformed/multiple content type and oversize body, authenticates/revalidates the subject, resolves the durable descriptor and trusted catalog, then rechecks revision/revalidation before minting (`📦️bin.rs:1986-2123`). Exchange repeats authentication, descriptor/catalog/revision checks and is bounded to two seconds (`:2125-2213`). The plan ledger holds its lock through bounded SocketGrant insertion and marks the receipt consumed only after insertion (`:1191-1274`). Socket consume/live rechecks the private plan identity, subject, actor, surface, descriptor/catalog/revision and checkpoint before use (`:697-810,2598-2663`). Capability debug is redacted and decoded temporary secret bytes wipe on drop (`:853-965`). | **Source/test only.** Exact server selectors and an independent fixture/oracle exist (`📜️script.ts:2269-2400`), but this audit did not run them or a hub process. |
| React browser OS | The physical ShellHost entry creates the module worker (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️.tsx:1426`) and passes document id/schema plus a hub binding to it (`:3437-3485`). The worker mints strict `DocumentOpenIntentV1`, boundedly reads/parses the plan, binds scope/schema/requested surface/`react` target/pack hash, checks cancellation, then exchanges the one-use receipt; only the resulting SocketGrant is offered to `/socket/v1` (`🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:381-467,817-864`). The URL is checked free of plan/grant/session/bearer fragments and the Hello test checks the same (`:2310-2356`). The hostile law covers mismatched and max-plus-one plan responses plus cancellation before exchange with no socket (`:2359-2410`). | **Source/test only, with a readiness-law gap.** The D1 fixture is neutral and physical (`🧫️fixtures/📇️directory/📄️browser-document-open-v1.json`), but the registered `browser-broker-check` selects only broker-ratchet and directory-queue tests, not either `browser document open` D1 law (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:1985-1993`). Moreover the receipt immediately sets `hubActorReady`/posts `socket-actor` before a matching server `Session`, and the relay sends whenever the WebSocket is open (`🟦️backbone-worker.ts:821-823,921-935,1243-1250`). The server itself serializes Session before its receive loop (`📦️bin.rs:3148-3177`), so this audit does not claim a server-side pre-Session write; it does require a browser no-pre-Session-delivery/stale-actor-on-close law. No independent browser gate/runtime was run here. Require that exact law pair in the registered browser transport gate, including fresh-plan-on-reconnect after one-use exchange failure. |
| Native WGPU | Native startup claims the native local credential before renderer activation (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs:35-89`); the Shell injects the authenticated directory client into `ArtifactHost` (`🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs:4071-4088`). But `ArtifactHost` only accepts `HubSocketGrantSource` (`🏪️store/🔄️sync/🦀️.rs:1046-1087`), whose sole operation directly POSTs `/socket-grants` as a plain SocketGrant request (`📇️directory/🔌️client/🦀️.rs:277-279,614-650`). `start_connect_hub` calls that old operation with a root cancellation token and directly dials the result (`🏪️store/🔄️sync/🦀️.rs:1846-1882`). | **RED.** It cannot issue/validate a D1 open plan, bind selected descriptor/package/surface, exchange the one-use receipt, or cancel the whole plan→grant→dial chain. The production hub now requires `DocumentPlanSocketGrantIntentV1` at that endpoint, so this native path fails closed/retries rather than functioning. Replace the direct trait with a D1 authority source that owns issuer, bounded response parse, receipt exchange and an actor-owned cancellation token; add real child proof for one-use/reconnect/redaction. |
| Native Tauri | No `tauri` source or executable path exists under the OS or hub trees in this current-tree census. | **Not applicable; no shipped Tauri transport found.** |
| MCP hub workspace | `HeadlessWorkspace::open_hub` creates the authenticated descriptor binding, then injects the same old `HubSocketGrantSource` into `ArtifactHost` (`🌉️mcp/🏠️workspace/🦀️.rs:1226-1236`). Descriptor discovery is separately authenticated/scope-bound by `NativeHubBindingDriver` (`🏠️workspace/🔗️remote/🦀️.rs:477-524`), and non-open hub resources stay descriptor-only (`🏠️workspace/🦀️.rs:1304-1324,1584-1635`). However `artifact_open` recognizes the authenticated probe then actually opens it through `ensure_probe_artifact` and `ArtifactHost` (`🌉️mcp/🗿️artifact/🦀️.rs:335-368`; `🏠️workspace/🦀️.rs:1332-1371`), which reaches the old native grant operation above. | **RED for actual hub document open; source-only accepted for descriptor-only listing/resource exposure.** The directory-stream post-open cancellation fence is unrelated to the document plan/grant exchange. MCP needs the same D1 source migration and a process-backed proof that plan receipt/grant never reaches MCP JSON-RPC stdout/stderr, a resource URI, argv, environment, or reconnect diagnostics. |

The only remaining authority activation route in the server is therefore D1.
The material cutover blocker is not a server legacy route but the shared
native `HubSocketGrantSource` abstraction, inherited by both WGPU and MCP.
Browser code has reached the D1 source/test boundary, but its separate
registered browser gate has not yet made those laws non-vacuous.
