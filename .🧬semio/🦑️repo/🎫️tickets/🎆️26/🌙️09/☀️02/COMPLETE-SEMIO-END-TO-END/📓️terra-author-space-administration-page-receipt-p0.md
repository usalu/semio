# Author Space Administration Page/Receipt P0

Status: source audit only, 2026-09-05. No production or test source was changed and no build was run.

## Decisive current RED

There is no author-scoped administration read API. `GET /directory/spaces/{id}` is registered at [`bin.rs:5525-5531`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5525) and `get_directory_space` at [`bin.rs:3936-3959`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3936) folds the whole directory into `DirectoryReadModel`, takes the caller role from that in-process fold, and returns complete member/document/invite vectors. It neither revalidates a role against the durable backend after the read nor has a cursor, response-size ceiling, snapshot revision, or receipt.

The leakage is also earlier than the present response mapper. The generic directory contract exposes `list_members` and `list_invites` at [`directory.rs:2083-2086,2164-2178`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2083). SQLite's bounded *global-admin* member page still constructs `UserRecord`, including password/SSO columns ([`sqlite.rs:915-936`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:915)); ordinary `list_members` is unbounded ([`sqlite.rs:939-954`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:939)). Its invite list reads selector and secret digest before `invite_view` redacts it ([`sqlite.rs:1306-1311`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1306)). PostgreSQL has the same patterns ([`postgres.rs:950-979,1425-1433`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:950)); Neo4j returns full user/invite nodes ([`neo4j.rs:745-780,1250-1257`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:745)). Current response DTOs contain redacted `InviteView`, but converting a secret-bearing record after it has crossed the backend boundary is not a suitable administration-page authority.

The global system-admin pagination helpers are useful size-limit precedent, not an authorization primitive. Their offsets and MAC bind an `AdminPrincipalV1` ([`bin.rs:4510-4574`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4510)); routing `authors` through them would turn a space role into global-admin authority. `ADMIN_PAGE_MAX=100`, fetch=101, and a 64 KiB response cap are already canonical hub limits ([`directory.rs:360-371`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:360)).

## Canonical P0 boundary

Add a new read-only endpoint, leaving the legacy detail route and global `/admin/api/*` unchanged:

```
GET /directory/spaces/{spaceId}/administration/v1?lane=members|invites&cursor=<opaque>
```

It accepts no client-selected limit. Each response carries at most 100 rows, reads at most 101 backend rows, and is at most 64 KiB canonical UTF-8. A single oversized safe row returns `413` with no partial page; malformed/mismatched cursor returns `400`; a changed administration revision returns `409` with no page; an invalid, expired, or revoked bearer returns `401`; no current `Author` role returns `404` (both membership loss and nonexistence stay non-enumerating); storage timeout/unavailability returns `503` with no receipt. A response authorized at the backend read's linearization point remains a valid one-time observation; it must never permit a later cursor/read after removal or role downgrade.

Define one new Rust/TypeScript schema-first value, `DirectoryAuthorAdministrationPageV1`, in the existing OS directory schema pair:

```
schema: "semio.directory.author-administration-page.v1"
sessionBindingSha256
authorizationGeneration
spaceId
administrationRevision
lane: "members" | "invites"
rows: DirectoryAuthorAdministrationRowV1[]
nextCursor?: string
receiptSha256
```

`DirectoryAuthorAdministrationRowV1` is a closed tagged union: `member { userId, email, displayName, role }` or `invite { id, role, createdAtMs, expiresAtMs, revoked, accepted }`. Validation requires rows to match `lane`, monotonic keyset order, nonempty bounded text, safe integers, the exact schema string, lower-case 64-hex hashes, `1..=100` rows, and the canonical SHA-256 receipt over every field except `receiptSha256`. No selector, secret digest, capability, session ID, password hash, SSO subject/provider, audit reason, or raw authorization object exists in this type.

Use the domain-separated SHA-256 frame
`semio.directory.author-administration-page/session-binding/v1\\0`
followed by length-prefixed private session id, user id, authorization generation, space id, lane, and administration revision for `sessionBindingSha256`. The client receives only the digest. A separate HMAC cursor key held by `HubState` authenticates cursor payloads; it is not a replacement for the backend role check.

Keyset cursors—not the existing offset cursor—must encode and MAC the version, lane, session id, user id, authorization generation, space id, immutable administration revision, and the lane's last sort key. Member order is `user_id ASC`; invite order is `(created_at_ms DESC, invite_id DESC)`. Only sort keys/revision need be in clear cursor payload; all identity bindings are MAC input. Thus an insert/delete cannot shift an offset page, a cursor cannot cross identity/session/scope/lane, and an old cursor cannot form a mixed administration snapshot.

## Required authoritative backend read

Add a closed `HubDirectory::read_author_administration_page(&AuthorAdministrationReadRequest) -> DirectoryResult<AuthorAdministrationReadPage>` beside the space/auth/invite trait regions, and delegate it from `HubDirectories`. It receives only backend-trusted data from the route: session id, user id, authorization generation, `now_ms`, space id, lane, optional pinned revision, optional keyset key, and fixed fetch limit 101. It returns one of `Authorized { revision, safe_space, safe_rows, next_key }`, `NotFound`, `Unauthenticated`, `NotAuthor`, or `StaleRevision`. It must not return `UserRecord` or `InviteRecord`.

Within one backend read transaction/snapshot it must:

1. Read the exact auth-session id and require the same user, generation, unrevoked state, and `expires_at > now_ms`.
2. Read the caller's current membership and require `SpaceRole::Author` (not merely a member and never the old `DirectoryReadModel`).
3. Read the space and its durable `administration_revision`; cursor revision, if present, must equal it.
4. Read exactly 101 *safe projected* lane rows under the fixed keyset ordering, then return 100 plus an internal next key.

Add `hub_space_administration_revision(space_id PRIMARY KEY, revision)` (or an equivalent non-null `hub_space` field) and bump it inside the same durable transaction that changes the author-visible set: space creation, `member.upserted`, `member.removed`, successful invite redemption/membership projection, direct invite issue, and direct invite revoke. The existing directory `head_seq` cannot pin this snapshot: `CreateInvite`/`RevokeInvite` deliberately have no events ([`directory.rs:1479-1488,2164-2168`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1479)). A global head also does not express a per-space author page.

SQLite should hold the existing connection lock for the complete bounded read; PostgreSQL should execute the auth/role/revision/page query in one read-only repeatable-read transaction; Neo4j should use one explicit read transaction/query scope rather than two `graph.execute` calls. All three query only display columns for members and invite metadata for invites. Do not reuse `socket_session_binding`: it reports any role and is a separate query ([`directory.rs:2143-2149`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2143)).

The HTTP handler does only: strict route/query validation; `resolve_bearer_user` ([`bin.rs:3576-3604`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3576)); cursor MAC decode; one deadline-bounded call to that backend operation; canonical page construction; deterministic byte fitting; cursor HMAC issuance; and response. Drop/timeout of this pure read produces no durable side effect. Put `tokio::time::timeout` around the single operation and abandon no retained resource; the fixed 101-row query bounds work even where the driver cannot interrupt a running SQL/Cypher query.

## Ordered independent implementation slices

1. **Shared wire law owner** — add Rust/TS schema, JSON schema and a directory `📜️script.ts` command/fixture. The fixture has valid member/invite pages, empty final page, 101-row probe/next cursor, canonical receipt, and hostiles for wrong lane-row tag/order, duplicate/bad key, out-of-range fields, secret-shaped extra field, altered binding/generation/revision, receipt mismatch, unknown field, bad/foreign/stale cursor. Use the project's existing Bun + AJV canonical JSON/`node:crypto` SHA-256 parity, then Rust parse/canonical/validation cases. This is the language-agnostic gate.
2. **Directory backend owner** — introduce private safe row structs and `AuthorAdministrationReadRequest/Page`; implement the single operation in SQLite, PostgreSQL, Neo4j and the `HubDirectories` dispatch. Add the revision storage/migration-free initial DDL projection alongside current backend schemas, and bump it at the listed transaction owners. Add one backend-generic contract suite run against all configured backends: author success, spectator/nonmember denial, expired/revoked/generation-mismatched session denial, cursor scope/lane/user rejection, keyset insert/delete no duplicate/skip, stale revision after role/invite mutation, 101 cap, safe-column proof, and rollback leaves revision/page unchanged.
3. **Hub route owner** — add the endpoint and an `AuthorAdministrationPageSource` test seam in `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`, modeled on `DirectoryEventPageSource` but no global/model fallback. Reuse only the response byte-fitting mechanics from `admin_fit_page`; create a separate author cursor encoder/MAC. Native route tests must drive real bearer sessions and real SQLite state, then cover cancellation/deadline (no receipt), post-read role removal racing the next cursor, tampered cursor, 413 oversized row, and no secret bytes in canonical response.
4. **Administration UI owner, later** — both React and WGPU call this read endpoint through the bounded DirectoryClient receipt transport packet; they render from canonical page bytes and never treat `DirectorySpaceDetailV1::Author` as the page. This report intentionally does not change the active command-result, event-page/Home, presence, Flow, or Stdio lanes.

## Non-vacuous process law

Launch a real hub with each available directory backend and two independently authenticated users. Author A obtains the first member and invite pages. Spectator B and a user from another space receive no administration page. Insert/remove between A's first and second cursor: the old cursor receives `409`, and a fresh page has no duplicate/missing sorted row. Downgrade or remove A: the next request returns `404`; revoke/expire A's session returns `401`; neither response includes page bytes. Rotate/restart the hub: persisted revision makes an old cursor stale only when its data changed, otherwise it resumes exactly. Capture raw HTTP body and assert neither invite selector/digest/token nor password/SSO fields occur.

This P0 is a bounded read/receipt, not a command transport, invite-token replay solution, event-page client, scoped-socket revocation implementation, or presence lease. No native/process result exists yet.
