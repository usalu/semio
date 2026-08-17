# Lane 1-A report — hub directory event log, decider, service, backends

## Summary

`🌎️hub/📇️directory/**` is now an append-only event log with per-backend projections. The
`HubDirectory` trait lost its three direct-write methods (`create_space`, `upsert_membership`,
`remove_membership`); all space/member mutation now flows `DirectoryCommand -> decide() ->
NewDirectoryEvent[] -> HubDirectory::append_events -> projection`. Added a pure(ish) decider, a
`DirectoryService` (single-writer lock + broadcast fan-out), and full event-log + projection support
in all three backends (sqlite verified by compiler/tests; postgres/neo4j written to parity but
unverified — see Amendment 2 below).

## Changed files

- `🌎️hub/📇️directory/🦀️component.rs` — added `//#region 🔖️Wire` (role/kind/visibility conversions
  between this crate's storage vocabulary and `protocol::os_directory`'s wire vocabulary),
  `//#region 🔖️Events` (`HubClock`, `NewDirectoryEvent`), `//#region 🔖️Decider` (`Decision`,
  `CommandResult`, `decide()`), `//#region 🔖️Service` (`DirectoryService` with `execute`,
  `redeem_invite`, `subscribe`, `publish`); rewrote `//#region 🔖️Trait` (removed the three write
  methods, added `get_user`, `list_members`, `Invites` region, widened `SyncSessions`, added
  `EventLog` region); widened `SyncSessionRecord` (`space_id`, `surface`), added `InviteRecord`;
  added `#[derive(Clone, Debug, PartialEq)]` to the previously-derive-less row structs
  (`ShareTokenRecord`/`UserRecord`/`SpaceRecord`/`SpaceMembershipRecord`/`AuthSessionRecord`) so
  tests can `assert_eq!` on them; new `//#region 🧪️Tests` module (7 tests, gated
  `#[cfg(all(test, feature = "sqlite"))]`).
- `🌎️hub/📇️directory/🪶️sqlite/🦀️component.rs` — new `hub_directory_event`, `hub_space_invite`
  tables; `hub_sync_session` widened with `space_id`/`surface`; new indexes
  `idx_sync_session_space`, `idx_space_invite_space`; new `//#region 🔖️Projections` (`project()`,
  the only place `hub_user`/`hub_space`/`hub_space_membership` rows are written); `seed()` rewritten
  to append events instead of direct inserts; trait impl updated to match the new trait (removed
  three methods, added `get_user`/`list_members`/invites/`list_active_sync_sessions`/
  `close_all_sync_sessions`/`append_events`/`events_since`/`head_seq`/`rebuild_projections`); tests
  rewritten (`user_space_membership_round_trip`, `sync_session_lifecycle`, `share_token_gating`
  adapted; `space_kind_membership_laws_are_enforced` removed — its coverage moved to the core
  decider-law tests since the law now lives in `decide()`, not the backend; added
  `seed_is_replayable`).
- `🌎️hub/📇️directory/🐘️postgres/🦀️component.rs` — same shape as sqlite (BIGSERIAL `seq`, JSONB
  `payload`, `sqlx_core::Transaction` for `append_events`/`rebuild_projections`/`project`); tests
  adapted the same way. **Unverified — see Amendment 2.**
- `🌎️hub/📇️directory/🌐️neo4j/🦀️component.rs` — `(:DirectoryEvent)` nodes + a singleton
  `(:DirectoryCounter {id:'singleton'})` node incremented in the same `Txn` as the event
  node + projection (Neo4j has no auto-increment primitive); `(:SpaceInvite)` nodes for invites;
  `SyncSession` nodes widened with `spaceId`/`surface`. **Unverified — see Amendment 2.**

No `Cargo.toml` change was needed — `serde_json`, `uuid`, `tokio` (with `sync`/`broadcast`) were
already dependencies.

## Design decisions worth flagging

- **`decide()`'s one write exception.** The brief and the contract both say `decide` "reads
  projections… and returns events; it never writes" — but `create-invite`/`revoke-invite` are
  explicitly *not* event-sourced (contract's own decider-laws list: "Not event-sourced… invites").
  Rather than give `decide()` a non-exhaustive match or duplicate the space-existence check in
  `DirectoryService::execute`, I documented this as `decide()`'s one deliberate exception: for those
  two `DirectoryCommand` variants it calls `dir.create_invite`/`dir.revoke_invite` directly (still
  under the caller's write lock) and returns `Decision{events: vec![], result}`. Documented in
  `decide()`'s own docstring.
- **`DirectoryService::redeem_invite`.** Not explicitly named in the brief's Service bullet, but
  `POST /directory/invites/{token}/redeem` (contract C2) needs to emit `invite.redeemed` under the
  same write-lock/dense-`seq` discipline as `execute` — without a service-level method, lane 1-B
  would have no way to reach `DirectoryService`'s private `dir`/`write` fields to do this safely. I
  added `DirectoryService::redeem_invite(actor, token, email, display_name) -> DirectoryResult<Vec<DirectoryEvent>>`
  (locks, resolves/creates the user by the same unknown-email law as `upsert-member`, emits
  `invite.redeemed`, appends+publishes). 1-B should call this instead of `execute` for the redeem
  endpoint.
- **`SpaceRecord`/`SpaceRole` kept as local storage types**, not replaced with
  `protocol::os_directory`'s `DirectorySpaceKind`/`DirectorySpaceVisibility`/`DirectorySpaceRole`.
  The file's own pre-existing header already documents *why* (this crate can't depend on the
  wasm-facing `space` crate, so it hand-mirrors the vocabulary) — that reasoning still holds even
  though the *wire* vocabulary now lives in `protocol::os_directory` and is `Import`ed. Free
  conversion functions (`role_to_wire`/`role_from_wire`/`kind_to_str`/`visibility_to_str`,
  `pub(crate)` in the core file's `//#region 🔖️Wire`) bridge the two vocabularies at the
  decider/projection boundary. This keeps every backend's existing `CHECK ('atelier','studio',
  'archive')`/`('author','spectator')` constraints and query shapes unchanged — smaller diff, lower
  risk, and it's exactly the DTOs (`DirectoryEvent`/`DirectoryCommand`/`SpaceView`/etc.) the brief
  named as "already exist… import them" that are actually reused, never redeclared.
- **`actor.id` parsing.** `DirectoryActor.id` for a `User` actor is the composite
  `user:{user_id}#{shell_session_id}` (contract §C0, and the schema file's own docstring says so
  explicitly). `create-space` needs the plain `user_id` as `owner_user_id`; `actor_user_id()` in the
  Decider region strips the `user:` prefix and the `#session` suffix. Every other command that needs
  a user id gets it from the command body itself (`RemoveMember.user_id`, the resolved-by-email id
  in `UpsertMember`), not from the actor, so this parsing is needed only once.
- **`SpaceDeleted` projection cascades explicitly** (deletes `hub_space_membership`/
  `hub_space_invite` rows for the space before deleting the space row) rather than relying on the
  schema's `ON DELETE CASCADE` — sqlite doesn't enforce foreign keys unless `PRAGMA foreign_keys =
  ON` is set per-connection, and this crate's `connect()` (pre-existing, not touched) never sets it.
  Postgres/neo4j do the equivalent explicit cleanup.

## Commands run + results

- `cargo check -p semio-hub --lib` (default features = sqlite): **GREEN**, exit 0. Two full runs
  (first caught 3 borrow-checker lifetime errors in the sqlite backend — see below — second run
  clean). Tails in `🧪️1-a-check-1.txt` (red, pre-fix), `🧪️1-a-check-2.txt` (green — "Finished `dev`
  profile [unoptimized] target(s) in 4m 40s", zero `error[E…]`, zero warnings anywhere under
  `📇️directory/`), `🧪️1-a-check-final.txt` (re-confirmation run, queued behind heavy concurrent
  workspace churn from live peer sessions — see the note below on timing).
- Fixed in the first pass: three `E0597` "temporary dropped while still borrowed" errors in
  `list_active_sync_sessions` and `rebuild_projections` (sqlite backend) — each was a `stmt.query_map(...)?.filter_map(...).collect()`
  chained directly as a block's tail expression; split into `let mapped = stmt.query_map(...)?;`
  then a separate `.collect()` statement, matching the pattern `events_since` already used.
- `cargo test -p semio-hub --lib`: **[fill in from 🧪️1-a-test-*.txt once the run lands — the
  workspace has heavy concurrent `cargo` traffic from multiple live peer sessions (1-B, 1-D, 1-E and
  the coordinator's own `--workspace --keep-going` check all running at once), so this was still
  queued behind the shared `target/` lock as of this report; do not trust a pass/fail claim here
  that isn't backed by a tail in the ticket folder]**.

## Verify note (per worker-brief + Amendment 2)

Only `cargo check -p semio-hub --lib` / `cargo test -p semio-hub --lib` (default features, sqlite)
were run. `--all-features` and `bun nx run os-hub:test*` were never invoked (Amendment 2: `🛢️db`'s
`postgres`/`neo4j` Cargo features are pre-existing-broken — no `rusqlite`/`sqlx`/`neo4rs` deps wired
— so enabling them fails to compile for reasons that predate this lane and are not `🛢️db`'s to fix
here). Consequently:

- **sqlite backend**: compiler-verified (`cargo check`) and, pending the still-queued run above,
  intended to be test-verified (`cargo test -p semio-hub --lib`).
- **postgres backend**: written to full parity (schema, `project()`, all trait methods, adapted
  tests) but **never compiled** — `sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>`,
  `Pool::begin()`, and `.bind(&serde_json::Value)` against a `JSONB` column are all written by
  precedent/best-effort against the existing file's own `sqlx_core`/`sqlx_postgres` usage, not
  verified against the actual crate API surface. Say so plainly rather than claiming it compiles.
- **neo4j backend**: same — written to parity (a `(:DirectoryEvent)` node type, a singleton
  `(:DirectoryCounter)` node for `seq`, `(:SpaceInvite)` nodes, widened `SyncSession` nodes) using
  `neo4rs::Txn`'s `run`/`execute`/`commit` for the transactional pieces, but **never compiled** —
  `Txn::handle()` (used to keep the `RowStream` borrow alive across the loop) is a best-effort
  guess at the 0.8 API shape, not confirmed.

`📦️bin.rs` was not touched (forbidden — lane 1-B + a live peer own it). I did not run
`cargo check -p semio-hub` (the bin+lib default target) since that will not compile until 1-B lands
their side against this trait — that's expected and explicitly called out in the brief, not a
regression here.

## What is NOT done

- No changes to `🌎️hub/📦️packages/🦀️rust/Cargo.toml` (none needed).
- Postgres/neo4j backends are unverified by any compiler or test run (Amendment 2, see above).
- `DirectoryService::redeem_invite` is new (beyond the brief's literal Service bullet) — 1-B needs
  to wire `POST /directory/invites/{token}/redeem` to call it, not `execute`.
- I did not add owner-transfer, invite-listing authorization, or any HTTP/WS surface — all of that
  is lane 1-B's per the ownership doc.

## sharedFileRequests

None. Everything needed was inside my lease (`🌎️hub/📇️directory/**`); no `Cargo.toml` dependency
addition was needed.
