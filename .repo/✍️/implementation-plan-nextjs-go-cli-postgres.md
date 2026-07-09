# Implementation Plan: Go CLI + Next.js Server + PostgreSQL + pg-boss

## 1. Goal

Refactor the current system into:

- a **Go CLI** that remains the only client-side executable,
- a **Next.js server** that contains the web app, dashboard, auth, API, and admin pages,
- a **PostgreSQL database** as the single durable store for all non-temporary state,
- a **pg-boss** queue for background work,
- a **Discord bot / Discord integration** that posts a message for every event,
- a **Docker Compose** deployment for a Linux VM.

The new design must ensure that the **publicly accessible server only accepts requests from trusted developers**, and that **all durable history currently living under `.repo` is migrated into PostgreSQL**.

---

## 2. Scope and non-negotiable requirements

### 2.1 Hard requirements from the brief

1. **CLI stays in Go.**
2. **Server becomes Next.js.**
3. **Server stack uses PostgreSQL and pg-boss.**
4. **Server is publicly reachable.**
5. **Server must only accept requests from trusted developers.**
6. **Everything durable currently under `.repo` must move to PostgreSQL.**
7. **Client keeps only:**
   - cache,
   - temporary folders,
   - prompt files.
8. **Ticket workflow requirement:**
   - opening a ticket creates a temporary folder under `.repo/🎫/{{ticket-id}}`,
   - closing a ticket uploads the full temporary folder to the server,
   - then removes it from the client.
9. **All logging moves to the server side.**
10. **Server also posts all events to Discord channels.**
11. **Docker Compose must set up the full server environment.**
12. **Existing history must be migrated into PostgreSQL.**
13. **When old data formats differ, attempt conversion; otherwise drop invalid/unmappable records.**
14. **System must be tested before deployment on a Linux VM.**

### 2.2 Constraints inferred from the uploaded codebase

- The current CLI already has extensive ticket, GraphQL, MCP, tree/search, and hook/session behavior and stores ticket state locally as JSON/files under repo metadata paths. This is visible in the CLI command wiring and ticket persistence helpers. The CLI currently persists tickets via `SaveTicket`, reads them via `ReadTicket`, and emits ticket lifecycle events. It also keeps drafts in a repo meta path.
- The current server is a Go HTTP server with SQLite, bearer-token auth, event persistence, ticket endpoints, diff ingestion, indexing endpoints, GitHub webhook handling, and Discord webhook notifications.
- The current server auth is only a single bearer token check and even allows unauthenticated access if no token is configured.
- The spec treats tickets, sessions, interactions, commands, hooks, and events as first-class domain objects and explicitly models `.repo` event files as persisted history.

---

## 3. Current-state assessment

### 3.1 CLI today

The Go CLI already has:

- a large Cobra command surface,
- GraphQL-backed command dispatch through an internal engine,
- ticket open / close / reopen flows,
- local ticket JSON persistence,
- local draft storage,
- local session / hook behavior,
- repo tree scanning and search,
- event emission.

This means the CLI should be **refactored, not replaced**. The migration strategy should keep its command UX as stable as possible and swap local persistence / local logging for server-backed APIs.

### 3.2 Server today

The current Go server provides:

- HTTP endpoints such as `/tickets/open`, `/tickets/close`, `/tickets/reopen`, `/warnings`, `/breachs`, `/scopes`, `/repo/reindex`, `/repo/index-file`, and `/events`,
- SQLite-backed persistence for tickets, scopes, claims, warnings, breaches, contributor work, and events,
- a bearer-token based `requireAuth` check,
- Discord webhook notifications for ticket lifecycle and CLI events,
- GitHub webhook handling.

This confirms the core domain already exists, but the implementation must move from **Go + SQLite** to **Next.js + PostgreSQL + pg-boss**.

### 3.3 Durable history today

The spec and CLI implementation show that `.repo` currently acts as a persistent local data store for:

- tickets,
- interactions,
- sessions,
- events,
- drafts,
- other durable metadata.

That is precisely what must be removed from the client and migrated to PostgreSQL, while preserving only caches, temporary ticket workspaces, and prompt files locally.

---

## 4. Target architecture

## 4.1 High-level architecture

```text
Go CLI
  -> HTTPS
Next.js app/server
  - dashboard
  - auth
  - admin pages
  - REST/JSON API
  - Discord event publisher
  - webhook endpoints
  - pg-boss producer endpoints / worker integration
  -> PostgreSQL
  -> pg-boss tables in PostgreSQL
  -> object storage on disk/S3-compatible store for uploaded ticket folders
```

## 4.2 Components

### A. Go CLI

Responsibilities:

- authenticate to the server,
- keep a tiny local state footprint only,
- manage temporary ticket folders under `.repo/🎫/{{ticket-id}}`,
- keep prompt files locally,
- upload close payloads and archives to the server,
- stream user-intent and execution events to the server,
- read dashboard-relevant and ticket-relevant state from server APIs.

### B. Next.js server

Responsibilities:

- render dashboard, admin pages, and developer UI,
- enforce authN + authZ,
- expose server APIs for the CLI,
- persist domain state in PostgreSQL,
- enqueue background jobs with pg-boss,
- ingest uploaded ticket folders,
- normalize and persist events,
- send Discord notifications,
- run migration jobs,
- expose operational health endpoints.

### C. PostgreSQL

Single source of truth for:

- users / developers / API keys / device registrations,
- tickets,
- prompts,
- summaries,
- interactions,
- sessions,
- commands,
- hooks,
- event log,
- warnings,
- breaches,
- scopes,
- contributor work,
- uploaded artifact metadata,
- migration ledger,
- audit log,
- queue data for pg-boss.

### D. Background jobs with pg-boss

Used for:

- ticket folder ingestion,
- archive extraction,
- event fanout,
- Discord delivery retries,
- repository indexing,
- history migration batches,
- nightly cleanup of expired temporary server-side upload staging,
- metrics aggregation jobs.

### E. File/blob storage

PostgreSQL should hold metadata and durable structured data.

Large ticket close payloads should be stored as:

- either **filesystem-backed object storage** on the Linux VM under a managed volume,
- or an S3-compatible bucket if available.

Recommendation for first deployment:

- keep structured metadata in PostgreSQL,
- keep uploaded bundle files / tarballs / extracted snapshots under `/srv/compose/blob` mounted into the container,
- store checksum, path, size, MIME type, and retention metadata in PostgreSQL.

---

## 5. Technology choices

## 5.1 Server runtime

- **Next.js 15+ (App Router)**
- **TypeScript**
- **Node.js 22 LTS**

### Why

- Single codebase for dashboard, auth, admin pages, and API routes.
- Good fit for internal web UI plus public API surface.
- Easy integration with PostgreSQL and background workers.

## 5.2 Database access

Recommended:

- **PostgreSQL 16+**
- **Drizzle ORM** for schema + migrations + typed queries
- Raw SQL for heavy migration/import routines

Reason:

- You explicitly want the canonical schema in `repo/postgres/schema.sql`.
- Drizzle can coexist with a checked-in canonical SQL file.
- Migration-heavy systems benefit from keeping SQL visible and reviewable.

## 5.3 Queue

- **pg-boss**

Reason:

- Uses PostgreSQL as the queue backend.
- Avoids adding Redis/Kafka for this scale.
- Good match for a small team and a single VM.

## 5.4 Validation and API contracts

- **Zod** for request validation
- **OpenAPI generation** from route schemas where practical

## 5.5 Auth

Use a layered model, not only a bearer token:

1. **Developer identity** in PostgreSQL
2. **Short-lived JWT access tokens**
3. **Long-lived refresh tokens** for web app only
4. **CLI API keys or signed device tokens** for Go CLI
5. **Developer allowlist** in PostgreSQL
6. Optional **IP allowlist** for admin endpoints
7. Optional **mTLS at reverse proxy** if your deployment environment allows it

For your requirement “publicly accessible but only trusted developers”, the minimum acceptable design is:

- reverse proxy on the public internet,
- all API routes require an authenticated developer principal,
- developer principal must be both **active** and **trusted=true** in DB,
- CLI uses a generated API key bound to a developer account and optionally device fingerprint,
- admin routes additionally require `role in ('admin','owner')`.

## 5.6 Discord integration

Prefer a real bot abstraction over one bare webhook.

Implementation:

- `discord_channels` table mapping event kinds to channels,
- publisher service that creates a normalized embed/message payload per event,
- delivery job via pg-boss,
- retries with exponential backoff,
- dead-letter table for failed sends.

If you want the fastest path, start with webhook-based channel routing. If you need richer channel targeting and community server features, use a bot token and channel IDs.

---

## 6. Security architecture

Because the server is public, security is part of the implementation plan, not an afterthought.

## 6.1 Authentication model

### Web dashboard users

- Email magic link or OAuth (GitHub) sign-in
- Account must map to a trusted developer record
- Session cookie for web app

### Go CLI users

Use one of these two patterns.

#### Preferred

- `repo auth login` opens a browser/device flow
- user authenticates in the web app
- server issues a CLI device token and refresh capability
- CLI stores encrypted credentials in OS keychain if available

#### Simpler v1

- admin creates API keys for each trusted developer
- CLI stores the key locally in OS keychain / config
- every request sends:
  - `Authorization: Bearer <api-key>`
  - `X-Client-Version`
  - `X-Device-Id`

## 6.2 Authorization model

Create a `developers` table with fields like:

- `id`
- `email`
- `github_login`
- `display_name`
- `trusted`
- `active`
- `role`
- `discord_user_id`
- `created_at`
- `revoked_at`

Every API request resolves to a developer and is allowed only when:

- `active = true`
- `trusted = true`
- token/key not revoked

## 6.3 API hardening

- TLS terminated at reverse proxy
- Rate limiting for all auth and upload endpoints
- Body size limits
- Zod validation for every request
- Request ID on every request
- Audit log for auth/login/admin actions
- CSRF protection for session-based web actions
- Secret management through `.env` + VM secret injection
- No fallback “allow all when token missing” behavior in production

## 6.4 Network layout

Public:

- `443/tcp` only via nginx or Caddy

Private/internal containers:

- Next.js app
- worker
- postgres

Do not expose PostgreSQL directly.

---

## 7. Target data model

The canonical durable schema must live under:

- `repo/postgres/schema.sql`

That file should become the source of truth for core relational structure.

## 7.1 Core tables

### Identity / auth

- `developers`
- `developer_api_keys`
- `developer_devices`
- `sessions_web`
- `audit_log`

### Domain core

- `repos`
- `contributors`
- `goals`
- `tickets`
- `ticket_prompts`
- `ticket_summaries`
- `ticket_interactions`
- `ticket_files`
- `ticket_session_refs`
- `drafts`
- `todos`

### Eventing / trace

- `events`
- `event_payloads` or `events.payload_json` if kept inline
- `commands`
- `hooks`
- `sessions`
- `session_events`
- `checkpoints`
- `releases`
- `versions`

### Analysis / conflict detection

- `scopes`
- `ticket_claims`
- `warnings`
- `breaches`
- `contributor_work`

### Uploads / artifacts

- `artifacts`
- `artifact_files`
- `ticket_close_uploads`

### Operations

- `migration_runs`
- `migration_failures`
- `discord_deliveries`
- `metrics_rollups`

## 7.2 Storage strategy for uploaded ticket folders

For each closed ticket:

1. CLI compresses `.repo/🎫/{{ticket-id}}` to `tar.gz`.
2. CLI uploads manifest + archive.
3. Server stores:
   - metadata in PostgreSQL,
   - archive in blob storage,
   - extracted index entries optionally in PostgreSQL.
4. Server verifies checksum.
5. Server marks upload complete.
6. CLI deletes local temp folder only after success.

## 7.3 Event model

Every user action and system action should result in a persisted server-side event row.

Required event classes:

- auth events
- ticket opened / changed / closed / reopened
- prompt updated
- summary updated
- temp folder uploaded
- upload extracted
- migration imported / dropped
- session started / ended
- hook event ingested
- command executed
- warning created / acknowledged
- breach detected / resolved
- Discord message queued / sent / failed

This aligns with the spec’s explicit event and hook semantics and the current CLI/server event flows.

---

## 8. File-system policy after refactor

## 8.1 Client-side allowed local data

Allowed on the client:

- cache
- temporary ticket folders
- prompt files
- minimal auth config / keychain references

## 8.2 Client-side forbidden durable data

Must be removed from `.repo` durability model:

- durable ticket JSON
- session history
- hook logs
- event history
- warning history
- breach history
- contributor work ledger
- any durable summary or audit records

## 8.3 Proposed local layout

```text
.repo/
  cache/
  prompts/
  🎫/
    <ticket-id>/
      ...temporary working files only...
```

## 8.4 Server-side durable storage mapping

Map old local paths to server tables roughly as follows:

- `.repo/...tickets...` -> `tickets`, `ticket_interactions`, `ticket_files`, `ticket_summaries`
- `.repo/...events...` -> `events`, `session_events`
- `.repo/...drafts...` -> `drafts` or migrate only if still needed
- `.repo/...sessions...` -> `sessions`, `session_events`
- `.repo/...hooks...` -> `hooks`, `commands`, `events`

---

## 9. API design

Use REST/JSON for the Go CLI. Keep GraphQL only if the dashboard truly benefits from it; do not make the CLI depend on internal UI GraphQL.

## 9.1 CLI-facing endpoints

### Auth

- `POST /api/v1/cli/auth/login`
- `POST /api/v1/cli/auth/refresh`
- `POST /api/v1/cli/auth/logout`
- `GET  /api/v1/cli/auth/whoami`

### Ticket lifecycle

- `POST /api/v1/tickets/open`
- `POST /api/v1/tickets/{id}/prompt`
- `POST /api/v1/tickets/{id}/close`
- `POST /api/v1/tickets/{id}/reopen`
- `GET  /api/v1/tickets/{id}`
- `GET  /api/v1/tickets`

### Temporary workspace upload

- `POST /api/v1/tickets/{id}/upload-init`
- `PUT  /api/v1/uploads/{uploadId}`
- `POST /api/v1/tickets/{id}/upload-complete`

### Events / hooks / logs

- `POST /api/v1/events`
- `POST /api/v1/hooks`
- `POST /api/v1/commands`
- `GET  /api/v1/sessions/{id}`

### Analysis / status

- `GET /api/v1/warnings`
- `GET /api/v1/breaches`
- `GET /api/v1/scopes`
- `POST /api/v1/repo/reindex`

## 9.2 Web/dashboard endpoints

Use Route Handlers or server actions for:

- auth/session state
- admin management
- dashboard metrics
- event timeline
- ticket detail pages
- developer administration
- Discord routing configuration
- migration observability

---

## 10. Go CLI refactor plan

## 10.1 Preserve

Keep:

- Cobra command surface
- major subcommands
- Go runtime
- local path utilities for prompt files and temp folders
- local repo scanning where still needed for packaging uploads

## 10.2 Remove or redirect

Refactor these categories away from local durability:

- `SaveTicket` should stop writing durable ticket JSON as source of truth
- `ReadTicket` should become server-backed
- `ListTickets` should become server-backed
- local event persistence should become HTTP event ingestion
- local session/hook logs should become server ingestion
- local warning/breach storage should become read-only server data

## 10.3 Add new CLI modules

### Auth module

- login / logout / whoami
- OS keychain integration
- token refresh

### API client module

- typed request/response structs
- retry policy
- backoff for transient errors
- request IDs
- gzip support for uploads

### Ticket workspace module

- create `.repo/🎫/{{ticket-id}}`
- zip/tar workspace on close
- upload archive + manifest
- delete local temp folder on confirmed success
- retain folder on failed upload

### Event shipper

- send events synchronously for critical lifecycle actions
- send batched/noncritical events opportunistically
- local fallback spool only for transient offline mode, with explicit TTL; spool is temporary only and not a durable source of truth

## 10.4 Ticket workflow changes

### Open ticket

1. CLI calls `POST /tickets/open`.
2. Server creates durable ticket row and event row.
3. CLI creates `.repo/🎫/{{ticket-id}}` locally.
4. CLI may place prompt seed files into that folder.
5. CLI sends `ticket.workspace.created` event.

### Close ticket

1. CLI computes manifest for `.repo/🎫/{{ticket-id}}`.
2. CLI creates tar.gz.
3. CLI uploads archive.
4. CLI calls `POST /tickets/{id}/close` with summary + upload reference.
5. Server persists summary, closes ticket, creates events, queues Discord notifications, queues extraction/index job.
6. Only after 200/confirmed response, CLI deletes `.repo/🎫/{{ticket-id}}`.

### Reopen ticket

1. Server reopens ticket.
2. CLI recreates `.repo/🎫/{{ticket-id}}` if absent.
3. Optional prompt/draft seed download if needed.

---

## 11. Next.js server implementation plan

## 11.1 Repository layout

Recommended structure:

```text
repo/
  apps/
    web/
      app/
      components/
      lib/
      server/
      api/
  packages/
    db/
    shared/
    api-contracts/
  postgres/
    schema.sql
    migrations/
  docker/
  scripts/
```

## 11.2 Core server modules

### `lib/auth`

- session auth for web
- API key / JWT auth for CLI
- trusted developer guard
- admin guard

### `lib/db`

- postgres pool
- transaction helpers
- migrations

### `lib/events`

- event normalization
- event insert helpers
- event fanout to pg-boss

### `lib/discord`

- message formatters
- channel routing
- retry handling

### `lib/uploads`

- upload init / finalize
- checksum validation
- blob storage path assignment
- extraction orchestration

### `lib/tickets`

- ticket open/close/reopen services
- prompt and summary services
- file manifest handling

### `lib/analysis`

- scopes / claims / warnings / breaches logic
- diff ingestion
- indexing orchestration

### `worker`

A separate Node process in the same codebase that runs pg-boss consumers.

Jobs:

- `discord.send`
- `ticket.upload.extract`
- `ticket.upload.index`
- `repo.reindex`
- `migration.import.batch`
- `metrics.rollup`

## 11.3 Dashboard pages

Minimum pages:

- `/login`
- `/dashboard`
- `/tickets`
- `/tickets/[id]`
- `/events`
- `/sessions`
- `/warnings`
- `/breaches`
- `/contributors`
- `/admin/developers`
- `/admin/discord`
- `/admin/migrations`
- `/admin/system`

## 11.4 Admin capabilities

- manage trusted developers
- issue/revoke CLI API keys
- map event kinds to Discord channels
- trigger backfill jobs
- inspect failed Discord deliveries
- inspect failed migrations
- trigger repo reindex

---

## 12. Discord integration plan

## 12.1 Event-to-Discord rule

The requirement says **all events trigger a message to the Discord server**.

To avoid chaos, implement event routing tiers.

### Tier 1: all raw events persisted

All events go to PostgreSQL.

### Tier 2: all events produce a Discord delivery record

Every event produces a `discord_deliveries` row.

### Tier 3: channel routing policy

Route by event family:

- ticket lifecycle -> `#tickets`
- warnings / breaches -> `#quality`
- deploy / migration -> `#ops`
- hook/session activity -> `#activity` or an internal mod-only channel
- auth/admin/security events -> `#admin-audit`

## 12.2 Message design

Each message should include:

- event kind
- actor/developer
- ticket id if present
- concise summary
- deep link to dashboard page
- request id or event id

## 12.3 Reliability

- pg-boss queue job per delivery
- retries: 30s, 2m, 10m, 1h
- dead-letter after max attempts
- admin UI to replay failed deliveries

---

## 13. Migration plan for existing history

This is the most important engineering risk.

## 13.1 Migration principles

1. **Never mutate source data in place during import.**
2. **Import into PostgreSQL with an import ledger.**
3. **Mark every imported row with source path + import run id.**
4. **Attempt format conversion where possible.**
5. **Drop only records that are malformed or unmappable, and log every drop reason.**

## 13.2 Source categories to migrate

From `.repo` and any current server DB files:

- local ticket JSON files
- interaction records embedded in tickets
- session traces
- hook logs
- event JSON files
- drafts, if still needed
- current Go server SQLite data

## 13.3 Order of migration

### Phase A: schema-first import scaffolding

- create PostgreSQL schema
- create import ledger tables
- create mapping tables for old IDs -> new IDs if needed

### Phase B: import current Go server SQLite

Reason:

- current server DB is already relational and closest to target structure
- easiest first win for tickets, warnings, breaches, claims, and events

Import strategy:

1. dump SQLite
2. transform to PostgreSQL-compatible inserts
3. upsert into target tables
4. verify row counts and samples

### Phase C: import `.repo` durable files

For each category:

- scan files
- parse JSON/YAML/text
- normalize identifiers
- derive ticket/session/event metadata
- write to staging tables
- transform into final tables

### Phase D: import unmatched files into a quarantine ledger

For malformed items:

- record source path
- record reason
- do not block the overall migration

## 13.4 Data conversion rules

### Tickets

- preserve original ticket id / slug when possible
- derive canonical ticket id if current format is path-based
- prompt, summary, author, client, llm, status, created/closed times should follow spec semantics

### Sessions and interactions

- preserve session ids where possible
- enforce session-reference uniqueness
- preserve ordering by timestamp

### Events

- normalize event kinds to canonical strings
- preserve raw payload JSON in `events.payload_json`
- compute structured columns where possible

### Drafts

- keep only if still useful in product workflow
- otherwise export once to server artifact storage and mark deprecated

## 13.5 Migration tooling

Create:

- `scripts/migrate-sqlite-to-postgres.ts`
- `scripts/migrate-repo-history.ts`
- `scripts/verify-migration.ts`

The migration runner should be idempotent.

## 13.6 Drop policy

Drop only when all are true:

- file cannot be parsed,
- no partial structured data can be salvaged,
- record cannot be mapped to any canonical entity,
- drop reason is written to `migration_failures`.

---

## 14. Canonical schema.sql plan

Create `repo/postgres/schema.sql` with:

1. extensions
2. enums where useful
3. core tables
4. indexes
5. foreign keys
6. pg-boss install/init section if you want bootstrap automation
7. views for dashboard summaries

## 14.1 Must-have indexes

- `tickets(status, created_at desc)`
- `events(created_at desc)`
- `events(type, created_at desc)`
- `ticket_interactions(ticket_id, created_at)`
- `session_events(session_id, created_at)`
- `warnings(ticket_id, created_at desc)`
- `breaches(ticket_id, detected_at desc)`
- `developer_api_keys(hash, revoked_at)`
- `ticket_files(ticket_id)`
- `artifacts(ticket_id)`

## 14.2 Suggested PostgreSQL features

- `jsonb` for event payloads and migration metadata
- `timestamptz` everywhere
- `uuid` or ULID ids where appropriate
- `gin` indexes on selected `jsonb` fields if event querying needs it

---

## 15. Docker Compose plan

## 15.1 Services

```yaml
services:
 reverse-proxy:
  image: caddy:latest
 web:
  build: ./apps/web
 worker:
  build: ./apps/web
 postgres:
  image: postgres:16
 backup:
  image: postgres:16
```

Optional later:

- `prometheus`
- `grafana`
- `loki`

## 15.2 Volumes

- `postgres_data`
- `blob_data`
- `caddy_data`
- `caddy_config`

## 15.3 Environment variables

### Web / worker

- `DATABASE_URL`
- `NEXTAUTH_SECRET` or equivalent
- `APP_BASE_URL`
- `DISCORD_BOT_TOKEN` or webhook secrets
- `DISCORD_DEFAULT_GUILD_ID`
- `UPLOAD_BLOB_ROOT=/srv/compose/blob`
- `CLI_TOKEN_SIGNING_SECRET`
- `ENCRYPTION_KEY`
- `PG_BOSS_SCHEMA=pgboss`
- `TRUSTED_DEVELOPER_EMAIL_DOMAINS` optional

### Postgres

- `POSTGRES_DB`
- `POSTGRES_USER`
- `POSTGRES_PASSWORD`

## 15.4 Compose deployment behavior

- `web` waits for `postgres`
- `worker` waits for `postgres`
- migrations run before app start or as an init command
- reverse proxy forwards `443 -> web`

## 15.5 Linux VM prerequisites

- Docker Engine
- Docker Compose plugin
- DNS record
- firewall allowing only 80/443
- mounted disk sized for database + blobs + backups

---

## 16. Testing plan

The requirement says to test everything before deployment. This needs a formal test matrix.

## 16.1 Unit tests

### Go CLI

- auth config parsing
- API client request/response handling
- temp ticket workspace lifecycle
- tar/gzip creation and checksuming
- retry behavior
- failure behavior when upload fails
- deletion only after close confirmation

### Next.js/server

- auth guards
- trusted developer checks
- ticket open/close/reopen services
- event creation
- Discord payload formatting
- migration transforms
- upload manifest validation
- pg-boss handlers

## 16.2 Integration tests

Run against real PostgreSQL in containers.

Scenarios:

1. trusted developer can log in and call CLI APIs
2. untrusted developer is rejected
3. ticket open creates DB rows and local temp folder
4. ticket close uploads archive, persists summary, enqueues jobs, deletes local temp folder only on success
5. failed upload leaves local temp folder intact
6. every event creates DB event row and Discord delivery row
7. worker sends Discord messages and retries failures
8. migration imports valid legacy data
9. malformed legacy files are recorded in failure ledger and skipped
10. dashboard pages render expected metrics from imported data

## 16.3 End-to-end tests

Using Playwright for the web app and shell-driven CLI tests for Go CLI.

E2E suite:

- web login
- developer admin approval
- create API key
- CLI login
- open ticket
- create temp files
- close ticket
- verify dashboard shows ticket and artifacts
- verify Discord delivery row exists
- verify event timeline entries exist

## 16.4 Performance tests

Because fewer than 10 devs use the CLI, scale is small, but test:

- 100k historical event import
- large ticket archive upload (e.g. 200 MB)
- dashboard event timeline query performance
- pg-boss backlog draining performance

## 16.5 Security tests

- unauthorized API request blocked
- revoked key blocked
- inactive developer blocked
- role escalation blocked on admin routes
- oversized upload rejected
- invalid checksum rejected
- forged Discord callback or webhook rejected if applicable

## 16.6 Linux VM smoke test checklist

On a clean VM:

1. `docker compose up -d`
2. run migrations
3. import seed admin developer
4. login to dashboard
5. issue CLI key
6. run CLI open/close ticket flow
7. verify PostgreSQL rows
8. verify Discord message
9. verify restart persistence
10. verify backup restore procedure

---

## 17. Rollout strategy

## 17.1 Phased rollout

### Phase 0 – discovery and schema freeze

Deliverables:

- inventory of current `.repo` durable data
- mapping document old -> new schema
- `schema.sql` first version
- API contract draft

### Phase 1 – Next.js foundation

Deliverables:

- app shell
- auth
- developer allowlist
- postgres connection
- pg-boss worker
- admin pages for developers

### Phase 2 – ticket lifecycle APIs

Deliverables:

- open / close / reopen APIs
- event persistence
- upload workflow
- Discord delivery jobs

### Phase 3 – CLI refactor

Deliverables:

- Go API client
- auth flow
- temp folder workflow
- event shipping
- server-backed ticket operations

### Phase 4 – history migration

Deliverables:

- SQLite importer
- `.repo` importer
- verification reports
- quarantine ledger

### Phase 5 – dashboard and admin

Deliverables:

- ticket pages
- event pages
- warnings / breaches pages
- migration admin page
- Discord routing admin page

### Phase 6 – hardening and deployment

Deliverables:

- compose stack
- backup scripts
- VM smoke test
- operational runbook

## 17.2 Cutover

1. freeze writes in old server
2. run final migration delta
3. point CLI default base URL to new server
4. keep old server read-only for rollback window
5. after validation, decommission old Go server

---

## 18. Acceptance criteria

The work is complete only when all of the following are true.

### Functional

- Go CLI still works and remains the client entrypoint.
- Next.js server provides dashboard, auth, API, and admin pages.
- All durable non-temporary data is stored in PostgreSQL.
- Opening a ticket creates `.repo/🎫/{{ticket-id}}` locally.
- Closing a ticket uploads that folder to the server and removes it locally after success.
- All client-side logging has moved to server-side event persistence.
- Every event produces a Discord delivery.
- Existing history has been migrated or explicitly dropped with recorded reasons.

### Security

- Public server rejects unauthenticated and untrusted callers.
- Admin pages are restricted to admin roles.
- Production cannot run with auth effectively disabled.

### Operations

- Entire stack boots with Docker Compose.
- Full test suite passes in CI.
- Linux VM smoke tests pass.
- Backups and restore are documented.

---

## 19. Risks and mitigations

## 19.1 Risk: hidden `.repo` durable formats

Mitigation:

- run a pre-migration scanner that inventories every file extension and folder family under `.repo`
- classify into migrate / ignore / drop

## 19.2 Risk: CLI compatibility breakage

Mitigation:

- keep Cobra command names stable
- introduce server-backed implementation behind existing commands first
- add compatibility flags only where necessary

## 19.3 Risk: Discord spam

Mitigation:

- route all events, but separate channels by class
- provide admin muting / summarization rules for noisy event families

## 19.4 Risk: upload corruption

Mitigation:

- manifest + checksum + size verification
- delete local folder only after server acknowledgment

## 19.5 Risk: migration quality

Mitigation:

- import ledger
- sampled verification
- immutable source snapshot during migration
- rollback path via old server read-only mode

---

## 20. Concrete implementation task list

## 20.1 Database and schema

- [ ] Design canonical ERD
- [ ] Create `repo/postgres/schema.sql`
- [ ] Create migration files
- [ ] Add indexes and views
- [ ] Add import ledger tables

## 20.2 Next.js app

- [ ] Bootstrap Next.js app with App Router
- [ ] Add Drizzle/Postgres layer
- [ ] Add auth and trusted developer guards
- [ ] Add ticket APIs
- [ ] Add event ingestion APIs
- [ ] Add upload APIs
- [ ] Add dashboard pages
- [ ] Add admin pages

## 20.3 Worker

- [ ] Add pg-boss setup
- [ ] Implement Discord delivery worker
- [ ] Implement upload extraction worker
- [ ] Implement indexing worker
- [ ] Implement migration worker

## 20.4 Go CLI

- [ ] Add API client package
- [ ] Add auth login flow
- [ ] Replace local ticket source of truth with server APIs
- [ ] Keep `.repo/🎫/{{ticket-id}}` temp workspace logic
- [ ] Add upload-on-close flow
- [ ] Move event/log shipping to server
- [ ] Keep prompt files local

## 20.5 Migration

- [ ] Build SQLite import tool
- [ ] Build `.repo` history scanner
- [ ] Build format converters
- [ ] Build quarantine/drop ledger
- [ ] Build verification script

## 20.6 Deployment

- [ ] Write Dockerfiles
- [ ] Write `docker-compose.yml`
- [ ] Write `.env.example`
- [ ] Write VM deployment guide
- [ ] Add backup/restore scripts

## 20.7 Testing

- [ ] Unit tests for CLI
- [ ] Unit tests for server
- [ ] Integration tests with PostgreSQL
- [ ] E2E tests for dashboard + CLI
- [ ] Linux VM smoke test checklist execution

---

## 21. Recommended implementation order for the first engineering sprint

1. Freeze the target schema and API contract.
2. Build Next.js auth + trusted developer enforcement.
3. Build PostgreSQL schema and pg-boss worker.
4. Implement ticket open/close/reopen APIs.
5. Refactor Go CLI to use those APIs while preserving command UX.
6. Implement temp-folder upload and server-side artifact storage.
7. Implement event ingestion and Discord delivery.
8. Build migration tooling.
9. Run test matrix locally and in CI.
10. Perform Linux VM dress rehearsal.

---

## 22. Final recommendation

Do **not** attempt a big-bang rewrite with hidden migration logic.

Use this sequence instead:

- make PostgreSQL the canonical store,
- stand up the Next.js server and worker,
- refactor the Go CLI to talk to it,
- migrate history with a ledger and verification passes,
- cut over only after the Linux VM smoke test passes.

That gives you a system that satisfies all stated requirements while keeping the CLI in Go, moving durability to PostgreSQL, and making the public server safe for trusted-developer-only access.
