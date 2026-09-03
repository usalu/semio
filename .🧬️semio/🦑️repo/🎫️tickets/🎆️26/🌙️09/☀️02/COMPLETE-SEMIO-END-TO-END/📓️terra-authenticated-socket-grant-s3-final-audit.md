# SocketGrant S3 Client Migration — Live Final Audit

Status: **REJECT — S3 is not a release boundary yet.** This is a read-only, live-source audit. It does not claim any build, test, or runtime result. The reported OS-kernel compile seam is not evidence of a real native launch or of an authenticated document session.

## Decision

S1/S2 may be the server foundation, but S3 requires one atomic greenfield cutover: every protected dial obtains a fresh protected grant, offers the ordered `semio.socket.v1, <grant>` subprotocol pair, sends credential-free `SocketHelloV1`, and retains the server receipt actor only in the live owner which constructs outgoing envelopes. There must be no token field, query credential, tag-0 `Hello`, legacy route, bridge URL secret, or compatibility carrier anywhere in the released client/server surface.

The React document/directory worker is materially closer to that target. Native document sync, MCP workspace/upstream, the legacy hub routes and fixtures, and the AgentBridge remain outside it. Therefore a mixed release would either retain forbidden carriers or leave callers unauthenticated.

## Live Source Findings

### Browser broker and React worker — provisional, not yet accepted

The current secure-local topology has a credible narrow authority boundary:

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` keeps the relay bearer in the relay, requires loopback peer plus exact host/origin/referer/fetch-site and relay secret, admits a small method/path matrix, bounds body/in-flight/deadline, and stores only the digest of the one-use browser proof.
- The initial proof is fragment-only and immediately removed by `ShellHost/🟦️.tsx`; its `MessagePort` is transferred only to `🟦️backbone-worker.ts`. Plugin shards receive typed mutations/snapshots, not `window`, the port, or a raw proof.
- `🟦️backbone-worker.ts:248-281` serializes proof use, uses a fresh next proof digest, and clears local proof on a missing advancement acknowledgement, `401`, or transport failure. The relay now returns `x-semio-browser-broker-advanced: 1` only after consuming the current proof. This closes the earlier pre-advance desynchronization.
- The same worker obtains directory/document grants, uses `SocketHelloV1`, and its directory client is restricted to the BFF allow-list.

This is **not accepted yet**. Required evidence is still a runtime/neutral adversarial oracle: a raw local HTTP client which spoofs browser headers but lacks the proof must have no hub effect; a stale proof replay must fail; a malicious plugin shard must be unable to obtain proof/port or issue a broker request; and startup must prove Vite readiness precedes one-use fragment navigation. No such run is claimed here.

The fragment/worker approach is viable only for the stated threat model (raw local HTTP caller and ordinary plugin shard). It cannot defend against arbitrary trusted main-thread code, browser extensions, or an OS-level process inspector. It must remain an explicitly private worker capability, never a generic same-origin header supplied by Vite.

### Native identity and directory client — partial structural closure

`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:315-383` defines `LocalHubCredential` as a private byte buffer: `Debug` redacts it and `Drop` zeroes it. Its inherited reader consumes exactly one bounded (16 KiB) fd-3 frame, requires EOF immediately after the frame, validates the native class, loopback origin, expiry, and session-capability grammar. `🪪️identity/🦀️.rs:54-77` gates bootstrap on the non-secret sentinel `S_LOCAL_CREDENTIAL_FD=3`, calls `/auth/sessions/me`, and projects a non-secret `Identity`.

This is a good consumer seam, and the old persistent OS identity `session_token` field is **superseded/closed** in the current tree: `🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🦀️.rs:14-20,61-67` contains no token.

For a final zeroization claim, also remove the transient duplicate made at `📇️directory/🔌️client/🦀️.rs:350-373`: the raw frame is zeroed, but it is parsed into a `DslValue` string and then copied into the credential buffer. Take ownership of the capability string/allocation from the decoded object before converting it to the sealed byte buffer, so the temporary secret allocation is not merely dropped.

It is nevertheless not a usable native S3 path:

1. `🌎️hub/📦️packages/🦀️rust/📜️script.ts:497-524` can create a direct child pipe at fd 3, serializes the bounded `semio.local.consumer-credential/v1` envelope, and clears the source envelope capability. But its exported native delivery function is reached only by the Node delivery proof helper, not by a real WGPU launcher.
2. The actual WGPU dev runner at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:260-281` invokes `cargo run` with `{ ...process.env }`. It neither calls the fd-3 delivery helper nor starts the final native executable as the direct child. Cargo is an intermediate process, so fd-3 delivery to it does not prove inheritance by the executable. The copied process environment is also not a secret-safe allow-list.
3. The delivery helper itself uses `{ ...process.env, S_LOCAL_CREDENTIAL_FD: "3" }` at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:500`. A launch boundary must use an allow-list of non-secret configuration, rather than forwarding arbitrary parent environment values.

Required repair: build first, resolve the final native executable, then have the local supervisor spawn *that executable* through one cross-platform fd/handle delivery owner. Pass only an explicit non-secret environment allow-list (`S_LOCAL_CREDENTIAL_FD`, modules root, asset origin, locale/config needed by the program); do not place the capability in argv, cwd, logs, readiness, disk, a config mutation, or any environment variable. Parent must write one bounded frame, close its write half, zero all mutable buffers, and terminate/revoke on child startup/EOF/deadline failure. Unix and Windows need equivalent direct-child handle tests, not only a Node consumer proof.

There is a second ownership condition for that repair: fd 3 must be claimed and made non-inheritable *before* plugin activation. `Shell/…/wgpu/🦀️.rs:2269-2319` creates an app/plugin instance before it starts `bootstrap_identity`; the actual fd read is deferred onto an I/O future at `:4018-4034`. `LocalHubCredential::read_inherited` adopts fd 3 at `directory/client/🦀️.rs:337-343` but does not first set close-on-exec/clear the inherited flag. The native plugin host can spawn process shards (`🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️.rs:214-215`). A descendant must never race to consume the one-shot endpoint. Earliest native entry must seal and consume the descriptor before any untrusted/plugin child exists; Unix requires close-on-exec ownership and Windows requires a non-inheritable duplicate plus cleared original inheritance. Add a malicious-child law that observes no usable fd-3 credential after this earliest step.

### Native document sync — hard S3 blocker

Native document sync is actively mid-migration and is not a compilable/acceptable S3 boundary:

- The current `PersistenceBinding::Hub` declaration at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:80-96` has removed `token`, which is the right public-schema direction. Its remaining fan-out is inconsistent: the wasm actor still destructures/copies `token` at `:3581-3607`; WGPU Shell still constructs/matches it at `Shell/…/wgpu/🦀️.rs:364,659,3032`; and the OS host helper still accepts it at `🖥️host/🦀️.rs:2459-2460`. This audit did not compile the intermediate tree.
- `ArtifactHost` now has a private credential slot at `🏪️store/🔄️sync/🦀️.rs:1046-1072`, but `set_local_hub_credential` has no live caller. `Shell::poll_identity_bootstrap` only constructs its directory client at `Shell/…/wgpu/🦀️.rs:4043-4051`; it does not hand `outcome.credential` to `document_host`. An actor opened before this handoff must fail closed and rebootstrap once the host has a credential; it must not dial anonymous or retain a stale credential.
- `:756-768` still constructs `/spaces/{space}/documents/{document}/ws`.
- `:1809-1837` dials that route and emits tag-0 `ClientFrame::Hello` containing a client-selected actor and the optional token. The wasm twin does the same at `:3198-3229`.
- `:4797-4804` still generates the token-bearing `client-hello` fixture.
- The WGPU Shell currently supplies `token: None` for the default Hub binding (`Shell/…/wgpu/🦀️.rs:350-367`) and derives `user:{id}#wgpu-*` locally, so it is neither a functioning protected document dial nor a receipt-actor owner.

Repair as one breaking structural packet: delete token from `PersistenceBinding`, `ArtifactActorConfig` serialization, sync actor state, fixtures, TS twins, and all tag-0 framing. Introduce a host-only, non-serializable document-socket issuer injected after actor construction. On each initial/reconnect/rebootstrap dial it must (a) issue the exact scoped HTTP grant with the private inherited/broker credential, (b) validate receipt grammar/audience/expiry, (c) install its `actorId` before any command/presence envelope construction, (d) offer only the two ordered protocols, and (e) send credential-free `SocketHelloV1`. Close/clear the grant immediately after WebSocket construction; cancellation before/while issuance must produce no dial. The same structural API must cover native and browser twins.

### MCP workspace/upstream — hard S3 blocker

MCP still takes and retains a raw hub bearer:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:440-463` accepts `token: &str` and injects it into a directory client; the fixture path at `:627-636` retains the old API.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:433-449,1216` owns `WorkspaceOrigin::Hub { token }` and maps it to `PersistenceBinding::Hub { token: Some(...) }`.
- The MCP bridge/transport still has bearer/bridge token configuration and a token-file writer. It needs an independent direct-child fd-3 consumer, not reuse of the browser BFF and not persistence in an MCP descriptor/workspace object.

There is also a live structural mismatch: those MCP lines call `DirectoryClient::new(...); client.set_token(...)`, while the current client exposes `DirectoryClient::authenticated(..., Arc<LocalHubCredential>)` and no `set_token` method (`📇️directory/🔌️client/🦀️.rs:419-429`). This audit did not run a build, so it records the source incompatibility rather than claiming a diagnostic; it is nevertheless a first implementation blocker for the MCP packet.

The packet needs an MCP `InheritedLocalCredential` owner analogous to native (class must be `mcp`), a private protected HTTP/grant issuer, and non-serializable upstream connection authority. Stdio transport remains an upstream delivery channel; it is not a socket grant and must not be copied into a URL, request log, audit record, workspace snapshot, or plugin-visible config.

### Server and global legacy removal — hard S3 blocker

The hub still serves both generations:

- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2872-2885` retains query `token` and `/directory/ws`.
- `:3588-3623` mounts `/directory/ws` and `/spaces/{space_id}/documents/{id}/ws` alongside the v1 grant routes.
- The server and package tests still construct tag-0 `ClientFrame::Hello` and query credentials, including `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:793-795` and the Rust tests around `📦️bin.rs:4441` onward.

S3 must remove these routes, handlers, query types, carrier fields, tag-0 parser/encoder/fixtures, and token-bearing test helpers in the same change. A server that merely rejects old frames after accepting an old route is not an S3 release boundary.

### Agent bridge and admin

`AgentBridge/🟦️.tsx:103-138,297-433` still discovers a Vite token and appends it as `?token=`. Its tests assert that behavior. This violates the no-secret-in-URL condition and must be removed, not adapted. The safe admin choice remains authenticated bounded REST snapshot/poll; no admin WebSocket audience is needed or audited for S3.

## Required Atomic Order

1. Land the shared schema/wire deletion: no tag-0 `Hello`, no `token` field in public binding/config/wire types, strict v1 socket header + credential-free hello only. Update Rust/TS fixtures and a neutral codec oracle together.
2. Land a host-private `ProtectedSocketIssuer` interface and receipt-actor installation for document and directory clients. It cannot be `ToValue`/`FromValue`, clone into a plugin, or expose a bearer string.
3. Implement direct supervisor-to-real-native and supervisor-to-real-MCP launches with bounded fd/handle frames, strict class/origin/expiry validation, environment allow-lists, zeroization, cancellation and teardown laws.
4. Migrate React through its worker broker; retain the BFF only if the raw-local/replay/malicious-plugin/readiness/redaction oracle passes. Otherwise fail closed and do not launch the browser.
5. Move admin to bounded authenticated REST polling and delete AgentBridge URL credentials.
6. Delete the old hub routes/handlers, all client old dials, bridge/token config, fixtures, test helpers, and launch paths in the same commit. No compatibility interval.

## Acceptance Laws and Gates

- A protected document and directory dial succeeds only with a newly issued exact-scope receipt, exact protocol order, and `SocketHelloV1`; a second use/restart/expiry/revoke fails before exposure.
- A forged actor, tag-0 hello, bearer header/query token, protocol reordering, extra protocol, and receipt scope/generation/selector mismatch all fail closed with no secret in URL/error/audit/readiness.
- Reconnect and rebootstrap reissue grants and replace the actor before any queued outbound envelope; cancellation/progress are bounded and no cancellable action reaches the transport after cancellation wins.
- Two native children and an MCP child prove fd/handle class isolation, bounded EOF framing, expiry, malformed/oversize rejection, no env/argv/disk/log leak, and cleanup on child/supervisor death. Cover Unix and Windows.
- The BFF oracle independently probes raw local HTTP spoofing, proof replay/rotation acknowledgement, malformed/over-limit routes, `401`, teardown, no bundle secret, and malicious plugin-shard isolation.
- A whole-repository caller census gate returns zero for `/directory/ws`, document `/ws`, `?token=`, `ClientFrame::Hello`, serialized hub token fields, and legacy bridge URL token behavior, except historical audit text outside the shipped source/test surface.

## Evidence Qualification

An implementation message reported an OS-kernel compile seam as green. That can establish only compilation of the cited Rust seam; it does not exercise a final native executable spawned as an fd-3 child, a real session, a document socket, cross-platform inheritance, or redaction. No run is attributed as acceptance evidence in this report.
