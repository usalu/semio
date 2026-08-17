# Lane 1-B report — hub REST/WS directory + admin API + presence per surface

## Changed files

- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` (only file touched; 1182 → 2136 lines, +290/-6 net per
  `git diff --stat`). All additions are **new regions only** as leased: `🔖️AdminAuth`, `🔖️Directory`,
  `🔖️Admin`, new `HubState` fields, new routes in `router()`, `main()` wiring, the `handle_ws` +
  `Presence` arm changes, and new tests in `🔖️Tests`. `submit_commands`, `merge_policy_from_env`,
  `encode_messages`, `messages_for_error`, `HubState.merge_policy`, and the `ClientFrame::Commands`
  arm were **not touched** (verified by re-reading the diff before finishing).

No other files were edited. `🌎️hub/📇️directory/**` (lane 1-A's lease) was read repeatedly but never
written to.

## What was built

1. **`HubState`** gains `directory_service: Arc<DirectoryService>`, `admin_dir: PathBuf` (from
   `OS_HUB_ADMIN_DIR`, else the §C0 compile-time default; carried through but not served — that's
   2-E's), `session_kicks: Arc<DashMap<String, Arc<Notify>>>`, and `surface_fanout: Arc<DashMap<
   (String,String), broadcast::Sender<ServerFrame>>>` (see the presence-per-surface design note
   below). `presence` is rekeyed `(scope_key, surface, actor)`.
2. **`🔖️AdminAuth`**: `is_admin(state, headers, peer)` — bearer `OS_HUB_ADMIN_TOKEN` when configured,
   else loopback-peer-is-admin. `create_share` now uses it (via a new `ConnectInfo<SocketAddr>`
   extractor) instead of its old "no token ⇒ 403". `main()` logs the loopback fallback loudly once,
   and `router()`/`spawn_server` now serve through `into_make_service_with_connect_info::<SocketAddr>()`
   so every handler can ask for the caller's peer address.
3. **`🔖️Directory`**: `POST /directory/commands` (bearer session required, `authorize_directory_command`
   matrix, then `DirectoryService::execute`), `GET /directory/spaces`, `GET /directory/spaces/{id}`
   (SpaceView flattened + members + documents + invites-for-authors-only), `POST /directory/invites/
   {token}/redeem` (→ `DirectoryService::redeem_invite`), `GET /directory/events?since=&limit=`
   (visibility-filtered), `GET /directory/ws?token=&since=` (subscribe → replay → live, gap-free/
   dedup via `seq <= last_replayed`, `Message::Text` JSON frames), `GET`/`DELETE /auth/sessions/me`.
   Reads fold the whole event log on demand (`load_read_model` = `events_since(0, MAX)` + `os_directory
   ::fold_all`) rather than maintaining a cached projection — simpler and always-correct; noted as the
   natural follow-up optimization once a real log is large, not attempted here given the scope already
   in flight.
4. **`🔖️Admin`** (API only, no static serving): `GET /admin/api/{overview,spaces,spaces/{id},users,
   connections,documents?space=,events}`, `POST /admin/api/commands` (actor kind `admin`, bypasses
   `authorize_directory_command`), `POST /admin/api/directory/rebuild`, `POST /admin/api/connections/
   {syncSessionId}/close` (fires the session's `Notify`), `POST /admin/api/users/{id}/sessions/revoke`
   (kicks that user's live connections — see the gap noted below). `overview`'s exact field set isn't
   pinned by contract §C2 (unlike the C1 schema), so it's a `serde_json::json!` shape rather than a
   frozen struct: `counts{spaces,users,connections}`, `backends{sqlite,postgres,neo4j}` (compiled
   features, since `HubState` doesn't carry which backend string was chosen at connect time),
   `dataDirBytes` (best-effort recursive size of `extensions_root`'s parent), `headSeq`, `openArtifacts`.
5. **Presence per surface**: `document_ws` accepts `?surface=` (missing ⇒ `""`); `handle_client_frame`
   gained a `surface: &str` parameter (additive, only the `Presence` arm reads it); the `Presence` arm
   now writes `state.presence[(key, surface, actor)]` and broadcasts on a NEW `surface_fanout`
   channel scoped to `(key, surface)`. **Design deviation from the worker brief's literal `enum
   Fanout`**: instead of widening the shared `fanout: Sender<ServerFrame>` payload with a routing tag
   (which would require rewriting its one existing publish site inside the HARD-OFF-LIMITS
   `ClientFrame::Commands` arm — a live peer lease, 2-E, is editing that exact arm concurrently), this
   lane added a SECOND, surface-scoped broadcast channel. Every session subscribes to both: the
   document-wide `fanout` (commands/preview, unchanged, zero edits to the off-limits arm) and its own
   `surface_fanout` (presence only). Behaviourally identical to the brief's spec (command frames reach
   every surface, presence frames only the matching one — proven by
   `presence_roster_is_scoped_per_surface`) with a strictly smaller, safer diff. Flagging this
   explicitly in case the coordinator wants the literal `Fanout` enum for some other reason.
   `record_sync_session_open` calls now pass `space_id`+`surface` (1-A's trait gained these params
   this session) and publish `DirectoryStreamMessage::Connection{phase}` on open/close via
   `DirectoryService::publish`. `main()` calls `directory.close_all_sync_sessions()` before serving.
6. Fixed the pre-existing tests that used the trait's removed `create_space`/`upsert_membership`
   write methods (`security_gate_rejects_spectator_writes_and_allows_author_writes`,
   `public_visibility_grants_anonymous_spectator_fallback`, `auth_session_grants_role_and_bypasses_
   share_gate`) to go through `DirectoryService::execute` instead, via two new test helpers
   (`create_space_for_test`, `upsert_member_for_test`).

## Tests added (all in `🔖️Tests`, all passing)

`directory_commands_append_events_and_project`, `directory_ws_replays_then_streams_live`,
`presence_roster_is_scoped_per_surface`, `connection_events_reach_admin_stream`,
`admin_api_lists_spaces_users_connections_and_kicks`, `admin_loopback_default_and_bearer_when_
configured`, `deleted_space_denies_ws_hello`, `auth_sessions_me_roundtrip`.

## Commands run + results (paste tails; all logs live in this ticket folder as `🧪️1-b-*.txt`)

`cargo check -p semio-hub` (default features / sqlite, per Amendment 2) — **GREEN, zero warnings
attributable to `bin.rs`** (final run, `🧪️1-b-phase2-check2.txt`):
```
    Checking semio-hub v0.1.0 (/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 32.02s
```

`cargo test -p semio-hub --bin os-hub` — **GREEN, 18/18** (final run, `🧪️1-b-cargo-test-bin-final.txt`):
```
running 18 tests
... (all 18 lines "... ok")
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.36s
```

`cargo test -p semio-hub --lib` — **RED, 4 passed / 7 failed, none of them mine** (final run,
`🧪️1-b-cargo-test-lib-final.txt`):
```
test result: FAILED. 4 passed; 7 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```
All 7 failures are in `directory::tests::*` — a module inside `🌎️hub/📇️directory/🦀️component.rs`
(lane 1-A's file, `#region 🧪️Tests` at line ~608, panics at line 628), never touched by this lane.
Every failure is the identical `Backend("FOREIGN KEY constraint failed")`: that module's own
`fresh_dir()`/`create_space()` test helpers open a bare `SqliteDirectory::connect(":memory:")`
**without calling `.seed()`**, then run `DirectoryCommand::CreateSpace` with an arbitrary owner actor
id that has no corresponding `hub_user` row — the sqlite backend's `hub_space.owner_user_id` FK then
rejects the insert. (This lane hit the exact same shape of bug in its own test helpers —
`create_space_for_test`/`upsert_member_for_test` — and fixed it by using the pre-seeded `"seed"` user
as the owner; 1-A's `directory::tests` module doesn't do that.) `🌎️hub/📇️directory/🦀️component.rs`
and its sqlite/postgres/neo4j siblings are **live, uncommitted edits** (git status `MM`/`M`, no commit
yet — `git log` on those paths still shows the pre-wave `2026-08-06` commit) — not this lane's to fix.
Per amendment 2, `--all-features`/`bun nx run os-hub:test*` were never run.

## Blockers encountered and resolved mid-session (not left open)

- `🌎️hub/📇️directory/🦀️component.rs` was mid-rewrite by 1-A for most of this session (`DirectoryService`
  landed before the trait declaration did; the sqlite backend landed after that). This lane worked
  independently on everything not requiring the new trait/service first (AdminAuth, presence-per-
  surface, `session_kicks`, `admin_dir`), then wrote `🔖️Directory`/`🔖️Admin` against the trait/service
  signatures once they stabilized (confirmed complete by re-reading the file before every dependent
  edit, per the lease rule), and finally re-ran `cargo check`/`cargo test` once 1-A's sqlite backend
  caught up. No blocker remains — the crate is green end to end for everything in this lease.

## sharedFileRequests

None. No foreign file/region needed editing.

## Known gaps (flagged, not blockers)

- `POST /admin/api/users/{id}/sessions/revoke`: `HubDirectory` has no "list this user's
  `AuthSessionRecord`s" read (only `revoke_auth_session(id)` by session id), so this route can only
  kick that user's currently-LIVE document-WS connections (via `session_kicks`), not revoke a bearer
  token for a browser tab that never opened a document WS. The route's contract name is honored for
  every realtime-relevant case; a full implementation needs a new `HubDirectory` read this lane
  doesn't own the file to add.
- Admin overview's exact JSON shape is not contract-frozen (unlike the C1 event/command schema), so it
  was implemented as a reasonable `serde_json::json!` rather than reverse-engineered from a spec that
  doesn't exist yet.
- `Fanout` enum: see the presence-per-surface design note above — implemented with an equivalent
  two-channel design instead of the literal enum, to avoid any edit to the peer-owned
  `ClientFrame::Commands` arm.

## What is NOT done (explicitly out of this lane's scope per the brief)

- `/admin` static SPA file-serving (lane 2-E's).
- `🌎️hub/🔨️modules/🛡️admin/**` (the admin SPA itself, lane 2-E's).
- Client-side `DirectoryClient`/identity (lanes 1-C/1-D).
