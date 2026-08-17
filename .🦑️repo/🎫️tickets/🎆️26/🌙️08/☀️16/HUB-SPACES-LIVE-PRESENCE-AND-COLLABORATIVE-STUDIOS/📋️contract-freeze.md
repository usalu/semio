# Contract freeze — Hub Spaces, Live Presence and Collaborative Studios

Frozen by the coordinator at W0. Every lane implements against THIS document. A lane that needs a
change here STOPS and writes a `sharedFileRequest:` block in its report; only the coordinator edits
this file.

## C0 Ports, env, ids

| Item | Value |
|---|---|
| Hub port | **8787** everywhere. Fix `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts` `OS_HUB_PORT = 6070` → `8787`, and `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` `unwrap_or(6070)` → `8787` |
| Hub data | `OS_HUB_DATA` (launcher: `${workspaceFolder}/.semio/hub-dev/`) |
| Hub admin auth | `OS_HUB_ADMIN_TOKEN` bearer when set; when unset, **loopback peer ⇒ admin** (dev default), logged loudly once at startup |
| Hub admin assets | `OS_HUB_ADMIN_DIR`, else `concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔨️modules/🛡️admin/📦️packages/🟦️typescript/📤️dist")` |
| Admin vite dev | **8790** (`🛠️dev🗄️os-hub🛡️admin`, proxies `/directory`,`/admin/api`,`/auth`,`/spaces` → `OS_HUB_URL ?? http://127.0.0.1:8787`, `ws:true`) |
| s user ports | react **6072** (user1) / **6073** (user2); wgpu **6067** / **6068**. Existing `s` 6070/6066 untouched; 6071 is the multi harness |
| Client env | `S_HUB_URL=http://127.0.0.1:8787`, `S_USER=user1@semio.dev` \| `user2@semio.dev`, `S_DATA_DIR=${workspaceFolder}/.semio/s-user1` \| `s-user2` → vite define `VITE_S_HUB_URL` / `VITE_S_USER` / `VITE_S_DATA_DIR`; wgpu native reads `S_*` directly; wgpu browser through `🟦️boot.ts` |
| e2e ports | scan pool 7400–7498, or `S_COLLAB_HUB_PORT` / `S_COLLAB_USER1_PORT` / `S_COLLAB_USER2_PORT` |
| Actor id | `user:{user_id}#{shell_session_id}` (hub groups by `user_id`; per tab/process suffix keeps actors distinct) |
| Surface id | existing canonical `<kind>@<standard>/<subset>#<role>`, e.g. `s.space.home@1/*#editor` |
| Presence scope | `(space_id, document_id, surface)`; `surface` travels **out of band** as `?surface=` on the document WS URL. No `PresencePeer` wire change (its flag byte is full and the file is peer-leased) |
| Document ids | space artifact index = document `index` inside hub space `{space_id}`; artifacts = minted ids |
| Shell routes | `/` home · `/spaces/{id}` → **space app** (`s.space` editor/viewer) · `/spaces/{id}/studio` → workflow studio |
| Channel | `CHANNEL_VERSION` stays **11** (peer's). We add NO tags: opening-relay `documentId`/`spaceId` ride inside the existing `ReplayShellCommand` JSON `args` |

### Test/e2e id grammar (both shells)
`data-row-id="space:<id>" | "artifact:<id>" | "peer:<actor>" | "history:<id>"`; element ids
`#s-home-create-space`, `#s-space-create-artifact`, `#s-space-share`, `#s-presence-peers`,
`#s-checkin`; every new node carries `data-ui-path` (the wgpu↔React parity join). **No `data-testid`.**

## C1 Directory schema (JSON control plane, event-sourced)

Schema triad lives in the OS framework (the hub already depends on the os kernel; the os must not
depend on the hub): `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/` with the leaf set
`🔣️taxonomy.json` requires for a schema dir (check it; at minimum `🔣️component.json` +
`🦀️component.rs` + `🟦️component.ts`).

```
DirectoryEvent {
  seq: u64,            // dense, 1-based, backend-assigned
  id: string,          // uuid v7
  hlc: { physicalMs: i64, logical: u32 },
  actor: { kind: "user" | "admin" | "system", id: string },
  spaceId?: string, userId?: string,
  body: DirectoryEventBody,
  recordedAtMs: i64,
}
DirectoryEventBody.kind ∈
  "user.created"            { userId, email, displayName }
| "space.created"           { spaceId, name, kind, visibility, ownerUserId }
| "space.renamed"           { spaceId, name }
| "space.visibility-changed"{ spaceId, visibility }
| "space.archived"          { spaceId }
| "space.deleted"           { spaceId }
| "member.upserted"         { spaceId, userId, role }
| "member.removed"          { spaceId, userId }
| "invite.redeemed"         { spaceId, userId, inviteId, role }

DirectoryCommand.kind ∈
  "create-space" { name, kind, visibility } | "rename-space" { spaceId, name }
| "set-visibility" { spaceId, visibility } | "archive-space" { spaceId }
| "delete-space" { spaceId } | "upsert-member" { spaceId, email, role }
| "remove-member" { spaceId, userId } | "create-invite" { spaceId, role, ttlSecs } -> { inviteToken }
| "revoke-invite" { spaceId, inviteId }

DirectoryStreamMessage ∈
  { kind: "event", event }
| { kind: "connection", phase: "opened" | "closed", connection: ConnectionView }
| { kind: "presence", spaceId, documentId, surface, actors: string[] }
| { kind: "heartbeat", headSeq }

SpaceView      { id, name, kind, visibility, ownerUserId, role?, memberCount, documentCount, activeConnections, createdAtMs, updatedAtMs }
MemberView     { userId, email, displayName, role }
UserView       { id, email, displayName, createdAtMs }
ConnectionView { syncSessionId, spaceId, documentId, surface, actor, userId?, email?, role, connectedAtMs, presenceKnown }
DocumentView   { id, headSeq, commitSeq, epoch }
InviteView     { id, spaceId, role, createdAtMs, expiresAtMs, revoked }

DirectoryReadModel { spaces: BTreeMap<spaceId, SpaceView + members: MemberView[]>, cursor: seq }
fold(model, event) -> model            // pure, Rust + TS twins, byte-identical over the fixture
```

`spaceKind ∈ atelier | studio | archive`, `visibility ∈ private | public`, `role ∈ author | spectator`
(existing `SpaceKind` / `SpaceVisibility` / `SpaceRole` vocabulary).

> **Amendment 1 (W0, lane 0-A).** The space-kind field on `space.created` and `create-space` is named
> **`spaceKind`**, not `kind`: both bodies are internally tagged with a discriminator already called
> `kind`, so a second `kind` field is a literal serde collision. `spaceKind` is what landed in both
> twins and in the fixture; every lane uses it.

> **Amendment 2 (W0 barrier).** `cargo check/test -p semio-hub --all-features` is **red repo-wide and
> pre-existing**: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml` declares
> `sqlite = []`, `postgres = []`, `neo4j = []` with no optional `rusqlite`/`sqlx`/`neo4rs` dependencies
> wired, so enabling them compiles storage code against crates that are not dependencies (76 errors;
> last touched 2026-08-12, i.e. neither ours nor this hour's peer work). `🛢️db` is peer-leased — we do
> **not** fix it. Consequence for every hub lane: verify with **default features (sqlite)** —
> `cargo check -p semio-hub`, `cargo test -p semio-hub --lib` — never `--all-features`, and never
> `bun nx run os-hub:test*` (that target hardcodes `--all-features`). The postgres and neo4j directory
> backends are still implemented to parity, but they cannot be compiled today; say so plainly in the
> lane report rather than claiming they pass.

Golden fixture (both twins decode it, both folds produce the same model):
`🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧾️events.json`.

### Decider laws (backend-agnostic, unit-tested in the hub core)
- atelier ⇒ at most one distinct author.
- archive ⇒ nobody writes: `archive-space` first emits `member.upserted{role: spectator}` for every
  current author, then `space.archived`, so each projection step stays a pure function of one event.
- The owner's membership can never be removed; owner transfer is out of scope this wave.
- Any command naming a deleted space ⇒ `NotFound`.
- Not event-sourced (secrets/ephemeral): share tokens, auth sessions, sync sessions, invites.
  Invite **redemption** is an event.

## C2 Hub HTTP/WS surface

JSON everywhere on this control plane; `Authorization: Bearer <session>` on REST, `?token=` on WS.

```
POST   /directory/commands            -> 202 { events: DirectoryEvent[], result?: {...} }
GET    /directory/spaces              -> SpaceView[]           (member spaces + public spaces)
GET    /directory/spaces/{id}         -> SpaceView + members + documents(DocumentView) + invites(authors only)
POST   /directory/invites/{token}/redeem
GET    /directory/events?since=&limit=-> DirectoryEvent[]      (visibility filtered)
GET    /directory/ws?token=&since=    -> DirectoryStreamMessage text frames (subscribe, then replay, gap-free)
GET    /auth/sessions/me              -> { userId, email, displayName, expiresAt } | 401
DELETE /auth/sessions/me              -> revoke
POST   /auth/sessions                 -> unchanged (dev email mint)
```
Command authorization: `create-space` any session; `delete-space`/`archive-space` owner or admin;
everything else any author of the space or admin; removing the owner ⇒ 409.

Admin (bearer `OS_HUB_ADMIN_TOKEN`, or loopback when unset):
```
GET  /admin/api/overview        { counts, backends, dataDirBytes, headSeq, openArtifacts }
GET  /admin/api/spaces | /admin/api/spaces/{id} | /admin/api/users | /admin/api/connections
GET  /admin/api/documents?space= | /admin/api/events?since=&limit=
POST /admin/api/commands                      (actor kind "admin", bypasses role authz)
POST /admin/api/directory/rebuild             (rebuild projections from the log)
POST /admin/api/connections/{syncSessionId}/close   (kick)
POST /admin/api/users/{id}/sessions/revoke
GET  /admin  and  GET /admin/{*path}          (static SPA, traversal-guarded, SPA fallback)
```

## C3 Client identity

New OS config facet `os.config.identity` (own triad dir, beside `os.config.opening`):
`Identity { userId, email, displayName, hubBaseUrl, sessionToken, issuedAtMs }`, mutations
`sign-in` / `sign-out`, folded over the config op log (event-sourced, no CRUD), persisted
local-only in the folder lane under `S_DATA_DIR/os`.

Boot (both shells): env `S_HUB_URL` + `S_USER` → `GET /auth/sessions/me` with the cached token →
on 401 `POST /auth/sessions {email}` → `sign-in` mutation. Hub unreachable ⇒ keep the last persisted
identity, show an offline chip, retry with the existing backoff. Never blocks the UI thread.

Every `openDocument` binds `[hub{ baseUrl, spaceId, token, surface }, folder{ S_DATA_DIR/spaces/<spaceId> }]`
when an identity exists, `[folder]` otherwise. The OS config + home documents are folder-only.

## C4 `s.space` artifact (the space's artifact index)

`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/{🧬️schema,🚪️io,📚️examples,👁️viewer,✏️editor}`
(scaffold with the registry's `new surface`). Kind `s.space`, dialect `s.space.space@1/*`.

```
SSpaceSnapshot { schema, spaceId, artifacts: SpaceArtifactRow[] }
SpaceArtifactRow { id, name, kindId, schema, dialect{artifactKind,standard,subset},
                   createdAtMs, createdBy, updatedAtMs, updatedBy }
SSpaceMutation ∈ CreateArtifact | DeleteArtifact | RenameArtifact | TouchArtifact
```
Outcome vocabulary (peer contract's 7 frozen codes): create on an existing id ⇒ `Fatal
mutation.duplicate-id`; delete/rename/touch of an absent id ⇒ `Error mutation.target-missing`;
rename to an existing name ⇒ `Fatal mutation.duplicate-id`; rename to the same name ⇒ `Warning
mutation.no-op`. Persisted **shared** (hub document `index` in the space) + folder lane.
`project_space_index_to_collection(&SSpaceSnapshot) -> CollectionSnapshot` feeds
`resolve_workflow_artifact_document` so the studio keeps one source of truth.

Space name/kind/visibility/members are **directory-owned** and rendered from the read model; they are
never duplicated into this document.

## C5 Save and check-in

Every accepted `Apply` = an Edit → hub relay (`Ack` `Persisted`) + folder snapshot. Status pill:
`persisted | pending(n) | remote(connected|connecting|backoff|detached)` from `ArtifactSyncStatus`.
Auto check-in per open **editor** session: uncommitted edits and idle ≥ 20 s, or ≥ 200 uncommitted
edits ⇒ `CommitCheckpoint { message: "auto", authors: [identity] }`. Explicit `#s-checkin` action
with a message dialog. Checkpoint on editor close when edits are pending. Viewers never checkpoint
(already guarded by `VcsArtifactApp`). After every checkpoint the shell dispatches `TouchArtifact` to
the space index. Uses the existing `ArtifactCommand::CommitCheckpoint`; **no store-internal change.**

## C6 Directory command flow (client → hub)

Plugin surfaces never talk to the network. A surface command emits
`HostEffect::ReplayShellCommand { action_id: "os.directory.<verb>", args }` (same relay the opening
commands already use). The shell's command funnel calls `DirectoryClient.command(...)` →
`POST /directory/commands`; the resulting events come back on `/directory/ws` and are pushed into the
home and space sessions as the `foldDirectoryEvents` view action → `…ConfigMutation::FoldDirectoryEvent`
(config lane, persisted local-only, no undo). **No optimistic mutation of the read model** — the hub
log is the single writer. Offline: commands queue in the shell (bounded, in-memory) and flush on
reconnect; the row shows "pending".

OS command ids: `os.directory.create-space`, `os.directory.delete-space`, `os.directory.rename-space`,
`os.directory.set-visibility`, `os.directory.upsert-member`, `os.directory.remove-member`,
`os.directory.share-link`.
