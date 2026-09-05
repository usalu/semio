# Terra P0 — Space Administration and Two-User Journey

## Decision

**GREEN components; RED composite acceptance journey.** The hub already has the required
administration intents, SQLite directory/membership facts, document sockets, document fanout,
bounded presence, membership-fenced live closure, and durable document WAL. What is absent is
one launched, two-user, same-data-dir acceptance that proves those facts compose in order:
administrator → space/membership → Author A mutation → peer B directory/document/presence
observation → B membership removal → restart with durable facts only.

The smallest P0 packet is test-only. It adds a schema-first fixture and one bounded Bun process
journey around the existing hub binary; it does not alter GIS, ShellHost, WGPU bootstrap,
directory-event-page, or Stdio product paths.

This was a read-only source audit: no existing test command was executed, so named tests below
are evidence of current coverage and the required launch packet, not a new passing result.

## Existing Seams and Evidence

| Concern | Existing owner and exact seam | Assessment |
| --- | --- | --- |
| Strict admin vocabulary | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`: `AdminIntentV1` includes `CreateSpace`, `UpsertSpaceMember`, `RemoveSpaceMember`, `RevokeUserSessions`, and `KickConnection`; `DirectoryCommand` includes `AnnounceDocument` | **GREEN.** Principal, session, and actor fields are server-derived by the hub, rather than client fields in the intent. |
| Admin HTTP routes | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`: `router` mounts `/admin/api/overview`, `/spaces`, `/spaces/{id}`, `/users`, `/connections`, `/documents`, `/events`, `/intents`, operation status/cancel, and `/admin` assets | **GREEN.** `admin_intents` validates strict JSON, records an accepted audit fact, re-authenticates, executes, and records a terminal fact. |
| Admin space/member execution | `bin.rs`: `admin_directory_command`, `execute_admin_intent`, `execute_directory_command_fenced`; `AdminIntentV1::RemoveSpaceMember` maps to `DirectoryCommand::RemoveMember` | **GREEN.** Create/upsert/remove use `DirectoryService`; a removal holds the exact membership gate, commits the directory event, then invalidates the affected socket membership binding. |
| Browser admin surface | `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🏛️SpacesPage/🟦️.tsx`: create, visibility, `upsertSpaceMember`, and `removeSpaceMember`; `.../🔑️AdminSession/🟦️.tsx`: `AdminClient` and `AdminSessionProvider` | **GREEN.** The UI uses same-origin fetch/cookie transport; its client does not own a bearer credential. |
| Admin relay | `🌎️hub/📦️packages/🦀️rust/📜️script.ts`: `startLocalAdminRelay`, `adminRelayApiPath`, `proveAdminLiveJourney`; `AdminLiveJourneyCheckScript` | **GREEN for development.** A loopback relay exchanges a one-use bootstrap proof for an HttpOnly cookie and limits routes/body sizes. Existing browser journey proves SQLite overview/create/read and EN/DE navigation, but only one administrator. |
| Development two-user issuance | `hub/📜️script.ts`: `startLocalHub`, `issueLocalCredential`, `waitForReadiness`, `finishLocalHub`; `🌎️hub/🚀️local-bootstrap/🦀️.rs`: local bootstrap issuance creates/gets a user from identity subject | **GREEN harness.** A single spawned hub can receive up to eight inherited local profiles. `GET /auth/sessions/me` returns the server-derived A/B IDs and email required by member-upsert. |
| Directory storage | `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`: `SqliteDirectory::connect`, schema tables `hub_space`, `hub_space_membership`, `hub_document_descriptor`, `hub_auth_session`, `hub_sync_session`, `hub_directory_event`, and `append_events` with an immediate SQLite transaction | **GREEN, SQLite.** Membership/descriptor/event facts are durable and projected atomically inside the directory database. |
| Directory order/live fanout | `🌎️hub/📇️directory/🦀️.rs`: `DirectoryService::{execute,append_and_publish_locked,publish_persisted_locked,subscribe}` | **GREEN.** One writer lock covers durable append and event publication; subscribers replay then consume live directory messages. |
| Directory visibility sockets | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`: `/directory/socket-grants`, `/directory/socket/v1`, scoped `/directory/spaces/{space_id}/documents/{document_id}/socket-grants` and `/socket/v1`; `handle_directory_ws_v1`, `send_socket_directory_message` | **GREEN.** Scoped sockets revalidate membership before replay/live sends, filter scope, and close `4401` on lost authority. |
| Document admission and mutation | `bin.rs`: `/spaces/{space_id}/documents/{id}/socket-grants`, `/socket/v1`, `document_ws_v1`, `socket_live_authority`, `handle_client_frame`; TypeScript twin `🧰️framework/🔨️modules/📡️replication/🟦️.ts`: `encodeClientFrame`, `decodeServerFrame`, `encodePresencePeer` | **GREEN.** One-use socket grants bind the session, membership, scope, and actor. The document loop checks authority before client command admission and before fanout. |
| Presence/connection relay | `bin.rs`: `HubState::{install_presence_slot,refresh_presence,expire_presence_for_live,close_presence_for_live}`, `publish_presence_delta`, document socket loop; directory `DirectoryStreamMessage::{Presence,Connection}` | **GREEN, ephemeral.** Presence is bounded, actor-sorted, scope-filtered, and fanned to the peer document socket plus authorized directory subscribers. `hub_sync_session` provides the admin connection snapshot. |
| Membership revocation | `bin.rs`: `execute_directory_command_fenced` and `SocketBindingKeyV1::Membership`; native laws `scoped_directory_socket_admin_removal_uses_the_same_membership_fence` and `scoped_directory_socket_removal_and_delivery_have_one_total_membership_order` | **GREEN.** A successful remove-member invalidates pending/live document and scoped-directory grants under one membership gate; a winning removal leaks no later scoped frame. |
| Session revocation | `bin.rs`: `AdminIntentV1::RevokeUserSessions` branch invalidates session bindings and wakes recorded sync sessions; `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`: `revoke_auth_sessions_matching`, `socket_session_binding` | **GREEN but distinct.** Session revocation increments authorization generation and closes live sessions; it is not implicit in membership removal. |
| Document durability | `bin.rs`: `connect_db` opens `{OS_HUB_DATA}/db`; `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs`: `a_document_survives_a_full_database_shutdown_and_reopen_at_the_same_root`; `.../🗿️artifact/🦀️.rs`: `open_replays_the_wal_and_reconstructs_state_and_frontier_identically` | **GREEN underlying durability.** The document WAL replays state/frontier; the hub process does not yet prove it jointly with admin/membership. |
| Restart semantics | `bin.rs::main` invokes `directory.close_all_sync_sessions()` before accepting connections; `HubState` keeps `fanout`, `presence`, `session_kicks`, socket grants, and open-plan ledger in memory; native law `presence_lease_restart_is_empty_and_directory_presence_is_member_only` | **GREEN for deliberate live-state retirement; RED for a single composed proof.** Restart must preserve directory/document facts while clearing presence/live grants/connections. |

## Exact Current Route Journey

```text
admin relay/browser ─POST /admin/api/intents──────────────────────▶ DirectoryService + SQLite
                    create-space, upsert A(author), upsert B(spectator)

Author A ─POST /directory/commands (announce-document)────────────▶ durable descriptor/event
Author A/B ─POST /spaces/{space}/documents/{doc}/socket-grants────▶ one-use scope/member grant
Author A/B ─GET  /spaces/{space}/documents/{doc}/socket/v1────────▶ binary Welcome/Session
Author A ─Commands / Presence─────────────────────────────────────▶ DB WAL + B document fanout
                                                                    ↘ directory Presence/Connection

admin relay/browser ─POST /admin/api/intents (remove-space-member B)▶ event commit + membership fence
                                                                    ↘ B scoped/document sockets close 4401

restart same OS_HUB_DATA ──────────────────────────────────────────▶ SQLite + DB WAL recovered;
                                                                      live grants/presence/connections retired
```

`SocketSubjectV1::revalidate` calls durable `socket_session_binding`; for a scoped/document
audience the binding includes the current role. `execute_directory_command_fenced` serializes
`RemoveMember` against these reads/sends, then calls `SocketGrantLedgerV1::invalidate_binding`.
This is why the journey must prove membership removal separately from a session revoke.

## Current Harnesses and the Gap

- `AdminLiveJourneyCheckScript` is a real local hub + relay + Chromium journey, but its fixture
  `🌎️hub/📇️directory/🧫️fixtures/🚶️admin-live-journey-v1/` only creates and reads one space; it
  neither creates A/B membership nor opens a document, observes presence, revokes a member, or
  restarts.
- `ScopedDirectorySocketCheckScript` and the named `os-hub` laws cover scope substitution,
  membership removal, and total removal/delivery ordering. Its `process` phase is a cargo binary
  test selection, not an independently launched `os-hub` process with two authenticated users.
- `PresenceLeaseCheckScript` exercises bounded native laws; `presence_lease_restart_is_empty...`
  correctly proves that presence is not durable. It is not a two-user launch/restart journey.
- `proveMcpWorkspaceProcess` launches a real hub and a direct child, proves a document connection,
  forced reconnection, and frontier progress, but is one authenticated user and does not use the
  administrator membership lifecycle.

Therefore no existing test can truthfully accept the complete requested sequence. There is no
need to invent a new administration, directory, document, or socket service to close this P0
gap; the missing artifact is the composition test.

## Smallest Bounded Implementation Packet

1. Add `🌎️hub/📇️directory/🧫️fixtures/👥️space-admin-two-user-journey-v1/🧬️.schema.json` and
   `.../🔣️.json`. The schema defines exactly three development profiles (administrator, A, B),
   one private studio space, A=`author`, B=`spectator`, a generic bounded descriptor, one
   pathmap mutation, one presence peer, response/frame/time limits, and expected terminal codes.
   Reject extra roles/users, public visibility, arbitrary provider credentials, unbounded frame
   payloads, and any expected durable presence/connectivity fact.

2. Extend only `🌎️hub/📦️packages/🦀️rust/📜️script.ts` with a
   `SpaceAdminTwoUserJourneyCheckScript` and a `space-admin-two-user-journey-check` router entry.
   Reuse `startLocalHub`, `issueLocalCredential`, `startLocalAdminRelay`, `encodeClientFrame`,
   `decodeServerFrame`, and `encodePresencePeer`; do not fork socket/auth/protocol code. Use an
   external temporary `dataRoot` passed to `startLocalHub({ dataDir })`, so the first run's normal
   cleanup cannot delete the facts required by process two; remove it in the test's finalizer.

3. Register the command in `🌎️hub/📦️packages/🦀️rust/📋️project.json` and the existing ordered
   `.vscode/🧩️launch.seed.jsonc` command catalogue. Both invoke only `bun ./📜️script.ts ...` in
   line with the repository command convention. No product Rust, UI, GIS, WGPU, hub event-page,
   or Stdio source changes are part of this packet.

4. The process body is fixed and bounded:
   - Start one development hub with admin/A/B local profiles and admin subject configuration.
     Obtain A/B server identities through `/auth/sessions/me`; authenticate the administrator
     through the loopback relay/browser cookie, not a browser bearer.
   - Submit `create-space`, `upsert-space-member(A, author)`, and
     `upsert-space-member(B, spectator)` to `/admin/api/intents`; assert terminal durable receipts
     and the server-returned space detail/membership projection.
   - A announces one generic document via `/directory/commands`. B obtains a scoped directory
     socket before document activity, then both A and B exchange document socket grants and send
     exact `SocketHelloV1` matching the durable descriptor's schema/hash.
   - A sends one actor-bound `Commands` frame and one bounded `Presence` frame. Assert A receives
     persisted/applied acknowledgement; B receives exactly the command once, the document-wide
     presence roster containing A, and scoped directory `DocumentAnnounced`, `Connection`, and
     `Presence` messages only for this space/document.
   - Remove B through `remove-space-member`. Require the durable receipt/event before accepting
     either B closure. Both B document and scoped directory sockets must close `4401`, no
     authority-bearing frame may follow, B cannot mint a replacement scoped/document grant, and B
     can no longer read the private space/document. Do not assert that B's unrelated login session
     is revoked.
   - Stop the hub, launch a second hub against the same `dataRoot`, issue fresh envelopes, and
     assert SQLite has the space, A membership, B removal, descriptor, and admin audit; DB has the
     exact post-mutation frontier/value. Assert new B access remains denied while A can reconnect.
     Assert no active sync connection/presence record from process one survives; no old grant is
     reused.

## Acceptance Laws

Schema/native laws:

1. **Fixture parity.** Ajv validates the new fixture and rejects each hostile role/scope/frame/
   durability mutation. Its descriptor, socket hello, command, and presence bytes round-trip via
   the existing TypeScript replication codec; the equivalent Rust `protocol` codec stays covered
   by its existing fixture suite.
2. **Admin receipt law.** Each distinct request ID yields one durable terminal audit receipt;
   reusing the key with a changed intent conflicts. Re-run
   `admin_operation_audit_concurrent_first_writer_is_idempotent_and_first_terminal_wins`.
3. **Membership fence law.** Re-run
   `scoped_directory_socket_admin_removal_uses_the_same_membership_fence` and
   `scoped_directory_socket_removal_and_delivery_have_one_total_membership_order`: removal wins
   with no post-removal private frame, while an already-admitted eligible frame is ordered before
   the terminal close.
4. **Durability boundary law.** Re-run
   `a_document_survives_a_full_database_shutdown_and_reopen_at_the_same_root`,
   `projection_rebuild_preserves_live_credential_invite_and_session_bindings`, and
   `presence_lease_restart_is_empty_and_directory_presence_is_member_only`.

Launched process laws:

1. **Two users, one scope.** B never receives another space/document's directory, command, or
   presence frame; A's command is observed by B exactly once and its acknowledged frontier is
   nondecreasing.
2. **Membership revocation is linearized.** The admin terminal removal receipt precedes B's
   `4401`; B sees no post-removal command/presence/directory authority frame and cannot reacquire
   either scoped-directory or document access.
3. **Restart facts, not leases.** Process two sees the same configured space, members/removal,
   descriptor, audit, and document WAL frontier/value. It sees zero carried presence/live sockets,
   must mint new grants, and B remains denied with a fresh credential.
4. **Bounded cleanup.** All HTTP, socket reads, launch/restart, relay/browser work, and shutdowns
   have deadlines; every capability/relay cookie and temporary directory is retired in `finally`.

## Strict Nonclaims

- This is a development-local, loopback, three-profile journey, not production OIDC, remote
  multi-device testing, or a claim that a browser contains raw session credentials.
- Presence, fanout channels, socket/open-plan ledgers, live grants, and connection wakeups are
  intentionally process-local. Restart persistence means directory/document facts only.
- Membership removal is scope revocation, not session revocation. A separate explicit
  `RevokeUserSessions` intent is required to invalidate B's entire login/session family.
- Directory SQLite and document DB/WAL are separate stores. The journey proves their resulting
  durable facts, not a distributed atomic transaction spanning admin membership and a document
  mutation.
- No browser ShellHost, WGPU, GIS typed binding/group change, directory-event-page behavior,
  Stdio taxonomy, real external provider, or PostgreSQL/Neo4j multi-process claim is added.
