# Terra Space Collaboration Repair Blueprint

Read-only, current-tree implementation blueprint — 2026-09-04. This turns the five REDs in [the collaboration audit](📓️terra-space-collaboration-end-to-end-audit.md) into schema-first, independently ownable repair lanes. It did not run a build, test, launch profile, or browser. File/line references are current-source evidence, not runtime proof.

## Scope and ordering

The active D1 document-open receipt → `SocketGrant` work remains the transport authority. Do **not** fork its issuer, receipt ledger, exchange route, or readiness semantics. The hub already makes a descriptor lookup, catalog selection, subject revalidation, revision fence, and bounded plan at [bin.rs](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1986)-[2099], and browser D1 derives its request from the scoped worker binding at [backbone-worker.ts](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:418)-[455].

Lanes 1–4 can be implemented and tested while D1 settles because they do not touch that ledger. Lane 5 starts only after D1's server exchange and browser binding interfaces are stable; it must consume that single path rather than create a space-specific socket or plan route.

| Order | Sol lane | Depends on | Delivers |
| --- | --- | --- | --- |
| 1 | **Public directory projection** | none | Public discovery with no member identity or activity telemetry |
| 2 | **Serialized redemption** | none | One persisted-event serialization/publication order |
| 3 | **Directory liveness** | lane 2's service helper | Bounded, truthful reconnect progress heartbeats |
| 4 | **Presence lease** | none | Server-owned stale-presence expiry without durable cursor history |
| 5 | **Descriptor-bound open relay** | D1 stable; lane 1 public DTOs | Artifact row/create/open reaches only the authenticated D1 document binding |

All new public contracts belong first in the paired Rust/TypeScript directory schema, then in one JSON fixture plus an independent Bun/AJV-or-WebCrypto oracle. Rust and browser tests may consume the fixture but must not be its only validator. Do not add a compatibility form: this is a greenfield protocol cutover.

## Lane 1 — Public directory projection boundary

### Current defect and owning symbols

`get_directory_space` grants an anonymous or nonmember caller access when a space is public, then always serializes `space.members.clone()` at [bin.rs](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3598)-[3611]. `MemberView` contains `user_id`, `email`, `display_name`, and role at [directory schema](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:418)-[426]. The same flattened `SpaceView` exposes `owner_user_id`, caller role, and `active_connections` at [schema](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:398)-[416].

This also cannot be considered a REST-only repair: public `DirectoryEvent` payloads include actor IDs, space owner IDs, member user IDs and a user-created email at [schema](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:145)-[177], while `event_visible` presently admits every event for a public space at [bin.rs](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3645)-[3659]. This conflicts with the deliberately stricter live-telemetry rule at [bin.rs](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3662)-[3676].

### Decision and schema

Public means *discoverable metadata*, not a weak membership role. Replace the flattened detail response with an explicit discriminated response, mirrored in `🧬️schema/🦀️.rs`, `🧬️schema/🟦️.ts`, and `🧬️schema/🔣️.json`:

```text
DirectorySpaceDetailV1 =
  { access: "public", space: PublicSpaceViewV1, documents: PublicDocumentCatalogEntryV1[] }
| { access: "member", space: MemberSpaceViewV1, members: MemberView[], documents: DocumentView[] }
| { access: "author", space: MemberSpaceViewV1, members: MemberView[], documents: DocumentView[], invites: InviteView[] }

PublicSpaceViewV1 = { id, name, kind, visibility, member_count, document_count,
                      created_at_ms, updated_at_ms }
PublicDocumentCatalogEntryV1 = { document_id, artifact_kind, artifact_schema,
                                 owner: DocumentOwner, pack_schema_hash }
```

`PublicSpaceViewV1` deliberately excludes `owner_user_id`, `role`, `active_connections`, members, invitations, connection/presence data, HLC/actor identity, and event sequence. The public document catalog carries only identity needed to show a document and choose a renderer; it excludes bootstrap/current frontier and checkpoint metadata, avoiding an activity/currentness claim. **Discoverability does not grant opening or write authority.** The existing D1 endpoint remains the sole authority and applies current subject/scope checks.

Do not make `members: []` or `invites: []` the public representation: a distinct `access` variant makes accidental identity expansion compile/type fail. `DirectoryClient.space` currently claims a flattened `DirectorySpaceDetail` at [os TS](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:3954)-[3959] and [4130](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:4130)-[4136]; change that client and every consumer to narrow the discriminator before accessing member-only fields.

For `/directory/events` and `/directory/socket/v1`, a public caller must receive an explicit `PublicDirectoryEventV1` projection limited to public space lifecycle/document-catalog changes, or no event stream at all. Never serialize or “redact in place” a `DirectoryEvent`: its `actor`, `user_id`, and variant body make omission fragile. Members keep the existing authenticated `DirectoryEvent` stream; connection/presence remains member-only as it is today. This preserves event sourcing internally while establishing a separate external read model.

### Boundaries and invariants

- `get_directory_spaces`, `get_directory_space`, `get_directory_events`, `event_visible`, `visibility_filter_events`, `socket_directory_message_visible`, and `directory_message_visible` in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` must all dispatch by the same access decision. Do not calculate it separately in REST and WebSocket paths.
- Make the decision after resolving the active session and membership once per response/send. A missing, expired, or revoked session is `public`, never `member`; a private nonmember remains 404.
- Build the public DTO from `DirectorySpace`/descriptor data, not by serializing `SpaceView` or `DocumentView` and deleting fields. `space_view` currently fills live counts and role at [bin.rs](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3483)-[3491], so it is member-detail-only.
- This lane needs no new durable event. It is an authorization-bound read projection and must not alter the durable command/event vocabulary at [schema](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:199)-[213].

### Neutral fixture and focused proof

Add `🌎️hub/📇️directory/🧪️fixtures/🧬️public-space-detail-v1/` with positive anonymous/public-nonmember/member/author vectors and negatives for every forbidden key (`ownerUserId`, `role`, `activeConnections`, `members`, `invites`, `email`, `userId`, `displayName`, `actor`, `hlc`, `headSeq`, `commitSeq`, `epoch`). The standalone oracle must reject a public event that contains any raw `DirectoryEvent` shape.

Add exact Rust route laws for anonymous public detail/catalog, private nonmember 404, member/author detail split, public REST event projection, and public socket denial of member telemetry/raw events. Add focused TypeScript tests for discriminated narrowing and unknown-field rejection. Register `os-hub:space-public-boundary-check` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts` and launch seed: oracle → exact-one preflight/list → exact Rust laws → targeted client test → hub all-feature check. No `--exact` invocation may run before proving one FQN was selected.

## Lane 2 — Directory writer linearization for invite redemption

### Current defect and decision

`DirectoryService` promises one serialized writer at [directory](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:1516)-[1526]. `execute` keeps the `write` mutex through `decide` and `append_events` but drops it before fanout ([directory](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:1599)-[1610]). `redeem_invite` is worse: it stamps an HLC event under the guard, drops it, then appends and sends at [directory](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:1665)-[1695]. A concurrent command can persist/publish a later HLC and then the redeem persists/publishes its older HLC.

Make `DirectoryService.write` the single linearization boundary for every **persisted** directory event from decision through backend transaction commit and ordered broadcast enqueue. Refactor the repeated entry points (`execute`, `execute_create_space_with_id`, `execute_artifact_authority`, `publish_reserved_artifact_checkpoint`, `redeem_invite`) through a private, non-async-after-commit helper that:

1. retains `MutexGuard<HubClock>` while deciding/stamping and awaiting the atomic `append_*` transaction;
2. sends each resulting `DirectoryStreamMessage::Event` in increasing returned `seq` while still holding the guard; and
3. only then releases the guard and returns the persisted rows.

`broadcast::Sender::send` is synchronous/nonblocking; do not await or perform network/directory reads while holding the guard beyond the existing decision/transaction work. Connection, presence, and heartbeat messages are explicitly non-persisted and may use `publish` outside this boundary; they must never establish ordering claims relative to durable event sequence.

The durable schema/events do not change: `InviteRedeemed` remains an existing event ([directory schema](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:165)-[172]). The backend contract already supplies dense transactional sequence/projection semantics ([directory](../../../../../../../../../🌎️hub/📇️directory/🦀️.rs:2105)-[2118]); the repair restores service-level HLC/append/publication ordering.

### Required laws and gate

Use a deterministic test gate between redeem’s event construction and append. Start redemption A, then command B; prove B cannot decide/append/publish until A has appended and its event has entered the broadcast sequence. Reverse the gate to prove normal command serialization too. Assert dense `seq`, nondecreasing HLC in receiver order, one `InviteRedeemed`, and correct membership projection after restart/replay. A backend failure must release the guard, emit no event, and leave invite/membership unconsumed.

Put concurrent schedule and expected receiver sequence in `🌎️hub/📇️directory/🧪️fixtures/🧬️directory-writer-order-v1/`, with a Bun oracle that checks the schedule rather than sharing Rust decision code. Register `os-hub:directory-writer-order-check` with exact-one laws and an all-feature hub check. This gate is independent of D1 and lane 1.

## Lane 3 — Directory stream liveness and progress

### Current defect and schema use

`DirectoryStreamMessage::Heartbeat { head_seq }` is already schema/client surface at [directory schema](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:994)-[1018]. The browser advances `lastSeq` on it at [os TS](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:4172)-[4229]. Current `handle_directory_ws_v1` subscribes, replays, and selects authorization/inbound/live only ([bin.rs](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3792)-[3918]); there is no production heartbeat publisher.

Do **not** publish a global heartbeat with `DirectoryService.publish`: it consumes broadcast capacity, can cause a lagged receiver, and gives no per-socket replay ordering guarantee. Keep the existing heartbeat schema, but add a per-directory-socket interval after replay completes. At each tick, use the same authorization/send funnel (`send_socket_directory_message`) and obtain a bounded authoritative head:

```text
head = directory.head_seq() under a 2 s timeout
send Heartbeat { head_seq: max(last_replayed, head) }
failure -> one 1013 authorization-unavailable close
```

`last_replayed` advances only after a successfully sent event, as it already does at [bin.rs](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3848)-[3892]. Never heartbeat before replay. A heartbeat head may be ahead of the last visible event only after the socket had an opportunity to receive the filtered stream; if an invisible event makes the global head higher, that is harmless only if client `lastSeq` is not used as an authorization bypass. To make this unambiguous, the safer contract is to expose `observed_head_seq` as liveness-only and leave reconnect cursor advancement to actual `Event.seq`; otherwise store and advance a per-caller *visible* head. The current client advances `lastSeq` on heartbeat, so **the repair must choose the latter or stop advancing `lastSeq` on heartbeat.** Recommended smallest correct cutover: heartbeats prove liveness only; change the TS client to retain `lastSeq` from events and track `lastHeartbeatHead` separately. This avoids global/invisible sequence holes altogether.

Expiry/revocation is checked by the existing send funnel; a heartbeat must be no exception (`Heartbeat` visibility is currently true only after caller revalidation at [bin.rs](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3665)-[3675] and [3699](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3699)-[3719]). Use a server interval such as 15 seconds, bounded response send timeout already used by the funnel, and no client-supplied clock.

### Proof and gate

Fixture `directory-heartbeat-v1` covers: replay event 7 then head 9 heartbeat leaves resume cursor 7; reconnect starts at 7; heartbeat is emitted only after replay; revoked/expired user gets 4401 rather than a heartbeat; head read timeout gives one 1013; and an idle healthy socket receives periodic progress. The Bun oracle validates cursor semantics independent of the Rust stream loop. Add exact Rust websocket laws with a controlled directory head/clock. Register `os-hub:directory-liveness-check` (oracle, exact-one Rust, targeted `DirectoryClient.stream` Vitest, all-feature check). It may follow lane 2's helper but otherwise does not depend on document transport.

## Lane 4 — Server-owned presence lease

### Current defect and model

`PresenceSession` has only `surface`, `user_id`, `color`, and opaque `peer`, and it is removed only when its document handler exits ([bin.rs](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:413)-[423], [3325](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:3325)-[3334]). A valid `ClientFrame::Presence` simply replaces `peer` and immediately fans out ([bin.rs](../../../../../../../../../🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2916)-[2924]). Browser sends every 5 seconds ([Shell helpers](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️.tsx:278)-[300]); native has a coalescing producer. Neither is a server-owned expiry law.

Keep presence ephemeral—**no `DirectoryCommand` or durable `DirectoryEvent` is added**. Replace the flat `(scope, actor) -> PresenceSession` snapshot pattern with one per-document `PresenceRoster` held behind an `Arc<std::sync::Mutex<...>>` (the outer sharded lookup may remain). The roster lock is the linearization boundary for heartbeat, expiry, clean disconnect, and the exact snapshots used for both fanouts. Its entries are:

```text
PresenceEntry { surface, user_id, color, peer: Option<Vec<u8>>, deadline: Option<MonotonicDeadline> }
```

Add `PresenceClock` to `HubState`: production supplies boot-relative monotonic elapsed time; tests supply a manual clock. Use `PRESENCE_LEASE_MS = 15_000` (three current browser intervals) as a server constant. Client time never participates. A valid, already-authorized `ClientFrame::Presence` sets `peer` and `deadline = now + lease`; ping, malformed frames, unauthorized frames, or merely sending a socket hello must not extend it. On expiry set that connection’s `peer = None` and `deadline = None`; do not tear down a live socket or free its color merely because it is idle. A later valid presence frame for that same connection reinstates it. Handler exit removes the entry and releases color as today.

On every existing document authorization tick, after successful current authorization, call `expire(now)` for its document roster. The winning lock holder returns at most one new peer/actor snapshot; it drops the roster lock before sending `ServerFrame::Presence` and `DirectoryStreamMessage::Presence`. Other handlers see no change and send nothing. Thus a heartbeat exactly at the deadline and the reaper have one order: whichever acquires the roster lock first; there is never a duplicate roster snapshot or a stale peer after the later operation. No async call or websocket send is permitted while holding this lock.

### Fixture, law, and gate

Fixture `presence-lease-v1` must describe two active actors, one refresh, controlled time advance, expiry, late reappearance, clean close, and exactly one fanout per effective roster change. Exact Rust integration law uses two actual document sockets plus the manual `PresenceClock`; it proves an unresponsive peer disappears without TCP close, an authorized refreshed peer remains, a stale entry can reappear only from its own valid frame, revocation never publishes telemetry, and reconnect cannot revive a removed old connection. Target the existing browser/native coalescer tests only to prove cadence; they are not the lease proof.

Register `os-hub:presence-lease-check` with neutral oracle → exact-one route law → browser worker presence test → native `ArtifactHost` producer law → all-feature hub check. Extend `@semio-tech/framework-os-dev:collab-e2e` only after lane 5 removes its current document-open vacuity.

## Lane 5 — Descriptor-bound artifact open relay

### Current defect and required cutover

The Space plugin emits `ReplayShellCommand { "os.open-artifact", documentId, spaceId, schema }` for a newly minted row at [create-artifact](../../../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs:43)-[45], but the app protocol is only `{ seq, artifact_ref, role, plugin_id, app_id }` ([channel](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1569)-[1578], [2130](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2235)-[2254]). `relay_open_artifact` consequently reconstructs an effect without scope/schema ([plugin host](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30393)-[30410]). The shell resolver can represent optional values ([os TS](../../../../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts:31)-[120]), but `ShellHost` attaches only when it gets a complete `(documentId, schema)` ([ShellHost](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️.tsx:3170)-[3181]). The registered two-user script explicitly records this as the step-3/4 cause ([collab script](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:3001)-[3052]).

Do not treat a plugin-provided `spaceId/documentId/schema` as socket authority. Add a required, tagged `OpenArtifactTargetV1` to the shared `AppCommand::OpenArtifact` channel and its Rust/TS/wire fixtures:

```text
OpenArtifactTargetV1 = Local
                     | Directory { scope: DocumentScope, artifact_schema: String }
```

The effect relay preserves that target byte-for-byte. At the shell boundary, `Directory` may attach only after an authenticated directory projection/lookup proves a matching `DocumentDescriptor` for **the complete scope and artifact schema**. The worker then performs the existing D1 issue/exchange/WebSocket flow; it never accepts a plugin-supplied socket URL, receipt, capability, actor, descriptor digest, or frontier. D1 remains the final anti-TOCTOU authorization/revalidation check.

Creation needs an explicit preceding durable declaration: an author creates/obtains the full `DocumentDescriptor`, sends `DirectoryCommand::AnnounceDocument`, waits for successful directory event/projection visibility, then emits/acts on `OpenArtifactTargetV1::Directory`. The existing command vocabulary already supports that declaration ([directory schema](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:199)-[213]); do not open a locally minted row as if it were a hub document. If declaration fails/cancels, no app session, D1 request, socket, or index row claiming a remotely openable document may be published. This makes row visibility, descriptor identity, and actual D1 scope one transactionally ordered user journey.

This is a full greenfield wire cutover: update the codec encode/decode and its fixed app-command fixture, `relay_open_artifact`, the plugin host dispatcher at [plugin host](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31443)-[31449], the run-host branch at [run host](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1999)-[2035], `ResolvedArtifactOpeningRelay`, `ShellHost`, and the WGPU equivalent. There must be no optional/legacy “partial document ref” path for a directory target; local targets remain local by an explicit tag.

### Fixture and gates

Create `artifact-open-target-v1` fixtures with local, valid directory, missing descriptor, mismatched schema, private other-space, descriptor-announcement race, cancelled announcement, forged receipt/URL fields, and stale descriptor negatives. A language-neutral oracle validates only structural target rules; Rust and TS prove their own codec/host behaviors against it.

Extend the existing `os-hub:browser-document-open-check` only after D1 is green, and add a lightweight `@semio-tech/framework-os` app-channel/opening fixture target for the cross-language codec. The final `@semio-tech/framework-os-dev:collab-e2e --skip-nx-cache` must run two distinct authenticated users and prove: created row declaration, D1-bound open by user 1, row-open by user 2, one durable mutation visible after reconnect, spectator write denial, and no local fallback. Its existing source comment must be removed only when the actual flow passes, not merely because a unit relay fixture does.

## Acceptance matrix

| RED | Required terminal evidence | Not sufficient |
| --- | --- | --- |
| Public detail | neutral projection oracle + anonymous/member/authored REST/socket negatives | hiding members only in React |
| Redemption order | controlled concurrent Rust law proving append/broadcast order | dense backend `seq` alone |
| Heartbeat | controlled socket replay/idle/revoke/cursor law plus client cursor test | schema variant or a test-only `publish(Heartbeat)` |
| Presence | manual-clock two-socket expiry/reappear/reconnect law | client heartbeat producer cadence |
| Open relay | channel fixture + descriptor declaration/lookup + D1-backed two-user process gate | resolving a `documentId` string in the UI |

All focused scripts must use exact-one discovery before exact execution, be registered through the relevant `📜️script.ts`, and receive a launch-seed entry. The existing launch registrations show the expected pattern for document-open at [.vscode/launch.json](../../../../../../../../../.vscode/launch.json:4444)-[4463] and collaboration E2E at [.vscode/launch.json](../../../../../../../../../.vscode/launch.json:5576)-[5584]. No acceptance claim should be made until uncached terminal output proves the specified gate; this blueprint supplies no runtime evidence.
