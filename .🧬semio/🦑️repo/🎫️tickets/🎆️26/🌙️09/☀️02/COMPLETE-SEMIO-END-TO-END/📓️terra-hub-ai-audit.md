# Hub, Spaces, Administration, and AI-over-Map Audit

Date: 2026-09-03  
Scope: read-only audit for `26/09/02/COMPLETE-SEMIO-END-TO-END`, goal `🎯r2603`.

## Result

The repository has a substantial, real collaboration foundation: a Rust Axum hub, independently selectable document and directory storage, append-only directory events with rebuildable projections, binary document sync, reconnecting browser-worker transports, and a bilingual admin SPA.  It is not an end-to-end safe or complete multi-user/AI-map system yet.

The immediate blockers are authorization isolation, MCP session/authority binding, and the absent MCP inference execution path.  The default high-level tests also produce false-green outcomes: hub E2E is skipped unless an environment flag is set, MCP tests skip the real-binary suites when no binary is present, and `os-hub-admin:test-quick` selects no test files.

This report is based on the current shared tree; it does not attribute concurrent edits to a particular author.

## Constraints and audit method

- Read `/Users/ueli/Documents/semio/AGENTS.md` in full.  The only relevant nested instruction file is `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/AGENTS.md`; it adds only GIS metadata.  There is no nested `AGENTS.md` in the hub, directory, MCP, or space trees.
- The configured repository-MCP ticket tools were not callable in this audit session.  The already-open umbrella ticket supplied by the coordinator was used as the sole report location.
- Production code was not modified.  This file is the only audit artifact written.
- Read-only commands used `rg`, `find`, `sed`, `bun nx show project`, and bounded `bun nx` test targets.  A real hub E2E build was started with `HUB_E2E=1`; it remains queued behind the shared Cargo target lock at the time of writing, so it is not presented as a pass or failure.

## Runtime entry points

| Slice | Registered launch target / runnable command | Evidence |
| --- | --- | --- |
| Hub plus admin assets | `🛠️dev🗄️os-hub` → `bun nx run os-hub:dev`; port `8787`, data at `.🧬semio/🌐hub/hub-dev/`, browser `/admin` | `/Users/ueli/Documents/semio/.vscode/launch.json:4342-4359`; `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts` |
| Admin SPA dev server | `🛠️dev🗄️os-hub🛡️admin` → `bun nx run os-hub-admin:dev`; port `8790`, proxying the hub | `/Users/ueli/Documents/semio/.vscode/launch.json:4362-4378`; `/Users/ueli/Documents/semio/🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts` |
| Two-user hub shell fixture | `🧭️compound🖥️s👥️users🗄️os-hub` starts hub plus React users on `6072` and `6073` | `/Users/ueli/Documents/semio/.vscode/launch.json:7590-7596` |
| Base shell + hub | `🧭️compound🖥️s⚛️react🗄️os-hub` | `/Users/ueli/Documents/semio/.vscode/launch.json:7581-7587` |
| GIS 2D | `🛠️dev🌐️gis📍️2d⚛️react` → `bun ./📜️script.ts dev gis 2d`, port `6040`; WGPU and native variants are also registered | `/Users/ueli/Documents/semio/.vscode/launch.json` GIS entries around `2460` and `6040` |
| MCP stdio / HTTP | `🛠️dev🌉️os-mcp🧵️stdio` and `🛠️dev🌉️os-mcp🌐️http` → root `📜️script.ts dev mcp …` | `/Users/ueli/Documents/semio/.vscode/launch.json:4381-4400`; `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts` |

There is no registered compound target that starts a GIS client, authenticated hub, and MCP agent together.  Such a target is required for the P4 acceptance journey, but should be added only with its real test harness rather than as a standalone launch convenience.

## What is implemented

### Hub, storage, and document synchronization

- `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs` is the live Axum server.  It exposes authenticated directory REST/WS APIs; blob `GET`/`PUT`/`HEAD`; document frontier status; document sharing; binary document WebSockets; extensions; and operator APIs.
- `connect_db` selects `fs` (default), SQLite, PostgreSQL, or Neo4j via `OS_HUB_STORAGE_BACKEND`; `connect_directory` independently selects SQLite (default), PostgreSQL, or Neo4j via `OS_HUB_DIRECTORY_BACKEND`.  SQLite paths are under `OS_HUB_DATA`; remote choices require their connection environment variables.  See `📦️bin.rs:1629-1727`.
- Documents are scoped in the hub by `scope_key(space_id, document_id)`, then lazily created in the database on first authorized access.  The database namespace itself is flat, so space scoping is a hub convention.  See `📦️bin.rs:101-110,265-278`.
- The document WS starts with binary `ClientFrame::Hello`, resolves authorization, pins a nonzero schema hash in memory, performs DB hello/welcome/catch-up, creates a per-connection `SecurityGate`, and persists command batches with Fsync.  Commands and previews fan out over a per-document `broadcast::channel(256)`.  See `📦️bin.rs:705-902` and `590-680`.
- Browser worker hub handling is real: pending command batches return to an outbox on an unacknowledged socket close, reconnect uses full jitter from 500 ms to 30 s, and Welcome flushes the outbox.  See `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:540-749` and `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3746-3765`.

### Directory CQRS/event sourcing and space administration

- `/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs` supplies the directory domain: users, spaces (`atelier`, `studio`, `archive`), visibility, member roles, invites, browser auth sessions, realtime sync-session records, directory events, and a typed `HubDirectory` port.
- A serialized `DirectoryService` executes `decide → append_events transaction → projection update → broadcast`.  Event sequence numbers are dense and `rebuild_projections` truncates and replays the read model.  See `🦀️.rs:257-295,433-479,509-588`.
- Space laws are represented in the decider: an atelier admits at most one author, archive demotes authors and denies writes through space grants, owners cannot be removed, and deleted/missing spaces are rejected.  See `🦀️.rs:334-396,996-1103`.
- The hub has directory control-plane APIs for session mint/me/delete, visible spaces/details/events, commands, invite redemption, and `/directory/ws` replay then live stream.  The admin APIs provide overview, spaces, users, connections, documents, events, directory-projection rebuild, connection close, and user-session revoke requests.  See `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:905-1237,1300-1474`.
- The admin React SPA has pages for Overview, Spaces, Users, Connections, Documents, and Events and persists its operator token only in `sessionStorage`.  Source: `/Users/ueli/Documents/semio/🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript`.

### Presence and short-disconnect handling

- Presence is explicitly ephemeral.  The hub stores `(space:document, actor) → PresenceSession` only in memory, releases palette color leases on disconnect, and publishes raw opaque peer bytes in document-wide rosters.  Directory presence summaries retain known actor/user/surface/color metadata.  See `📦️bin.rs:116-225,673-680,826-901`.
- A document-wide roster is intentional in current hub code: `surface` is in the opaque peer payload rather than selecting a separate presence fan-out.  Rust test `presence_roster_is_document_wide_and_frames_carry_surface_only_inside_peer` captures that contract at `📦️bin.rs:2260-…`.
- `DirectoryClient.stream` reconnects from the latest observed event sequence with full jitter and resets after a healthy 30-second connection.  REST calls are cancellable and time out after 10 seconds.  See `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3791-4067`.

### GIS and AI/MCP boundaries

- GIS map and terrain schemas are genuinely schema-first and multi-language.  The map has snapshots, diffs, mutations, binary/text facets, editor/viewer surfaces, camera presence, import/export, and language-agnostic fixtures under `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap` and `🏔️gisterrain`.
- Map inference is deterministic, not LLM-backed: `GisMapInference` produces position/route/region counts and longitude-latitude bounds from a snapshot.  It has determinism/default laws.  See `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:14-100`.
- The reusable map surface has concrete Web-Mercator/tile/LOD logic and independent oracle fixtures from the prior GIS ticket, in `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs`.
- The MCP gateway does discover capabilities from the installed plugin registry and supports `--hub <url> --space <id> [--token <t>]` by creating a `PersistenceBinding::Hub`.  See `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:624-727` and `🏠️workspace/🦀️.rs:424-448,1182-1184`.

## Dependency-ordered gaps and risks

### P0 — must resolve before calling spaces or AI collaboration safe

1. **Share tokens are unscoped and grant writer authority.**

   `POST /spaces/{space}/documents/{document}/share` ignores `space_id` and stores a token under only `document_id`; `resolve_auth` queries the same unscoped key.  A token for document `index` in one space can therefore authorize the same document id in another space, despite persisted document data being scoped.  In addition, `AuthOutcome::ShareToken` is mapped to `author`, so the token receives write authority.  Evidence: `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:101-110,337-348,430-436,714-752`; `/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:509-512`.

   Risk: cross-space data access/modification.  This is a release blocker.

2. **Directory WS leaks live connection and presence data across space boundaries; global user events disclose identities.**

   Event replay and live `Event` messages call `event_visible`, but every other `DirectoryStreamMessage` is forwarded unchanged.  `Connection` and `Presence` have a space/document/user dimension and are consequently observable by any directory WebSocket subscriber.  Separately, `event_visible` makes every event with no `space_id` public; `user.created` contains user id, email, and display name.  Evidence: `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1110-1215`; `DirectoryPresenceActor` and `ConnectionView` imports at `:20-25`.

   Risk: private-space membership/activity and account email disclosure.  This is a release blocker.

3. **MCP authority is not bound to the authenticated hub user or transport connection.**

   `--principal` and `--scopes` are caller-supplied process options; `AgentPrincipal` has no hub token field despite its design comment.  `--token` is merely passed to the workspace hub binding.  Every mutation protocol call uses `DEFAULT_SESSION_ID = "sess_default"`; the implementation explicitly says it is not connection/session-aware.  `context_resolve` also accepts a request-supplied principal string.  Evidence: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🛡️policy/🦀️.rs:63-88`; `🌉️mcp/🦀️.rs:247-252,294-300,644-658,666-727`.

   Risk: principals, approvals, transactions, jobs, audit records, and potentially capability grants are not isolated by MCP client or linked to space membership.  This directly fails P4's per-user/session authority requirement.

4. **AI inference over an existing map cannot execute.**

   MCP `inference_list` reads declarations only.  `inference_get` always returns a retryable `channel.not-wired` error once a service is found; no `artifact-infer` command exists in the workspace artifact channel.  A hub workspace also cannot enumerate cold remote artifact ids and returns `None` for a remote artifact not already opened by that MCP process.  Evidence: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:1-20,135-167,276-315`; `🏠️workspace/🦀️.rs:1207-1234`.

   Result: a generic gateway may discover the GIS descriptor, but it cannot select an existing shared map in a hub space nor invoke the map's deterministic inference.  A real AI map change is therefore unavailable.

5. **The claimed hub E2E contract is internally contradictory and normally skipped.**

   The TypeScript scenario is gated by `HUB_E2E=1`; ordinary `os-hub-ts:test` skips it.  Its current title and assertions demand that a viewer-surface peer never appears in editor rosters, while the current Rust hub and its unit test deliberately make presence document-wide.  The real E2E run will fail until one contract is selected and both implementations/tests match.  Evidence: `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts:8-13,180-360`; `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:168-173,2260-…`.

### P1 — required for reliable administration and short-outage collaboration

6. **Directory offline commands can be lost or stranded.**

   The worker bounds the queue at 200 by dropping the oldest request after only a console error.  `closeDirectory` clears queued commands with no command result.  Queue flushing occurs only after a received directory stream message; a successful reconnect that replays no event and emits no heartbeat does not flush pending work.  Evidence: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:790-890`; `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:3912-4067`; hub directory WS `📦️bin.rs:1164-1215`.

   This conflicts with the required short-outage behavior: it must surface cancellation/rejection deterministically rather than silently discard intent.

7. **Hub blob routes exist, but the browser blob cache cannot use them.**

   Hub endpoints are authenticated and content-addressed, yet `getCachedBlob`/`putCachedBlob` use only the dev middleware `/semio-blob`, are unexposed from worker messages, and their own comment says no UI/plugin consumes them.  Evidence: `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:451-…`; `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:892-1029`.

   This prevents shared map media or attachments from following an opened hub document.

8. **Admin “revoke user sessions” only kicks current realtime sockets.**

   The directory port has only revoke-by-session-id, no read/enumeration for a user's browser sessions.  The admin endpoint consequently sends kick notifications to live document sockets but does not invalidate existing/reconnectable bearer tokens.  The implementation states this limitation.  Evidence: `/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:538-542`; `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1450-1474`.

9. **The document live fan-out has no recovery on subscriber lag, and schema compatibility is process-local.**

   A `broadcast::RecvError::Lagged` is silently dropped; the client receives no catch-up requirement.  Nonzero pack-schema pins live only in memory and vanish on restart.  The browser worker also accepts/ignores a snapshot bootstrap because a client-side pack decoder is not wired.  Evidence: `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:160-166,183-190,884-885`; `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:718-724`.

10. **Operator and browser auth remain development-grade.**

    `POST /auth/sessions` mints a 30-day bearer session for any supplied email without credential/SSO validation.  With no `OS_HUB_ADMIN_TOKEN`, loopback has admin access; the hub binds its listener broadly.  CORS reflects any `Origin` and allows credentials rather than enforcing configured shell origins.  Evidence: `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:438-449,1240-1294,1729-…`.

    A local-first developer bootstrap can remain behind an explicit development mode, but it cannot be the production administrator/user identity system.

11. **Observability and read scaling gaps remain.**

    Accepted degraded-merge messages are intentionally dropped because current wire frames cannot carry them.  Admin/read endpoints reconstruct directory read state by replaying the full event log from zero.  Evidence: `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:576-603`; `load_read_model` call sites in `:1300-1450`.

### P2 — quality and maintainability gaps

12. The directory model is event-sourced, but document creation/lifecycle is lazy DB creation rather than a space-directory event, so administrator document listing cannot distinguish an intended artifact from an arbitrary first authorized route hit.  See `📦️bin.rs:265-278,422-427`.
13. Current presence keys one live entry by `(scope, actor)`.  Two connections that reuse an actor id overwrite each other's peer state and one disconnect removes the shared entry.  Actor-id uniqueness must be guaranteed or the key must include a session id.  See `📦️bin.rs:168-173,832,898`.
14. The directory stream/admin connection paths have unit coverage, but no browser-admin live backend E2E or bilingual runtime evidence.  Existing admin UI tests prove components only.

## Verification evidence and limitations

| Command | Observed result | Meaning |
| --- | --- | --- |
| `bun nx show project os-hub-ts --json` | Target identifies its one real hub scenario and calls it only with `HUB_E2E=1`. | Confirms default target cannot prove collaboration. |
| `HUB_E2E=1 bun nx run os-hub-ts:test` | Began a default-feature hub build and blocked behind the shared Cargo lock; process still active when this report was written. | Inconclusive; do not count as test evidence. |
| `bun nx run os-hub-admin:test` | 2 files, 8 tests passed. | Component/admin-client coverage, not live hub workflow. |
| `bun nx run os-hub-admin:test-quick` | Exited 0 with “No test files found”; Nx used cache. | False-green quick target. |
| `bun nx run @semio-tech/framework-os-mcp:test` | 3 passed, 30 skipped. | Its compiled-binary conformance/E2E suites were skipped; no live MCP proof. |
| `bun nx run @semio-tech/framework-os-mcp:test-quick` | Same 3 passed, 30 skipped. | Not an acceptance gate. |

The pre-existing ticket history confirms the same distinction:

| Ticket | State | Audit relevance |
| --- | --- | --- |
| `26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END` | open | Still names browser-collaboration, DB-feature, native parity, and plugin build work as remaining. |
| `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` | open | Its P7 report explicitly records hub workspace listing/read as empty/`None` and the MCP session work as pending. |
| `26/08/29/AI-MCP-END-TO-END` | closed | Its close summary records a static 22-tool surface but leaves `DEFAULT_SESSION_ID` unfixed; current source still has it and current TS run skipped 30 real-binary tests. |
| `26/08/29/GIS-MAP-END-TO-END` | closed | Provides strong map-rendering/oracle evidence, while explicitly reporting the GIS dev server as unbootable at closure due to concurrent plugin compilation failures. |
| `26/07/18/HUB-STUDIO-ROUTES-AUTH-AND-PRESENCE-SCHEMA`, `26/07/19/HUB-BLOB-ROUTES` | closed | Their hub route/blob work exists in the current tree, but browser blob and cross-space share-token integration remain incomplete. |

Paths: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END/🎫️ticket.json`; `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/🎫️ticket.json`; `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🎫️ticket.json`; `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END/🎫️ticket.json`.

## Test-first implementation packets

### H1 — Secure space/document authorization boundary (P0)

1. Add a schema-first `SpaceDocumentId`/equivalent to the directory share-token port and persistence tables; make share create, resolve, revoke, and all tests use it.  Decide explicitly whether share links are read-only (recommended) or delegated write grants with a separate scope.
2. Add a single `directory_message_visible(caller, message)` gate and apply it to event, connection, and presence replay/live frames.  Remove public `UserCreated` disclosure; send member identity only through authorized space detail/read-model projections.
3. Write tests first using two private spaces with the same document id, an anonymous directory socket, and a member socket.  Assert a token from space A cannot open/write space B; unaffiliated observers see neither presence nor connection; no untrusted stream can read another account's email.
4. Exercise the exact binary routes through independent Rust `tokio-tungstenite` and TypeScript/browser WebSocket clients, then promote the scenario into `os-hub-ts`.

### H2 — Real identity and administrator revocation (P0/P1)

1. Put development email mint behind an explicit dev-only identity adapter.  Define a project-owned identity-provider interface and a signed/session-backed implementation without leaking external API types.
2. Extend `HubDirectory` with user-session enumeration/revocation or an atomic revoke-before timestamp, then make the admin operation actually invalidate browser sessions as well as close connections.  Append auditable administrator actions as events.
3. Test: a revoked user cannot call `/auth/sessions/me`, cannot reconnect document WS, cannot issue directory commands, while an unaffected user retains access.  Test admin/no-token/remote-origin policy from a real HTTP client.

### H3 — Bounded, observable outage handling (P1)

1. Emit a successful directory-connection signal/heartbeat and flush on it; never depend on an unrelated event to flush commands.
2. Replace silent oldest-drop/close-clear behavior with visible command outcomes and explicit cancellation or durable local queue semantics.  Preserve ordering and idempotency keys.
3. On hub broadcast lag, signal a required frontier revalidation/catch-up rather than silently accepting data loss.  Persist schema compatibility metadata or define an explicit restart negotiation.
4. Add deterministic fake-clock/network tests for reconnect-with-no-events, overflow, cancellation, repeated reconnect, and lag recovery; then a two-browser real hub interruption scenario.

### H4 — Hub-backed workspace and AI map execution (P0)

1. Add authorized hub read projections/endpoints to enumerate documents in a space and read an artifact’s actual pack/snapshot/frontier.  Use these from `HeadlessWorkspace::workspace_artifact_ids`/`read_artifact_bytes`; keep document writes on the existing event/command lane.
2. Add an `artifact-infer` channel command with typed input/result/progress/cancellation.  Wire MCP `inference_get` to a job handle instead of `channel.not-wired`, and bind it to the map artifact schema.
3. Make MCP transport sessions immutable and per connection.  Derive the agent principal and effective capability grants from authenticated hub delegation/membership rather than flags or JSON arguments; scope approval, transaction, job, audit, and handle records to that connection and space.
4. Test the complete path with a real GIS map fixture: member A invokes map bounds inference, then an approved typed map mutation; member B observes it via the hub; A undoes it; non-member C and a second MCP session cannot read/act on either artifact or approval handle.  Use the existing Rust/TypeScript map inference implementations and the existing third-party GIS oracle fixture as independent agreement checks.

### H5 — Honest acceptance targets and operator journey (P0)

1. Make `test-quick` fail on zero selected tests.  Split cheap contract tests from expensive E2E, but require each acceptance launch target to execute a nonzero named suite.
2. Reconcile the presence contract: current code says document-wide, while TypeScript E2E asserts surface-isolated.  Update only after recording the product decision, then prove it with both raw frames and browser UI behavior.
3. Add one non-interactive registered full-stack target: hub + two authenticated shells + GIS map + MCP client.  It must record progress/cancellation, use isolated data, and verify EN/DE admin flows against the real hub.

## Recommended execution order

`H1 → H2 → H3 → H4 → H5`.

H1 closes current cross-space/private-data authorization holes.  H2 establishes real revocation/identity semantics on which MCP delegation depends.  H3 ensures that the client cannot lose collaboration intent during a short outage.  H4 then has a safe, enumerable, authorized shared-map substrate.  H5 turns the result into trustworthy, repeatable release evidence.
