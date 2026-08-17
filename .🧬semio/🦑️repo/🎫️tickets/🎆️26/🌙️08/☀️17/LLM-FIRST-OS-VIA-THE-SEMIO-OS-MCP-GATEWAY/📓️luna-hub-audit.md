# Hub + Directory Identity & Token Flow Audit

Read-only exploration for agent-principal mint. Mapped hub/directory to enable first-class LLM agent scope + audit.

---

## 1. Auth Region: AuthOutcome, resolve_auth, Admin Detection, Routes

**File**: `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs`  
**Region**: `#region 🔖️Auth` (L268–321)  
**Hash**: ef479fc96b51935f428a211c2b761ba8968c865fb9af896bb403ddffff55e115  
**Log**: 
```
101a6b4ea8 🐙️ueli🎆️26🌙️06☀️04🚩️528
0b9f1d3a04 🐙️ueli🎆️26🌙️06☀️04🚩️525
20252aa16d 🐙️ueli🎆️26🌙️06☀️04🚩️496
```

### AuthOutcome Enum (L272–282)
```rust
enum AuthOutcome {
    Session { user_id: String, role: SpaceRole },
    ShareToken,
    Public,          // implicit anonymous spectator when space visibility == "public"
    Denied,
}
```

### resolve_auth Function (L289–306)
- **Signature**: `async fn resolve_auth(state: &HubState, space_id: &str, document_id: &str, token: Option<&str>) -> AuthOutcome`
- **Logic**:
  1. If `token` provided: look up `AuthSessionRecord` via `state.directory.get_auth_session(session_id).await`
  2. If found + `expires_at > now_ms()`: look up `SpaceRole` via `state.directory.get_role(space_id, &session.user_id).await`
  3. If no session: fall back to `state.directory.authorized_by_token(document_id, token)` for share-token scheme
  4. If no share-token: check `state.directory.get_space(space_id)` visibility; `"public"` ⇒ `AuthOutcome::Public`
  5. Otherwise: `AuthOutcome::Denied`

### Admin Detection (L329–334)
```rust
fn is_admin(state: &HubState, headers: &HeaderMap, peer: Option<SocketAddr>) -> bool {
    match state.admin_token.as_deref() {
        Some(expected) => bearer(headers).as_deref() == Some(expected),  // bearer token match
        None => peer.is_some_and(|addr| addr.ip().is_loopback()),         // dev: loopback ⇒ admin
    }
}
```
- Contract §C2: `OS_HUB_ADMIN_TOKEN` env var or loopback fallback (logged at startup).

### Route Table (L1488–1515)

| Method | Path | Handler | Auth |
|---|---|---|---|
| POST | `/auth/sessions` | `create_auth_session` | None (dev email mint) |
| GET | `/auth/sessions/me` | `get_session_me` | Bearer session |
| DELETE | `/auth/sessions/me` | `delete_session_me` | Bearer session |
| POST | `/directory/commands` | `post_directory_commands` | Bearer session OR loopback |
| GET | `/directory/spaces` | `get_directory_spaces` | Bearer session OR none (public spaces) |
| GET | `/directory/spaces/{id}` | `get_directory_space` | Bearer session OR none (public spaces) |
| POST | `/directory/invites/{token}/redeem` | `post_redeem_invite` | Invite token (path) |
| GET | `/directory/events` | `get_directory_events` | Bearer session OR none (visibility filtered) |
| GET | `/directory/ws` | `directory_ws` | Bearer session OR none (token query param) |
| GET | `/spaces/{space_id}/documents/{id}` | `get_document_status` | Bearer/share-token/public |
| POST | `/spaces/{space_id}/documents/{id}/share` | `create_share` | Admin only (`is_admin`) |
| GET/HEAD/PUT | `/spaces/{space_id}/blobs/{hash}` | `get_blob`/`head_blob`/`put_blob` | Bearer/share-token/public |
| GET | `/spaces/{space_id}/documents/{id}/ws` | `document_ws` | Bearer/share-token/public + contract §C0 surface |
| **Admin routes** | `/admin/api/*` | (8 routes) | Admin only (`is_admin`) |
| GET | `/extensions` | `list_extensions` | None |
| GET | `/extensions/{extension_id}/{*rest}` | `get_extension_asset` | None |
| GET | `/admin` / `/admin/{*path}` | Admin SPA | Admin only |

---

## 2. Session/Token Records: Persistence & Mint Entry Points

**Files**:
- Model defs: `/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️component.rs` (L44–156)
- Trait: same file (L510–560)

### AuthSessionRecord (L119–125)
```rust
pub struct AuthSessionRecord {
    pub id: String,              // session token
    pub user_id: String,
    pub created_at: i64,
    pub expires_at: i64,
    pub sso_provider: Option<String>,
}
```
- **Persisted in**: Each directory backend (sqlite/postgres/neo4j) — trait `HubDirectory::create_auth_session`, `get_auth_session`
- **Mint entry point** (L399, bin.rs): `state.directory.create_auth_session(&user.id, 60 * 60 * 24 * 30, None).await`
  - TTL: 30 days (hardcoded)
  - SSO provider: None (dev mode)

### SyncSessionRecord (L132–142)
```rust
pub struct SyncSessionRecord {
    pub id: String,              // session id (not a token)
    pub space_id: String,
    pub document_id: String,
    pub surface: String,
    pub user_id: Option<String>,
    pub space_role: Option<SpaceRole>,
    pub client_label: String,    // ActorId
    pub connected_at: i64,
    pub disconnected_at: Option<i64>,
}
```
- **Not persisted as events** (decider law) — ephemeral, written on WebSocket `Hello`/disconnect
- **Mint entry point** (L745, bin.rs): `state.directory.record_sync_session_open(&space_id, &document_id, &surface, user_id.as_deref(), role, &actor.0).await`

### ShareTokenRecord (L44–48)
```rust
pub struct ShareTokenRecord {
    pub token: String,           // bearer token
    pub document_id: String,
    pub created_at: i64,
}
```
- **Persisted in**: Each backend's storage
- **Mint entry point** (L386, bin.rs): `state.directory.create_share_token(&document_id).await`
  - Requires admin auth (`is_admin` check, L382–383)

### InviteRecord (L148–156)
```rust
pub struct InviteRecord {
    pub id: String,
    pub token: String,           // bearer secret (never in event log)
    pub space_id: String,
    pub role: SpaceRole,
    pub created_at: i64,
    pub expires_at: i64,
    pub revoked_at: Option<i64>,
}
```
- **Not event-sourced** (only `invite.redeemed` is an event)
- **Trait methods**: `create_invite`, `revoke_invite`, `redeem_invite` (in `HubDirectory` trait, L540–560)

---

## 3. DirectoryActorKind: Schema, Generated Twins, Variant Addition Path

**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️component.json` (L10)

### Schema Definition (Line 10)
```json
"DirectoryActorKind": { "type": "string", "enum": ["user", "admin", "system"] }
```

### Current Variants
- `"user"` — authenticated platform user
- `"admin"` — hub admin (bearer token or loopback)
- `"system"` — system-generated events (unused currently)

### Generated Twins
**TypeScript** (`🟦️component.ts`, L10):
```typescript
export type DirectoryActorKind = "user" | "admin" | "system";
```

**Rust** (`🦀️component.rs`, schema L29):
```rust
pub use schema::{DirectoryActorKind, /* ... */};
// Auto-generated in schema/🦀️component.rs as serde enum
```

### **To Add `DirectoryActorKind::Agent`**

1. **Edit schema JSON** (`🔣️component.json` L10):
   ```json
   "DirectoryActorKind": { "type": "string", "enum": ["user", "admin", "system", "agent"] }
   ```
   - **Handcrafted** — no generate target

2. **Generated Rust** (`🧬️schema/🦀️component.rs`):
   - Auto-generated from JSON; enum will update on next schema gen
   - Check for `generate` target in `/🔨️modules/📇️directory/📋️project.json`

3. **Generated TypeScript** (`🧬️schema/🟦️component.ts`):
   - Auto-generated union type; TS will update

4. **No other files needed** — the generated twins are exhaustive.

**Generation check** (bash): `grep -n "generate\|schema" /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/📋️project.json`

---

## 4. DirectoryEvent, DirectoryCommand, DirectoryStreamMessage Variants & Streaming

### DirectoryEvent Schema (L131–145, component.json)
- **Base fields**: `seq` (1-based), `id` (uuid v7), `hlc`, `actor` (DirectoryActor), `body` (discriminated), `recordedAtMs`
- **Optional fields**: `spaceId`, `userId`

### DirectoryEventBody Variants (L29–130, component.json)
All variants in the schema `oneOf`:
1. `"user.created"` — `{ userId, email, displayName }`
2. `"space.created"` — `{ spaceId, name, spaceKind, visibility, ownerUserId }`
3. `"space.renamed"` — `{ spaceId, name }`
4. `"space.visibility-changed"` — `{ spaceId, visibility }`
5. `"space.archived"` — `{ spaceId }`
6. `"space.deleted"` — `{ spaceId }`
7. `"member.upserted"` — `{ spaceId, userId, role }`
8. `"member.removed"` — `{ spaceId, userId }`
9. `"invite.redeemed"` — `{ spaceId, userId, inviteId, role }`

**Folded variants** (TS `fold`, L91–146 in `🟦️component.ts`; Rust twin identical):
- All 9 body variants above are folded into the projection

### DirectoryStreamMessage Variants (L272–316, component.json)
1. `{ kind: "event", event: DirectoryEvent }` — streams all 9 event kinds
2. `{ kind: "connection", phase: "opened"|"closed", connection: ConnectionView }` — ephemeral
3. `{ kind: "presence", spaceId, documentId, actors: DirectoryPresenceActor[] }` — from hub presence (C7 amendment 3)
4. `{ kind: "heartbeat", headSeq }` — keep-alive

**Streamed over directory WebSocket** (L1067, bin.rs `handle_directory_ws`):
- Path: `GET /directory/ws?token=&since=`
- All four message kinds published via `state.directory_service.publish(DirectoryStreamMessage::*)` at various points
- `event` kind published for every 9 body variants (L1037–1050, `visibility_filter_events`)

### DirectoryCommand Variants (L146–242, component.json)
1. `"create-space"` — `{ name, spaceKind, visibility }`
2. `"rename-space"` — `{ spaceId, name }`
3. `"set-visibility"` — `{ spaceId, visibility }`
4. `"archive-space"` — `{ spaceId }`
5. `"delete-space"` — `{ spaceId }`
6. `"upsert-member"` — `{ spaceId, email, role }`
7. `"remove-member"` — `{ spaceId, userId }`
8. `"create-invite"` — `{ spaceId, role, ttlSecs }` → response `{ inviteToken }`
9. `"revoke-invite"` — `{ spaceId, inviteId }`

---

## 5. Presence: Actor Identity Grammar & Session Colors

**ActorId format** (L956, 1003, 1707 in bin.rs):
```rust
format!("user:{}#{}", user_id, shell_session_id)
// Example: "user:u-alice#react-session-1"
```
- Prefix `"user:"` indicates kind (no separate enum for wire)
- `user_id` follows
- `#` + shell session label (e.g., react tab id, wgpu process uuid)

**ArtifactPresencePeer** (`🧰️framework/🛍️products/💻️os/🟦️component.ts` L351–371):
```typescript
export type ArtifactPresencePeer = {
  actor: string;              // "user:u-alice#react-1"
  connectedAtMs: number;
  label?: string;
  presencePack?: readonly number[];
  userId?: string;
  role?: string;
  color?: number;             // hub-assigned palette index (0–255)
  surface?: string;           // canonical surface id
  views: readonly ArtifactPresenceWindowView[];
  ui?: ArtifactPresenceUi;
  // ...
};
```

**Session Colors** (contract C7.3, SHARED-PRESENCE ticket):
- `HubState.session_colors: DashMap<space_id, SpaceColors>`
- `SpaceColors { by_actor: BTreeMap<String, ColorLease { index: u8, refs: u32 }> }`
- `acquire(space, actor) -> u8`: lowest free index 0..255 per space; reuse `n % 256` if > 256 live
- `release(space, actor)`: ref-count; drop at 0
- **Never persisted** — ephemeral
- Acquired after `Hello`/auth; released at disconnect
- Hub sends `ServerFrame::Session { actor, color }` after `Welcome` (C7.2)

**DirectoryPresenceActor** (C7 amendment, schema L261–270):
```json
"DirectoryPresenceActor": {
  "actor": "string",         // "user:u-alice#react-1"
  "userId": "string",        // optional
  "surface": "string",       // canonical surface id
  "color": "integer (0–255)"
}
```
- Published in `DirectoryStreamMessage::Presence` on roster changes
- Hub knows all four fields without decoding peer bytes

---

## 6. DirectoryClient: Public Methods & REST Endpoints

**File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️component.ts` (L4061–4199)

### Public Methods & Endpoints

| Method | Endpoint | Returns | Auth |
|---|---|---|---|
| `mintSession(email)` | `POST /auth/sessions` | `{ token, userId }` | None |
| `me()` | `GET /auth/sessions/me` | `{ userId, email, displayName, expiresAt } \| null` | Bearer |
| `spaces()` | `GET /directory/spaces` | `readonly SpaceView[]` | Bearer or public |
| `space(id)` | `GET /directory/spaces/{id}` | `DirectorySpaceDetail` (members, docs, invites) | Bearer or public |
| `command(cmd)` | `POST /directory/commands` | `{ events: DirectoryEvent[], result? }` | Bearer |
| `events(since)` | `GET /directory/events?since=` | `readonly DirectoryEvent[]` | Bearer or visibility-filtered |
| `stream(since, onMsg)` | `GET /directory/ws?token=&since=` (WebSocket) | `DirectoryStream` (live subscription) | Bearer or none |

**Auth patterns**:
- Constructor stores optional bearer token (set by `mintSession`)
- `headers()` method includes `Authorization: Bearer ${token}` if token present
- WebSocket `stream()` passes token as query param `?token=`

---

## 7. Churn Warning: Git History & Live Ticket Status

### bin.rs Recent Commits
```
101a6b4ea8 🐙️ueli🎆️26🌙️06☀️04🚩️528
0b9f1d3a04 🐙️ueli🎆️26🌙️06☀️04🚩️525
20252aa16d 🐙️ueli🎆️26🌙️06☀️04🚩️496
c31024cc6c 🐙️ueli🎆️26🌙️06☀️04🚩️480
daee507d43 🐙️ueli🎆️26🌙️06☀️04🚩️466
```
- Heavy churn on auth flow (likely 528, 525 related to session/admin)

### Directory Module Recent Commits
```
0b9f1d3a04 🐙️ueli🎆️26🌙️06☀️04🚩️525  [ONLY ENTRY]
```
- Last significant change ~525 ago (relative age)

### Live Tickets Rewriting Hub/Directory

#### 1. **FINISH-HUB-SPACES-COLLABORATION-END-TO-END** (26/08/17)
- **Status files**: `📓️busy-fix-report.md`, `📓️directory-lane-report.md` (both 2026-08-17)
- **Scope**: Contract C0–C6 (hub HTTP/WS, directory schema, membership, invites, share tokens)
- **Files modified**: No explicit `📓️status.md` (no tracking file); reports indicate **active concurrent work**
- **Collision risk**: ⚠️ **HIGH** — this ticket is the directory foundation; agent principal sits atop it

#### 2. **SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION** (26/08/17)
- **Status files**: `📓️0-h1-inventory.md`, `📓️0-h2-concurrency.md`, `📌️important.md` (1.7KB, non-empty)
- **Scope**: Contract C7 (presence, session colors, universal artifact creation)
- **Files modified**: Mostly `🟦️component.ts`, `🦀️bin.rs` for presence streaming + session colors
- **Collision risk**: ⚠️ **MEDIUM** — touches `DirectoryPresenceActor`, `DirectoryStreamMessage::Presence`, session color acquisition in bin.rs

#### 3. No active work on:
- `📇️directory/🧬️schema/🔣️component.json` (schema is stable; the JSON is handcrafted)
- `DirectoryActorKind` enum definition itself

---

## Consequences for the Agent Principal

### Minimal Edit Set: Files + Regions

#### **1. Schema (Handcrafted)**
- **File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️component.json`
- **Line 10**: Add `"agent"` to `DirectoryActorKind` enum
- **Region**: No region marker in JSON; one-line edit inline

#### **2. Directory Model (Handcrafted)**
- **File**: `/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️component.rs`
- **Region**: `//#region 🔖️Model` (L36–157)
- **Insert after `InviteRecord`** (after L156): New `AgentSessionRecord` struct
  ```rust
  /// @emoji 🤖️ An LLM agent acting through the OS MCP gateway
  pub struct AgentSessionRecord {
      pub id: String,           // session token
      pub agent_label: String,  // e.g. "claude-opus-2025-01-01"
      pub owner_user_id: String,
      pub scopes: Vec<String>,  // e.g. ["read:spaces", "write:artifacts"]
      pub created_at: i64,
      pub expires_at: i64,
  }
  ```

#### **3. Mint Routes (Handcrafted)**
- **File**: `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- **Region**: `//#region 🔖️Rest` (L337+)
- **Add two new handlers**:
  1. `create_agent_session_admin` (admin-level) — POST `/admin/api/agents/sessions`
     - Input: `{ agentLabel, ownerUserId, scopes, ttlSecs }`
     - Output: `{ token, agentSessionId }`
     - Auth: `is_admin`
  2. `create_agent_session_member` (space-member-level) — POST `/directory/agents/sessions`
     - Input: `{ agentLabel, spaceId, scopes, ttlSecs }`
     - Output: `{ token, agentSessionId }`
     - Auth: Bearer session + author role in space

#### **4. Hub Directory Trait (Handcrafted)**
- **File**: `/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️component.rs`
- **Region**: `//#region 🔖️Trait` (after L500)
- **Add trait methods**:
  ```rust
  async fn create_agent_session(&self, label: &str, owner_user_id: &str, scopes: &[&str], ttl_secs: i64) -> DirectoryResult<AgentSessionRecord>;
  async fn get_agent_session(&self, session_id: &str) -> DirectoryResult<Option<AgentSessionRecord>>;
  async fn revoke_agent_session(&self, session_id: &str) -> DirectoryResult<()>;
  ```

#### **5. Audit Event (Schema + Projection)**
- **File**: `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️component.json`
- **Location**: `DirectoryEventBody.oneOf` (after L128, invite.redeemed)
- **New variant**:
  ```json
  {
    "type": "object",
    "additionalProperties": false,
    "required": ["kind", "agentSessionId", "agentLabel", "ownerUserId", "scopes"],
    "properties": {
      "kind": { "const": "agent.session-created" },
      "agentSessionId": { "type": "string" },
      "agentLabel": { "type": "string" },
      "ownerUserId": { "type": "string" },
      "scopes": { "type": "array", "items": { "type": "string" } }
    }
  }
  ```
- **Projection**: Update TS/Rust `fold` to handle (no-op for read model; audit log only)

#### **6. REST Routes (bin.rs)**
- **File**: `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- **Region**: `//#region 🔖️Main` (L1488–1520)
- **Add two routes**:
  ```rust
  .route("/admin/api/agents/sessions", post(create_agent_session_admin))
  .route("/directory/agents/sessions", post(create_agent_session_member))
  ```

---

### Risk List: Collisions with Live Tickets

| Collision | Severity | Mitigation |
|---|---|---|
| `FINISH-HUB-SPACES-COLLABORATION-END-TO-END` is **actively rewriting** `/directory/ws`, `AuthOutcome`, membership model | 🔴 HIGH | Freeze agent work until that ticket's directory lane ✓; agent principal does NOT touch shared-file routes (C0–C2 are final) |
| `SHARED-PRESENCE-SESSION-COLORS` touches `DirectoryStreamMessage::Presence` schema + bin.rs presence routing | 🟡 MEDIUM | Agent principal adds only `DirectoryStreamMessage::Agent` (new kind), no modification to existing kinds; parallel changes safe if both edit disjoint regions |
| Both tickets touch `/auth/sessions` routes (FINISH-HUB for session mint, SHARED-PRESENCE for identity) | 🟡 MEDIUM | Agent principal adds `/directory/agents/sessions` + `/admin/api/agents/sessions` (disjoint paths); no collision |
| Schema regeneration (TS/Rust from JSON) may overwrite hand-edits | 🔴 HIGH | Confirm generation target in `📋️project.json`; if automated, apply edits to JSON schema only |

**Blockers**:
1. **FINISH-HUB-SPACES-COLLABORATION-END-TO-END** must complete `DirectoryActor` enforcement (currently only `{ kind: "user", id }` used; `"agent"` variant unused until first agent route posts an event)
2. **Schema generation pipeline** must be confirmed (hand-edit JSON only, or full generation cycle?)

---

## Summary

- **AuthOutcome**: 4 variants (Session, ShareToken, Public, Denied); fallback chain: bearer → share-token → public → denied
- **Admin**: bearer token or loopback IP (dev default, logged at startup)
- **Routes**: 23 public + admin endpoints; three layers of auth (session, share-token, public visibility)
- **Records**: AuthSessionRecord (30d TTL), SyncSessionRecord (ephemeral), ShareTokenRecord, InviteRecord (secret not in log)
- **DirectoryActorKind**: `["user", "admin", "system"]` — add `"agent"` to JSON schema enum, auto-gen twins
- **Presence**: Actor ID `"user:{user_id}#{label}"`; colors 0–255 per space, acquired on `Hello`, released on disconnect
- **DirectoryClient**: 7 public methods (mint, me, spaces, space, command, events, stream)
- **Churn**: High risk on FINISH-HUB auth flow & directory; medium risk on SHARED-PRESENCE presence streaming

**Agent principal can start after**:
1. FINISH-HUB-SPACES ticket completes directory lane (unblocks `DirectoryActor::Agent` use)
2. Confirm schema generation target (hand-edit JSON or full pipeline?)
3. Freeze agent edits to: schema JSON (1 line), new `AgentSessionRecord`, two new routes, one new audit event kind
