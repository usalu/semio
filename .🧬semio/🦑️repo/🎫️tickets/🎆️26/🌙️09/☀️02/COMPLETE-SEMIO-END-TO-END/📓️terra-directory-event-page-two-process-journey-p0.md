# Terra — Directory Event-Page Two-Process Reconnect Journey P0

## Decision

**RED for the required browser/WGPU projection journey.** The repository has a real two-OS-process hub boundary (the Bun gate process supervises an `os-hub` child) with durable SQLite, two authenticated identities, canonical-page receipts, stale-session denial, and hub restart. It does **not** put either browser or WGPU retained-Home owner on that boundary, open `/directory/socket/v1` in that process journey, inject the fetch-to-socket gap, or observe a projection receipt/frontier after an ACK. It cannot therefore prove reconnect, cancellation, rebootstrap, or duplicate-free replay for either shell.

This is a read-only audit. No product source or gate was changed or run. “Existing” below means source-inspected, not newly executed evidence.

## Exact Current Seams

| Need | Existing seam | What is established | What it does not establish |
|---|---|---|---|
| Actual hub process | `startLocalHub` in [`🌎️hub/📦️packages/🦀️rust/📜️script.ts`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L649) creates a loopback port, then `spawn`s the built `os-hub` binary at lines 670–708. | The test driver and hub are distinct OS processes; the child is bound to `127.0.0.1`. | The profiles are not client child processes; they are credentials used by the one Bun driver. |
| Durable SQLite and restart | `startLocalHub` sets `OS_HUB_DIRECTORY_BACKEND=sqlite` and filesystem storage when `isolatedSecuritySmoke` is true ([`📜️script.ts:670-686`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L670)). `proveDirectoryEventPageV1Process` reuses its `dataRoot` for `first` and then `second` hub children ([`📜️script.ts:5131-5149`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5131), [`:5197-5207`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5197)). | Directory records survive a real hub stop/start on the same data root. | It does not reconnect a live socket across the restart, nor restore a shell projection. |
| Independent authenticated identities | The process proof declares `event-page-a` and `event-page-b`, then mints distinct `native` envelopes through the private bootstrap pipe ([`📜️script.ts:5137-5154`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5137)). `issueLocalCredential` validates the envelope class, run, profile, and nonzero authorization generation ([`📜️script.ts:747-776`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L747)). | Two identities can issue authenticated REST commands against one real hub. | It does not launch two Home projections or two app processes. |
| Real page request and receipt verification | `fetchLiveDirectoryEventPage` performs authenticated `GET /directory/event-page/v1?after=…`, requires exact canonical JSON, recomputes SHA-256, and checks binding, generation, receipt and frontier ([`📜️script.ts:5084-5110`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5084)). | The process gate independently verifies the producer’s exact page bytes. | It does not pass the page to a retained Home action or observe the required ACK. |
| Real command/event production | `submitLiveDirectoryCommand` posts authenticated commands and requires `202` plus events ([`📜️script.ts:5113-5129`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5113)). The hub route authorizes authors for rename/member operations before execution ([`🚀️bin.rs:3814-3846`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L3814), [`:3899-3913`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L3899)). | A can promote B to author, then B can produce an A-visible `rename-space` event using only real REST authority. | Current process proof only creates hidden/visible page cases; it does not arrange a peer-authored gap after A has ACKed. |
| Hub page route | Router owns `/directory/event-page/v1` at [`🚀️bin.rs:5736-5749`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L5736). `get_directory_event_page_v1` owns one 5-second request control and calls the bounded, revalidating builder ([`🚀️bin.rs:4117-4188`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L4117)). | Canonical pages are bounded, session-bound, checked before and after read, and cancelable inside the handler. | The route has no server-side ACK; ACK is intentionally a client/Home boundary. |
| Shared client frontier law | `DirectoryEventPageBootstrapV1` retains a pending page and moves only on the exact `DirectoryEventPageAckV1` ([`🔌️client/🦀️.rs:260-360`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs#L260)). `stream_acknowledged` creates an untracked wake stream, and `DirectoryStream::acknowledge` is the sole frontier advance ([`🔌️client/🦀️.rs:859-868`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs#L859), [`:1120-1143`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs#L1120)). | The shared native protocol has the correct ACK/wakeup primitives. | No source found that mounts this owner in the active browser or WGPU shell journey; that is the blocker identified in the companion retained-Home audit. |
| Real global WebSocket | Hub routes issue a directory socket grant and upgrade `/directory/socket/v1` ([`🚀️bin.rs:2450-2458`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L2450), [`:4329-4353`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L4329)). The native client’s acknowledged stream dials this route without letting observed messages advance its cursor ([`🔌️client/🦀️.rs:1190-1227`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs#L1190)). | The protocol can treat frames as dirty wakeups rather than committed history. | `proveDirectoryEventPageV1Process` contains no `WebSocket`, `stream_acknowledged`, `/directory/socket-grants`, or `/directory/socket/v1` use. |

## What the Existing Tests Give Us

### Process-grade hub evidence

`DirectoryEventPageV1CheckScript` makes `process` run the five focused hub laws, build `os-hub`, run `proveDirectoryEventPageV1Process`, then `cargo check` ([`🌎️hub/📦️packages/🦀️rust/📜️script.ts:5222-5264`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5222)). The registered Nx target is [`🌎️hub/📦️packages/🦀️rust/📋️project.json:135-142`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📋️project.json#L135), with the launch entry at [`.vscode/🧩️launch.seed.jsonc:3881-3894`](../../../../../../../../../.vscode/🧩️launch.seed.jsonc#L3881).

The process proof already verifies all of these:

- A and B issue private-space commands; A’s page exposes A-visible sequences and does not leak B’s private identities ([`📜️script.ts:5155-5169`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5155)).
- A byte-limited first page and continuation contain the deferred visible event exactly once ([`📜️script.ts:5171-5181`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5171)).
- A saturated foreign/private range advances the raw frontier without exposing foreign content ([`📜️script.ts:5183-5191`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5183)).
- B self-revokes through `DELETE /auth/sessions/me`; a stale event-page bearer gets empty `401` ([`📜️script.ts:5193-5195`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5193)).
- A fresh session after actual SQLite hub restart reads persisted history with a changed binding ([`📜️script.ts:5201-5209`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5201)).

These are meaningful producer/transport laws. They are not a browser/WGPU projection test.

### Loopback TCP/WS integration harness

The in-bin tests are useful for deterministic race construction, but not an OS-process client test:

- `test_state_with_capacity` creates a unique test DB root, `db::Database` with `Profile::Test`, an in-memory `SqliteDirectory`, real service, and socket ledger ([`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6373-6428`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L6373)). `spawn_server` binds a real loopback `TcpListener`, but serves it from `tokio::spawn` in the same Rust test process ([`🚀️bin.rs:6586-6594`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L6586)).
- Raw HTTP uses a real `TcpStream`, owns request bytes, waits at most five seconds, and can report a dropped connection ([`🚀️bin.rs:6602-6657`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L6602)).
- `next_directory_message`, `next_close_code`, `socket_request`, and `issue_test_session` are focused real-WebSocket/session helpers ([`🚀️bin.rs:6918-7023`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L6918)).
- `socket_grant_directory_route_uses_credential_free_hello_and_revokes_live` opens an actual `/directory/socket/v1`, receives a live event, then observes `4401` after self-revocation ([`🚀️bin.rs:8157-8180`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L8157)).
- Server replay loads the suffix from the requested `since`, remembers every delivered sequence in `last_replayed`, and ignores a live event at or below it ([`🚀️bin.rs:4498-4558`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L4498)). This is the right server-side duplicate fence to exercise.

## Session Generation, Cancellation, and Disconnect Facts

### Session rotation/revocation

There is no observed in-place session *rotation* journey. The supported concrete operation is revoke old session plus mint a new envelope/session:

- SQLite `revoke_auth_session` sets `revoked_at`, records a reason/audit, and increments `authorization_generation` transactionally ([`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1101-1114`](../../../../../../../../../🌎️hub/📇️directory/🪶️sqlite/🦀️.rs#L1101)); batch revocation has the same increment at [`:427-450`](../../../../../../../../../🌎️hub/📇️directory/🪶️sqlite/🦀️.rs#L427).
- Socket binding revalidation rejects a revoked session or generation mismatch ([`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1069-1099`](../../../../../../../../../🌎️hub/📇️directory/🪶️sqlite/🦀️.rs#L1069)). The global socket checks that binding when issuing/consuming the grant and again before outbound frames ([`🚀️bin.rs:2921-2930`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L2921), [`:4379-4420`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L4379)).
- `DELETE /auth/sessions/me` serializes on the session binding, revokes it, invalidates socket grants/plan records, and returns `204` ([`🚀️bin.rs:4619-4641`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L4619)). An open global socket sees a terminal `4401` either from the ledger notification or the one-second authority tick ([`🚀️bin.rs:4529-4595`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L4529)).

The P0 journey must call this “rebootstrap with a newly issued session”, not “transparent token rotation”, until a real client test proves an automatic re-acquisition policy.

### Cancellation/disconnect and race injection

- The page handler owns `DirectoryEventPageHttpControl`; a request drop cancels it unless the response became response-owned ([`🚀️bin.rs:4020-4070`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L4020)). The five-second route timeout also cancels it ([`🚀️bin.rs:4166-4188`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L4166)).
- `TestLiveGate` exposes a post-read/pre-revalidation page fence and records its request-owned control ([`🚀️bin.rs:471-537`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L471)). The focused law revokes between read and response, then separately aborts the requester and waits for server cancellation ([`🚀️bin.rs:9068-9106`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L9068)).
- For a client-disconnect-at-the-wire reference, the canonical-pair test drops its `TcpStream` at an admission gate and checks control cancellation becomes terminal without progress movement ([`🚀️bin.rs:6890-6910`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L6890)). No event-page process test currently performs that literal TCP drop.
- A global directory socket registers a live lease and its `Drop` unregisters it ([`🚀️bin.rs:1417-1428`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L1417)); `handle_directory_ws_v1` exits on an incoming close, EOF, send failure, or error ([`🚀️bin.rs:4543-4548`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L4543)).

## Smallest Executable Packet After Both Owners Land

Keep the existing page route, envelope bootstrap, socket grant, and process runner. Do **not** create a second fake hub, an alternate event protocol, a test-only cursor type, or an in-memory replacement for the Home owner.

1. Extend the existing `directory-event-page-v1-check process` implementation in [`🌎️hub/📦️packages/🦀️rust/📜️script.ts`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5222), retaining `startLocalHub` and its ticket-local SQLite root. It is already the cross-platform process supervisor and is registered with Nx and launch configuration.
2. Add one bounded, test-only **projection probe** mode to the WGPU entrypoint beside `--credential-probe` and `--socket-grant-probe` ([`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs:58-88`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs#L58)). Launch it via the existing `deliverNativeCredentialEnvelope` / fd3 path ([`🌎️hub/📦️packages/🦀️rust/📜️script.ts:892-940`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L892)); do not put a bearer in argv or environment.
3. The native probe must use the landed actual WGPU retained-Home integration, not instantiate a parallel `DirectoryEventPageBootstrapV1`. It reports only non-secret observation records: `pageAccepted { epoch, receipt, after, through }`, `socketDialed { since }`, `wakeObserved`, `retryFrom { after }`, `rebootstrapped { epoch, after }`, and `closed`. The parent validates ordering and never logs the credential.
4. Add the matching browser process journey by reusing the established real hub + headless Chromium architecture from `proveAdminLiveJourney` ([`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2226-2267`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L2226)). It must observe the landed worker/Home owner through a narrowly scoped test surface, not replace it with raw `fetch` calls. This is a separate target-runtime variant of the same fixture/laws, not a new protocol.
5. Use real A and B credentials. A creates a private Studio and adds B as an `author`; the route permits B’s rename as an author ([`🚀️bin.rs:3814-3846`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/🚀️bin.rs#L3814)). A is the probe projection; B is the real independently authenticated producer.

### One deterministic journey, per target runtime

| Step | Coordinator action | Required observation/law |
|---|---|---|
| 1. Bootstrap A | Start real SQLite hub; mint A and B; launch target A through its real credential boundary. A consumes all pages and applies each to retained Home. | Every accepted page is canonical and its exact ACK matches epoch, binding, generation, `through`, and receipt. Only ACK moves A’s committed frontier. |
| 2. Establish live | A reports final `pageAccepted` then `socketDialed { since: committedThrough }`. | No socket is opened from `0` or from any observed event sequence. |
| 3. Gap injection | Only after A reports that dial cursor, have B `rename-space` through `POST /directory/commands`; intentionally release the dial after that command commits. | A’s first global-socket replay contains the B event at most once; it is only a dirty wakeup. This deterministically covers response/ACK-to-dial loss without a timing sleep. |
| 4. Fetch/ACK once | Let A process the dirty wake and fetch from its last acknowledged cursor. | The page includes the injected sequence once; retained Home observes it once; A ACKs the returned receipt and advances to that page’s `through`. A second wake before the ACK must not produce a second fetch. |
| 5. Socket disconnect/retry | Abruptly close A’s socket transport after the ACK, then have B append one further visible rename before redial. | A reconnects with exactly the last ACKed `since`, obtains the later event once, and the resulting retained Home sequence/id set is strictly increasing with no duplicate. |
| 6. Page cancellation | Stall the page read at the existing page fence (native focused law) or terminate the target operation while its request is pending. | The target reports cancel/close; there is no Home apply or ACK after close, no socket dial, and the server request becomes inactive. A fresh target may restart from its last durable ACK only. |
| 7. Session rebootstrap | Open a clean A socket, then revoke A with `DELETE /auth/sessions/me`; wait for `4401`; mint a new A session and restart its owner epoch. | Old A gets no frame after winning revocation. New A starts a distinct epoch at `after=0` unless the landed owner has a separately proven persisted projection contract; its page binding/generation/receipt are from the new session, never ACKed across generations. |
| 8. Hub restart | Stop the first hub child after a final A ACK; start the second against the same SQLite root and mint a fresh A session. | A’s post-restart page preserves durable visible history, has a new session binding, and emits no duplicate Home application for a sequence already represented by the reconstructed/proven retained frontier. |

### Exact acceptance laws

The new fixture and each runtime adapter must make these assertions observable, not infer them from console text:

1. **Receipt-ACK law:** accepted page bytes parse canonically; `ACK = (epoch, binding, generation, through, receipt)` equals the retained page exactly. A modified receipt, epoch, generation, binding, or `through` leaves the frontier unchanged and retries from the same `after`.
2. **Frontier law:** `committedFrontier` changes only after Home ACK. Socket events/heartbeats and repeated dirty wakes do not change it. This matches `stream_acknowledged` and its focused test ([`🔌️client/🦀️.rs:2021-2036`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs#L2021)).
3. **Gap law:** an event committed after final page ACK and before/while the socket opens is visible either in that socket’s replay or in the next page, and exactly once in Home’s sequence/id set.
4. **Retry law:** a rejected/failed page repeats from the prior ACKed cursor; no partial Home projection becomes visible. A closed/cancelled owner accepts neither late response nor late ACK.
5. **Disconnect law:** socket transport loss backs off/retries at the ACKed cursor; the next visible event is applied once. An authority `4401` is terminal for the old owner, not an ordinary reconnect.
6. **Rebootstrap law:** old-generation receipt/ACK cannot affect the new epoch. A new session uses the current binding/generation and rehydrates only through authenticated pages.
7. **Privacy law:** B’s unrelated private events never appear in A’s page, socket frame, Home state, probe record, or test diagnostics.
8. **No-duplicates law:** across initial pages, replay, dirty bursts, reconnect, rebootstrap, and restart, `HomeAppliedEventIds` has no repeat and accepted visible sequences are strictly increasing within each page/continuation chain. Do not equate a raw global sequence gap with a duplicate: private rows may advance `through` without being visible.

## Honest Nonclaims and Exit Criteria

- The current `directory-event-page-v1-process-check` is a real hub-process / driver-process proof, but not a WGPU process probe, browser process probe, or two app-process proof. It contains no WebSocket path and cannot prove the above laws.
- The in-bin WebSocket tests are genuine TCP/WebSocket transport tests, but their server and client tasks share one Rust process. They are race-seam evidence, not process-isolation evidence.
- The current process proof’s B revocation proves stale REST bearer denial. It does not prove a live app’s reauthentication or continuation after session-generation change.
- There is no basis to claim retained projection persistence across hub restart. The current route process test proves server SQLite durability and a fresh session’s read only.
- Do not call this P0 GREEN until both target-runtime variants use their actual retained Home ACK owners and all eight acceptance laws pass through their registered Nx/launch gates. The existing source and hub tests remain necessary but are insufficient.

## Files to Touch in the Follow-up (Bounded)

1. Existing hub process runner: [`🌎️hub/📦️packages/🦀️rust/📜️script.ts`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts) — extend, do not create another runner.
2. Existing WGPU test entrypoint/probe seam: [`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs) — add only a test-mode observer after the actual owner lands.
3. The landed WGPU and browser retained-Home owner tests — expose non-secret fixture observations at their owner boundary; do not add a second `DirectoryEventPageBootstrapV1` owner.
4. Existing project/launch registration only if a distinct named journey target is added: [`🌎️hub/📦️packages/🦀️rust/📋️project.json`](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📋️project.json) and [`.vscode/🧩️launch.seed.jsonc`](../../../../../../../../../.vscode/🧩️launch.seed.jsonc). Reuse the ticket-local `SEMIO_TEST_ARTIFACT_DIR` convention; clean generated artifacts after the gate.

