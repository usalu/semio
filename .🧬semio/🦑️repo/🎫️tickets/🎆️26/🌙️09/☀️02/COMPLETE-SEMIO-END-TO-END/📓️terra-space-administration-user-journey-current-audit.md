# 🏛️ Space Administration User Journey — Current Audit

## Verdict

The bounded, authenticated write path is real in current source: the admin SPA issues a closed `upsert-space-member` intent over its loopback relay; the hub derives the administrator principal, audits the intent, converts it to `DirectoryCommand::UpsertMember`, and SQLite commits the event and membership projection in one immediate transaction. The ordinary author path is also present: the Space/Home commands emit `os.directory.upsert-member`, the Shell maps that exact action to a `DirectoryCommand`, and the browser worker posts it through the broker-only directory route.

The requested **end-to-end observed journey is RED**. The first decisive client-side boundary is the only path which turns a persisted/broadcast directory event into collaborator-visible OS state: `foldDirectoryEvents`. It accepts an unbounded raw JSON array, treats malformed input as an empty array, and is explicitly declared `BatchOnlyPendingRewrite`. Thus it is not a qualified retained/interactive operation and has no gap/sequence recovery contract. The existing two-browser command does not prove the journey either: it still boots each user with `S_USER` and uses a direct `COLLAB_E2E_ADMIN_TOKEN`, whereas the current browser path requires a broker proof and the admin UI requires its relay bootstrap.

There are two additional production-quality REDs before a full member lifecycle can be claimed:

- `DirectoryService` releases its writer mutex before publishing durable events, despite its own comment saying the inverse. Separate executor workers can therefore broadcast committed sequence `n+1` before `n`; the client fold has no sequence-gap recovery.
- A membership removal does not close an already-admitted directory socket. A later frame fails the current-membership test and is silently suppressed; the one-second authorization tick only checks session/grant authority, not membership. This avoids a new private frame but leaves a visibly live, stale client connection.

No build, browser run, or process check was executed by this audit.

## Current Journey Evidence

| Stage | Current source authority | Status |
|---|---|---|
| Admin UI action | `AdminClient` uses same-origin, cookie-authenticated `POST /admin/api/intents`; `SpacesPage` calls `upsertSpaceMember` and refreshes its detail after success. [AdminSession](/Users/ueli/Documents/semio/🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx:66), [SpacesPage](/Users/ueli/Documents/semio/🌎️hub/🔨️modules/🛡️admin/🧱️elements/🏛️SpacesPage/🟦️.tsx:149) | Source-present; no error/pending UI state. |
| Admin authentication | Fragment proof is consumed into an HttpOnly same-origin relay cookie, then the SPA clears the fragment. The hub derives an `AdminPrincipalV1` for every endpoint. [AdminSession](/Users/ueli/Documents/semio/🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx:208), [hub](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1794) | Source-present. |
| Intent to command | `UpsertSpaceMember` and `RemoveSpaceMember` map only to their closed `DirectoryCommand` variants. The intent executor uses the derived principal actor and returns a durable event range. [hub](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4220), [hub](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4543) | Source-present. |
| Author authorization | A non-admin needs `SpaceRole::Author` for upsert/remove; viewers are forbidden. [hub](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3553) | Source-present. |
| Event/projection durability | The SQLite backend persists each event and applies its projection in the same `BEGIN IMMEDIATE` transaction. `MemberUpserted` is an UPSERT; `MemberRemoved` deletes the exact pair. [SQLite](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1631), [projection](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:643) | Source-present. |
| Socket/replay access control | A socket receives raw directory events only while its session and current membership are valid. [hub](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3726), [hub](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3950) | Source-present, but member-removal closure is RED. |
| Ordinary Shell action | `os.directory.upsert-member` is typed from the shell action and the worker posts through the broker-relayed directory API. [ShellHost](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:980), [Space command](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💌invite-member/🦀️.rs:19) | Source-present; not runtime-proven. |
| Shell collaborator observation | Shell serializes a full event batch into `foldDirectoryEvents`; Home parses it with `unwrap_or_default()` and emits one config mutation per event. [ShellHost](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4233), [Home command](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📇️fold-directory-events/🦀️.rs:22) | **RED**: its declared classification is `BatchOnlyPendingRewrite`. [Home editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:577) |
| Existing live check | `admin-live-journey-check` runs the real loopback relay/SQLite/Chromium admin journey, but only creates and reads a space. It does not submit a member action or establish a second user stream. [script](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2166), [registered target](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4608) | Existing process target, insufficient coverage. |

## Exact REDs

### 1. Collaborator projection is an explicitly unqualified batch action

`dispatchDirectoryEventBatch` sends a JSON string containing every supplied event into the currently mounted Home/Studio/Space-index action. The Home and Space plugins decode the complete string eagerly and construct a mutation for each item. The retained-command corpus names the exact blocker: complete-array parse plus repeated model work in one dispatch.

This is not just a missing test. A large replay, duplicate delivery, malformed frame, or out-of-order live frame has no bounded cursor, expected-sequence field, rejection state, or resync request. The Shell only writes a failed directory command to `console.error`; it renders neither operation failure nor `directoryPendingCommands` as accessible UI. [Shell result handling](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1458).

### 2. Durable order is not live-fanout order

`DirectoryService::execute`, `execute_create_space_with_id`, and `execute_artifact_authority` all acquire the same write mutex, append durable events, explicitly `drop(clock)`, then synchronously send each event to the broadcast channel. [directory service](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1599). SQLite makes the event and projection atomic, but the send is outside that serialization. On a multi-thread executor a later command may acquire the mutex, commit, and broadcast before the first task enters `tx.send`. The websocket keeps only a maximum `last_replayed`; it does not buffer a gap or ask for a replay when it sees `n+1` first. [websocket](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3950).

### 3. Membership removal leaves a stale connected directory client

`socket_directory_message_visible` returns `Unauthorized` after a membership loss, but `handle_directory_ws_v1` consumes that case with `{}` for a live event. The periodic tick checks `socket_live_authority` (session/grant) rather than `directory_space_access_for_user`. Consequently no private later event crosses, but the member’s client retains an open stream and no authoritative “access removed” terminal state.

### 4. Current acceptance command is obsolete for the authentication boundary

`collab-e2e` starts two dev servers with `S_USER` and queries `/admin/api/*` using `COLLAB_E2E_ADMIN_TOKEN`. [runner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2846), [step 7](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:3138). The current frontend uses broker proof for ordinary routes and an admin fragment/relay for administration. Even its membership step only navigates the second page; it never asserts a received role/member event. [step 2](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2971).

## Smallest Executable P0

Keep the scope deliberately narrow: **the shipped admin SPA performs one upsert/removal; a second authenticated directory client observes the durable membership consequence.** It is an honest user-journey P0 without claiming that a general Shell/plugin installation is bootable.

### Packet A — linearize ordinary directory commit and live fanout

1. In [directory service](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1599), introduce one private `append_and_publish_locked` seam used by `execute`, `execute_create_space_with_id`, and `execute_artifact_authority`.
2. It must retain the existing `HubClock` mutex from decision through the synchronous `broadcast::Sender::send` loop. No await may occur between durable commit and the final enqueue. A no-receiver broadcast error remains non-fatal: the durable transaction is authoritative and later replay is valid.
3. Preserve `append_events`' existing database transaction. Do not add a second projection, optimistic client row, or a client supplied event/sequence.

**Native laws** in `🌎️hub/📇️directory/🦀️.rs`:

- Concurrent A/B member-upserts, barrier immediately after A commits: every subscriber observes the durable `seq` order `A…B`; SQLite `events_since` and membership projection agree.
- An unknown email still emits `user.created` immediately before the paired `member.upserted`, never a partial projection.
- DB append failure emits neither an event nor a projection; receiver timeout proves no fanout.
- Two spaces: a recipient admitted only to A never gets an A/B event for B.

### Packet B — make membership revocation a terminal socket outcome

1. At [directory websocket delivery](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3950), distinguish failed current **membership** visibility from a merely not-visible unrelated event. For an active stream scoped to a space, a revoked/removed participant must receive no raw event and must close with the existing authorization terminal (`4401`).
2. Revalidate the directory-space scope on the one-second tick only when a scope is present. An unscoped directory stream cannot be closed just because one of its many visible spaces changes; it must instead expose a bounded `directory access changed` control or re-open with a stable `since` cursor. That control is outside this P0 if the P0 uses a scoped stream.
3. Keep the existing grant/session authority checks. Membership is an additional visibility fence, never a substitute for session generation or grant revalidation.

**Native/socket laws** in `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`:

- Scoped member stream: `MemberRemoved` commits, raw event is not sent, connection closes `4401`, and reconnect with the same credential cannot receive replay or mint a scoped grant.
- Spectator receives the upsert event and current `GET /directory/spaces/:id` membership detail, but a viewer/spectator `POST /directory/commands` upsert is `403` with no head/projection advance.
- Cross-space member event never crosses the scoped grant; session revocation while an admin intent is in flight yields no terminal durable member event.

### Packet C — a bounded observed P0 instead of the current Shell batch fold

For the first executable P0, do **not** pretend the current Home config fold is repaired. Extend the existing admin-live process fixture with a second browser-relay profile and one bounded directory socket client; assert the exact `MemberUpserted` sequence and then a bounded read model request. The admin browser supplies the actual visual action; the collaborator client supplies the authoritative observation.

Exact files to extend:

- `🌎️hub/📇️directory/🧫️fixtures/🚶️admin-live-journey-v1/{🧬️.schema.json,🔣️.json}`: add two local profiles, one member intent, expected event range/body/role, maximum response/event bytes, and EN/DE visible copy identifiers.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`: extend `adminLiveJourneyFixture` and `proveAdminLiveJourney` (currently [lines 2143–2289](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2143)). Reuse `startLocalBrowserRelay` and `issueLocalCredential`; issue/consume the real directory socket grant and binary `SocketHelloV1`, rather than adding a bearer shortcut.
- `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🏛️SpacesPage/🟦️.tsx`: add a local terminal mutation state for every member action. Disable only the active control, expose a localized `role=status`/`aria-live` outcome, refresh only after the succeeded receipt, and retain the form on failure. Do not put credentials or authority in React state.
- `🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🟦️.tsx`: add explicit English and German pending/succeeded/denied/unavailable member-operation labels.
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🛡️admin.test.tsx`: add UI laws for pending state, failed receipt, and success readback; mocks do not count as the process proof.

**Process law** — add one registered `space-admin-live-journey-check` command alongside `admin-live-journey-check` in [project.json](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📋️project.json:95) and [script router](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4608). It must:

1. Start the real SQLite hub plus isolated admin relay and two browser relay identities.
2. Use the admin SPA controls to create/open a space and submit the member role; wait for its row and accessible succeeded state.
3. Prove the collaborator's credential-free socket Hello receives exactly the durable upsert event once; its authenticated bounded detail/list read shows the role.
4. Attempt viewer write, cross-space replay, stale session/revoked admin, duplicate socket resume, member removal/reconnect, and hub restart. Each denial checks both event head and SQLite projection are unchanged where appropriate.
5. On restart, reissue credentials/grants: no retained bearer, nonce, cookie, or stale socket grant may be reused.

Run after implementation via `bun nx run os-hub:space-admin-live-journey-check --skip-nx-cache`. The existing `bun nx run os-hub:admin-live-journey-check --skip-nx-cache` and `bun nx run os-hub:socket-grant-check --skip-nx-cache` remain focused prerequisite gates, not evidence for the full P0.

## Explicit Nonclaims

- This does not claim the general React/WGPU Shell can boot every artifact; the trusted provider/member-open work is a separate prerequisite.
- This does not repair the general unscoped directory stream resync protocol, invite redemption linearization, or presence lease expiry.
- This does not claim PostgreSQL, Neo4j, OIDC, a model provider, artifact mutation, or a generic all-plugin collaboration journey.
- A mocked SPA unit test and a source-only schema/oracle are not process evidence.
